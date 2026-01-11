use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgConnection, RunQueryDsl, Selectable,
    SelectableHelper, delete, insert_into,
    prelude::{Insertable, Queryable},
    query_dsl::methods::{FilterDsl, SelectDsl},
    update,
};
use uuid::Uuid;

use crate::{error::StoreError, store::Store};

#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::monitor)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Monitor {
    pub id: Uuid,
    pub url: String,
    pub name: Option<String>,
    pub interval: i32,
    pub timeout_ms: i32,
    pub is_paused: bool,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name=crate::schema::monitor)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SchedulerMonitor {
    pub id: Uuid,
    pub url: String,
}

#[derive(Insertable)]
#[diesel(table_name=crate::schema::monitor)]
pub struct NewMonitor {
    pub url: String,
    pub name: String,
    pub interval: Option<i32>,
    pub timeout_ms: Option<i32>,
    pub is_paused: Option<bool>,
    pub user_id: Uuid,
}

impl Store {
    pub fn create_monitor(
        conn: &mut PgConnection,
        new_monitor: NewMonitor,
    ) -> Result<Monitor, StoreError> {
        use crate::schema::monitor;

        insert_into(monitor::table)
            .values(new_monitor)
            .returning(Monitor::as_returning())
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

    pub fn list_monitors_by_user(
        conn: &mut PgConnection,
        uid: Uuid,
    ) -> Result<Vec<Monitor>, StoreError> {
        use crate::schema::monitor::dsl::*;

        monitor
            .filter(user_id.eq(uid))
            .select(Monitor::as_select())
            .load(conn)
            .map_err(|err| match err {
                diesel::result::Error::NotFound => StoreError::NotFound,
                _ => StoreError::Internal,
            })
    }

    pub fn pause_monitor(
        conn: &mut PgConnection,
        mid: Uuid, //Monitor Id
        uid: Uuid,
    ) -> Result<(), StoreError> {
        use crate::schema::monitor::dsl::*;

        let affected = update(monitor.filter(id.eq(mid).and(user_id.eq(uid))))
            .set(is_paused.eq(true))
            .execute(conn)
            .map_err(|_| StoreError::Internal)?;

        if affected == 0 {
            return Err(StoreError::NotFound);
        }

        Ok(())
    }

    pub fn resume_monitor(conn: &mut PgConnection, mid: Uuid, uid: Uuid) -> Result<(), StoreError> {
        use crate::schema::monitor::dsl::*;

        let affected = update(monitor.filter(id.eq(mid).and(user_id.eq(uid))))
            .set(is_paused.eq(false))
            .execute(conn)
            .map_err(|_| StoreError::Internal)?;

        if affected == 0 {
            return Err(StoreError::NotFound);
        }

        Ok(())
    }

    pub fn delete_monitor(conn: &mut PgConnection, mid: Uuid, uid: Uuid) -> Result<(), StoreError> {
        use crate::schema::monitor::dsl::*;

        let affected = delete(monitor.filter(id.eq(mid).and(user_id.eq(uid))))
            .execute(conn)
            .map_err(|_| StoreError::Internal)?;

        if affected == 0 {
            return Err(StoreError::NotFound);
        }

        Ok(())
    }

    pub fn get_monitors(conn: &mut PgConnection) -> Result<Vec<SchedulerMonitor>, StoreError> {
        use crate::schema::monitor::dsl::*;

        monitor
            .filter(is_paused.eq(false))
            .select(SchedulerMonitor::as_select())
            .load(conn)
            .map_err(|err| match err {
                diesel::result::Error::NotFound => StoreError::NotFound,
                _ => StoreError::Internal,
            })
    }
}
