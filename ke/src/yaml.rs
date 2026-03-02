use std::string::ToString;

use anyhow::{anyhow, bail, Result};
use yaml_rust::{Yaml, YamlLoader};

use crate::paths::expand_tilde;

fn load_entries(yaml_str: &str) -> Result<Vec<Yaml>> {
    let doc = YamlLoader::load_from_str(yaml_str)?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("Empty commands.yaml"))?;

    match doc {
        Yaml::Array(arr) => Ok(arr),
        _ => bail!("commands.yaml must be a list of entries"),
    }
}

fn find_entry<'a>(entries: &'a [Yaml], folder: &str) -> Option<&'a Yaml> {
    entries.iter().find(|e| {
        let Yaml::Hash(map) = e else { return false };
        map.get(&Yaml::String("folder".to_string()))
            .and_then(|v| v.as_str())
            .and_then(|f| expand_tilde(f).ok())
            .is_some_and(|f| f == folder)
    })
}

pub fn find_command(yaml_str: &str, folder: &str, command_name: &str) -> Result<String> {
    let entries = load_entries(yaml_str)?;

    let entry = find_entry(&entries, folder)
        .ok_or_else(|| anyhow!("No entry found for folder '{folder}' in commands.yaml"))?;

    let Yaml::Hash(map) = entry else { unreachable!() };

    let commands = map
        .get(&Yaml::String("commands".to_string()))
        .ok_or_else(|| anyhow!("Entry for folder '{folder}' has no 'commands' key"))?;

    let Yaml::Hash(commands_map) = commands else {
        bail!("'commands' in folder '{folder}' must be a map");
    };

    commands_map
        .get(&Yaml::String(command_name.to_string()))
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
        .ok_or_else(|| anyhow!("Command '{command_name}' not found for folder '{folder}'"))
}
