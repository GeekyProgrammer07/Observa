#[derive(Debug)]
pub enum AppError {
    Env(dotenvy::Error),
}

impl From<dotenvy::Error> for AppError {
    fn from(value: dotenvy::Error) -> Self {
        AppError::Env(value)
    }
}
