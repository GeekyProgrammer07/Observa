use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateWebsiteRequest {
    pub url: String
}

#[derive(Serialize)]
pub struct CreateWebsiteResponse {
    pub message: String
}