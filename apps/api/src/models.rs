use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use store::models::notification::ChannelType;
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

#[derive(Serialize)]
pub struct GetMonitorResponse {
    pub id: Uuid,
    pub url: String,
    pub name: Option<String>,
    pub interval: i32,
    pub timeout_ms: i32,
    pub is_paused: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct MonitorActionResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct CreateNotificationChannelRequest {
    pub channel_type: ChannelType,
    pub value: String,
}

#[derive(Serialize)]
pub struct CreateNotificationChannelResponse {
    pub channel_id: Uuid,
}

#[derive(Serialize)]
pub struct GetNotificationChannelResponse {
    pub id: Uuid,
    pub channel_type: ChannelType,
    pub value: String,
    pub verified: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct VerifyNotificationChannelResponse {
    pub message: String,
}
