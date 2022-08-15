//! command login
use crate::{metadata::Metadata, result::Result};
use std::{fs, path::PathBuf};
use structopt::StructOpt;

/// Action of command meta.
#[derive(Debug, StructOpt)]
pub enum Action {
    /// Display the structure of the metadata.
    Display,
}

/// Login to account
#[derive(Debug, StructOpt)]
pub struct Meta {
    /// Path of "*.meta.wasm".
    pub metadata: PathBuf,
    #[structopt(subcommand)]
    pub action: Action,
}

impl Meta {
    /// Run command meta.
    pub fn exec(&self) -> Result<()> {
        let wasm = fs::read(&self.metadata)?;
        let meta = Metadata::of(&wasm)?;

        match self.action {
            Action::Display => println!("{}", format!("{:#}", meta).replace('"', "")),
        }

        Ok(())
    }
}

// /// Creates a unique identifier by passing given argument to blake2b hash-function.
// fn hash(bin: &[u8]) -> [u8; 32] {
//     let mut arr: [u8; 32] = Default::default();
//
//     let blake2b_hash = blake2b::blake2b(32, &[], bin);
//     arr[..].copy_from_slice(blake2b_hash.as_bytes());
//
//     arr
// }
