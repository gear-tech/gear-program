//! Command `send`
use crate::{
    api::{
        generated::api::{gear::calls::SendMessage, runtime_types::gear_core::ids::ProgramId},
        Api,
    },
    Result,
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Send {
    /// Send to
    pub destination: String,
    /// Send payload
    pub payload: String,
    /// Send gas limit
    pub gas_limit: u64,
    /// Send value
    pub value: u128,
}

impl Send {
    pub async fn exec(&self, api: Api) -> Result<()> {
        let mut destination = [0; 32];
        destination
            .copy_from_slice(&mut hex::decode(self.destination.trim_start_matches("0x"))?.as_ref());

        api.send_message(SendMessage {
            destination: ProgramId(destination),
            payload: hex::decode(self.payload.trim_start_matches("0x"))?,
            gas_limit: self.gas_limit,
            value: self.value,
        })
        .await?;

        Ok(())
    }
}
