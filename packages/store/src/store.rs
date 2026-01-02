use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};

use crate::{config::Config, error::StoreError};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct Store {
    pub pool: PgPool,
}

impl Store {
    pub fn new() -> Result<Self, StoreError> {
        let database_url = Config::from_env()?.db_url;
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .map_err(|_| StoreError::Internal)?;
        Ok(Self { pool })
    }
}
