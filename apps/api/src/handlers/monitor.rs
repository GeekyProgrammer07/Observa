use std::sync::Arc;

use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json, Path},
    Request,
};
use store::{error::StoreError, models::monitor::NewMonitor, store::Store};
use uuid::Uuid;

use crate::models::{
    CreateMonitor, CreateMonitorResponse, GetMonitorResponse, MonitorActionResponse,
};

#[handler]
pub fn create_monitor(
    Json(body): Json<CreateMonitor>,
    Data(store): Data<&Arc<Store>>,
    req: &Request,
) -> Result<(StatusCode, Json<CreateMonitorResponse>), StatusCode> {
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
        }),
    ))
}

#[handler]
pub fn get_monitor(
    Data(store): Data<&Arc<Store>>,
    req: &Request,
) -> Result<(StatusCode, Json<Vec<GetMonitorResponse>>), StatusCode> {
    let mut conn = store
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uid = req
        .extensions()
        .get::<Uuid>()
        .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR)?;

    let monitors = Store::list_monitors_by_user(&mut conn, *uid).map_err(|err| match err {
        StoreError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let response: Vec<GetMonitorResponse> = monitors
        .into_iter()
        .map(|m| GetMonitorResponse {
            id: m.id,
            url: m.url,
            name: m.name,
            interval: m.interval,
            timeout_ms: m.timeout_ms,
            is_paused: m.is_paused,
            created_at: m.created_at,
        })
        .collect();

    Ok((StatusCode::OK, Json(response)))
}

#[handler]
pub fn pause_monitor(
    Data(store): Data<&Arc<Store>>,
    req: &Request,
    Path(monitor_id): Path<Uuid>,
) -> Result<(StatusCode, Json<MonitorActionResponse>), StatusCode> {
    let mut conn = store
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uid = req
        .extensions()
        .get::<Uuid>()
        .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR)?;

    Store::pause_monitor(&mut conn, monitor_id, *uid).map_err(|err| match err {
        StoreError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((
        StatusCode::OK,
        Json(MonitorActionResponse {
            message: "paused".to_string(),
        }),
    ))
}

#[handler]
pub fn resume_monitor(
    Data(store): Data<&Arc<Store>>,
    req: &Request,
    Path(monitor_id): Path<Uuid>,
) -> Result<(StatusCode, Json<MonitorActionResponse>), StatusCode> {
    let mut conn = store
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uid = req
        .extensions()
        .get::<Uuid>()
        .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR)?;

    Store::resume_monitor(&mut conn, monitor_id, *uid).map_err(|err| match err {
        StoreError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((
        StatusCode::OK,
        Json(MonitorActionResponse {
            message: "resumed".to_string(),
        }),
    ))
}

#[handler]
pub fn delete_monitor(
    Data(store): Data<&Arc<Store>>,
    req: &Request,
    Path(monitor_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = store
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uid = req
        .extensions()
        .get::<Uuid>()
        .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR)?;

    Store::delete_monitor(&mut conn, monitor_id, *uid).map_err(|err| match err {
        StoreError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(StatusCode::NO_CONTENT)
}
