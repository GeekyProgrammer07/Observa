use diesel::{ConnectionError, result::Error};

#[derive(Debug)]
pub enum StoreError {
    Env(dotenvy::Error),
    Db(ConnectionError),
    Res(Error),
}

impl From<dotenvy::Error> for StoreError {
    fn from(e: dotenvy::Error) -> Self {
        StoreError::Env(e)
    }
}

impl From<ConnectionError> for StoreError {
    fn from(e: ConnectionError) -> Self {
        StoreError::Db(e)
    }
}

impl From<Error> for StoreError {
    fn from(e: Error) -> Self {
        StoreError::Res(e)
    }
}
