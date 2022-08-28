use std::{
    env::current_dir,
    fs::File,
    io::Write,
    process::{Command, ExitStatus, Output},
};

use anyhow::{anyhow, Error as AnyhowError};
use thiserror::Error;

const ASSEMBLE_FILE: &str = "tmp.s";
const BINARY_FILE: &str = "tmp";

#[derive(Debug, Error)]
#[error("{}", String::from_utf8_lossy(stderr))]
struct CommandError {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl From<Output> for CommandError {
    fn from(o: Output) -> Self {
        Self {
            status: o.status,
            stdout: o.stdout,
            stderr: o.stderr,
        }
    }
}

fn compile(input: &str, output: &str) -> Result<(), AnyhowError> {
    let o = Command::new("cargo").args(&["run", input]).output()?;
    if !o.status.success() {
        return Err(anyhow!(CommandError::from(o)));
    }

    let mut f = File::create(output)?;
    f.write_all(&o.stdout)?;

    Ok(())
}

fn assemble(input: &str, output: &str) -> Result<(), AnyhowError> {
    let o = Command::new("cc").args(&["-o", output, input]).output()?;

    if !o.status.success() {
        return Err(anyhow!(CommandError::from(o)));
    }

    Ok(())
}

fn execute(input: &str) -> Result<i32, AnyhowError> {
    compile(input, ASSEMBLE_FILE)?;
    assemble(ASSEMBLE_FILE, BINARY_FILE)?;

    let output = Command::new(current_dir()?.join(BINARY_FILE)).output()?;
    let status_code = output.status.code().unwrap_or_default();

    Ok(status_code)
}

#[test]
fn test() {
    assert_eq!(execute("1").unwrap(), 1);
    assert_eq!(execute("42").unwrap(), 42);
    assert_eq!(execute("5+20-4").unwrap(), 21);
    assert_eq!(execute("5+6*7").unwrap(), 47);
    assert_eq!(execute("5*(9-6)").unwrap(), 15);
    assert_eq!(execute("(3+5)/2").unwrap(), 4);
}
