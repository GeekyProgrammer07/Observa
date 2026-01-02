use poem::{Route, post};

use crate::handlers::auth::signup;


pub fn api_v1_routes() -> Route {
    Route::new().at("/signup", post(signup))
}

pub fn routes() -> Route {
    Route::new().nest("/api/v1", api_v1_routes())
}
