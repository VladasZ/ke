#![cfg(test)]

mod utils;

use anyhow::Result;

use crate::utils::{build_cli, call_cli};

#[test]
fn simple_commands() -> Result<()> {
    build_cli()?;

    assert_eq!(call_cli("hello")?.0, "hello\n");
    assert_eq!(call_cli("bye")?.0, "bye\n");
    assert_eq!(call_cli("path")?.0, "hello\n");

    Ok(())
}

#[test]
fn wrong_command() -> Result<()> {
    build_cli()?;

    assert!(call_cli("a")?.1.starts_with("Error: Command 'a' not found"));

    Ok(())
}
