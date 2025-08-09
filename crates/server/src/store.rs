use std::{fmt::Display, path::Path, str::FromStr, sync::Arc};

use axum::extract::FromRef;
use chrono::{DateTime, Utc};
#[cfg(debug_assertions)]
use redb::ReadableTable;
use redb::{Database, Error, TableDefinition};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::{Receiver, Sender, channel, error::SendError};
use uuid::Uuid;

use common::saves::Packed;

const REGISTER_TABLE: TableDefinition<String, String> = TableDefinition::new("registrations");

#[derive(Debug, Clone)]
pub struct Store {
    db: Arc<Database>,
    // storing it here because axum can't handle multiple states
    watches: Watches,
}

impl Store {
    #[expect(clippy::result_large_err)]
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let db = Database::create(path)?;

        // create table
        let tx = db.begin_write()?;
        tx.open_table(REGISTER_TABLE)?;
        tx.commit()?;

        Ok(Self {
            db: Arc::new(db),
            watches: Watches::new(),
        })
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
    #[cfg(debug_assertions)]
    #[expect(clippy::result_large_err)]
    pub fn list_registers(&self) -> Result<Vec<(Uuid, Registration)>, Error> {
        let tx = self.db.begin_read()?;
        let table = tx.open_table(REGISTER_TABLE)?;
        Ok(table
            .iter()
            .map(|r| r.into_iter())?
            .flatten()
            .map(|(key, value)| {
                (
                    key.value().parse().expect("uuid should be valid in db"),
                    value.value().parse().expect("json should be valid in db"),
                )
            })
            .collect())
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

#[derive(Debug)]
pub struct Watches {
    tx: Sender<Uuid>,
    pub rx: Receiver<Uuid>,
}

impl Watches {
    fn new() -> Self {
        let (tx, rx) = channel(100);
        Self { tx, rx }
    }
    pub async fn send(&self, id: Uuid) -> Result<(), SendError<Uuid>> {
        self.tx.send(id).map(|_| ())
    }
}

impl FromRef<Store> for Watches {
    fn from_ref(store: &Store) -> Self {
        store.watches.clone()
    }
}

impl Clone for Watches {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            rx: self.rx.resubscribe(),
        }
    }
}
