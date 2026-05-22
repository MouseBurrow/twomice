# Commit Message Convention

All commits across all repos must follow this format:

```text
operation: description

optional body with more detail...
```

## Rules
- **Operation** must be a lowercase single word e.g. `feat`, `fix`, `ci`, `deploy`, `test`, `refactor`, `docs`, `chore`
- **Colon + space** between operation and description
- **No parentheses** wrapping the title
- **Description** is imperative mood, concise but descriptive
- **Body** (optional) adds context when the title isn't enough

## Examples

```text
feat: add user registration endpoint
fix: handle null pointer in post renderer
ci: add cargo fmt --check and cargo clippy -- -D warnings
deploy: add Dockerfile, .dockerignore, and Docker CI workflow
test: add session_token module and easy_errors tests
```

## Project Structure

See [PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md) for the full architecture, service breakdown, route tables, and file layout.

## Repos in this project

This is a monorepo of independent git repos. Each has its own origin:

- `./` — twomice (root)
- `frontend/` — twomice-frontend
- `libs/` — twomice-libs
- `services/auth/` — twomice-auth
- `services/gateway/` — twomice-gateway
- `services/moderation/` — twomice-moderation
- `services/post/` — twomice-post
- `services/social-feed/` — twomice-social-feed
- `services/social/` — twomice-social
- `tools/git-dashboard/` — twomice-dashboard
