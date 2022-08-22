//! command key
use crate::{
    keystore::{key::Key as KeyT, node},
    result::Result,
};
use std::{fmt::Display, result::Result as StdResult, str::FromStr};
use structopt::StructOpt;
use subxt::{
    sp_core::{ecdsa, ed25519, sr25519, Pair},
    sp_runtime::traits::IdentifyAccount,
};

/// Cryptography scheme
#[derive(Debug)]
pub enum Scheme {
    Ecdsa,
    Ed25519,
    Sr25519,
}

impl FromStr for Scheme {
    type Err = &'static str;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "ecdsa" => Scheme::Ecdsa,
            "ed25519" => Scheme::Ed25519,
            _ => Scheme::Sr25519,
        })
    }
}

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Generate a random account
    Generate,

    /// Generate a random node libp2p key
    #[structopt(name = "generate-node-key")]
    GenerateNodeKey,

    /// Gets a public key and a SS58 address from the provided Secret URI
    Inspect {
        /// Secret uri, if none, will get keypair from cache.
        suri: String,
    },

    /// Print the peer ID corresponding to the node key in the given file
    InspectNodeKey {
        /// Hex encoding of the secret key
        secret: String,
    },

    /// Sign a message, with a given (secret) key
    Sign {
        /// Secret uri, if none, will get keypair from cache
        suri: Option<String>,
        /// Message to sign
        message: String,
    },

    /// Verify a signature for a message
    Verify {
        /// Signature to verify
        signature: String,
        /// Raw message
        message: String,
        /// Public key of the signer of this signature, if none,
        /// will get keypair from cache
        pubkey: Option<String>,
    },
}

/// Keypair utils
#[derive(Debug, StructOpt)]
pub struct Key {
    /// Cryptography scheme
    #[structopt(short, long, default_value = "sr25519")]
    scheme: Scheme,
    /// Key actions
    #[structopt(subcommand)]
    action: Action,
}

macro_rules! match_scheme {
    ($scheme:expr, $op:tt, ($($arg:ident),*), $res:ident, $repeat:expr) => {
        match $scheme {
            Scheme::Ecdsa => {
                let $res = KeyT::$op::<ecdsa::Pair>($($arg),*)?;
                $repeat
            }
            Scheme::Ed25519 => {
                let $res = KeyT::$op::<ed25519::Pair>($($arg),*)?;
                $repeat
            }
            Scheme::Sr25519 => {
                let $res = KeyT::$op::<sr25519::Pair>($($arg),*)?;
                $repeat
            }
        }
    };
}

impl Key {
    pub fn exec(&self, passwd: Option<&str>) -> Result<()> {
        match &self.action {
            Action::Generate => self.generate(passwd)?,
            Action::GenerateNodeKey => Self::generate_node_key(),
            Action::Inspect { suri } => self.inspect(&suri, passwd)?,
            Action::InspectNodeKey { secret } => Self::inspect_node_key(secret)?,
            _ => {}
        }

        Ok(())
    }

    fn generate(&self, passwd: Option<&str>) -> Result<()> {
        match_scheme!(self.scheme, generate_with_phrase, (passwd), res, {
            let (pair, phrase) = res;
            let signer = pair.signer();

            Self::info(&format!("Secret Phrase `{}`", phrase), signer);
        });

        Ok(())
    }

    fn generate_node_key() {
        let key = node::generate();

        println!("Secret:  0x{}", hex::encode(key.0.secret().as_ref()));
        println!("Peer ID: {}", key.1)
    }

    fn info<P>(title: &str, signer: &P)
    where
        P: Pair,
        <P as Pair>::Public: IdentifyAccount,
        <<P as Pair>::Public as IdentifyAccount>::AccountId: Display,
    {
        println!("{title} is account:");
        println!(
            "\tSecret Seed:  0x{}",
            hex::encode(&signer.to_raw_vec()[..32])
        );
        println!("\tPublic key:   0x{}", hex::encode(signer.public()));
        println!("\tSS58 Address: {}", signer.public().into_account());
    }

    fn inspect(&self, suri: &str, passwd: Option<&str>) -> Result<()> {
        let key = KeyT::from_string(suri);
        let key_ref = &key;
        match_scheme!(self.scheme, pair, (key_ref, passwd), pair, {
            Self::info(&format!("Secret Key URI `{}`", suri), pair.signer())
        });

        Ok(())
    }

    fn inspect_node_key(secret: &str) -> Result<()> {
        let data = hex::decode(secret.trim_start_matches("0x"))?;
        let key = node::inspect(data)?;

        println!("Peer ID: {}", key.1);
        Ok(())
    }
}
