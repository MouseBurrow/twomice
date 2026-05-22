# TwoMice — Project Structure

Anonymous imageboard-style social media platform. Rust microservices backend + React/TypeScript frontend. Users are anonymous until mutual befriending.

- **Language**: Rust (backend), TypeScript (frontend)
- **Database**: PostgreSQL (one per service)
- **Framework**: axum (web), sqlx (DB), React 19 + Vite 7 (frontend)
- **Ports**: Gateway=8080, auth=8081, post=8082, moderation=8083, social=8084, social-feed=8085, frontend=5173

## Architecture

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
    │  (5432) │     │ (5433)  │     │   (5434)   │
    └─────────┘     └─────────┘     └────────────┘

            ┌──────────┐     ┌──────────────┐
            │  Social  │     │  Social Feed │
            │  (8084)  │     │   (8085)     │
            └────┬─────┘     └──────┬───────┘
                 │                   │
            ┌────▼────┐       ┌─────▼───────┐
            │social-db│       │social-feed-db│
            │ (5435)  │       │   (5436)     │
            └─────────┘       └─────────────┘
```

---

## Root Directory

| Path | Purpose |
|---|---|
| `Makefile` | `make dev` (Docker), `make dev-native` (tmux), `make migrate`, per-service build/run |
| `docker-compose.yaml` | Production: 5 DBs + 5 services + gateway |
| `docker-compose.dev.yaml` | Dev: cargo-watch hot-reload + frontend on 5173 |
| `Dockerfile.dev` | Shared dev image: rust:1.91 + cargo-watch + sqlx-cli |
| `.env.example` | Template for DB URLs, service URLs, port config |
| `.github/workflows/ci.yml` | Validates docker-compose and Makefile syntax |
| `.github/workflows/deploy.yml` | SSH deploy on push to main with deploy/** changes |
| `TASKS.md` | Backlog items |
| `scripts/` | `build-all.sh`, `dev-native.sh`, `migrate-all.sh`, `fix-and-migrate.sh` |

---

## `libs/` — Shared Libraries (repo: `twomice-libs`)

Workspace root: `libs/Cargo.toml` — members: `[config, custom_headers, easy_errors, utils]`

Consumed as git deps: `{ git = "https://github.com/MouseBurrow/twomice-libs.git", branch = "main" }`

### `libs/config/`
One-liner server bootstrap: load env config, create DB pool, bind + serve axum app.

| File | Purpose |
|---|---|
| `config.rs` | `Config { database_url, port, app_env }` from env |
| `app_data.rs` | `AppData { pool: Pool<Postgres>, config: Config }` |
| `app_envs.rs` | `AppEnvs` enum: DEV / STAGING / PROD |
| `server.rs` | `serve(service_name, Router)` — init logger, load config, create pool, serve |
| `health.rs` | `health_response(service)` → `{"status":"ok","service":"<name>"}` |
| `logger.rs` | `init()` / `init_with_filter(filter)` via env_logger |

### `libs/custom_headers/`
Axum extractors for auth headers forwarded by the gateway.

| File | Purpose |
|---|---|
| `session_token.rs` | `SessionToken(String)` — from `X-Session-Token` header; sqlx encode/decode; Set-Cookie helpers |
| `user_id.rs` | `UserId(Uuid)` — from `X-User-Id` header; sqlx encode/decode |

### `libs/easy_errors/`
Error handling macros and helpers.

| File | Purpose |
|---|---|
| `lib.rs` | `define_errors!` macro (typed error enums with HTTP status codes + IntoResponse); `map_sqlx_error()`; `json_ok()` / `json_empty()`; `insert_retry_on_duplicate()` |

### `libs/utils/`
General utilities.

| File | Purpose |
|---|---|
| `lib.rs` | `random_b62(len)` — random base62 string generator; `PaginatedResponse<T>` — `{ data, total, limit, offset }` |

---

## `db/` — Database Management CLI

Crate: `twomice-db` | Entry: `db/src/main.rs`
- Subcommands: `run <service>`, `revert <service>`, `reset <service>`, `seed`
- `seed()` creates test users (mouse/alice/bob, password: `testpass123`), 6 topics, 12 posts, 15 comments

**Migrations** (`db/migrations/`):

| Service | Migrations | Tables |
|---|---|---|
| `auth/` | `000_auth_schema.sql` + `001_auth_funcs.sql` | `accounts`, `sessions` + 6 PL/pgSQL functions |
| `post/` | `000_post_schema.sql` + `001_post_funcs.sql` | `topics`, `posts`, `comments`, `replies` + 10 functions |
| `moderation/` | `000_moderation_schema.sql` | `reports`, `moderation_actions` |
| `social/` | `000_social_schema.sql` | `friend_requests`, `friendships` |
| `social-feed/` | `000_social_feed_schema.sql` | `follows`, `feed_preferences` |

---

## `services/auth/` — Authentication Service (crate: `auth`)

Port 8081 | DB port 5432

```
src/
  main.rs              — Router: /login, /signup, /logout, /validate, /account
  service.rs           — Business logic: DB queries for accounts + sessions
  errors.rs            — AuthError enum (6 variants via define_errors!)
  password_utils.rs    — Argon2id password hashing/verification + unit tests
  routes/
    common.rs          — CredentialsBody { username, password }
    login.rs           — POST /login: verify password, create session, set cookie
    signup.rs          — POST /signup: hash password, create account + session, set cookie
    logout.rs          — POST /logout: delete session, clear cookie
    validate.rs        — POST /validate: check session token validity, return user_id
    account.rs         — GET /account: return account info (requires X-User-Id)
