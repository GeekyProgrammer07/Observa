#[derive(Debug)]
pub enum WorkerError {
    RedisError,
    Internal,
    NotFound,
    Conflict
}