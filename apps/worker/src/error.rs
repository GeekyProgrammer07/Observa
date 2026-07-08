use std::fmt;

#[derive(Debug)]
pub enum WorkerError {
    RedisError(String),
    Internal,
    NotFound,
    Conflict,
}

impl fmt::Display for WorkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkerError::RedisError(msg) => write!(f, "Redis error: {msg}"),
            WorkerError::Internal => write!(f, "Internal error"),
            WorkerError::NotFound => write!(f, "Not found"),
            WorkerError::Conflict => write!(f, "Conflict"),
        }
    }
}
