use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

/// we can just match `spec_version` since this is
/// auto-generated in `src/client/generated.rs`.
const SPEC_VERSION_FIELD: &str = "spec_version:";
const GEAR_NODE_BIN_PATH: &str = "/usr/local/bin/gear-node";
const GEAR_NODE_DOCKER_IMAGE: &str = "ghcr.io/gear-tech/node:latest";
const GEAR_NODE_SPEC_VERSION_PATTERN: &str = "Native runtime: gear-node-";

/// Find spec version from file.
fn find_spec_version(f: &File) -> Option<u16> {
    let parse_spec_version = |s: String| -> Option<u16> {
        s.split_whitespace()
            .last()?
            .trim_end_matches(',')
            .parse()
            .ok()
    };

    for maybe_line in BufReader::new(f).lines() {
        let line = maybe_line.ok()?;
        if !line.contains(SPEC_VERSION_FIELD) {
            continue;
        }

        if let Some(version) = parse_spec_version(line) {
            return Some(version);
        }
    }

    None
}

#[test]
fn check_spec_version() {
    let mut ps = Command::new("docker")
        .args(&[
            "run",
            "--rm",
            GEAR_NODE_DOCKER_IMAGE,
            GEAR_NODE_BIN_PATH,
            "--tmp",
            "--dev",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Spawn gear-node with docker failed.");

    let reader = BufReader::new(
        ps.stderr
            .take()
            .expect("No stderr from gear-node was found."),
    );

    for line in reader.lines().filter_map(|line| line.ok()) {
        if line.contains(GEAR_NODE_SPEC_VERSION_PATTERN) {
            let current_version = find_spec_version(
                &File::open("src/api/generated.rs").expect("genreated.rs not found."),
            )
            .expect("Failed to parse spec_version from generated.rs");
            let latest_version = line
                .split(GEAR_NODE_SPEC_VERSION_PATTERN)
                .collect::<Vec<_>>()[1]
                .split(|p: char| p.is_whitespace())
                .collect::<Vec<_>>()[0]
                .parse()
                .expect("Failed to parse spec_version");

            assert_eq!(current_version, latest_version);
            break;
        }
    }

    ps.kill().expect("Failed to kill gear-node.");
}
