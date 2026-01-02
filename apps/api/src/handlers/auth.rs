use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json},
};
use std::sync::Arc;
use store::{error::StoreError, models::user::NewUser, store::Store};

use crate::{
    auth::claims::Claims,
    config::Config,
    models::{SignupRequest, SignupResponse},
};

#[handler]
pub fn signup(
    Json(req): Json<SignupRequest>,
    Data(store): Data<&Arc<Store>>,
    Data(config): Data<&Arc<Config>>,
) -> Result<(StatusCode, Json<SignupResponse>), StatusCode> {
    // let mut conn = Store::connect().unwrap_or_else(|err| panic!("connction error: {:?}",
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
        StoreError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let now = Utc::now().timestamp() as usize;
    let claim = Claims {
        iss: "Observa".to_string(),
        sub: user.id.to_string(),
        iat: now,
        exp: (now + 60 * 60) as usize,
    };
    let header = Header::default();
    let key = EncodingKey::from_secret(config.jwt_secret.as_bytes());

    let token = encode(&header, &claim, &key).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::OK,
        Json(SignupResponse {
            id: user.id,
            username: user.username().to_string(),
            token,
        }),
    ))
}
