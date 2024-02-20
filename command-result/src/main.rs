#![allow(unused, dead_code)]
use std::error::Error as StdErr;
use std::result::Result as StdResult;
use std::{
    io,
    path::{Path, PathBuf},
};

use anyhow::Context;

#[tokio::main]
async fn main() -> StdResult<(), Box<dyn StdErr>> {
    let output = Command::new("ls")
        .arg("/")
        .output()
        .await
        .context("command failed")?;
    println!("output: {output:?}");
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Could not spawn command: {err}")]
    Spawn { err: io::Error },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct Output {
    inner: std::process::Output,
}

#[derive(Default)]
struct Command {
    name: PathBuf,
    args: Vec<String>,
}

impl Command {
    fn new(name: impl AsRef<Path>) -> Self {
        Self {
            name: name.as_ref().to_path_buf(),
            ..Default::default()
        }
    }
    fn arg(&mut self, arg: impl AsRef<str>) -> &mut Self {
        self.args.push(arg.as_ref().to_string());
        self
    }

    async fn output(&mut self) -> Result<Output> {
        let mut cmd = tokio::process::Command::new(&self.name);
        for arg in &self.args {
            cmd.arg(&arg);
        }
        cmd.output()
            .await
            .map(|o| Output { inner: o })
            .map_err(|err| Error::Spawn { err })
    }
}
