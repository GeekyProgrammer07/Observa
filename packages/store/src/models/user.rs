use crate::{error::StoreError, store::Store};
use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, PgConnection, RunQueryDsl, Selectable, SelectableHelper,
    dsl::insert_into,
    prelude::{Insertable, Queryable},
    query_dsl::methods::FilterDsl,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)] // This rust type exactly mathches on row of the user table
#[diesel(table_name = crate::schema::user)] // Links this struct to user table type in schema.rs
#[diesel(check_for_backend(diesel::pg::Pg))] // Ensures DB colum type == Rust field Type and errors at compile time
// TODO: add explicit getters to see the pass and username as they are private
pub struct User {
    pub id: Uuid,
    pub firstname: String,
    pub lastname: String,
    username: String,
    password: String,
    created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::user )]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub password: String,
}

impl Store {
    pub fn create_user(conn: &mut PgConnection, new_user: NewUser) -> Result<User, StoreError> {
        use crate::schema::user;

        insert_into(user::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(conn)
            .map_err(|err| match err {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => StoreError::Conflict,

                diesel::result::Error::NotFound => StoreError::NotFound,

                _ => StoreError::Internal,
            })
    }

    pub fn get_user_by_username(
        conn: &mut PgConnection,
        user_name: &str,
    ) -> Result<User, StoreError> {
        use crate::schema::user::dsl::*;

        user.filter(username.eq(user_name))
            .first::<User>(conn)
            .map_err(|err| match err {
                diesel::result::Error::NotFound => StoreError::NotFound,
                _ => StoreError::Internal,
            })
    }
}

impl User {
    pub fn password_hash(&self) -> &str {
        &self.password
    }

    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}
