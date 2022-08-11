use std::fs::File;

mod cmd;
mod common;

#[test]
fn check_spec_version() {
    use common::spec_version::{self, GEAR_NODE_SPEC_VERSION_PATTERN};

    let mut node = common::Node::dev(9999).expect("Failed to spawn gear-node.");

    for line in node.logs().expect("Failed to spwan logs of gear-node.") {
        if line.contains(GEAR_NODE_SPEC_VERSION_PATTERN) {
            let current_version = spec_version::find(
                &File::open("src/api/generated.rs").expect("genreated.rs not found."),
            )
            .expect("Failed to parse spec_version from generated.rs");

            let latest_version: u16 =
                spec_version::parse(&line).expect("Failed to parse spec version from log.");

            assert_eq!(current_version, latest_version);
            break;
        }
    }
}
