//! command transfer
use crate::{
    api::{generated::api::balances::calls::Transfer as TransferCall, Api},
    keystore, Result,
};
use structopt::StructOpt;
use subxt::{sp_core::crypto::Ss58Codec, sp_runtime::AccountId32};

/// command transfer
#[derive(Debug, StructOpt)]
pub struct Transfer {
    /// gear node rpc endpoint
    #[structopt(short, long)]
    endpoint: Option<String>,
    /// password of the signer account
    #[structopt(short, long)]
    passwd: Option<String>,
    /// transfer to destination (ss58address)
    destination: String,
    /// balance will be transfered
    value: u128,
}

impl Transfer {
    /// execute command transfer
    pub async fn exec(&self) -> Result<()> {
        let passwd = self.passwd.as_ref().map(|s| s.as_ref());
        let pair = keystore::cache(passwd)?;
        let address = pair.account_id();

        let api = Api::new(self.endpoint.as_ref().map(|s| s.as_ref()), passwd).await?;
        let balance = api.get_balance(&address.to_ss58check()).await?;

        println!("Address: {address:?}");
        println!(
            "Current balance: {balance:?} ~= {} UINT",
            balance / 10u128.pow(12)
        );

        api.transfer(TransferCall {
            dest: AccountId32::from_ss58check(&self.destination)?.into(),
            value: self.value,
        })
        .await?;

        let balance = api.get_balance(&address.to_ss58check()).await?;
        println!(
            "Current balance: {balance:?} ~= {} UINT",
            balance / 10u128.pow(12)
        );

        Ok(())
    }
}
