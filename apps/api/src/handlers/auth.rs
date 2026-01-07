use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use poem::{
    handler,
    http::{header, StatusCode},
    web::{
        cookie::{Cookie, SameSite},
        Data, Json,
    },
    Response,
};
use std::{sync::Arc, time::Duration};
use store::{
    error::StoreError,
    models::{sessions::Session, user::NewUser},
    store::Store,
};

use crate::{
    auth::claims::Claims,
    config::Config,
    models::{SigninRequest, SigninResponse, SignupRequest, SignupResponse},
};

#[handler]
pub async fn signup(
    Json(req): Json<SignupRequest>,
    Data(store): Data<&Arc<Store>>,
) -> Result<(StatusCode, Json<SignupResponse>), StatusCode> {
    let mut conn = store
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let password = req.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password, &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    let user_to_insert = NewUser {
        firstname: req.firstname,
        lastname: req.lastname,
        username: req.username,
        password: password_hash,
    };

    let user = Store::create_user(&mut conn, user_to_insert).map_err(|err| match err {
        StoreError::Conflict => StatusCode::CONFLICT,
        StoreError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::OK, Json(SignupResponse { id: user.id })))
}

#[handler]
pub async fn signin(
    Json(req): Json<SigninRequest>,
    Data(config): Data<&Arc<Config>>,
    Data(store): Data<&Arc<Store>>,
) -> Result<Response, StatusCode> {
    let mut conn = store
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = Store::get_user_by_username(&mut conn, &req.username).map_err(|err| match err {
        StoreError::NotFound => StatusCode::UNAUTHORIZED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let hashed_password =
        PasswordHash::new(user.password_hash()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Argon2::default()
        .verify_password(req.password.as_bytes(), &hashed_password)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let now = Utc::now().timestamp() as usize;
    let claim = Claims {
        iss: "Observa".to_string(),
        sub: user.id,
        iat: now,
        exp: (now + 60 * 60),
    };
    let header = Header::default();
    let key = EncodingKey::from_secret(config.jwt_secret.as_bytes());

    let access_token =
        encode(&header, &claim, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let refresh_token =
        Session::create_refresh_token(&mut conn, user.id).map_err(|err| match err {
            StoreError::Conflict => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let mut cookie = Cookie::new_with_str("refreshToken", refresh_token);
    cookie.set_http_only(true);
    cookie.set_max_age(Duration::from_secs(30 * 24 * 60 * 60));
    cookie.set_path("/");
    cookie.set_same_site(SameSite::Lax);

    let resp = SigninResponse {
        access_token,
        token_type: "Bearer".into(),
        expires_in: 3600,
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::SET_COOKIE, cookie.to_string())
        .content_type("application/json")
        .body(serde_json::to_string(&resp).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?))
}
