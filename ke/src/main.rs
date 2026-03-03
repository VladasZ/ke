use std::{fs, path::PathBuf, process};

use anyhow::{bail, Result};
use clap::Parser;

const DEFAULT_CONFIG: &str = "
- global:
    hello: echo Hello world

- folder: ~/
  commands:
    greet: |
      echo Hello
      echo World
";

mod paths;
mod runner;
mod yaml;

#[derive(Parser)]
#[command(name = "ke", about = "Half make")]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, value_names = ["NAME", "CMD"], num_args = 2..)]
    add: Option<Vec<String>>,

    #[arg(long = "add-global", value_names = ["NAME", "CMD"], num_args = 2..)]
    add_global: Option<Vec<String>>,

    #[arg(short, long)]
    edit: bool,

    command: Option<String>,
}

fn ensure_default_config(config: &PathBuf) -> Result<()> {
    let is_empty =
        !config.exists() || fs::read_to_string(config).map(|s| s.trim().is_empty()).unwrap_or(true);

    if is_empty {
        if let Some(parent) = config.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(config, DEFAULT_CONFIG)?;
    }

    Ok(())
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let is_default_config = cli.config.is_none();
    let config = cli.config.unwrap_or(paths::default_config_path()?);

    if is_default_config {
        ensure_default_config(&config)?;
    }
    let folder = paths::current_dir()?;

    if cli.edit {
        if let Some(parent) = config.parent() {
            fs::create_dir_all(parent)?;
        }
        if !config.exists() {
            fs::write(&config, "")?;
        }
        open::that(&config)?;
        return Ok(());
    }

    if let Some(args) = cli.add_global {
        let (name, cmd) = args.split_first().unwrap();
        let command_str = cmd.join(" ");

        let yaml_str = if config.exists() {
            fs::read_to_string(&config)
                .map_err(|e| anyhow::anyhow!("Could not read {}: {e}", config.display()))?
        } else {
            String::new()
        };

        let updated = yaml::add_global_command(&yaml_str, name, &command_str)?;

        if let Some(parent) = config.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&config, updated)?;
        println!("Added global command '{name}'");
        return Ok(());
    }

    if let Some(args) = cli.add {
        let (name, cmd) = args.split_first().unwrap();
        let command_str = cmd.join(" ");

        let yaml_str = if config.exists() {
            fs::read_to_string(&config)
                .map_err(|e| anyhow::anyhow!("Could not read {}: {e}", config.display()))?
        } else {
            String::new()
        };

        let updated = yaml::add_command(&yaml_str, &folder, name, &command_str)?;

        if let Some(parent) = config.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&config, updated)?;
        println!("Added '{name}' to '{folder}'");
        return Ok(());
    }

    let Some(command) = cli.command else {
        bail!("Usage: ke <command>  or  ke --add <name> <cmd>");
    };

    let yaml_str = fs::read_to_string(&config)
        .map_err(|e| anyhow::anyhow!("Could not read {}: {e}", config.display()))?;

    let command_str = yaml::find_command(&yaml_str, &folder, &command)?;

    runner::run_command(&command_str)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        process::exit(1);
    }
}
