use std::sync::Arc;

use poem::{get, listener::TcpListener, EndpointExt, Result, Server};
use store::store::Store;
use tokio::time::{sleep, Duration};

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

    let mut r = redis::Client::open(Arc::clone(&config).redis_url.clone())
        .expect("Connection Success")
        .get_connection()
        .expect("Connect");

    let mut conn = store.pool.get().expect("Database Connected");

    let config_for_task = Arc::clone(&config);
    tokio::spawn(async move {
        let stream_key = &config_for_task.stream_key;
        loop {
            let monitors = match Store::get_monitors(&mut conn) {
                Ok(m) => m,
                Err(_) => {
                    eprintln!("Failed to fetch monitors");
                    continue;
                }
            };

            if monitors.is_empty() {
                sleep(Duration::from_secs(30)).await;
                continue;
            }

            let mut pipe = redis::pipe();
            for monitor in monitors {
                pipe.cmd("XADD")
                    .arg(stream_key)
                    .arg("MAXLEN")
                    .arg("~")
                    .arg(10000)
                    .arg("*")
                    .arg("url")
                    .arg(&monitor.url)
                    .arg("monitor_id")
                    .arg(monitor.id.to_string());
            }

            pipe.query::<()>(&mut r).expect("Redis pipeline failed");

            sleep(Duration::from_mins(1)).await;
        }
    });

    let app = routes::routes()
        .at("/", get(health_check))
        .data(store)
        .data(config);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("Observa")
        .run(app)
        .await
}
