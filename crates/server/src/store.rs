use std::{path::Path, sync::Arc};

use axum::extract::FromRef;
use bincode::{Decode, Encode, serde::Compat};
use chrono::{DateTime, Utc};
#[cfg(debug_assertions)]
use redb::ReadableTable;
use redb::{Database, Error, ReadableDatabase, TableDefinition};
use tokio::sync::broadcast::{Receiver, Sender, channel, error::SendError};
use uuid::Uuid;

use common::saves::Packed;

const REGISTER_TABLE: TableDefinition<String, Registration> = TableDefinition::new("registrations");
const USERS_TABLE: TableDefinition<String, User> = TableDefinition::new("users");

const ENCODE_CONFIG: bincode::config::Configuration =
    bincode::config::standard().with_little_endian();

#[derive(Debug, Clone)]
pub struct Store {
    db: Arc<Database>,
    // storing it here because axum can't handle multiple states
    watches: Watches,
}

impl Store {
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
    pub fn save_user(&self, id: Uuid, name: Option<String>) -> Result<(), Error> {
        let tx = self.db.begin_write()?;
        {
            let mut table = tx.open_table(USERS_TABLE)?;
            table.insert(id.to_string(), User::new(name))?;
        }
        tx.commit()?;

        Ok(())
    }
    #[expect(unused)]
    pub fn get_user(&self, id: Uuid) -> Result<Option<User>, Error> {
        let tx = self.db.begin_read()?;
        let table = tx.open_table(USERS_TABLE)?;
        Ok(table.get(id.to_string())?.map(|o| o.value()))
    }
    pub fn save_register(&self, id: Uuid, user: Uuid, save: Vec<Packed>) -> Result<(), Error> {
        let tx = self.db.begin_write()?;
        {
            let mut table = tx.open_table(REGISTER_TABLE)?;
            table.insert(id.to_string(), Registration::new(save, user))?;
        }
        tx.commit()?;

        Ok(())
    }
    pub fn get_register(&self, id: Uuid) -> Result<Option<Registration>, Error> {
        let tx = self.db.begin_read()?;
        let table = tx.open_table(REGISTER_TABLE)?;
        Ok(table.get(id.to_string())?.map(|o| o.value()))
    }
    #[cfg(debug_assertions)]
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
                    value.value(),
                )
            })
            .collect())
    }
}

/// Implement redb::Value for bincode::Encode/Decode-d type
macro_rules! bincode_redb_value {
    () => {};
    ($name:ty) => {
        impl redb::Value for $name {
            type SelfType<'a>
                = $name
            where
                Self: 'a;

            type AsBytes<'a>
                = Vec<u8>
            where
                Self: 'a;

            fn fixed_width() -> Option<usize> {
                None
            }

            fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
            where
                Self: 'a,
            {
                bincode::decode_from_slice(data, ENCODE_CONFIG).unwrap().0
            }

            fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
            where
                Self: 'b,
            {
                bincode::encode_to_vec(value, ENCODE_CONFIG).unwrap()
            }

            fn type_name() -> redb::TypeName {
                redb::TypeName::new(stringify!($name))
            }
        }
    };
}

#[derive(Debug, Encode, Decode)]
pub struct User {
    name: Option<String>,
}

impl User {
    fn new(name: Option<String>) -> Self {
        Self { name }
    }
}

bincode_redb_value!(User);

#[derive(Debug, Encode, Decode)]
pub struct Registration {
    pub save: Vec<Packed>,
    pub user: Compat<Uuid>,
    updated: Compat<DateTime<Utc>>,
}

impl Registration {
    fn new(save: Vec<Packed>, user: Uuid) -> Self {
        Self {
            save,
            user: Compat(user),
            updated: Compat(Utc::now()),
        }
    }
    pub fn user(&self) -> Uuid {
        self.user.0
    }
    pub fn updated(&self) -> DateTime<Utc> {
        self.updated.0
    }
}

bincode_redb_value!(Registration);

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
