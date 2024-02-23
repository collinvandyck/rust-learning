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
    let output = Command::new("lssdfkj").arg("/").output().await?;
    println!("output: {output:?}");
    Ok(())
}

async fn do_stuff() -> StdResult<Outputs, Error> {
    spawn_valid().await.map(Into::into)
}

async fn spawn_valid() -> StdResult<Output, Error> {
    Command::new("ls").arg("/").output().await
}

async fn spawn_invalid() -> StdResult<Output, Error> {
    Command::new("ls").arg("/").output().await
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Could not spawn command: {err}")]
    Spawn { cmd: Command, err: io::Error },
}

impl Error {
    fn command(&self) -> Command {
        match self {
            Error::Spawn { cmd, err } => cmd.clone(),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

struct Outputs(Vec<Output>);

impl From<Output> for Outputs {
    fn from(value: Output) -> Self {
        Outputs(vec![value])
    }
}

#[derive(Debug)]
struct Output {
    command: Command,
    inner: std::process::Output,
}

#[derive(Debug, Default, Clone)]
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
            .map_err(|err| Error::Spawn {
                cmd: self.clone(),
                err,
            })
    }
}
