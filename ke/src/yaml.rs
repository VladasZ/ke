use std::{fmt::Write, string::ToString};

use anyhow::{anyhow, bail, Result};
use yaml_rust::{Yaml, YamlLoader};

use crate::paths::{expand_tilde, tildify};

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

pub fn add_command(yaml_str: &str, folder: &str, name: &str, command: &str) -> Result<String> {
    let mut entries = if yaml_str.trim().is_empty() {
        vec![]
    } else {
        load_entries(yaml_str)?
    };

    if let Some(entry) = find_entry(&entries, folder) {
        let Yaml::Hash(map) = entry else { unreachable!() };
        if let Some(Yaml::Hash(commands_map)) = map.get(&Yaml::String("commands".to_string())) {
            if commands_map.contains_key(&Yaml::String(name.to_string())) {
                bail!("Command '{name}' already exists for folder '{folder}'");
            }
        }
    }

    let folder_key = Yaml::String("folder".to_string());
    let commands_key = Yaml::String("commands".to_string());

    if let Some(entry) = entries.iter_mut().find(|e| {
        let Yaml::Hash(map) = e else { return false };
        map.get(&Yaml::String("folder".to_string()))
            .and_then(|v| v.as_str())
            .and_then(|f| expand_tilde(f).ok())
            .is_some_and(|f| f == folder)
    }) {
        let Yaml::Hash(map) = entry else { unreachable!() };
        if let Some(Yaml::Hash(commands_map)) = map.get_mut(&commands_key) {
            commands_map.insert(Yaml::String(name.to_string()), Yaml::String(command.to_string()));
        } else {
            let mut commands_map = yaml_rust::yaml::Hash::new();
            commands_map.insert(Yaml::String(name.to_string()), Yaml::String(command.to_string()));
            map.insert(commands_key, Yaml::Hash(commands_map));
        }
    } else {
        let mut commands_map = yaml_rust::yaml::Hash::new();
        commands_map.insert(Yaml::String(name.to_string()), Yaml::String(command.to_string()));

        let mut entry_map = yaml_rust::yaml::Hash::new();
        entry_map.insert(folder_key, Yaml::String(folder.to_string()));
        entry_map.insert(commands_key, Yaml::Hash(commands_map));

        entries.push(Yaml::Hash(entry_map));
    }

    Ok(serialize_entries(&entries))
}

fn serialize_entries(entries: &[Yaml]) -> String {
    let mut out = String::new();

    for (i, entry) in entries.iter().enumerate() {
        let Yaml::Hash(map) = entry else { continue };

        if i > 0 {
            out.push('\n');
        }

        let folder = map
            .get(&Yaml::String("folder".to_string()))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let folder = tildify(folder).unwrap_or_else(|_| folder.to_string());
        writeln!(out, "- folder: {folder}").unwrap();
        out.push_str("  commands:\n");

        if let Some(Yaml::Hash(commands_map)) = map.get(&Yaml::String("commands".to_string())) {
            for (k, v) in commands_map {
                let name = k.as_str().unwrap_or("");
                let cmd = v.as_str().unwrap_or("");
                if cmd.contains('\n') {
                    writeln!(out, "    {name}: |").unwrap();
                    for line in cmd.lines() {
                        writeln!(out, "      {line}").unwrap();
                    }
                } else {
                    writeln!(out, "    {name}: {cmd}").unwrap();
                }
            }
        }
    }

    out
}
