use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

use anyhow::{bail, Result};

const WIN: bool = cfg!(target_os = "windows");
const SHELL: &str = if WIN { "powershell" } else { "bash" };
const SHELL_FLAG: &str = if WIN { "/C" } else { "-c" };

pub fn run_command(command_str: &str) -> Result<()> {
    let output = Command::new(SHELL)
        .arg(SHELL_FLAG)
        .arg(command_str)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if !output.status.success() {
        bail!("Command exited with status: {}", output.status);
    }

    Ok(())
}
