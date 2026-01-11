use crate::error::AppError;

pub struct Config {
    pub jwt_secret: String,
    pub redis_url: String,
    pub stream_key: String
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        dotenvy::dotenv()?;
        let jwt_secret = dotenvy::var("JWT_SECRET")?;
        let redis_url = dotenvy::var("REDIS_URL")?;
        let stream_key = dotenvy::var("STREAM_KEY")?;
        Ok(Self {
            jwt_secret,
            redis_url,
            stream_key
        })
    }
}
