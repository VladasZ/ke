use std::{env, path::PathBuf};

use anyhow::{anyhow, Result};

pub fn home_dir() -> Result<PathBuf> {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map(PathBuf::from)
        .map_err(|_| anyhow!("Could not determine home directory"))
}

pub fn expand_tilde(path: &str) -> Result<String> {
    if path.starts_with("~/") || path == "~" {
        let home = home_dir()?;
        let stripped = path.trim_start_matches('~').trim_start_matches('/');
        let expanded = if stripped.is_empty() {
            home
        } else {
            home.join(stripped)
        };
        Ok(expanded.to_string_lossy().to_string())
    } else {
        Ok(path.to_string())
    }
}

pub fn tildify(path: &str) -> Result<String> {
    let home = home_dir()?.to_string_lossy().to_string();
    if path.starts_with(&home) {
        Ok(format!("~/{}", path[home.len()..].trim_start_matches('/')))
    } else {
        Ok(path.to_string())
    }
}

pub fn default_config_path() -> Result<PathBuf> {
    Ok(home_dir()?.join(".ke").join("commands.yaml"))
}

pub fn current_dir() -> Result<String> {
    let dir = env::current_dir().map_err(|e| anyhow!("Could not get current directory: {e}"))?;
    Ok(dir.to_string_lossy().to_string())
}
