use std::process::{Command, Stdio};

use anyhow::{bail, Result};

const WIN: bool = cfg!(target_os = "windows");
const SHELL: &str = if WIN { "powershell" } else { "bash" };
const SHELL_FLAG: &str = if WIN { "/C" } else { "-c" };

pub fn run_command(command_str: &str) -> Result<()> {
    let status = Command::new(SHELL)
        .arg(SHELL_FLAG)
        .arg(command_str)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        bail!("Command exited with status: {status}");
    }

    Ok(())
}
