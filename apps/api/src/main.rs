use std::sync::Arc;

use poem::{get, listener::TcpListener, EndpointExt, Result, Server};
use store::store::Store;

use crate::handlers::health::health_check;

mod auth;
mod config;
mod error;
mod handlers;
mod middleware;
mod models;
mod routes;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {
    let store = Arc::new(Store::new().expect("Failed to initialize Store"));
    let config = Arc::new(config::Config::from_env().unwrap_or_else(|err| match err {
        error::AppError::Env(e) => {
            eprintln!("Environment configuration error: {e}");
            std::process::exit(1);
        }
    }));

    let app = routes::routes()
        .at("/", get(health_check))
        .data(store)
        .data(config);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}
