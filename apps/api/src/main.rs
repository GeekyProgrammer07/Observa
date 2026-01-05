use std::sync::Arc;

use poem::{get, listener::TcpListener, EndpointExt, Result, Server};
use store::store::Store;

use crate::handlers::health::health_check;

mod config;
mod error;
mod handlers;
mod models;
mod routes;
mod auth;
mod middleware;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let store = Arc::new(Store::new().expect("Failed to initialize Store"));
    let config = Arc::new(config::Config::from_env().expect("Invalid Config"));

    let app = routes::routes()
        .at("/", get(health_check))
        .data(store)
        .data(config);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