```

---

## `services/gateway/` — API Gateway (crate: `gateway`)

Port 8080 | Single entry point

```
src/
  main.rs              — Creates GatewayApp, binds 0.0.0.0:8080, fallback handler
  gateway_app.rs       — GatewayApp with token validation cache (HashMap, 1hr TTL)
  request_handler.rs   — Path → service URL routing + request forwarding
```

URL routing:
| Prefix | Service |
|---|---|
| `/login`, `/logout`, `/signup`, `/account` | auth |
| `/mcf` | post |
| `/moderation` | moderation |
| `/social` | social |
| `/feed` | social-feed |

Caches `X-Session-Token` validation for 1hr; invalidates on logout. Strips internal `X-` headers from upstream responses (except `x-session-token`, `x-user-id`).

---

## `services/post/` — Content Service (crate: `post`)

Port 8082 | DB port 5433 | **Fully implemented**

```
src/
  main.rs              — Router
  service.rs           — Business logic: all DB operations for topics/posts/comments/replies
  errors.rs            — PostError enum (6 variants)
  routes/
    mischief.rs        — Topics: POST /mcf, GET /mcf, GET /mcf/:topic
    nibbles.rs         — Posts: POST /mcf/:topic/nib, GET /mcf/:topic/nib, GET /mcf/:topic/nib/:post
    squeaks.rs         — Comments: POST /mcf/:topic/nib/:post/sqk, GET /mcf/:topic/nib/:post/sqk
    echoes.rs          — Replies: POST /mcf/:topic/nib/:post/sqk/:comment/echoes, GET same
