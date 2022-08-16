use crate::common::{logs::Logs, Error, Result};
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
};

pub const GEAR_NODE_BIN_PATH: &str = "/usr/local/bin/gear-node";
pub const GEAR_NODE_DOCKER_IMAGE: &str = "ghcr.io/gear-tech/node:latest";

/// Run gear-node with docker.
pub struct Node(Child);

impl Node {
    /// Run gear-node with docker in development mode.
    pub fn dev(ws: u16) -> Result<Self> {
        Ok(Self(
            Command::new("docker")
                .args(&[
                    "--rm",
                    "-p",
                    &format!("{}:9944", ws),
                    GEAR_NODE_DOCKER_IMAGE,
                    GEAR_NODE_BIN_PATH,
                    "--tmp",
                    "--dev",
                    "--unsafe-ws-external",
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?,
        ))
    }

    /// Spawn logs of gear-node.
    pub fn logs(&mut self) -> Result<Logs> {
        Ok(
            BufReader::new(self.0.stderr.take().ok_or(Error::EmptyStderr)?)
                .lines()
                .filter_map(|line| line.ok()),
        )
    }

    /// Wait for the block importing
    pub fn wait(&mut self, log: &str) -> Result<()> {
        for line in self.logs()? {
            if line.contains(log) {
                break;
            }
        }

        Ok(())
    }

    /// Kill self.
    pub fn kill(&mut self) -> Result<()> {
        self.0.kill()?;

        loop {
            match self.0.try_wait() {
                Ok(Some(_)) => break,
                _ => self.0.kill()?,
            }
        }

        Ok(())
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        self.kill().expect("Failed to kill gear-node");
    }
}
