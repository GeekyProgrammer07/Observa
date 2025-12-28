use poem::{post, Route};

use crate::handlers::create_website;

pub fn api_v1_routes() -> Route {
    Route::new().at("/create", post(create_website))
}

pub fn routes() -> Route {
    Route::new().nest("/api/v1", api_v1_routes())
}
