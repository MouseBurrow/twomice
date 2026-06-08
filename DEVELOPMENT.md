# Development

Each backend service runs inside its own Docker container. In development, services hot-reload via `cargo-watch`. In production they compile with `--release`.

## Requirements

- Docker and Docker Compose
- Rust (optional — only needed for native development without Docker)
- Copy `.env.example` to `.env` and fill in your database URLs

## Running

**Docker (recommended)**

```sh
docker compose -f docker-compose.dev.yaml up --build
```

Starts all services, databases, and the frontend dev server. The frontend is available at `http://localhost:5173`.

**Native**

```sh
make dev-native
```

Requires tmux. Starts PostgreSQL locally and all services via `cargo-watch`.

## Database Migrations

```sh
make migrate              # run all pending migrations
./db run <service>        # migrate a single service
./db seed                 # seed test data (mouse/alice/bob, password: testpass123)
```

## Usage

All API traffic goes through `http://localhost:8080` (the gateway). The frontend proxies `/api` to the gateway automatically in dev.

Internal service URLs follow `http://<service-name>:8080` within Docker networking. These are never exposed directly.

## Session Handling

Each user gets a session token on login, stored as an HTTP-only cookie. Every request to the gateway is validated against this token before being forwarded.

```
Request comes in
└─> Check cookie
    ├─ missing         → return 401
    └─ present
       ├─ check cache  → valid   → continue
       ├─ check cache  → invalid → return 401
       └─ not in cache → call auth service /validate
          ├─ success   → cache (1hr TTL) and continue
          └─ error     → return 401
```

## Repository Structure

Each service lives in its own git repository under the monorepo layout:

| Directory | Repo |
|---|---|
| `./` | [MouseBurrow/twomice](https://github.com/MouseBurrow/twomice) |
| `frontend/` | [MouseBurrow/twomice-frontend](https://github.com/MouseBurrow/twomice-frontend) |
| `libs/` | [MouseBurrow/twomice-libs](https://github.com/MouseBurrow/twomice-libs) |
| `services/auth/` | [MouseBurrow/twomice-auth](https://github.com/MouseBurrow/twomice-auth) |
| `services/gateway/` | [MouseBurrow/twomice-gateway](https://github.com/MouseBurrow/twomice-gateway) |
| `services/post/` | [MouseBurrow/twomice-post](https://github.com/MouseBurrow/twomice-post) |
| `services/moderation/` | [MouseBurrow/twomice-moderation](https://github.com/MouseBurrow/twomice-moderation) |
| `services/social/` | [MouseBurrow/twomice-social](https://github.com/MouseBurrow/twomice-social) |
| `services/social-feed/` | [MouseBurrow/twomice-social-feed](https://github.com/MouseBurrow/twomice-social-feed) |
| `tools/git-dashboard/` | [MouseBurrow/twomice-dashboard](https://github.com/MouseBurrow/twomice-dashboard) |

Shared Rust libraries (`config`, `custom_headers`, `easy_errors`, `utils`) are consumed as git dependencies from `twomice-libs`.

## Deployment

Production deployment uses Docker Compose with Caddy as the reverse proxy. A GitHub Actions workflow triggers an SSH deploy on pushes to `main` when files under `deploy/` change.

```sh
./deploy/deploy.sh <service> prod   # deploy a single service
```

The live instance is hosted at **[twomice.skimnerphi.net](https://twomice.skimnerphi.net)**.
