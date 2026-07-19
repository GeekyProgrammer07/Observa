use std::io::Write;

use chrono::NaiveDateTime;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgConnection, RunQueryDsl, Selectable,
    SelectableHelper,
    deserialize::{self, FromSql, FromSqlRow},
    dsl::{delete, insert_into, update},
    expression::AsExpression,
    pg::{Pg, PgValue},
    prelude::{Insertable, Queryable},
    query_dsl::methods::{FilterDsl, SelectDsl},
    serialize::{self, IsNull, Output, ToSql},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::StoreError, store::Store};

#[derive(Debug, AsExpression, FromSqlRow, Deserialize, Serialize)]
#[diesel(sql_type = crate::schema::sql_types::ChannelType)]
pub enum ChannelType {
    Email,
    Sms,
    VoiceCall,
    Webhook,
}

impl ToSql<crate::schema::sql_types::ChannelType, Pg> for ChannelType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            ChannelType::Email => out.write_all(b"Email")?,
            ChannelType::Sms => out.write_all(b"Sms")?,
            ChannelType::VoiceCall => out.write_all(b"VoiceCall")?,
            ChannelType::Webhook => out.write_all(b"Webhook")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::ChannelType, Pg> for ChannelType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"Email" => Ok(ChannelType::Email),
            b"Sms" => Ok(ChannelType::Sms),
            b"VoiceCall" => Ok(ChannelType::VoiceCall),
            b"Webhook" => Ok(ChannelType::Webhook),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::notification_channel)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NotificationChannel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub type_: ChannelType,
    pub value: String,
    pub verified: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=crate::schema::notification_channel)]
pub struct NewChannel {
    pub user_id: Uuid,
    pub type_: ChannelType,
    pub value: String,
}

impl Store {
    pub fn add_channel(
        conn: &mut PgConnection,
        new_channel: NewChannel,
    ) -> Result<NotificationChannel, StoreError> {
        use crate::schema::notification_channel;

        insert_into(notification_channel::table)
            .values(new_channel)
            .returning(NotificationChannel::as_returning())
            .get_result(conn)
            .map_err(|err| match err {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => StoreError::Conflict,
                diesel::result::Error::NotFound => StoreError::NotFound,
                _ => {
                    tracing::error!(error = ?err, "unexpected database error");
                    StoreError::Internal
                }
            })
    }

    pub fn list_channels_by_user(
        conn: &mut PgConnection,
        uid: Uuid,
    ) -> Result<Vec<NotificationChannel>, StoreError> {
        use crate::schema::notification_channel::dsl::*;

        notification_channel
            .filter(user_id.eq(uid))
            .select(NotificationChannel::as_select())
            .load(conn)
            .map_err(|err| match err {
                diesel::result::Error::NotFound => StoreError::NotFound,
                _ => {
                    tracing::error!(error = ?err, "unexpected database error");
                    StoreError::Internal
                }
            })
    }

    pub fn verify_channel(
        conn: &mut PgConnection,
        uid: Uuid,
        channel_id: Uuid,
    ) -> Result<(), StoreError> {
        use crate::schema::notification_channel::dsl::*;

        let affected = update(notification_channel.filter(id.eq(channel_id).and(user_id.eq(uid))))
            .set(verified.eq(true))
            .execute(conn)
            .map_err(|err| {
                tracing::error!(error = ?err, "unexpected database error");
                StoreError::Internal
            })?;

        if affected == 0 {
            return Err(StoreError::NotFound);
        }

        Ok(())
    }

    pub fn delete_channel(
        conn: &mut PgConnection,
        uid: Uuid,
        channel_id: Uuid,
    ) -> Result<(), StoreError> {
        use crate::schema::notification_channel::dsl::*;

        let affected = delete(notification_channel.filter(id.eq(channel_id).and(user_id.eq(uid))))
            .execute(conn)
            .map_err(|err| {
                tracing::error!(error = ?err, "unexpected database error");
                StoreError::Internal
            })?;

        if affected == 0 {
            return Err(StoreError::NotFound);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelType;

    #[test]
    fn serializes_variants_to_their_db_names() {
        assert_eq!(
            serde_json::to_string(&ChannelType::Email).unwrap(),
            "\"Email\""
        );
        assert_eq!(serde_json::to_string(&ChannelType::Sms).unwrap(), "\"Sms\"");
        assert_eq!(
            serde_json::to_string(&ChannelType::VoiceCall).unwrap(),
            "\"VoiceCall\""
        );
        assert_eq!(
            serde_json::to_string(&ChannelType::Webhook).unwrap(),
            "\"Webhook\""
        );
    }

    #[test]
    fn deserializes_from_the_same_names() {
        assert!(matches!(
            serde_json::from_str::<ChannelType>("\"Email\"").unwrap(),
            ChannelType::Email
        ));
        assert!(matches!(
            serde_json::from_str::<ChannelType>("\"Sms\"").unwrap(),
            ChannelType::Sms
        ));
        assert!(matches!(
            serde_json::from_str::<ChannelType>("\"Webhook\"").unwrap(),
            ChannelType::Webhook
        ));
    }

    #[test]
    fn rejects_unknown_variants() {
        assert!(serde_json::from_str::<ChannelType>("\"Pager\"").is_err());
        assert!(serde_json::from_str::<ChannelType>("\"sms\"").is_err());
    }
}
