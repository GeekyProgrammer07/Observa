use std::{str::FromStr, time::Instant};

use redis::{
    streams::{StreamId, StreamReadOptions, StreamReadReply},
    Commands,
};
use reqwest::StatusCode;
use store::{
    error::StoreError,
    models::checks::{MonitorStatusType, NewCheck},
    store::Store,
};
use tokio::time::Duration;
use uuid::Uuid;

use crate::error::WorkerError;

mod config;
mod error;

#[tokio::main]
async fn main() -> Result<(), WorkerError> {
    let worker_uuid = Uuid::new_v4();

    let mut conn = Store::new()
        .expect("Failed to Initialize Store")
        .pool
        .get()
        .expect("Get Connection To Database");

    let config = config::Config::from_env().expect("Environment Varaibles loaded");

    let mut r = redis::Client::open(config.redis_url)
        .expect("Connection Success")
        .get_connection()
        .expect("Connected");

    let opts = StreamReadOptions::default()
        .group(
            config.consumer_group.as_str(),
            format!("worker-{}", worker_uuid),
        )
        .count(2)
        .block(10000);

    let stream_key = config.stream_key;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .map_err(|_| WorkerError::RedisError)?;

    let reg_id = Uuid::from_str(&config.region_id).map_err(|_| WorkerError::Internal)?;

    loop {
        let reply: StreamReadReply = r
            .xread_options(&[&stream_key], &[">"], &opts)
            .map_err(|_| WorkerError::RedisError)?;

        if reply.keys.is_empty() {
            continue;
        }

        for key in reply.keys {
            for id in key.ids {
                let url: String = StreamId::get(&id, "url").ok_or(WorkerError::Internal)?;

                let m_id = StreamId::get(&id, "monitor_id")
                    .ok_or(WorkerError::NotFound)
                    .and_then(|v: String| Uuid::from_str(&v).map_err(|_| WorkerError::Internal))?;

                let start = Instant::now();

                let body = client
                    .get(url)
                    .send()
                    .await
                    .map_err(|_| WorkerError::Internal)?;

                let elapsed = start.elapsed();
                if body.status() == StatusCode::OK {
                    Store::create_check(
                        &mut conn,
                        NewCheck {
                            response_time_ms: elapsed.as_millis() as i32,
                            status: MonitorStatusType::Up,
                            region_id: reg_id,
                            monitor_id: m_id,
                        },
                    )
                    .map_err(|err| match err {
                        StoreError::Conflict => WorkerError::Conflict,
                        StoreError::NotFound => WorkerError::NotFound,
                        _ => WorkerError::Internal,
                    })?;
                } else {
                    Store::create_check(
                        &mut conn,
                        NewCheck {
                            response_time_ms: 5000,
                            status: MonitorStatusType::Down,
                            region_id: reg_id,
                            monitor_id: m_id,
                        },
                    )
                    .map_err(|err| match err {
                        StoreError::Conflict => WorkerError::Conflict,
                        StoreError::NotFound => WorkerError::NotFound,
                        _ => WorkerError::Internal,
                    })?;
                }
                let _: () = r
                    .xack(&stream_key, config.consumer_group.as_str(), &[id.id])
                    .map_err(|_| WorkerError::Internal)?;
            }
        }
    }
}
