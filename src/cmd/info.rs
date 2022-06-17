//! command transfer
use crate::{api::Api, Result};
use structopt::StructOpt;

/// Get account info of ss58address
#[derive(Debug, StructOpt)]
pub struct Info {
    /// gear node rpc endpoint
    #[structopt(short, long)]
    endpoint: Option<String>,
    /// password of the signer account
    #[structopt(short, long)]
    passwd: Option<String>,
    /// Get info of this address (ss58address)
    address: String,
}

impl Info {
    /// execute command transfer
    pub async fn exec(&self) -> Result<()> {
        let passwd = self.passwd.as_ref().map(|s| s.as_ref());

        let api = Api::new(self.endpoint.as_ref().map(|s| s.as_ref()), passwd).await?;
        let info = api.info(&self.address).await?;

        println!("{info:#?}");

        Ok(())
    }
}