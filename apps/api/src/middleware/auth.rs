use std::sync::Arc;

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use poem::{
    http::{header, StatusCode},
    Endpoint, Error, IntoResponse, Request, Response,
};

use crate::{auth::claims::Claims, config::Config};

pub async fn auth_middleware<E: Endpoint>(next: E, mut req: Request) -> Result<Response, Error> {
    let config = req
        .data::<Arc<Config>>()
        .ok_or_else(|| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| Error::from_status(StatusCode::UNAUTHORIZED))?;

    let key = DecodingKey::from_secret(config.jwt_secret.as_bytes());

    let token_data = decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256))
        .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;

    let user_id = token_data.claims.sub;

    req.extensions_mut().insert(user_id);

    let res = next
        .call(req)
        .await
        .map_err(|_| Error::from_status(StatusCode::INTERNAL_SERVER_ERROR))?
        .into_response();

    Ok(res)
}
