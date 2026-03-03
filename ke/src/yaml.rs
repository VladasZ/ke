use std::collections::BTreeMap;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::paths::{expand_tilde, tildify};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Entry {
    Global {
        global:   (),
        #[serde(flatten)]
        commands: BTreeMap<String, String>,
    },
    Folder {
        folder:   String,
        #[serde(flatten)]
        commands: BTreeMap<String, String>,
    },
}

impl Entry {
    fn commands(&self) -> &BTreeMap<String, String> {
        match self {
            Entry::Global { commands, .. } | Entry::Folder { commands, .. } => commands,
        }
    }

    fn commands_mut(&mut self) -> &mut BTreeMap<String, String> {
        match self {
            Entry::Global { commands, .. } | Entry::Folder { commands, .. } => commands,
        }
    }

    fn matches_folder(&self, folder: &str) -> bool {
        let Entry::Folder { folder: f, .. } = self else {
            return false;
        };
        expand_tilde(f).map(|expanded| expanded == folder).unwrap_or(false)
    }

    fn is_global(&self) -> bool {
        matches!(self, Entry::Global { .. })
    }
}

pub struct CommandList {
    pub folder: Vec<String>,
    pub global: Vec<String>,
}

fn load(yaml_str: &str) -> Result<Vec<Entry>> {
    Ok(serde_yaml_ng::from_str(yaml_str)?)
}

fn save(entries: &[Entry]) -> Result<String> {
    Ok(serde_yaml_ng::to_string(entries)?)
}

pub fn find_command(yaml_str: &str, folder: &str, command_name: &str) -> Result<String> {
    let entries = load(yaml_str)?;

    if let Some(entry) = entries.iter().find(|e| e.matches_folder(folder)) {
        if let Some(cmd) = entry.commands().get(command_name) {
            return Ok(cmd.clone());
        }
    }

    if let Some(entry) = entries.iter().find(|e| e.is_global()) {
        if let Some(cmd) = entry.commands().get(command_name) {
            return Ok(cmd.clone());
        }
    }

    bail!("Command '{command_name}' not found for folder '{folder}' and not in global commands")
}

pub fn list_commands(yaml_str: &str, folder: &str) -> Result<CommandList> {
    let entries = load(yaml_str)?;

    let folder_cmds = entries
        .iter()
        .find(|e| e.matches_folder(folder))
        .map(|e| e.commands().keys().cloned().collect())
        .unwrap_or_default();

    let global_cmds = entries
        .iter()
        .find(|e| e.is_global())
        .map(|e| e.commands().keys().cloned().collect())
        .unwrap_or_default();

    Ok(CommandList {
        folder: folder_cmds,
        global: global_cmds,
    })
}

pub fn add_command(yaml_str: &str, folder: &str, name: &str, command: &str) -> Result<String> {
    let mut entries = if yaml_str.trim().is_empty() {
        vec![]
    } else {
        load(yaml_str)?
    };

    if let Some(entry) = entries.iter().find(|e| e.matches_folder(folder)) {
        if entry.commands().contains_key(name) {
            bail!("Command '{name}' already exists for folder '{folder}'");
        }
    }

    if let Some(entry) = entries.iter_mut().find(|e| e.matches_folder(folder)) {
        entry.commands_mut().insert(name.to_string(), command.to_string());
    } else {
        let folder_tildified = tildify(folder).unwrap_or_else(|_| folder.to_string());
        let mut commands = BTreeMap::new();
        commands.insert(name.to_string(), command.to_string());
        entries.push(Entry::Folder {
            folder: folder_tildified,
            commands,
        });
    }

    save(&entries)
}

pub fn add_global_command(yaml_str: &str, name: &str, command: &str) -> Result<String> {
    let mut entries = if yaml_str.trim().is_empty() {
        vec![]
    } else {
        load(yaml_str)?
    };

    if let Some(entry) = entries.iter().find(|e| e.is_global()) {
        if entry.commands().contains_key(name) {
            bail!("Global command '{name}' already exists");
        }
    }

    if let Some(entry) = entries.iter_mut().find(|e| e.is_global()) {
        entry.commands_mut().insert(name.to_string(), command.to_string());
    } else {
        let mut commands = BTreeMap::new();
        commands.insert(name.to_string(), command.to_string());
        entries.insert(0, Entry::Global { global: (), commands });
    }

    save(&entries)
}
