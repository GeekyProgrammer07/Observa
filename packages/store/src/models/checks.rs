use std::io::Write;

use diesel::{
    PgConnection, RunQueryDsl,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    insert_into,
    pg::{Pg, PgValue},
    prelude::Insertable,
    serialize::{self, IsNull, Output, ToSql},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::StoreError, store::Store};

#[derive(Debug, AsExpression, FromSqlRow, Deserialize, Serialize)]
#[diesel(sql_type = crate::schema::sql_types::MonitorStatus)]
pub enum MonitorStatusType {
    Up,
    Down,
    Unknown,
}

impl ToSql<crate::schema::sql_types::MonitorStatus, Pg> for MonitorStatusType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            MonitorStatusType::Up => out.write_all(b"Up")?,
            MonitorStatusType::Down => out.write_all(b"Down")?,
            MonitorStatusType::Unknown => out.write_all(b"Unknown")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::MonitorStatus, Pg> for MonitorStatusType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Up" => Ok(MonitorStatusType::Up),
            b"Down" => Ok(MonitorStatusType::Down),
            b"Unknown" => Ok(MonitorStatusType::Unknown),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name=crate::schema::checks)]
pub struct NewCheck {
    pub response_time_ms: i32,
    pub status: MonitorStatusType,
    pub region_id: Uuid,
    pub monitor_id: Uuid,
}

impl Store {
    pub fn create_check(conn: &mut PgConnection, check: NewCheck) -> Result<(), StoreError> {
        use crate::schema::checks;

        insert_into(checks::table)
            .values(check)
            .execute(conn)
            .map(|_| ())
            .map_err(|err| match err {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => StoreError::Conflict,
                diesel::result::Error::NotFound => StoreError::NotFound,
                _ => StoreError::Internal,
            })
    }
}
