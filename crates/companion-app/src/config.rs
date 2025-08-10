use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tracing::{debug, error, trace};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Config {
    config: StoredConfig,
    path: PathBuf,
}

// "default" is required to not crash deserializer if some field not found
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct StoredConfig {
    #[serde(skip_serializing_if = "Auth::is_empty")]
    auth: Auth,
    addresses: Option<LoadedConfig>,
    profiles: Vec<Profile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoadedConfig {
    #[serde(rename = "server_address")]
    server: String,
    #[serde(rename = "web_address")]
    web: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Auth {
    pub key: Option<Uuid>,
}

impl Auth {
    fn is_empty(&self) -> bool {
        self.key.is_none()
    }
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
        trace!("config path: {}", path.display());

        let config = StoredConfig::new(&path)?;
        trace!("config loaded");

        Ok(Self { config, path })
    }
    pub fn auth_key(&self) -> Option<Uuid> {
        self.config.auth.key
    }
    pub fn set_auth_key(&mut self, key: Uuid) {
        self.config.auth.key = Some(key);
    }
    pub fn set_addresses(&mut self, addresses: LoadedConfig) {
        self.config.addresses.replace(addresses);
    }
    pub fn profiles(&self) -> &[Profile] {
        &self.config.profiles
    }
    pub fn find_profile(&self, name: &str) -> Option<Uuid> {
        self.config
            .profiles
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.id)
    }
    pub fn get_profile(&self, id: Uuid) -> Option<&Profile> {
        self.config.profiles.iter().find(|p| p.id == id)
    }
    pub fn add_register(&mut self, id: Uuid, name: &str) {
        self.config.profiles.push(Profile {
            id,
            name: name.to_string(),
        });
    }
    pub fn remove_register(&mut self, id: Uuid) {
        self.config.profiles.retain(|p| p.id != id);
    }
    pub fn save_on_disk(&self) -> Result<(), ConfigError> {
        debug!("saving config");
        Ok(std::fs::write(
            &self.path,
            serde_json::to_string(&self.config)?,
        )?)
    }
}

impl StoredConfig {
    fn new(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            trace!("config not exists, using default");
            std::fs::create_dir_all(path.parent().expect("config path should have dir name"))?;

            return Ok(StoredConfig::default());
        }

        trace!("loading config");
        Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
    }
}

fn config_path() -> Option<PathBuf> {
    config_path_dir().map(|p| p.join("config.json"))
}

#[cfg(debug_assertions)]
fn config_path_dir() -> Option<PathBuf> {
    Some(PathBuf::from("target"))
}

#[cfg(not(debug_assertions))]
fn config_path_dir() -> Option<PathBuf> {
    use directories::ProjectDirs;

    ProjectDirs::from("", "", "Outer Wilds Tracker Companion").map(|d| d.config_dir().to_owned())
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
