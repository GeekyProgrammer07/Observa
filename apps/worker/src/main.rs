use std::{str::FromStr, time::Instant};

use redis::{
    streams::{StreamId, StreamReadOptions, StreamReadReply},
    Commands,
};
use store::{
    error::StoreError,
    models::checks::{MonitorStatusType, NewCheck},
    store::Store,
};
use tokio::time::Duration;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::error::WorkerError;

mod config;
mod error;

#[tokio::main]
async fn main() -> Result<(), WorkerError> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let worker_uuid = Uuid::new_v4();
    info!(worker_id = %worker_uuid, "Worker starting up");

    let config = config::Config::from_env().map_err(|e| {
        error!(error = %e, "Failed to load environment config");
        WorkerError::Internal
    })?;
    info!(
        stream_key = %config.stream_key,
        consumer_group = %config.consumer_group,
        region_id = %config.region_id,
        "Config loaded"
    );

    let mut conn = Store::new()
        .map_err(|e| {
            error!(error = ?e, "Failed to initialize database store");
            WorkerError::Internal
        })?
        .pool
        .get()
        .map_err(|e| {
            error!(error = %e, "Failed to get database connection from pool");
            WorkerError::Internal
        })?;
    info!("Database connection established");

    let redis_url_display = config.redis_url.split('@').last().unwrap_or("(hidden)");
    let mut r = redis::Client::open(config.redis_url.clone())
        .map_err(|e| {
            error!(error = %e, url = %redis_url_display, "Failed to open Redis client");
            WorkerError::RedisError(e.to_string())
        })?
        .get_connection()
        .map_err(|e| {
            error!(error = %e, url = %redis_url_display, "Failed to connect to Redis — is Redis running?");
            WorkerError::RedisError(e.to_string())
        })?;
    info!(url = %redis_url_display, "Redis connection established");

    // Auto-create the consumer group. BUSYGROUP means it already exists — that's fine.
    let stream_key = config.stream_key.clone();
    let group_name = config.consumer_group.clone();
    let create_result: redis::RedisResult<()> = redis::cmd("XGROUP")
        .arg("CREATE")
        .arg(&stream_key)
        .arg(&group_name)
        .arg("$")
        .arg("MKSTREAM")
        .query(&mut r);

    match create_result {
        Ok(_) => info!(stream = %stream_key, group = %group_name, "Consumer group created"),
        Err(e) if e.to_string().contains("BUSYGROUP") => {
            info!(stream = %stream_key, group = %group_name, "Consumer group already exists, skipping creation");
        }
        Err(e) => {
            error!(error = %e, stream = %stream_key, group = %group_name, "Failed to create consumer group");
            return Err(WorkerError::RedisError(e.to_string()));
        }
    }

    let opts = StreamReadOptions::default()
        .group(
            config.consumer_group.as_str(),
            format!("worker-{}", worker_uuid),
        )
        .count(2)
        .block(10000);

    let consumer_name = format!("worker-{}", worker_uuid);
    info!(
        consumer = %consumer_name,
        stream = %stream_key,
        group = %config.consumer_group,
        "Joining consumer group — if this errors with NOGROUP, the group does not exist yet (run XGROUP CREATE)"
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|e| {
            error!(error = %e, "Failed to build HTTP client");
            WorkerError::Internal
        })?;

    let reg_id = Uuid::from_str(&config.region_id).map_err(|e| {
        error!(error = %e, region_id = %config.region_id, "Invalid REGION_ID — must be a valid UUID");
        WorkerError::Internal
    })?;

    info!("Worker ready, entering event loop");

    loop {
        let reply: StreamReadReply = match r.xread_options(&[&stream_key], &[">"], &opts) {
            Ok(r) => r,
            Err(e) => {
                error!(
                    error = %e,
                    stream = %stream_key,
                    group = %config.consumer_group,
                    "xread_options failed — common causes: consumer group does not exist (NOGROUP), \
                     Redis connection dropped, or stream key is wrong"
                );
                return Err(WorkerError::RedisError(e.to_string()));
            }
        };

        if reply.keys.is_empty() {
            continue;
        }

        for key in reply.keys {
            let entry_count = key.ids.len();
            info!(stream = %key.key, count = entry_count, "Processing stream entries");

            for id in key.ids {
                let url: String = match StreamId::get(&id, "url") {
                    Some(u) => u,
                    None => {
                        warn!(entry_id = %id.id, "Stream entry missing 'url' field, skipping");
                        continue;
                    }
                };

                let m_id = match StreamId::get(&id, "monitor_id")
                    .ok_or(WorkerError::NotFound)
                    .and_then(|v: String| {
                        Uuid::from_str(&v).map_err(|e| {
                            error!(error = %e, raw = %v, "monitor_id in stream entry is not a valid UUID");
                            WorkerError::Internal
                        })
                    }) {
                    Ok(id) => id,
                    Err(e) => {
                        warn!(entry_id = %id.id, error = %e, "Skipping entry with invalid monitor_id");
                        continue;
                    }
                };

                info!(monitor_id = %m_id, url = %url, "Checking monitor");
                let start = Instant::now();

                let response = match client.get(&url).send().await {
                    Ok(r) => r,
                    Err(e) => {
                        warn!(monitor_id = %m_id, url = %url, error = %e, "HTTP request failed, recording as Down");
                        Store::create_check(
                            &mut conn,
                            NewCheck {
                                response_time_ms: 5000,
                                status: MonitorStatusType::Down,
                                region_id: reg_id,
                                monitor_id: m_id,
                            },
                        )
                        .map_err(|err| {
                            error!(monitor_id = %m_id, error = ?err, "Failed to write Down check to DB");
                            match err {
                                StoreError::Conflict => WorkerError::Conflict,
                                StoreError::NotFound => WorkerError::NotFound,
                                _ => WorkerError::Internal,
                            }
                        })?;
                        let _: () = r
                            .xack(&stream_key, config.consumer_group.as_str(), &[&id.id])
                            .map_err(|e| {
                                error!(entry_id = %id.id, error = %e, "xack failed after HTTP error");
                                WorkerError::RedisError(e.to_string())
                            })?;
                        continue;
                    }
                };

                let elapsed = start.elapsed();
                let http_status = response.status();

                // 5xx and connection errors = Down. Everything else (2xx/3xx/4xx) = Up.
                // A 401/403 means the server is alive and responding — not a failure.
                let is_up = !http_status.is_server_error();

                if is_up {
                    info!(
                        monitor_id = %m_id,
                        url = %url,
                        status = %http_status,
                        response_ms = elapsed.as_millis(),
                        "Monitor is UP"
                    );
                    Store::create_check(
                        &mut conn,
                        NewCheck {
                            response_time_ms: elapsed.as_millis() as i32,
                            status: MonitorStatusType::Up,
                            region_id: reg_id,
                            monitor_id: m_id,
                        },
                    )
                    .map_err(|err| {
                        error!(monitor_id = %m_id, error = ?err, "Failed to write Up check to DB");
                        match err {
                            StoreError::Conflict => WorkerError::Conflict,
                            StoreError::NotFound => WorkerError::NotFound,
                            _ => WorkerError::Internal,
                        }
                    })?;
                } else {
                    warn!(
                        monitor_id = %m_id,
                        url = %url,
                        status = %http_status,
                        response_ms = elapsed.as_millis(),
                        "Monitor is DOWN (5xx response)"
                    );
                    Store::create_check(
                        &mut conn,
                        NewCheck {
                            response_time_ms: elapsed.as_millis() as i32,
                            status: MonitorStatusType::Down,
                            region_id: reg_id,
                            monitor_id: m_id,
                        },
                    )
                    .map_err(|err| {
                        error!(monitor_id = %m_id, error = ?err, "Failed to write Down check to DB");
                        match err {
                            StoreError::Conflict => WorkerError::Conflict,
                            StoreError::NotFound => WorkerError::NotFound,
                            _ => WorkerError::Internal,
                        }
                    })?;
                }

                let _: () = r
                    .xack(&stream_key, config.consumer_group.as_str(), &[&id.id])
                    .map_err(|e| {
                        error!(entry_id = %id.id, error = %e, "xack failed");
                        WorkerError::RedisError(e.to_string())
                    })?;

                info!(entry_id = %id.id, monitor_id = %m_id, "Entry acknowledged");
            }
        }
    }
}
