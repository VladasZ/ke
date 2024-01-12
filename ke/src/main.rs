use std::{
    env::args,
    fs::read_to_string,
    process::{Command, Stdio},
};

use anyhow::{anyhow, bail, Result};
pub use yaml_rust::{Yaml, YamlLoader};

const WIN: bool = cfg!(target_os = "windows");

const RUN: &str = if WIN { "powershell" } else { "bash" };
const C: &str = if WIN { "/C" } else { "-c" };

fn run_command(yaml: Yaml) -> Result<()> {
    let Yaml::String(command) = yaml else {
        bail!("Command is not a string")
    };

    Command::new(RUN)
        .arg(C)
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    Ok(())
}

fn run(yaml: &str, argument: &str) -> Result<()> {
    let yaml = YamlLoader::load_from_str(yaml)?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("No doc in yaml"))?;

    let Yaml::Hash(hash) = yaml else {
        bail!("Invalid yaml format")
    };

    for (command, val) in hash {
        let command = command.into_string().ok_or_else(|| anyhow!("Command is not a string"))?;

        if command == argument {
            return run_command(val);
        }
    }

    bail!("Command '{argument}' not found")
}

fn main() -> Result<()> {
    let yaml = read_to_string("ke.yaml")?;

    let Some(argument) = args().nth(1) else {
        println!("{yaml}");
        return Ok(());
    };

    run(&yaml, &argument)
}
