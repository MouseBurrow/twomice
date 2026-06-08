<div align="center">

# TwoMice

> **Live at [twomice.skimnerphi.net](https://twomice.skimnerphi.net)**

![Language](https://img.shields.io/badge/language-Rust-red?logo=rust)
![License](https://img.shields.io/github/license/TcePrepK/TwoMice)
![Commits](https://img.shields.io/github/commit-activity/t/TcePrepK/TwoMice)  
![Last Commit](https://img.shields.io/github/last-commit/TcePrepK/TwoMice)
![Team](https://img.shields.io/badge/team-TcePrepK%20%7C%20Alaatftin-yellow)

Anonymous Imageboard-Style Social Media Platform  
A web-based social media that combines the anonymity and topic-focused discussion of imageboards with friend connectivity. Users remain anonymous to each other until they mutually befriend each other.
</div>

## Features

- ### User System
    - Username/password accounts with Argon2id password hashing
    - Profile customization
    - Friendship system — mutual befriending reveals each other's profile and post history
- ### Content System
    - Boards (topics) → Posts → Comments → Replies hierarchy
    - Upvote/downvote on posts and comments
    - Soft-delete for posts and comments
    - Base62 post slugs, pagination on all list endpoints
- ### Anonymity Layer
    - Usernames, friend lists, and post histories hidden by default
    - Profile details revealed only after mutual friendship
- ### Moderation
    - Report posts or comments
    - Moderators can view reports and take actions (mute, ban)
    - Admin panel with thread lock, post deletion, and moderation log

## Project Goals

- Create an anonymous platform that focuses on topics and conversations without the pressure of popularity or engagement metrics.
- Enable meaningful interactions while preserving user safety and anonymity by default.

## Architecture

This project uses a **microservices** architecture. Each service is an independent Rust binary with its own PostgreSQL database. The React frontend talks only to the API gateway, which validates sessions and routes requests.

```
                  ┌──────────────┐
                  │   Browser    │
                  │  React SPA   │ (5173 dev / 80 prod)
                  └──────┬───────┘
                         │ /api → gateway
                  ┌──────▼───────┐
                  │   Gateway    │ (8080)
                  │   (axum)     │ Session validation + routing
                  └──┬───┬───┬───┘
            ┌────────┘   │   └────────────┐
            ▼            ▼                ▼
    ┌──────────┐   ┌──────────┐   ┌──────────────┐
    │   Auth   │   │   Post   │   │  Moderation  │
    │  (8081)  │   │  (8082)  │   │   (8083)     │
    └────┬─────┘   └────┬─────┘   └──────┬───────┘
         │               │                │
    ┌────▼────┐     ┌────▼────┐     ┌─────▼──────┐
    │ auth-db │     │ post-db │     │moderation-db│
    └─────────┘     └─────────┘     └────────────┘

            ┌──────────┐     ┌──────────────┐
            │  Social  │     │  Social Feed │
            │  (8084)  │     │   (8085)     │
            └────┬─────┘     └──────┬───────┘
                 │                   │
            ┌────▼────┐       ┌─────▼───────┐
            │social-db│       │social-feed-db│
            └─────────┘       └─────────────┘
```

| Service | Port | Responsibility |
|---|---|---|
| `gateway` | 8080 | Single entry point — session validation, token caching, request routing |
| `auth` | 8081 | Accounts, sessions, login/signup/logout |
| `post` | 8082 | Boards, posts, comments, replies, votes |
| `moderation` | 8083 | Reports, moderation actions |
| `social` | 8084 | Friend requests, friendships |
| `social-feed` | 8085 | Board follows, personalized feed |

## Technology Stack

| Layer | Technology |
|---|---|
| Backend language | Rust |
| Web framework | axum |
| Database access | sqlx (compile-time checked queries) |
| Databases | PostgreSQL (one per service) |
| Frontend | React 19 + TypeScript + Vite 7 |
| Containerization | Docker + Docker Compose |
| Reverse proxy | Caddy (prod) / Nginx (alt) |
| CI/CD | GitHub Actions |
| Password hashing | Argon2id |
| IDs | Snowflake BIGINT (distributed-friendly) |

## Development

Each backend service runs inside its own Docker container. In development, services hot-reload via `cargo-watch`. In production they compile with `--release`.

### Requirements

- Docker and Docker Compose
- Rust (optional — only needed for native development without Docker)
- Copy `.env.example` to `.env` and fill in your database URLs

### Running (Docker)

```sh
docker compose -f docker-compose.dev.yaml up --build
```

This starts all services, databases, and the frontend dev server. The frontend is available at `http://localhost:5173`.

### Running (Native)

```sh
make dev-native
```

Requires tmux. Starts PostgreSQL locally and all services via `cargo-watch`.

### Database Migrations

```sh
make migrate              # run all pending migrations
./db run <service>        # migrate a single service
./db seed                 # seed test data (mouse/alice/bob, password: testpass123)
```

### Usage

All API traffic goes through `http://localhost:8080` (the gateway). The frontend proxies `/api` to the gateway automatically in dev.

Internal service URLs follow `http://<service-name>:8080` within Docker networking. These are never exposed directly.

### Session Handling

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
| `./` | twomice (root / monorepo) |
| `frontend/` | twomice-frontend |
| `libs/` | twomice-libs (shared Rust crates) |
| `services/auth/` | twomice-auth |
| `services/gateway/` | twomice-gateway |
| `services/post/` | twomice-post |
| `services/moderation/` | twomice-moderation |
| `services/social/` | twomice-social |
| `services/social-feed/` | twomice-social-feed |
| `tools/git-dashboard/` | twomice-dashboard |

Shared Rust libraries (`config`, `custom_headers`, `easy_errors`, `utils`) are consumed as git dependencies from `twomice-libs`.

## Deployment

Production deployment uses Docker Compose with Caddy as the reverse proxy. A GitHub Actions workflow triggers an SSH deploy on pushes to `main` when files under `deploy/` change.

```sh
./deploy/deploy.sh <service> prod   # deploy a single service
```

The live instance is hosted at **[twomice.skimnerphi.net](https://twomice.skimnerphi.net)**.
