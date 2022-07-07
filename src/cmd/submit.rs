//! Command `submit`
use crate::{
    api::{generated::api::gear::calls::SubmitCode, Api},
    Result,
};
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Submit {
    /// gear program code <*.wasm>
    code: PathBuf,
}

impl Submit {
    pub async fn exec(&self, api: Api) -> Result<()> {
        api.submit_code(SubmitCode {
            code: fs::read(&self.code)?,
        })
        .await?;

        Ok(())
    }
}
