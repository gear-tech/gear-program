//! command update
use crate::result::Result;
use std::process::{self, Command};
use structopt::StructOpt;

/// Update resources
#[derive(Debug, StructOpt)]
pub struct Update {
    /// Update self
    #[structopt(short, long)]
    pub gear: bool,
}

impl Update {
    /// update self
    async fn update_self(&self) -> Result<()> {
        if !Command::new("cargo")
            .args(&["install", "gear-program"])
            .status()?
            .success()
        {
            process::exit(1);
        }

        Ok(())
    }

    /// exec command update
    pub async fn exec(&self) -> Result<()> {
        if self.gear {
            self.update_self().await?;
        }

        Ok(())
    }
}
