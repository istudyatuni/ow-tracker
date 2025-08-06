use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Config {
    config: StoredConfig,
    path: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StoredConfig {
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub id: Uuid,
    pub name: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let Some(path) = config_path() else {
            error!("config dir not found");
            return Err(ConfigError::NotFound);
        };

        if !path.exists() {
            return Ok(Self {
                config: StoredConfig { profiles: vec![] },
                path,
            });
        }

        let config: StoredConfig = serde_json::from_str(&std::fs::read_to_string(&path)?)?;

        Ok(Self { config, path })
    }
    pub fn profiles(&self) -> &[Profile] {
        &self.config.profiles
    }
    pub fn add_register(&mut self, id: Uuid, name: &str) {
        self.config.profiles.push(Profile {
            id,
            name: name.to_string(),
        });
    }
    pub fn save_on_disk(&self) -> Result<(), ConfigError> {
        info!("saving config to {}", self.path.display());
        Ok(std::fs::write(
            &self.path,
            serde_json::to_string(&self.config)?,
        )?)
    }
}

#[cfg(debug_assertions)]
fn config_path() -> Option<PathBuf> {
    Some(PathBuf::from("target/config.json"))
}

#[cfg(not(debug_assertions))]
fn config_path() -> Option<PathBuf> {
    use directories::ProjectDirs;

    ProjectDirs::from("", "", "Outer Wilds Tracker Companion")
        .map(|d| d.config_dir().to_owned().join("config.json"))
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("config not found")]
    NotFound,
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
