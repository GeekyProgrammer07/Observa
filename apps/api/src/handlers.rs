use poem::{handler, http::StatusCode, web::Json};

use crate::models::{CreateWebsiteRequest, CreateWebsiteResponse};

#[handler]
pub fn health_check() -> String {
    String::from("Server is Healthy")
}

#[handler]
pub fn create_website(
    Json(req): Json<CreateWebsiteRequest>,
) -> (StatusCode, Json<CreateWebsiteResponse>) {
    (
        StatusCode::OK,
        Json(CreateWebsiteResponse {
            message: format!("Your Website: {} Added Successfully", req.url),
        }),
    )
}
