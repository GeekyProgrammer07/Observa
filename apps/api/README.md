# api

The HTTP API server for Observa. Handles user auth, monitor management, and notification channel configuration. Also runs a background task that dispatches active monitors to the Redis stream on a fixed schedule.

## Responsibilities

- User signup/signin with JWT auth and argon2 password hashing
- CRUD for monitors (URLs to watch) and notification channels
- Background loop: every 60s, fetches all non-paused monitors from Postgres and pushes them onto the Redis stream for workers to consume

## Stack

- [Poem](https://github.com/poem-web/poem) — HTTP framework
- [Diesel](https://diesel.rs/) + PostgreSQL — data layer via the shared `store` crate
- Redis Streams — dispatching monitors to workers
- Tokio — async runtime (multi-thread flavor)
- `tracing` + `tracing-subscriber` — structured logging

## Environment variables

| Variable | Description |
|---|---|
| `DATABASE_URL` | Postgres connection string |
| `JWT_SECRET` | Secret used to sign and verify JWTs |
| `REDIS_URL` | Redis connection URL (e.g. `redis://127.0.0.1:6379`) |
| `STREAM_KEY` | Redis stream key to push monitors onto (e.g. `observa:main`) |

Create a `.env` file in `apps/api/` or export these in your shell.

## Running

```bash
cargo run -p api
```

Starts on `http://0.0.0.0:3000`.

## Log verbosity

Defaults to `info`. Set `RUST_LOG` to override:

```bash
RUST_LOG=debug cargo run -p api
```

## Routes

All routes are under `/api/v1`.

| Method | Path | Auth | Description |
|---|---|---|---|
| POST | `/signup` | No | Create account |
| POST | `/signin` | No | Get JWT token |
| GET | `/monitors` | Yes | List monitors |
| POST | `/monitors` | Yes | Create monitor |
| PATCH | `/monitors/:id/pause` | Yes | Pause a monitor |
| PATCH | `/monitors/:id/resume` | Yes | Resume a monitor |
| DELETE | `/monitors/:id` | Yes | Delete a monitor |
| GET | `/notification-channels` | Yes | List channels |
| POST | `/notification-channels` | Yes | Create channel |
| POST | `/notification-channels/:id/verify` | Yes | Verify channel |
| DELETE | `/notification-channels/:id` | Yes | Delete channel |
| GET | `/` | No | Health check |
