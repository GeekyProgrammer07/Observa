use poem::{post, EndpointExt, Route};

use crate::{
    handlers::{
        auth::{signin, signup},
        monitor::create_monitor,
    },
    middleware::auth::auth_middleware,
};

pub fn api_v1_routes() -> Route {
    Route::new()
        .at("/signup", post(signup))
        .at("/signin", post(signin))
        .at(
            "/create_monitor",
            post(create_monitor).around(auth_middleware),
        )
}

pub fn routes() -> Route {
    Route::new().nest("/api/v1", api_v1_routes())
}
