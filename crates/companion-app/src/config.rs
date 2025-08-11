use std::path::{Path, PathBuf};

use bincode::{Decode, Encode, serde::Compat};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, trace};
use uuid::Uuid;

const ENCODE_CONFIG: bincode::config::Configuration =
    bincode::config::standard().with_little_endian();

#[derive(Debug, Clone)]
pub struct Config {
    auth: AuthConfig,
    config: StoredConfig,
    config_path: PathBuf,
    auth_config_path: PathBuf,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct StoredConfig {
    addresses: Option<LoadedConfig>,
    profiles: Vec<Profile>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoadedConfig {
    #[serde(rename = "server_address")]
    pub server: String,
    #[serde(rename = "web_address")]
    pub web: String,
}

#[derive(Debug, Default, Clone, Decode, Encode)]
pub struct AuthConfig {
    pub key: Option<Compat<Uuid>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub id: Uuid,
    pub name: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let Some(config_path) = config_path() else {
            error!("config dir not found");
            return Err(ConfigError::NotFound);
        };
        trace!("config path: {}", config_path.display());
        let Some(auth_config_path) = auth_config_path() else {
            error!("auth config dir not found");
            return Err(ConfigError::NotFound);
        };
        trace!("auth config path: {}", auth_config_path.display());

        let config = StoredConfig::new(&config_path)?;
        trace!("config loaded");

        let auth = AuthConfig::new(&auth_config_path)?;
        trace!("auth config loaded");

        Ok(Self {
            auth,
            config,
            config_path,
            auth_config_path,
        })
    }
    pub fn auth_key(&self) -> Option<Uuid> {
        self.auth.key.clone().map(|k| k.0)
    }
    pub fn set_auth_key(&mut self, key: Uuid) {
        self.auth.key = Some(Compat(key));
    }
    pub fn set_addresses(&mut self, addresses: LoadedConfig) {
        self.config.addresses.replace(addresses);
    }
    /*pub fn server_address(&self) -> Option<&str> {
        self.config.addresses.as_ref().map(|a| a.server.as_str())
    }*/
    pub fn web_address(&self) -> Option<&str> {
        self.config.addresses.as_ref().map(|a| a.web.as_str())
    }
    // "_owned" to not call many times .map(ToOwned::to_owned)
    pub fn server_address_owned(&self) -> Option<String> {
        self.config.addresses.as_ref().map(|a| a.server.clone())
    }
    pub fn web_address_owned(&self) -> Option<String> {
        self.config.addresses.as_ref().map(|a| a.web.clone())
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
        std::fs::write(&self.config_path, serde_json::to_string(&self.config)?)?;

        debug!("saving auth config");
        bincode::encode_into_std_write(
            &self.auth,
            &mut std::fs::File::create(&self.auth_config_path)?,
            ENCODE_CONFIG,
        )?;

        Ok(())
    }
}

impl StoredConfig {
    fn new(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            trace!("config not exists, using default");
            std::fs::create_dir_all(path.parent().expect("config path should have dir name"))?;

            return Ok(Self::default());
        }

        trace!("loading config");
        Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
    }
}

impl AuthConfig {
    fn new(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            trace!("auth config not exists, using default");
            std::fs::create_dir_all(
                path.parent()
                    .expect("auth config path should have dir name"),
            )?;

            return Ok(Self::default());
        }

        trace!("loading auth config");
        Ok(bincode::decode_from_std_read(
            &mut std::fs::File::open(path)?,
            ENCODE_CONFIG,
        )?)
    }
}

fn config_path() -> Option<PathBuf> {
    config_path_dir().map(|p| p.join("config.json"))
}

fn auth_config_path() -> Option<PathBuf> {
    config_path_dir().map(|p| p.join("auth"))
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
    BincodeDecode(#[from] bincode::error::DecodeError),
    #[error("{0}")]
    BincodeEncode(#[from] bincode::error::EncodeError),
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
