use diesel::{Connection, PgConnection};

use crate::{config::Config, error::StoreError};

pub struct Store {
    pub conn: PgConnection,
}

impl Store {
    pub fn connect() -> Result<Self, StoreError> {
        let database_url = Config::from_env()?.db_url;
        let connection = PgConnection::establish(&database_url)?;
        Ok(Self { conn: connection })
    }
}
