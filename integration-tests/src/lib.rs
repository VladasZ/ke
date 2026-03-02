#![cfg(test)]

mod utils;

use std::fs;

use anyhow::Result;

use crate::utils::{build_cli, call_cli, root_dir};

fn temp_config() -> String {
    let root = root_dir().unwrap();
    format!("{root}/target/test-commands.yaml")
}

#[test]
fn hello_world() -> Result<()> {
    build_cli()?;

    let root = root_dir()?;
    let config = format!("{root}/commands.example.yaml");

    let (stdout, _stderr) = call_cli(&["hello"], &root, &config)?;

    assert!(stdout.to_lowercase().contains("hello world"));

    Ok(())
}

#[test]
fn add_command_creates_and_runs() -> Result<()> {
    build_cli()?;

    let root = root_dir()?;
    let config = temp_config();

    let _ = fs::remove_file(&config);

    let (_stdout, stderr) = call_cli(&["--add", "greet", "echo", "hi there"], &root, &config)?;
    assert!(stderr.is_empty(), "unexpected stderr: {stderr}");

    let config_contents = fs::read_to_string(&config)?;
    assert!(config_contents.contains("greet"));
    assert!(config_contents.contains("echo hi there"));

    let (stdout, _stderr) = call_cli(&["greet"], &root, &config)?;
    assert!(stdout.contains("hi there"));

    let _ = fs::remove_file(&config);

    Ok(())
}

#[test]
fn add_command_duplicate_errors() -> Result<()> {
    build_cli()?;

    let root = root_dir()?;
    let config = temp_config();

    let _ = fs::remove_file(&config);

    call_cli(&["--add", "greet", "echo", "hi there"], &root, &config)?;

    let (_stdout, stderr) = call_cli(&["--add", "greet", "echo", "hi again"], &root, &config)?;
    assert!(stderr.contains("already exists"));

    let _ = fs::remove_file(&config);

    Ok(())
}
