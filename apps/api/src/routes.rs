use poem::{delete, get, patch, post, EndpointExt, Route};

use crate::{
    handlers::{
        auth::{signin, signup},
        monitor::{create_monitor, delete_monitor, get_monitor, pause_monitor, resume_monitor},
    },
    middleware::auth::auth_middleware,
};

pub fn api_v1_routes() -> Route {
    Route::new()
        .at("/signup", post(signup))
        .at("/signin", post(signin))
        .at(
            "/monitors",
            get(get_monitor)
                .post(create_monitor)
                .around(auth_middleware),
        )
        .at(
            "/monitors/:monitor_id/pause",
            patch(pause_monitor).around(auth_middleware),
        )
        .at(
            "/monitors/:monitor_id/resume",
            patch(resume_monitor).around(auth_middleware),
        )
        .at(
            "/monitors/:monitor_id",
            delete(delete_monitor).around(auth_middleware),
        )
}

pub fn routes() -> Route {
    Route::new().nest("/api/v1", api_v1_routes())
}
