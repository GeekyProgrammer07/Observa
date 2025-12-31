use poem::{get, listener::TcpListener, Result, Server};
use store::store::Store;

use crate::handlers::health_check;

mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = routes::routes().at("/", get(health_check));

    let mut conn = Store::connect().unwrap_or_else(|err| panic!("database conn error: {:?}", err));

    // Store::create_user(&mut conn);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}