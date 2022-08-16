//! Common utils for integration tests
pub use self::{
    node::Node,
    result::{Error, Result},
};
use std::{
    path::PathBuf,
    process::{Command, Output},
};

pub mod logs;
mod node;
mod result;
pub mod spec_version;
pub mod traits;

/// Run binary `gear`
pub fn gear(args: &[&str]) -> Result<Output> {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };

    Ok(
        Command::new(PathBuf::from("target/".to_owned() + profile + "/gear"))
            .args(args)
            .output()?,
    )
}

/// Login as //Alice
pub fn login_as_alice() -> Result<()> {
    let _ = gear(&["login", "//Alice"])?;

    Ok(())
}

/// Init env logger
#[allow(dead_code)]
pub fn init_logger() {
    let _ = env_logger::builder().is_test(true).try_init();
}
