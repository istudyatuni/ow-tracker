use std::{fmt::Display, path::Path, str::FromStr, sync::Arc};

use chrono::{DateTime, Utc};
use redb::{Database, Error, TableDefinition};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::{Receiver, Sender, channel, error::SendError};
use uuid::Uuid;

use common::saves::Packed;

const REGISTER_TABLE: TableDefinition<String, String> = TableDefinition::new("registrations");

#[derive(Debug, Clone)]
pub struct Store {
    db: Arc<Database>,
}

impl Store {
    #[expect(clippy::result_large_err)]
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let db = Database::create(path)?;

        // create table
        let tx = db.begin_write()?;
        tx.open_table(REGISTER_TABLE)?;
        tx.commit()?;

        Ok(Self { db: Arc::new(db) })
    }
    #[expect(clippy::result_large_err)]
    pub fn save_register(&self, id: Uuid, save: Vec<Packed>) -> Result<(), Error> {
        let tx = self.db.begin_write()?;
        {
            let mut table = tx.open_table(REGISTER_TABLE)?;
            table.insert(id.to_string(), Registration::new(save).to_string())?;
        }
        tx.commit()?;

        Ok(())
    }
    #[expect(clippy::result_large_err)]
    pub fn get_register(&self, id: Uuid) -> Result<Option<Registration>, Error> {
        let tx = self.db.begin_read()?;
        let table = tx.open_table(REGISTER_TABLE)?;
        Ok(table
            .get(id.to_string())?
            .map(|o| o.value().parse())
            .transpose()
            .unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Registration {
    pub save: Vec<Packed>,
    pub updated: DateTime<Utc>,
}

impl Registration {
    fn new(save: Vec<Packed>) -> Self {
        Self {
            save,
            updated: Utc::now(),
        }
    }
}

impl Display for Registration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

impl FromStr for Registration {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
