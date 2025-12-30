use crate::error::StoreError;

pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, StoreError> {
        dotenvy::from_filename("../../..")?;
        let db_url = dotenvy::var("DATABASE_URL")?;
        Ok(Self { db_url })
    }
}