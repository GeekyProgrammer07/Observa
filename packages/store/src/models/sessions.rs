use crate::error::StoreError;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{
        SaltString,
        rand_core::{OsRng, RngCore},
    },
};
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{
    ExpressionMethods, PgConnection, RunQueryDsl, Selectable,
    dsl::{delete, insert_into, update},
    prelude::{Insertable, Queryable},
    query_dsl::methods::FilterDsl,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::session)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    refresh_token: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::session)]
pub struct NewSession {
    pub user_id: Uuid,
    pub expires_at: NaiveDateTime,
    pub refresh_token: String,
}

impl Session {
    pub fn create_refresh_token(conn: &mut PgConnection, uid: Uuid) -> Result<String, StoreError> {
        use crate::schema::session;

        delete(session::table.filter(session::user_id.eq(uid)))
            .execute(conn)
            .map_err(|_| StoreError::Internal)?;

        let token = Self::generate_secure_token();
        let hash = Self::hash_refresh_token(&token)?;
        let exp = Self::new_expiry();

        insert_into(session::table)
            .values(&NewSession {
                user_id: uid,
                expires_at: exp,
                refresh_token: hash,
            })
            .execute(conn)
            .map_err(|err| match err {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => StoreError::Conflict,
                _ => StoreError::Internal,
            })?;

        Ok(token)
    }

    pub fn verify_refresh_token(
        conn: &mut PgConnection,
        uid: Uuid,
        token: &str,
    ) -> Result<(), StoreError> {
        use crate::schema::session::dsl::*;
        let sess: Session =
            session
                .filter(user_id.eq(uid))
                .first(conn)
                .map_err(|err| match err {
                    diesel::result::Error::NotFound => StoreError::Unauthorized,
                    _ => StoreError::Internal,
                })?;

        let parsed =
            PasswordHash::new(&sess.refresh_token).map_err(|_| StoreError::Unauthorized)?;
        Argon2::default()
            .verify_password(token.as_bytes(), &parsed)
            .map_err(|_| StoreError::Unauthorized)?;

        if sess.expires_at < Utc::now().naive_utc() {
            return Err(StoreError::Unauthorized);
        }

        Ok(())
    }

    pub fn rotate_refresh_token(
        conn: &mut PgConnection,
        uid: Uuid,
        token: &str,
    ) -> Result<String, StoreError> {
        Self::verify_refresh_token(conn, uid, token)?; // Self:: for consistency

        let new_refresh = Self::generate_secure_token();
        let new_hash = Self::hash_refresh_token(&new_refresh)?;

        use crate::schema::session::dsl::*;
        update(session.filter(user_id.eq(uid)))
            .set((
                refresh_token.eq(new_hash),
                expires_at.eq(Self::new_expiry()),
            ))
            .execute(conn)
            .map_err(|_| StoreError::Internal)?;

        Ok(new_refresh)
    }

    fn hash_refresh_token(token: &str) -> Result<String, StoreError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let token_hash = argon2
            .hash_password(token.as_bytes(), &salt)
            .map_err(|_| StoreError::Internal)?
            .to_string();
        Ok(token_hash)
    }

    fn generate_secure_token() -> String {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        hex::encode(bytes) // Add `hex = "0.4"` to Cargo.toml if not already
    }

    fn new_expiry() -> NaiveDateTime {
        (Utc::now() + Duration::days(30)).naive_utc()
    }
}