```

Content hierarchy: **Topics → Posts → Comments → Replies**. All support soft-delete (`deleted` bool). Posts have auto-generated base62 slugs. Comments/replies have random 5-char base62 hashes. Unique-violation retry for hash/slug collisions.

---

## `services/moderation/` — Moderation Service (crate: `moderation`)

Port 8083 | DB port 5434 | **STUB**

```
src/main.rs  — Single file, stub handlers only
```

Routes: `POST /reports`, `GET /reports`, `POST /action` — all return placeholder JSON.
DB schema exists (`reports` + `moderation_actions` tables) but no implementation.

---

## `services/social/` — Social/Friendship Service (crate: `social`)

Port 8084 | DB port 5435 | **STUB**

```
src/main.rs  — Single file, stub handlers only
```

Routes: `POST /friend-request`, `POST /friend-accept`, `GET /friends` — all return placeholder JSON.
DB schema exists (`friend_requests` + `friendships` tables) but no implementation.

---

## `services/social-feed/` — Social Feed Service (crate: `social-feed`)

Port 8085 | DB port 5436 | **STUB**

```
src/main.rs  — Single file, stub handlers only
```

Routes: `GET /feed`, `POST /feed/preferences` — all return placeholder JSON.
DB schema exists (`follows` + `feed_preferences` tables) but no implementation.

---

## `frontend/` — React SPA (repo: `twomice-frontend`)

React 19 + TypeScript + Vite 7. Vite proxies `/api` → gateway (default `localhost:8080`), strips `/api` prefix.

```
src/
  main.tsx          — React 19 StrictMode: ToastProvider > AuthProvider > ThemeProvider > BrowserRouter > App
  App.tsx           — Route definitions
  api.ts            — Typed fetch wrapper for all backend endpoints
  apiError.ts       — ApiError class with route, status, code, payload
  types.ts          — TypeScript types: AuthState, AccountData, TopicData, PostData, CommentData, ReplyData
  pages/
    Auth.tsx        — /auth — Login/Signup
    MCF.tsx         — / — Home: list of all topics
    Topic.tsx       — /b/:board — List of posts in a topic
    Post.tsx        — /b/:board/nib/:post — Post detail with comments + replies
    CreateNib.tsx   — /b/:board/new — Create new post
    Settings.tsx    — /settings — User settings
    Profile.tsx     — /profile — User profile
  contexts/
    AuthContext.tsx   — Auth state machine: unknown → guest | user | admin
    ThemeContext.tsx  — Theme support
    ToastContext.tsx  — Toast notifications
  components/
    layout/          — Layout.tsx, NavBar.tsx, NavBar.scss
    mcf/             — Header.tsx, TopicGrid.tsx, TopicCard.tsx, CreateTopicCard.tsx
    topic/           — TopicHeader.tsx, PostGrid.tsx, PostCard.tsx, CreatePostCard.tsx
    post/            — PostHeader.tsx, CommentGrid.tsx, CommentCard.tsx, CreateCommentCard.tsx, CreateReplyCard.tsx
    shared/          — VoteColumn.tsx (upvote/downvote UI)
    ErrorMessage.tsx
    ToastContainer.tsx
```

---

## `tools/git-dashboard/` — TUI Git Dashboard (crate: `git-dashboard`)

Ratatui + crossterm TUI for monitoring all monorepo repos.

```
src/
  main.rs   — TUI event loop, auto-refresh every 500ms
  app.rs    — App state machine: repo list, file list, diff view, push, CI fetch
  git.rs    — Git operations via git2: open repo, status, files, commits, diffs, push
  repo.rs   — Discover git repos in directory tree
  ui.rs     — Three-pane layout: repos | files | diff
  ci.rs     — Fetch GitHub Actions CI/CD status
```

---

## `deploy/` — Deployment

| File | Purpose |
|---|---|
| `docker-compose.base.yaml` | Base services using GHCR images |
| `docker-compose.prod.yaml` | Prod: includes base + DBs + Caddy reverse proxy |
| `docker-compose.test.yaml` | Test environment variant |
| `deploy.sh` | `./deploy.sh <service> <env>` (e.g. `./deploy.sh auth prod`) |
| `caddy/Caddyfile` | Caddy reverse proxy config |
| `nginx/twomice.conf` | Alternative nginx reverse proxy config |
| `.env.prod` / `.env.test` | Deploy env files |

---

## Independent Git Repos

Each directory below has its own `origin` remote:

| Directory | Repo |
|---|---|
| `./` | twomice (root) |
| `frontend/` | twomice-frontend |
| `libs/` | twomice-libs |
| `services/auth/` | twomice-auth |
| `services/gateway/` | twomice-gateway |
| `services/moderation/` | twomice-moderation |
| `services/post/` | twomice-post |
| `services/social-feed/` | twomice-social-feed |
| `services/social/` | twomice-social |
| `tools/git-dashboard/` | twomice-dashboard |
