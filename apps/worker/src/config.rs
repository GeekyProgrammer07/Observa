pub struct Config {
    pub redis_url: String,
    pub stream_key: String,
    pub region_id: String,
    pub consumer_group: String,
}

impl Config {
    pub fn from_env() -> Result<Self, dotenvy::Error> {
        dotenvy::dotenv()?;
        let redis_url = dotenvy::var("REDIS_URL")?;
        let stream_key = dotenvy::var("STREAM_KEY")?;
        let region_id = dotenvy::var("REGION_ID")?;
        let consumer_group = dotenvy::var("CONSUMER_GROUP")?;
        Ok(Self {
            redis_url,
            stream_key,
            region_id,
            consumer_group,
        })
    }
}
