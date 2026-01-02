use crate::error::StoreError;

pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, StoreError> {
        dotenvy::dotenv().map_err(|_| StoreError::Internal)?;
        let db_url = dotenvy::var("DATABASE_URL").map_err(|_| StoreError::Internal)?;
        Ok(Self { db_url })
    }
}
