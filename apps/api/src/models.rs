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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signup_request_deserializes_all_fields() {
        let json = r#"{
            "firstname": "Ada",
            "lastname": "Lovelace",
            "username": "ada",
            "password": "hunter2"
        }"#;
        let req: SignupRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.firstname, "Ada");
        assert_eq!(req.lastname, "Lovelace");
        assert_eq!(req.username, "ada");
        assert_eq!(req.password, "hunter2");
    }

    #[test]
    fn signup_request_rejects_missing_fields() {
        let json = r#"{"username": "ada", "password": "hunter2"}"#;
        assert!(serde_json::from_str::<SignupRequest>(json).is_err());
    }

    #[test]
    fn signin_response_serializes_oauth_style_fields() {
        let resp = SigninResponse {
            access_token: "tok".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
        };
        let json: serde_json::Value = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["access_token"], "tok");
        assert_eq!(json["token_type"], "Bearer");
        assert_eq!(json["expires_in"], 3600);
    }

    #[test]
    fn create_monitor_defaults_optional_fields_to_none() {
        let json = r#"{"url": "https://example.com", "name": "Example"}"#;
        let req: CreateMonitor = serde_json::from_str(json).unwrap();
        assert_eq!(req.url, "https://example.com");
        assert_eq!(req.name, "Example");
        assert!(req.interval.is_none());
        assert!(req.timeout_ms.is_none());
        assert!(req.is_paused.is_none());
    }

    #[test]
    fn create_monitor_accepts_explicit_optional_fields() {
        let json = r#"{
            "url": "https://example.com",
            "name": "Example",
            "interval": 30,
            "timeout_ms": 2000,
            "is_paused": true
        }"#;
        let req: CreateMonitor = serde_json::from_str(json).unwrap();
        assert_eq!(req.interval, Some(30));
        assert_eq!(req.timeout_ms, Some(2000));
        assert_eq!(req.is_paused, Some(true));
    }

    #[test]
    fn notification_channel_request_deserializes_channel_type() {
        let json = r#"{"channel_type": "Email", "value": "a@b.com"}"#;
        let req: CreateNotificationChannelRequest = serde_json::from_str(json).unwrap();
        assert!(matches!(
            req.channel_type,
            store::models::notification::ChannelType::Email
        ));
        assert_eq!(req.value, "a@b.com");
    }

    #[test]
    fn notification_channel_request_rejects_unknown_channel_type() {
        let json = r#"{"channel_type": "Carrier-Pigeon", "value": "coo"}"#;
        assert!(serde_json::from_str::<CreateNotificationChannelRequest>(json).is_err());
    }
}
