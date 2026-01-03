use poem::{post, Route};

use crate::handlers::auth::{signin, signup};

pub fn api_v1_routes() -> Route {
    Route::new()
        .at("/signup", post(signup))
        .at("/signin", post(signin))
}

pub fn routes() -> Route {
    Route::new().nest("/api/v1", api_v1_routes())
}
