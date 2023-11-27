use std::{
    error::Error,
    fmt::{Debug, Display},
    io::{self, BufRead, BufReader},
    process::Command,
};

#[allow(dead_code)]
enum SysctlError {
    SysctlFailed { stdout: Vec<u8>, stderr: Vec<u8> },
    IO(io::Error),
}

impl Debug for SysctlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SysctlError::SysctlFailed { .. } => write!(f, "sysctl failed"),
            SysctlError::IO(err) => write!(f, "io error: {err}"),
        }
    }
}

impl Display for SysctlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Error for SysctlError {}

fn main() -> Result<(), Box<dyn Error>> {
    let output = Command::new("sysctl").arg("-a").output()?;
    if !output.status.success() {
        return Err(SysctlError::SysctlFailed {
            stdout: output.stdout,
            stderr: output.stderr,
        }
        .into());
    }
    let stdout = BufReader::new(output.stdout.as_slice());
    for line in stdout
        .lines()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| SysctlError::IO(err))?
    {
        println!("Line: {line}");
    }

    Ok(())
}
