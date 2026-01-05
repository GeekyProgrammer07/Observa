use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateMonitor {
    pub url: String,
    pub name: String,
    pub interval: Option<i32>,
    pub timeout_ms: Option<i32>,
    pub is_paused: Option<bool>,
}

#[derive(Serialize)]
pub struct CreateMonitorResponse {
    pub monitor_id: Uuid,
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
    pub id: Uuid,
    pub message: String,
}

#[derive(Deserialize)]
pub struct SigninRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SigninResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
}
