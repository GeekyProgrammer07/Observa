use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateWebsiteRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct CreateWebsiteResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}