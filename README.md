<div align="center">

# TwoMice

**Anonymous imageboard-style social media — where identity is earned, not assumed.**

[![Live Site](https://img.shields.io/badge/Live%20at-twomice.skimnerphi.net-4f46e5?style=for-the-badge&logoColor=white)](https://twomice.skimnerphi.net)

![Language](https://img.shields.io/badge/language-Rust-red?logo=rust)
![License](https://img.shields.io/github/license/MouseBurrow/twomice)
[![GitHub Org](https://img.shields.io/badge/GitHub-MouseBurrow-181717?logo=github)](https://github.com/MouseBurrow)
![Team](https://img.shields.io/badge/team-Sylviromi%20%7C%20Alaatftin-yellow)

</div>

TwoMice combines the topic-focused discussion of imageboards with a friendship layer that makes identity optional. Everyone is anonymous by default — your profile, username, and post history are invisible until a mutual friend request is accepted by both sides.

## Features

<table>
<tr>
<td width="50%">

**Anonymous by default**
Usernames, profiles, and post histories are hidden from everyone. You're just another voice in the thread.

</td>
<td width="50%">

**Earned identity**
Friendship is mutual — only when both sides accept does either person's identity become visible to the other.

</td>
</tr>
<tr>
<td>

**Imageboard structure**
Boards → Posts → Comments → Replies, with upvotes and downvotes throughout.

</td>
<td>

**Moderation tools**
Reports, mutes, bans, thread locks, post deletion, and a full moderation log accessible from an admin panel.

</td>
</tr>
</table>

## Architecture

Six independent Rust microservices, each with its own PostgreSQL database. The React frontend talks only to the API gateway, which validates sessions and routes requests.

```mermaid
graph TD
    Browser["Browser\nReact SPA\n5173 dev / 80 prod"]
    Gateway["Gateway (8080)\naxum — session validation + routing"]
    Auth["Auth (8081)"]
    Post["Post (8082)"]
    Mod["Moderation (8083)"]
    Social["Social (8084)"]
    Feed["Social Feed (8085)"]
    auth-db[(auth-db)]
    post-db[(post-db)]
    mod-db[(moderation-db)]
    social-db[(social-db)]
    feed-db[(social-feed-db)]

    Browser -->|"/api → gateway"| Gateway
    Gateway --> Auth
    Gateway --> Post
    Gateway --> Mod
    Gateway --> Social
    Gateway --> Feed
    Auth --> auth-db
    Post --> post-db
    Mod --> mod-db
    Social --> social-db
    Feed --> feed-db
```

| Service | Port | Responsibility |
|---|---|---|
| `gateway` | 8080 | Single entry point — session validation, token caching, request routing |
| `auth` | 8081 | Accounts, sessions, login/signup/logout |
| `post` | 8082 | Boards, posts, comments, replies, votes |
| `moderation` | 8083 | Reports, moderation actions |
| `social` | 8084 | Friend requests, friendships |
| `social-feed` | 8085 | Board follows, personalized feed |

## Stack

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![axum](https://img.shields.io/badge/axum-000000?style=flat&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=flat&logo=postgresql&logoColor=white)
![React](https://img.shields.io/badge/React_19-20232A?style=flat&logo=react&logoColor=61DAFB)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=flat&logo=typescript&logoColor=white)
![Vite](https://img.shields.io/badge/Vite_7-646CFF?style=flat&logo=vite&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=flat&logo=docker&logoColor=white)
![Caddy](https://img.shields.io/badge/Caddy-00ADD8?style=flat&logo=caddy&logoColor=white)

</div>

| Layer | Technology |
|---|---|
| Backend | Rust + axum |
| Database | PostgreSQL — one per service, accessed via sqlx with compile-time checked queries |
| Frontend | React 19 + TypeScript + Vite 7 |
| IDs | Snowflake BIGINT (distributed-friendly) |
| Passwords | Argon2id |
| Deployment | Docker Compose + Caddy + GitHub Actions |

## Development

See [DEVELOPMENT.md](DEVELOPMENT.md) for setup instructions, running locally, database migrations, and deployment.
