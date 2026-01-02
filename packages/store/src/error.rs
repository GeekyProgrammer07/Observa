use diesel::{ConnectionError, r2d2, result::Error};

#[derive(Debug)]
pub enum StoreError {
    Env(dotenvy::Error),
    Db(ConnectionError),
    Res(Error),
    Pool(r2d2::PoolError),
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

impl From<r2d2::PoolError> for StoreError {
    fn from(e: r2d2::PoolError) -> Self {
        StoreError::Pool(e)
    }
}
