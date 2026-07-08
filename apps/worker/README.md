# worker

The background check worker for Observa. Reads monitor jobs from a Redis consumer group, performs the HTTP health check, and writes the result (status + response time) to Postgres.

## Responsibilities

- Consume monitor jobs from the Redis stream using a consumer group
- Perform an HTTP GET against each monitor's URL with a 5s timeout
- Classify the result: any 5xx response or connection failure = **Down**, everything else (2xx, 3xx, 4xx) = **Up**
- Write a check record (status + response time + region) to Postgres via the shared `store` crate
- Acknowledge the stream entry after a successful write

## Stack

- [Tokio](https://tokio.rs/) — async runtime
- [reqwest](https://docs.rs/reqwest) — HTTP client
- [redis](https://docs.rs/redis) — Redis streams / consumer groups
- [Diesel](https://diesel.rs/) + PostgreSQL — writing check results via `store`
- `tracing` + `tracing-subscriber` — structured logging

## Environment variables

| Variable | Description |
|---|---|
| `DATABASE_URL` | Postgres connection string |
| `REDIS_URL` | Redis connection URL (e.g. `redis://127.0.0.1:6379`) |
| `STREAM_KEY` | Redis stream key to consume from (must match the API's `STREAM_KEY`) |
| `CONSUMER_GROUP` | Redis consumer group name (e.g. `india`) |
| `REGION_ID` | UUID identifying the region this worker represents |

Create a `.env` file in `apps/worker/` or export these in your shell.

## Running

```bash
cargo run -p worker
```

Multiple worker instances can run simultaneously with the same `CONSUMER_GROUP` — Redis will distribute entries across them. Use different `REGION_ID` values per instance to track which region performed each check.

## Consumer group bootstrap

The worker automatically creates the consumer group and stream on startup if they don't exist. If you ever flush Redis and need to reset manually:

```bash
redis-cli XGROUP CREATE <STREAM_KEY> <CONSUMER_GROUP> $ MKSTREAM
```

Use `$` to process only new messages, or `0` to reprocess everything already in the stream.

## Log verbosity

Defaults to `info`. Set `RUST_LOG` to override:

```bash
RUST_LOG=debug cargo run -p worker
```
