#![cfg(test)]

use std::{
    process::{Command, Stdio},
    sync::Mutex,
};

use anyhow::{bail, Result};

static CLI_BUILT: Mutex<bool> = Mutex::new(false);

fn root_dir() -> Result<String> {
    let output = Command::new("git").args(["rev-parse", "--show-toplevel"]).output()?;
    assert!(output.status.success(), "Failed to get Git repository root path");
    let git_root = String::from_utf8_lossy(&output.stdout).trim_end_matches('\n').to_string();

    Ok(git_root)
}

pub fn call_cli(arg: &str) -> Result<(String, String)> {
    let output = Command::new("./target/debug/ke")
        .arg(arg)
        .current_dir(root_dir()?)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    Ok((
        String::from_utf8_lossy(output.stdout.as_slice()).to_string(),
        String::from_utf8_lossy(output.stderr.as_slice()).to_string(),
    ))
}

pub fn build_cli() -> Result<()> {
    let mut ready = CLI_BUILT.lock().unwrap();

    if *ready {
        return Ok(());
    }

    let output = Command::new("cargo")
        .arg("build")
        .arg("-p")
        .arg("ke")
        .current_dir(root_dir()?)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!("Failed to build contract. Output: {output:?}");
    }

    *ready = true;

    Ok(())
}
