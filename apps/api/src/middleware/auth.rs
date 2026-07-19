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
        .ok_or_else(|| {
            tracing::warn!(path = %req.uri().path(), "missing or malformed Authorization header");
            Error::from_status(StatusCode::UNAUTHORIZED)
        })?;

    let key = DecodingKey::from_secret(config.jwt_secret.as_bytes());

    let token_data =
        decode::<Claims>(token, &key, &Validation::new(Algorithm::HS256)).map_err(|e| {
            tracing::warn!(path = %req.uri().path(), error = %e, "JWT validation failed");
            Error::from_status(StatusCode::UNAUTHORIZED)
        })?;

    let uid = token_data.claims.sub;

    req.extensions_mut().insert(uid);

    let res = next.call(req).await?.into_response();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use chrono::Utc;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use poem::{get, handler, test::TestClient, EndpointExt, Route};
    use uuid::Uuid;

    #[handler]
    fn whoami(req: &Request) -> String {
        req.extensions()
            .get::<Uuid>()
            .map(|u| u.to_string())
            .unwrap_or_default()
    }

    fn test_app(secret: &str) -> impl Endpoint {
        let config = Arc::new(Config {
            jwt_secret: secret.to_string(),
            redis_url: String::new(),
            stream_key: String::new(),
        });
        Route::new()
            .at("/whoami", get(whoami).around(auth_middleware))
            .data(config)
    }

    fn make_token(secret: &str, sub: Uuid, expires_in_secs: i64) -> String {
        let now = Utc::now().timestamp();
        let claims = Claims {
            iss: "Observa".to_string(),
            sub,
            iat: now as usize,
            exp: (now + expires_in_secs) as usize,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn rejects_requests_without_an_authorization_header() {
        let cli = TestClient::new(test_app("secret"));
        let resp = cli.get("/whoami").send().await;
        resp.assert_status(StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn rejects_non_bearer_authorization_schemes() {
        let cli = TestClient::new(test_app("secret"));
        let resp = cli
            .get("/whoami")
            .header("Authorization", "Basic dXNlcjpwYXNz")
            .send()
            .await;
        resp.assert_status(StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn rejects_garbage_tokens() {
        let cli = TestClient::new(test_app("secret"));
        let resp = cli
            .get("/whoami")
            .header("Authorization", "Bearer not.a.jwt")
            .send()
            .await;
        resp.assert_status(StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn rejects_tokens_signed_with_a_different_secret() {
        let token = make_token("wrong-secret", Uuid::new_v4(), 3600);
        let cli = TestClient::new(test_app("secret"));
        let resp = cli
            .get("/whoami")
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await;
        resp.assert_status(StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn rejects_expired_tokens() {
        let token = make_token("secret", Uuid::new_v4(), -3600);
        let cli = TestClient::new(test_app("secret"));
        let resp = cli
            .get("/whoami")
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await;
        resp.assert_status(StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn accepts_valid_tokens_and_exposes_the_user_id() {
        let uid = Uuid::new_v4();
        let token = make_token("secret", uid, 3600);
        let cli = TestClient::new(test_app("secret"));
        let resp = cli
            .get("/whoami")
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await;
        resp.assert_status_is_ok();
        resp.assert_text(uid.to_string()).await;
    }
}
