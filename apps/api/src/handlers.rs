use poem::{handler, http::StatusCode, web::Json};
use store::{models::user::NewUser, store::Store};

use crate::models::{CreateWebsiteRequest, CreateWebsiteResponse, SignupRequest, SignupResponse};

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

#[handler]
pub fn signup(Json(req): Json<NewUser>) -> (StatusCode, Json<SignupResponse>) {
    let mut conn = Store::connect().unwrap_or_else(|err| panic!("connction error: {:?}", err));
    let user = Store::create_user(&mut conn, req)
        .unwrap_or_else(|err| panic!("insertion error: {:?}", err));
    (
        StatusCode::OK,
        Json(SignupResponse {
            message: format!("{:?}", user),
        }),
    )
}
