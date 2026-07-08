# store

Shared data layer for Observa. Wraps Diesel + PostgreSQL and exposes a typed `Store` struct with methods used by both the `api` and `worker` crates.

## What's in here

- `Store` — connection pool wrapper (`r2d2` + `PgConnection`)
- Models and DB operations for:
  - `monitor` — create, list, pause, resume, delete, fetch active monitors for the scheduler
  - `checks` — insert check results (status + response time + region)
  - `user` — create and look up users
  - `sessions` — session management
  - `notification` — notification channel CRUD
- `schema.rs` — Diesel-generated schema (do not edit by hand)
- `StoreError` — shared error type (`Conflict`, `NotFound`, `Internal`, `Unauthorized`)

## Environment variables

| Variable | Description |
|---|---|
| `DATABASE_URL` | Postgres connection string (e.g. `postgres://postgres:mysecret@localhost:5432/observa-main`) |

## Migrations

Migrations live in `packages/store/migrations/`. Run them with:

```bash
cd packages/store
diesel migration run
```

To create a new migration:

```bash
diesel migration generate <name>
```

## Usage

Add it as a path dependency:

```toml
store = { path = "../../packages/store" }
```

Then:

```rust
use store::store::Store;

let store = Store::new().expect("failed to init store");
let mut conn = store.pool.get().expect("failed to get connection");
```
