use std::{
    env::args,
    fs::read_to_string,
    process::{Command, Stdio},
};

use anyhow::{anyhow, bail, Result};
pub use yaml_rust::{Yaml, YamlLoader};

const WIN: bool = cfg!(target_os = "windows");

const RUN: &str = if WIN { "cmd" } else { "bash" };
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

fn main() -> Result<()> {
    let argument = args().nth(1).ok_or_else(|| anyhow!("No argument"))?;

    let string = read_to_string("ke.yaml")?;

    let yaml = YamlLoader::load_from_str(&string)?;

    let yaml = yaml.into_iter().next().ok_or_else(|| anyhow!("No doc in yaml"))?;

    let Yaml::Hash(hash) = yaml else {
        bail!("Invalid yaml format")
    };

    for (command, val) in hash.into_iter() {
        let command = command.into_string().ok_or_else(|| anyhow!("Command is not a string"))?;

        if command == argument {
            return run_command(val);
        }
    }

    bail!("Command '{argument}' not found")
}
