use crate::error::AppError;

pub struct Config {
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        dotenvy::dotenv()?;
        let jwt_secret = dotenvy::var("JWT_SECRET")?;
        Ok(Self { jwt_secret })
    }
}
