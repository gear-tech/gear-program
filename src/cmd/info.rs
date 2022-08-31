//! command transfer
use crate::{api::signer::Signer, result::Result};
use structopt::StructOpt;
use subxt::sp_core::{crypto::Ss58Codec, sr25519::Pair, Pair as PairT};

#[derive(Debug, StructOpt)]
pub enum Action {
    Balance,
}

impl Action {
    /// run action
    pub async fn exec(&self, signer: Signer, address: &str) -> Result<()> {
        match self {
            Action::Balance => Self::balance(signer, address).await,
        }
    }

    /// Get balance of address
    pub async fn balance(signer: Signer, address: &str) -> Result<()> {
        let address = if address.starts_with("//") {
            Pair::from_string(&address, None)
                .expect("Parse development address failed")
                .public()
                .to_ss58check()
        } else {
            address.into()
        };

        let info = signer.info(&address).await?;

        println!("{info:#?}");

        Ok(())
    }
}

/// Get account info from ss58address.
#[derive(Debug, StructOpt)]
pub struct Info {
    /// Info of this address
    pub address: Option<String>,

    /// Info of balance, mailbox, etc.
    #[structopt(subcommand)]
    pub action: Action,
}

impl Info {
    /// execute command transfer
    pub async fn exec(&self, signer: Signer) -> Result<()> {
        let address = self.address.clone().unwrap_or(signer.address());

        self.action.exec(signer, &address).await
    }
}
