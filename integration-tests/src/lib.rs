#![cfg(test)]

mod utils;

use anyhow::Result;

use crate::utils::{build_cli, call_cli_with_config, root_dir};

#[test]
fn hello_world() -> Result<()> {
    build_cli()?;

    let root = root_dir()?;
    let config = format!("{root}/commands.example.yaml");

    let (stdout, _stderr) = call_cli_with_config("hello", &root, &config)?;

    assert!(stdout.to_lowercase().contains("hello world"));

    Ok(())
}
