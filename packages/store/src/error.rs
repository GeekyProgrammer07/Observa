use diesel::ConnectionError;

#[derive(Debug)]
pub enum StoreError {
    Env(dotenvy::Error),
    Db(ConnectionError),
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
