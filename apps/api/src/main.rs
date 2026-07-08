use std::sync::Arc;

use poem::{get, listener::TcpListener, EndpointExt, Result, Server};
use store::store::Store;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

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
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("API server starting up");

    let store = Arc::new(Store::new().unwrap_or_else(|e| {
        error!(error = ?e, "Failed to initialize Store — check DATABASE_URL");
        std::process::exit(1);
    }));
    info!("Database store initialized");

    let config = Arc::new(config::Config::from_env().unwrap_or_else(|err| match err {
        error::AppError::Env(e) => {
            error!(error = %e, "Environment configuration error");
            std::process::exit(1);
        }
    }));

    let redis_url_display = config
        .redis_url
        .split('@')
        .last()
        .unwrap_or("(hidden)")
        .to_string();

    let mut r = match redis::Client::open(Arc::clone(&config).redis_url.clone()) {
        Ok(client) => match client.get_connection() {
            Ok(conn) => {
                info!(url = %redis_url_display, "Redis connection established");
                conn
            }
            Err(e) => {
                error!(
                    error = %e,
                    url = %redis_url_display,
                    "Failed to connect to Redis — is Redis running and is REDIS_URL correct?"
                );
                std::process::exit(1);
            }
        },
        Err(e) => {
            error!(
                error = %e,
                url = %redis_url_display,
                "Failed to open Redis client — check REDIS_URL format"
            );
            std::process::exit(1);
        }
    };

    let mut conn = store.pool.get().unwrap_or_else(|e| {
        error!(error = %e, "Failed to get DB connection from pool");
        std::process::exit(1);
    });

    let config_for_task = Arc::clone(&config);
    tokio::spawn(async move {
        let stream_key = &config_for_task.stream_key;
        info!(stream = %stream_key, "Monitor dispatcher task started");

        loop {
            let monitors = match Store::get_monitors(&mut conn) {
                Ok(m) => m,
                Err(e) => {
                    error!(error = ?e, "Failed to fetch monitors from DB");
                    sleep(Duration::from_secs(10)).await;
                    continue;
                }
            };

            if monitors.is_empty() {
                info!("No active monitors found, sleeping 30s");
                sleep(Duration::from_secs(30)).await;
                continue;
            }

            info!(count = monitors.len(), stream = %stream_key, "Dispatching monitors to Redis stream");

            let mut pipe = redis::pipe();
            for monitor in &monitors {
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

            match pipe.query::<()>(&mut r) {
                Ok(_) => {
                    info!(count = monitors.len(), stream = %stream_key, "Dispatched monitors to stream");
                }
                Err(e) => {
                    error!(
                        error = %e,
                        stream = %stream_key,
                        "Redis pipeline failed — monitors not dispatched this cycle"
                    );
                    warn!("Will retry in 60s");
                }
            }

            sleep(Duration::from_mins(1)).await;
        }
    });

    info!("Starting HTTP server on 0.0.0.0:3000");
    let app = routes::routes()
        .at("/", get(health_check))
        .data(store)
        .data(config);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("Observa")
        .run(app)
        .await
}
