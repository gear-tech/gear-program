//! Common utils for integration tests
pub use node::Node;
use std::{io::Result, path::PathBuf, process::Command};

mod node;
pub mod spec_version;

/// Run binary `gear`
pub fn gear(args: &[&str]) -> Result<String> {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    Ok(String::from_utf8_lossy(
        &Command::new(PathBuf::from("target/".to_owned() + profile + "/gear"))
            .args(args)
            .output()?
            .stdout,
    )
    .to_string())
}
