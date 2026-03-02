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

fn is_global(entry: &Yaml) -> bool {
    let Yaml::Hash(map) = entry else { return false };
    map.contains_key(&Yaml::String("global".to_string()))
        && !map.contains_key(&Yaml::String("folder".to_string()))
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

fn find_global_entry(entries: &[Yaml]) -> Option<&Yaml> {
    entries.iter().find(|e| is_global(e))
}

fn get_command_from_entry(entry: &Yaml, command_name: &str) -> Option<String> {
    let Yaml::Hash(map) = entry else { return None };
    let commands_map = if is_global(entry) {
        map.get(&Yaml::String("global".to_string()))?
    } else {
        map.get(&Yaml::String("commands".to_string()))?
    };
    let Yaml::Hash(commands_map) = commands_map else {
        return None;
    };
    commands_map
        .get(&Yaml::String(command_name.to_string()))
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
}

pub fn find_command(yaml_str: &str, folder: &str, command_name: &str) -> Result<String> {
    let entries = load_entries(yaml_str)?;

    if let Some(entry) = find_entry(&entries, folder) {
        if let Some(cmd) = get_command_from_entry(entry, command_name) {
            return Ok(cmd);
        }
    }

    if let Some(entry) = find_global_entry(&entries) {
        if let Some(cmd) = get_command_from_entry(entry, command_name) {
            return Ok(cmd);
        }
    }

    bail!("Command '{command_name}' not found for folder '{folder}' and not in global commands")
}

fn insert_into_commands(map: &mut yaml_rust::yaml::Hash, key: &Yaml, name: &str, command: &str) {
    if let Some(Yaml::Hash(commands_map)) = map.get_mut(key) {
        commands_map.insert(Yaml::String(name.to_string()), Yaml::String(command.to_string()));
    } else {
        let mut commands_map = yaml_rust::yaml::Hash::new();
        commands_map.insert(Yaml::String(name.to_string()), Yaml::String(command.to_string()));
        map.insert(key.clone(), Yaml::Hash(commands_map));
    }
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

    let commands_key = Yaml::String("commands".to_string());

    if let Some(entry) = entries.iter_mut().find(|e| {
        let Yaml::Hash(map) = e else { return false };
        map.get(&Yaml::String("folder".to_string()))
            .and_then(|v| v.as_str())
            .and_then(|f| expand_tilde(f).ok())
            .is_some_and(|f| f == folder)
    }) {
        let Yaml::Hash(map) = entry else { unreachable!() };
        insert_into_commands(map, &commands_key, name, command);
    } else {
        let mut commands_map = yaml_rust::yaml::Hash::new();
        commands_map.insert(Yaml::String(name.to_string()), Yaml::String(command.to_string()));

        let mut entry_map = yaml_rust::yaml::Hash::new();
        entry_map.insert(
            Yaml::String("folder".to_string()),
            Yaml::String(folder.to_string()),
        );
        entry_map.insert(commands_key, Yaml::Hash(commands_map));

        entries.push(Yaml::Hash(entry_map));
    }

    Ok(serialize_entries(&entries))
}

pub fn add_global_command(yaml_str: &str, name: &str, command: &str) -> Result<String> {
    let mut entries = if yaml_str.trim().is_empty() {
        vec![]
    } else {
        load_entries(yaml_str)?
    };

    if let Some(entry) = find_global_entry(&entries) {
        let Yaml::Hash(map) = entry else { unreachable!() };
        if let Some(Yaml::Hash(commands_map)) = map.get(&Yaml::String("global".to_string())) {
            if commands_map.contains_key(&Yaml::String(name.to_string())) {
                bail!("Global command '{name}' already exists");
            }
        }
    }

    let global_key = Yaml::String("global".to_string());

    if let Some(entry) = entries.iter_mut().find(|e| is_global(e)) {
        let Yaml::Hash(map) = entry else { unreachable!() };
        insert_into_commands(map, &global_key, name, command);
    } else {
        let mut commands_map = yaml_rust::yaml::Hash::new();
        commands_map.insert(Yaml::String(name.to_string()), Yaml::String(command.to_string()));

        let mut entry_map = yaml_rust::yaml::Hash::new();
        entry_map.insert(global_key, Yaml::Hash(commands_map));

        entries.insert(0, Yaml::Hash(entry_map));
    }

    Ok(serialize_entries(&entries))
}

fn write_commands(out: &mut String, commands_map: &yaml_rust::yaml::Hash, indent: &str) {
    for (k, v) in commands_map {
        let name = k.as_str().unwrap_or("");
        let cmd = v.as_str().unwrap_or("");
        if cmd.contains('\n') {
            writeln!(out, "{indent}{name}: |").unwrap();
            for line in cmd.lines() {
                writeln!(out, "{indent}  {line}").unwrap();
            }
        } else {
            writeln!(out, "{indent}{name}: {cmd}").unwrap();
        }
    }
}

fn serialize_entries(entries: &[Yaml]) -> String {
    let mut out = String::new();

    for (i, entry) in entries.iter().enumerate() {
        let Yaml::Hash(map) = entry else { continue };

        if i > 0 {
            out.push('\n');
        }

        if is_global(entry) {
            let Some(Yaml::Hash(commands_map)) = map.get(&Yaml::String("global".to_string())) else {
                continue;
            };
            out.push_str("- global:\n");
            write_commands(&mut out, commands_map, "    ");
        } else {
            let folder = map
                .get(&Yaml::String("folder".to_string()))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let folder = tildify(folder).unwrap_or_else(|_| folder.to_string());
            let Some(Yaml::Hash(commands_map)) = map.get(&Yaml::String("commands".to_string())) else {
                continue;
            };
            writeln!(out, "- folder: {folder}").unwrap();
            out.push_str("  commands:\n");
            write_commands(&mut out, commands_map, "    ");
        }
    }

    out
}
