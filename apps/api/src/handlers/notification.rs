use std::sync::Arc;

use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json, Path},
    Request,
};
use store::{error::StoreError, models::notification::NewChannel, store::Store};
use uuid::Uuid;

use crate::models::{
    CreateNotificationChannelRequest, CreateNotificationChannelResponse,
    GetNotificationChannelResponse, VerifyNotificationChannelResponse,
};

#[handler]
pub fn create_notification_channel(
    Json(body): Json<CreateNotificationChannelRequest>,
    Data(store): Data<&Arc<Store>>,
    req: &Request,
) -> Result<(StatusCode, Json<CreateNotificationChannelResponse>), StatusCode> {
    let mut conn = store
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uid = req
        .extensions()
        .get::<Uuid>()
        .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_channel = NewChannel {
        user_id: *uid,
        type_: body.channel_type,
        value: body.value,
    };

    let notification_channel =
        Store::add_channel(&mut conn, new_channel).map_err(|err| match err {
            StoreError::Conflict => StatusCode::CONFLICT,
            StoreError::NotFound => StatusCode::NOT_FOUND,
            _ => {
                println!("{:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    Ok((
        StatusCode::OK,
        Json(CreateNotificationChannelResponse {
            channel_id: notification_channel.id,
        }),
    ))
}

#[handler]
pub fn get_notification_channel(
    Data(store): Data<&Arc<Store>>,
    req: &Request,
) -> Result<(StatusCode, Json<Vec<GetNotificationChannelResponse>>), StatusCode> {
    let mut conn = store
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uid = req
        .extensions()
        .get::<Uuid>()
        .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR)?;

    let notification_channels =
        Store::list_channels_by_user(&mut conn, *uid).map_err(|err| match err {
            StoreError::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    let response: Vec<GetNotificationChannelResponse> = notification_channels
        .into_iter()
        .map(|c| GetNotificationChannelResponse {
            id: c.id,
            channel_type: c.type_,
            value: c.value,
            verified: c.verified,
            created_at: c.created_at,
        })
        .collect();

    Ok((StatusCode::OK, Json(response)))
}

//TODO: Verify the channel truly with notification
#[handler]
pub fn verify_channel(
    Data(store): Data<&Arc<Store>>,
    req: &Request,
    Path(channel_id): Path<Uuid>,
) -> Result<(StatusCode, Json<VerifyNotificationChannelResponse>), StatusCode> {
    let mut conn = store
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uid = req
        .extensions()
        .get::<Uuid>()
        .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR)?;

    Store::verify_channel(&mut conn, *uid, channel_id).map_err(|err| match err {
        StoreError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((
        StatusCode::OK,
        Json(VerifyNotificationChannelResponse {
            message: "Verified".to_string(),
        }),
    ))
}

#[handler]
pub fn delete_channel(
    Data(store): Data<&Arc<Store>>,
    req: &Request,
    Path(channel_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut conn = store
        .pool
        .get()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uid = req
        .extensions()
        .get::<Uuid>()
        .ok_or_else(|| StatusCode::INTERNAL_SERVER_ERROR)?;

    Store::delete_channel(&mut conn, *uid, channel_id).map_err(|err| match err {
        StoreError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok(StatusCode::OK)
}
