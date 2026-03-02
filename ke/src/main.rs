use std::{fs, path::PathBuf};

use anyhow::{bail, Result};
use clap::Parser;

mod paths;
mod runner;
mod yaml;

#[derive(Parser)]
#[command(name = "ke", about = "Half make")]
struct Cli {
    command: String,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = cli.config.unwrap_or(paths::default_config_path()?);

    if !config.exists() {
        bail!(
            "Config file not found. Create it at: {}\n\nExample:\n- folder: \
             /path/to/project\ncommands:\nbuild: cargo build\ntest: |\ncargo test\ncargo clippy",
            config.display()
        );
    }

    let yaml_str = fs::read_to_string(&config)
        .map_err(|e| anyhow::anyhow!("Could not read {}: {e}", config.display()))?;

    let folder = paths::current_dir()?;
    let command_str = yaml::find_command(&yaml_str, &folder, &cli.command)?;

    runner::run_command(&command_str)
}
