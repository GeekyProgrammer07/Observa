# Observa

Observa is a self-hosted uptime and API monitoring service. It periodically checks the URLs you register, tracks their status and response times, and notifies you when something goes down — built with a Rust backend designed around a simple, scalable check-and-alert pipeline.

> ⚠️ **Status:** Backend (API + worker + store) is functional. The frontend (`apps/web`) is scaffolded but not yet implemented — this project currently runs and is tested via the API directly (e.g. with curl/Postman).

## How it works

Observa is an Nx-managed monorepo with three Rust crates and a shared store:

```
apps/
  api/      -> HTTP API server (Poem framework) — auth, monitors, notification channels
  worker/   -> Background worker(s) that perform the actual HTTP health checks
  web/      -> Frontend (not yet implemented)
packages/
  store/    -> Shared data layer (Diesel + PostgreSQL) used by both api and worker
```

The flow looks like this:

1. **API** lets a user sign up, sign in, and register monitors (URLs to watch) and notification channels (email, SMS, voice call, webhook).
2. A background task inside the API pushes due monitors onto a **Redis Stream** on a fixed schedule.
3. One or more **worker** processes consume that stream (using a Redis consumer group, so checks can be scaled horizontally across regions), perform the actual HTTP request, measure response time, and write the result back to Postgres via the shared `store` crate.
4. Check results are stored per monitor/region, so uptime history and latency can be queried back out through the API.

## Tech stack

- **Language:** Rust (edition 2021)
- **API framework:** [Poem](https://github.com/poem-web/poem)
- **Database:** PostgreSQL via [Diesel](https://diesel.rs/)
- **Queue:** Redis Streams (consumer groups) for distributing checks to workers
- **Auth:** JWT (`jsonwebtoken`) + password hashing with `argon2`
- **Async runtime:** Tokio
- **Monorepo tooling:** [Nx](https://nx.dev/) + pnpm workspaces (for the eventual frontend)

## API overview

All routes are namespaced under `/api/v1`.

| Method | Route | Description |
|---|---|---|
| POST | `/signup` | Create a new user |
| POST | `/signin` | Authenticate and receive an access token |
| GET | `/monitors` | List monitors |
| POST | `/monitors` | Create a monitor |
| PATCH | `/monitors/:monitor_id/pause` | Pause a monitor |
| PATCH | `/monitors/:monitor_id/resume` | Resume a monitor |
| DELETE | `/monitors/:monitor_id` | Delete a monitor |
| GET | `/notification-channels` | List notification channels |
| POST | `/notification-channels` | Create a notification channel |
| POST | `/notification-channels/:channel_id/verify` | Verify a channel |
| DELETE | `/notification-channels/:channel_id` | Delete a channel |
| GET | `/` | Health check |

## Getting started

### Prerequisites

- Rust (version pinned in `rust-toolchain.toml`)
- Docker (for Postgres + Redis)

### 1. Start dependencies

```bash
docker compose up -d
```

This spins up:
- **Postgres** on `localhost:5432` (db: `observa-main`)
- **Redis** on `localhost:6379`

### 2. Configure environment

Create a `.env` file for the API (`apps/api`) with at least:

```env
JWT_SECRET=your-secret-here
REDIS_URL=redis://127.0.0.1:6379
STREAM_KEY=monitor-checks
DATABASE_URL=postgres://postgres:mysecret@localhost:5432/observa-main
```

And one for the worker (`apps/worker`):

```env
REDIS_URL=redis://127.0.0.1:6379
STREAM_KEY=monitor-checks
CONSUMER_GROUP=monitor-workers
REGION_ID=<a-valid-uuid>
DATABASE_URL=postgres://postgres:mysecret@localhost:5432/observa-main
```

### 3. Run database migrations

```bash
cd packages/store
diesel migration run
```

### 4. Run the API

```bash
cargo run -p api
```

The API will start on `http://0.0.0.0:3000`.

### 5. Run the worker

```bash
cargo run -p worker
```

You can run multiple worker instances (even with different `REGION_ID`s) to simulate distributed, multi-region checking.

## Roadmap

- [ ] Frontend dashboard (`apps/web`) for managing monitors and viewing uptime history
- [ ] Actual outbound notification delivery (email/SMS/voice/webhook firing on incidents)
- [ ] Incident/status history views
- [ ] Multi-region check aggregation and reporting
