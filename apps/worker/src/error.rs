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

#[cfg(test)]
mod tests {
    use super::WorkerError;

    #[test]
    fn redis_error_display_includes_the_message() {
        let err = WorkerError::RedisError("NOGROUP no such group".into());
        assert_eq!(err.to_string(), "Redis error: NOGROUP no such group");
    }

    #[test]
    fn simple_variants_have_stable_messages() {
        assert_eq!(WorkerError::Internal.to_string(), "Internal error");
        assert_eq!(WorkerError::NotFound.to_string(), "Not found");
        assert_eq!(WorkerError::Conflict.to_string(), "Conflict");
    }

    #[test]
    fn debug_formatting_is_available_for_logging() {
        let err = WorkerError::RedisError("io".into());
        assert_eq!(format!("{err:?}"), "RedisError(\"io\")");
    }
}
