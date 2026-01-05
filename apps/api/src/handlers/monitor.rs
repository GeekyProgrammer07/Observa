use std::sync::Arc;

use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json},
    Request,
};
use store::{error::StoreError, models::monitor::NewMonitor, store::Store};
use uuid::Uuid;

use crate::models::{CreateMonitor, CreateMonitorResponse};

#[handler]
pub fn create_monitor(
    Json(body): Json<CreateMonitor>,
    Data(store): Data<&Arc<Store>>,
    req: &Request,
) -> Result<(StatusCode, Json<CreateMonitorResponse>), StatusCode> {
    println!("Hello");
    let mut conn = store
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uid = req
        .extensions()
        .get::<Uuid>()
        .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_monitor = NewMonitor {
        url: body.url,
        name: body.name,
        interval: body.interval,
        timeout_ms: body.timeout_ms,
        is_paused: body.is_paused,
        user_id: *uid,
    };

    let monitor = Store::create_monitor(&mut conn, new_monitor).map_err(|err| match err {
        StoreError::Conflict => StatusCode::CONFLICT,
        StoreError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((
        StatusCode::OK,
        Json(CreateMonitorResponse {
            monitor_id: monitor.id,
            message: "Monitor Created Successfully".to_string(),
        }),
    ))
}
