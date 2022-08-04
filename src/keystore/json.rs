use crate::result::{Error, Result};
use scrypt::Params;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use subxt::sp_core::{crypto::Ss58Codec, sr25519, Pair};

const NONCE_LENGTH: usize = 24;
const SCRYPT_LENGTH: usize = 32 + (3 * 4);
const PKCS8_DIVIDER: [u8; 5] = [161, 35, 3, 33, 0];
const PKCS8_HEADER: [u8; 16] = [48, 83, 2, 1, 1, 48, 5, 6, 3, 43, 101, 112, 4, 34, 4, 32];
const SEED_OFFSET: usize = 16;
const PUB_LENGTH: usize = 32;
const SALT_LENGTH: usize = 32;
const SEC_LENGTH: usize = 64;
const SEED_LENGTH: usize = 32;
const DIV_OFFSET: usize = 80;
const PUB_OFFSET: usize = 96;

/// JSON keypair encoding.
///
/// # Example
///
/// ```json
/// "encoding": {
///     "content": [
///         "pkcs8",
///         "sr25519"
///     ],
///     "type": [
///         "scrypt",
///         "xsalsa20-poly1305"
///     ],
///     "version": "3"
/// },
/// ```
#[derive(Serialize, Deserialize)]
pub struct EncryptedEncoding {
    pub content: [String; 2],
    pub r#type: [String; 2],
    pub version: String,
}

/// JSON keypair meta.
///
/// # Example
///
/// ```json
/// "meta": {
///     "genesisHash": "",
///     "name": "GEAR",
///     "whenCreated": 1659544420591
///  }
/// ```
#[derive(Serialize, Deserialize)]
pub struct EncryptedMeta {
    #[serde(rename(deserialize = "genesisHash"))]
    pub genesis_hash: String,
    pub name: String,
    #[serde(rename(deserialize = "whenCreated"))]
    pub when_created: u64,
}

/// Json keypair.
///
/// # Example
///
/// ```json
/// {
///     "encoded": "X/sAaS3pNejnqvbHk0lne8tcXXmTu2gPQgXvtbf3azgAgAAAAQAAAAgAAABxGGfnP+9PCbP7Gp0+7jxxl8twTthzIq4pLfC0m6NvA8hk557A4dkDapszVKhlyDhTvnQQE2WwhqzkfDwvq0XtFl9PDW6ShvVM/lSVLkZTF6QGnTzRZ2dwT7+X5v+gjFIJftI5z3vLFg7NM+NXy7kxU039iooVTxYDqzCnMSjXMBtnY2cqNedlGUcrbDGE0lNdWqu3MWT9J27kmysC",
///     "encoding": {
///         "content": [
///             "pkcs8",
///             "sr25519"
///         ],
///         "type": [
///             "scrypt",
///             "xsalsa20-poly1305"
///         ],
///         "version": "3"
///     },
///     "address": "5Hax9tpSjfiX1nYrqhFf8F3sLiaa2ZfPv2VeDQzPBLzKNjRq",
///     "meta": {
///         "genesisHash": "",
///         "name": "GEAR",
///         "whenCreated": 1659544420591
///     }
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct Encrypted {
    pub encoded: String,
    pub encoding: EncryptedEncoding,
    pub address: String,
    pub meta: EncryptedMeta,
}

impl Encrypted {
    fn decoded(&self) -> Result<Vec<u8>> {
        Ok(base64::decode(&self.encoded)?)
    }

    fn decrypt(&self, passphrase: &str) -> Result<Vec<u8>> {
        assert_eq!(
            self.encoding.r#type.to_owned(),
            ["scrypt", "xsalsa20-poly1305"].to_owned()
        );
        let decoded = self.decoded()?;
        let password = scrypt_from_slice(passphrase.as_bytes(), &decoded)?;
        let encrypted = &decoded[SCRYPT_LENGTH..];

        Ok(nacl::secret_box::open(
            &encrypted[NONCE_LENGTH..],
            &encrypted[..NONCE_LENGTH],
            &password,
        )?)
    }

    fn create(self, passphrase: String) -> Result<()> {
        assert!(
            self.encoding.version != "3".to_string() || self.encoding.content[0] == "pkcs8",
            "Unable to decode non-pkcs8 type, [{}] found",
            self.encoding.content.join(",")
        );

        assert_eq!(
            self.encoding.content[1],
            "sr25519".to_string(),
            "Only supports sr25519 for now."
        );

        let decrypted = self.decrypt(&passphrase)?;
        assert_eq!(
            &decrypted[0..PKCS8_HEADER.len()],
            &PKCS8_HEADER,
            "Invalid Pkcs8 header found in body"
        );

        let secret_key = &decrypted[SEED_OFFSET..(SEED_OFFSET + SEC_LENGTH)];
        let divider = &decrypted[DIV_OFFSET..(DIV_OFFSET + PKCS8_DIVIDER.len())];
        assert_eq!(
            divider, PKCS8_DIVIDER,
            "Invalid Pkcs8 divider found in body"
        );

        let public_key = &decrypted[PUB_OFFSET..(PUB_OFFSET + PUB_LENGTH)];
        let public = sr25519::Public::from_ss58check(&self.address)?;
        assert_eq!(public_key, public.0);

        Ok(())
    }
}

/// Get password with scrypt.
///
/// # TODO
///
/// Check the params.
///
/// ```typescript
/// export const DEFAULT_PARAMS = {
///     N: 1 << 15,
///     p: 1,
///     r: 8
/// };
/// ```
/// https://github.com/polkadot-js/common/blob/master/packages/util-crypto/src/scrypt/defaults.ts
pub fn scrypt_from_slice(passphrase: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    let mut salt = [0; 32];
    salt.copy_from_slice(&data[..SALT_LENGTH]);

    let mut password: Vec<u8> = vec![];
    scrypt::scrypt(passphrase, &salt, &Params::new(15, 1, 8)?, &mut password);

    Ok(password)
}

// /// Decode pair from json file.
// ///
// /// @WARNING: THIS WILL ONLY BE SECURE IF THE keystore IS SECURE.
// /// when you have NO PASSWORD, If it can be got by an attacker then
// /// they can also get your key.
// pub fn decode_json_file(p: PathBuf, passphrase: Option<String>) -> Result<()> {
//     let json = serde_json::from_slice::<Encrypted>(&fs::read(p)?)?;
//
//     Ok(())
// }

#[test]
fn test_can_create_pair_from_json_file() {
    let root = env!("CARGO_MANIFEST_DIR");
    let json =
        fs::read(PathBuf::from(root).join("res/pair.json")).expect("Read res/pair.json failed.");

    let encrypted = serde_json::from_slice::<Encrypted>(&json).expect("Parse json pair failed.");

    encrypted.create("000000".to_string()).unwrap();
}
