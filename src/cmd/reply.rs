//! Command `reply`
use crate::{
    api::{
        generated::api::{gear::calls::SendReply, runtime_types::gear_core::ids::MessageId},
        Api,
    },
    Result,
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Reply {
    /// Reply to
    pub reply_to_id: String,
    /// Reply payload
    payload: String,
    /// Reply gas limit
    gas_limit: u64,
    /// Reply value
    value: u128,
}

impl Reply {
    pub async fn exec(&self, api: Api) -> Result<()> {
        let mut reply_to_id = [0; 32];
        reply_to_id
            .copy_from_slice(&mut hex::decode(self.reply_to_id.trim_start_matches("0x"))?.as_ref());

        api.send_reply(SendReply {
            reply_to_id: MessageId(reply_to_id),
            payload: hex::decode(self.payload.trim_start_matches("0x"))?,
            gas_limit: self.gas_limit,
            value: self.value,
        })
        .await?;

        Ok(())
    }
}
