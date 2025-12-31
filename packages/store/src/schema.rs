// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "alert_type"))]
    pub struct AlertType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "channel_type"))]
    pub struct ChannelType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "monitor_status"))]
    pub struct MonitorStatus;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "plan_type"))]
    pub struct PlanType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "subscription_status"))]
    pub struct SubscriptionStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AlertType;

    alert (id) {
        id -> Uuid,
        #[sql_name = "type"]
        type_ -> AlertType,
        message -> Text,
        sent_at -> Timestamp,
        monitor_id -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MonitorStatus;

    checks (id) {
        id -> Uuid,
        response_time_ms -> Int4,
        status -> MonitorStatus,
        region_id -> Uuid,
        monitor_id -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    monitor (id) {
        id -> Uuid,
        url -> Text,
        name -> Nullable<Text>,
        interval -> Int4,
        timeout_ms -> Int4,
        is_paused -> Bool,
        user_id -> Uuid,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ChannelType;

    notification_channel (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[sql_name = "type"]
        type_ -> ChannelType,
        value -> Text,
        verified -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    region (id) {
        id -> Uuid,
        name -> Text,
    }
}

diesel::table! {
    status_page (id) {
        id -> Uuid,
        slug -> Text,
        user_id -> Uuid,
        is_public -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PlanType;
    use super::sql_types::SubscriptionStatus;

    subscription (id) {
        id -> Uuid,
        user_id -> Uuid,
        plan -> PlanType,
        status -> SubscriptionStatus,
        starts_at -> Timestamp,
        ends_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        firstname -> Text,
        lastname -> Text,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(alert -> monitor (monitor_id));
diesel::joinable!(checks -> monitor (monitor_id));
diesel::joinable!(checks -> region (region_id));
diesel::joinable!(monitor -> user (user_id));
diesel::joinable!(notification_channel -> user (user_id));
diesel::joinable!(status_page -> user (user_id));
diesel::joinable!(subscription -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    alert,
    checks,
    monitor,
    notification_channel,
    region,
    status_page,
    subscription,
    user,
);
