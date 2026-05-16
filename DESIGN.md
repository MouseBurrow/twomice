# TwoMice Redesign Plan

## Current State
A 3-microservice Rust backend (gateway, auth, post) using actix-web with shared libraries in a Cargo workspace. Business logic lives in PostgreSQL functions. Docker builds copy all shared crates to every service.

## Goals
- True microservice isolation (separate DB per service, no cross-schema references)
- Faster, independent per-service builds (no shared target dir)
- Switch to axum for performance and ecosystem alignment
- Move business logic from PL/pgSQL to Rust
- Standalone database project (local dev + prod parity)
- Extensible — stub services for future features

---

## Phase 1: Project Structure

### New Layout
```
TwoMice/
├── Cargo.toml              # root metadata only, NO workspace
├── libs/                   # shared crates (renamed from shared/)
│   ├── easy_db/
│   ├── easy_errors/
│   ├── custom_headers/
│   └── config/
├── services/
│   ├── gateway/            # API gateway
│   ├── auth/               # auth service
│   ├── post/               # post service
│   ├── moderation/         # STUB — moderation/reports
│   ├── social/             # STUB — friendships, messaging
│   └── social-feed/        # STUB — feed/follow
├── db/                     # standalone Rust CLI project
│   ├── Cargo.toml          # own dependencies (sqlx, clap, etc.)
│   ├── src/
│   │   └── main.rs         # CLI that manages migrations per service
│   ├── migrations/         # all service migrations, one dir per service
│   │   ├── auth/
│   │   ├── post/
│   │   ├── moderation/
│   │   └── ...
│   └── compose.yaml        # spins up one postgres per service
├── scripts/
│   └── build-all.sh
└── Makefile
```

### Key Changes
- **No Cargo workspace** — each service is standalone with path deps to `../../libs/*`
- **No shared target dir** — `cargo build` in `services/auth/` only builds auth deps
- **Docker only copies what each service needs** — gateway copies nothing from libs/
- **`libs/` replaces `shared/`** to avoid confusion with Docker shared volumes
- **`db/` is a standalone Cargo project** — handles creating, migrating, and maintaining per-service databases; runs via CLI, not as a service

---

## Phase 2: Framework Switch (actix-web → axum)

### Rationale
- Axum is faster, more modern, better async ergonomics
- The log filters already reference axum (it was always the intended direction)
- Tower middleware ecosystem is richer

### Migration per service
- Replace `actix-web::App` with `axum::Router`
- Replace `actix-web::HttpResponse` with `axum::response::IntoResponse`
- Replace `actix_web::web::Data` with `axum::Extension`
- Replace `actix_web::FromRequest` extractors with axum's `FromRequestParts`/`FromRequest`
- Update `custom_headers` crate to axum extractor patterns
- Update `easy_errors` `ResponseError` impl for axum compatibility
- Update `config` crate's `launch_service!` macro to axum
- Replace `awc` HTTP client with `reqwest` in gateway

---

## Phase 3: Business Logic Migration

### Current
```sql
-- PL/pgSQL functions in migrations
CREATE OR REPLACE FUNCTION create_account(...) RETURNS TEXT ...
```
### Target
```rust
// Rust service logic
impl AuthService {
    pub async fn create_account(&self, username: &str, password: &str) -> Result<String> {
        let hash = hash_password(password)?;
        let account_id = sqlx::query_scalar::<_, Uuid>("INSERT INTO ... RETURNING id")
            .execute(&self.pool).await?;
        let token = self.create_session(account_id).await?;
        Ok(token)
    }
}
```

### Approach
- Extract src/routes/* handler functions into business-logic modules (e.g., `src/service.rs`)
- Handlers call service methods; services call the DB directly via sqlx
- Keep validation/complex queries as sqlx queries (no ORM)
- Remove the `easy_db` proc-macro — plain sqlx with `query_as`/`query_scalar` is clearer

---

## Phase 4: Database Isolation

### Current
- Post service references `auth.accounts` via cross-schema foreign key
- Auth/post schemas live in the same database

### Target
- **Each service gets its own Postgres database**
- Auth service connects to `auth-db`, post service to `post-db`, etc.
- No cross-service foreign keys — services communicate via the gateway
- `db/` project holds all migrations, one subdirectory per service
- Docker compose spins up postgres per service

```
services:
  auth-db:
    image: postgres:16
    environment:
      POSTGRES_DB: auth
  post-db:
    image: postgres:16
    environment:
      POSTGRES_DB: post
```

### Cross-service consistency
- Auth service exposes user existence as an API endpoint (the gateway already does this)
- Post service queries auth service via gateway when it needs to verify a user
- No direct DB access between services

---

## Phase 5: Docker & Build Isolation

### Per-service Dockerfile pattern
Each Dockerfile:
1. Only copies its own `Cargo.toml`, `Cargo.lock`, and the `libs/` it depends on
2. Builds only its own binary via `-p service_name`
3. Copies only the binary into a slim runtime image

Gateway Dockerfile copies nothing from `libs/` (zero shared deps).

### Docker Compose
- One compose file for all services + databases
- Dev mode: each service runs with `cargo-watch` mounted from host
- Prod mode: pre-built images

---

## Phase 6: Stub Services

Create minimal shells for future services:

### moderation
- Endpoints: `POST /reports`, `GET /reports`, `POST /action`
- DB schema: reports, actions, mutes, bans

### social
- Endpoints: `POST /friend-request`, `POST /friend-accept`, `GET /friends`
- DB schema: friend_requests, friendships

### social-feed
- Endpoints: `GET /feed`, `POST /feed/preferences`
- DB schema: follows, feed_items

Each stub has: `Cargo.toml`, `src/main.rs` with a health-check endpoint, Dockerfile, and migrations.

---

## Phase 7: Bug Fixes & Cleanup

- Fix `migrator/src/main.rs:11` — `"post" => "SET search_path TO post"`
- Fix logger filter strings (remove axum/tower_http references, add correct targets)
- Remove dead code (commented-out `reply_a_reply`)
- Remove unused `Logger` field from `AppData`
- Remove `regex` dependency — replace with simple char validation
- Strip internal headers in gateway before forwarding to client
- Make auth service URL configurable in gateway
- Remove `launch_service!` macro — explicit HttpServer code per service

---

## Phase 8: Quality of Life

- `scripts/build-all.sh` — builds all services sequentially
- `Makefile` — aliases for common tasks (`make dev`, `make build-auth`, etc.)
- Root `Cargo.toml` with metadata only
- Update `.env.example` and `.gitignore`
- Remove `.idea/` from repo

---

## Order of Execution

We'll go through each phase sequentially, creating commits as we go. Each phase is self-contained and reviewable.

| # | Phase | Description |
|---|-------|-------------|
| 1 | Restructure | New layout, no workspace, rename shared/ → libs/ |
| 2 | DB isolation | Standalone db project, per-service postgres |
| 3 | Fix bugs | Critical bugs + cleanup |
| 4 | Switch to axum | Replace actix-web with axum |
| 5 | Move logic to Rust | Extract PL/pgSQL into Rust services |
| 6 | Stub services | moderation, social, social-feed |
| 7 | Build/Docker | Per-service Dockerfiles, compose |
| 8 | QoL | Scripts, Makefile, cleanup |
