//! command new
use crate::result::Result;
use structopt::StructOpt;

/// Create a new gear program
#[derive(Debug, StructOpt)]
pub struct New {
    /// Create gear program from templates
    pub template: Option<String>,
}

impl New {
    /// run command new
    pub async fn exec(&self) -> Result<()> {
        // let templates = templates()?;

        if let Some(template) = &self.template {
            crate::template::create(template)?;

            println!("Successfully created {}!", template);
            return Ok(());
        }

        // println!("AVAILABLE TEMPLATES: \n\t{}", templates.join("\n\t"));

        Ok(())
    }
}
