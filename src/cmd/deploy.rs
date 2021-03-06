//! command submit
use crate::{
    api::{
        generated::api::gear::{calls::SubmitProgram, Event as GearEvent},
        Api,
    },
    Result,
};
use std::{fs, path::PathBuf};
use structopt::StructOpt;

/// Deploy program to gear node
#[derive(StructOpt, Debug)]
pub struct Deploy {
    /// gear program code <*.wasm>
    code: PathBuf,
    /// gear program salt ( hex encoding )
    #[structopt(default_value = "0x00")]
    salt: String,
    /// gear program init payload ( hex encoding )
    #[structopt(default_value = "0x00")]
    init_payload: String,
    /// gear program gas limit
    ///
    /// if zero, gear will estimate this automatically
    #[structopt(default_value = "0")]
    gas_limit: u64,
    /// gear program balance
    #[structopt(default_value = "0")]
    value: u128,
}

impl Deploy {
    /// Exec command submit
    pub async fn exec(&self, api: Api) -> Result<()> {
        let events = api.events().await?;

        tokio::try_join!(
            self.submit_program(&api),
            Api::wait_for(events, |event| {
                if let GearEvent::MessageEnqueued { .. } = event {
                    true
                } else {
                    false
                }
            })
        )?;

        Ok(())
    }

    async fn submit_program(&self, api: &Api) -> Result<()> {
        let gas = if self.gas_limit == 0 {
            api.get_init_gas_spent(
                fs::read(&self.code)?.into(),
                hex::decode(&self.init_payload.trim_start_matches("0x"))?.into(),
                0,
                None,
            )
            .await?
            .min_limit
        } else {
            self.gas_limit
        };

        // estimate gas
        let gas_limit = api.cmp_gas_limit(gas).await?;

        // submit program
        api.submit_program(SubmitProgram {
            code: fs::read(&self.code)?,
            salt: hex::decode(&self.salt.trim_start_matches("0x"))?,
            init_payload: hex::decode(&self.init_payload.trim_start_matches("0x"))?,
            gas_limit,
            value: self.value,
        })
        .await?;

        Ok(())
    }
}
