mod utils;

use anyhow::Result;

use crate::utils::{build_cli, call_cli};

#[test]
fn test() -> Result<()> {
    build_cli()?;

    call_cli("a")?;

    dbg!("A");

    Ok(())
}

#[test]
fn wrong_command() -> Result<()> {
    build_cli()?;

    let (_, err) = call_cli("a")?;

    assert!(err.starts_with("Error: Command 'a' not found"));

    Ok(())
}
