#[derive(Debug)]
pub enum StoreError {
    Conflict,
    NotFound,
    Internal,
    Unauthorized,
}
