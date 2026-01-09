use crate::error::AppError;

pub struct Config {
    pub jwt_secret: String,
    pub redis_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        dotenvy::dotenv()?;
        let jwt_secret = dotenvy::var("JWT_SECRET")?;
        let redis_url = dotenvy::var("REDIS_URL")?;
        Ok(Self {
            jwt_secret,
            redis_url,
        })
    }
}
