use std::{collections::BTreeMap, path::Path};

use serde::Deserialize;

use common::saves::pack_bools;

use crate::log::LogError;

#[derive(Debug, Deserialize)]
pub struct SaveFile {
    #[serde(rename = "shipLogFactSaves")]
    pub fact_saves: BTreeMap<String, SaveFact>,
}

#[derive(Debug, Deserialize)]
pub struct SaveFact {
    #[serde(rename = "revealOrder")]
    pub reveal_order: i32,
}

impl SaveFile {
    pub fn load(path: &Path) -> Result<Self, SaveLoadError> {
        Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
    }
    pub fn learned_as_bools(&self) -> Vec<bool> {
        self.fact_saves.values().map(SaveFact::is_learned).collect()
    }
}

impl SaveFact {
    fn is_learned(&self) -> bool {
        self.reveal_order >= 0
    }
}

pub fn read_save_packed(path: &Path) -> Option<Vec<common::saves::Packed>> {
    let save = SaveFile::load(path)
        .log_msg("failed to load save file")
        .ok()?;
    let bools = save.learned_as_bools();
    Some(pack_bools(&bools))
}

#[derive(Debug, thiserror::Error)]
pub enum SaveLoadError {
    #[error("{0}")]
    Parse(#[from] serde_json::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
}
