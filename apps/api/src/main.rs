use poem::{get, listener::TcpListener, Result, Server};

use crate::handlers::health_check;

mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = routes::routes().at("/", get(health_check));

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}