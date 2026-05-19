.PHONY: build-auth build-post build-moderation build-social build-feed build-gateway build-frontend build-all
.PHONY: dev-auth dev-post dev-moderation dev-social dev-feed dev-gateway dev-frontend
.PHONY: dev migrate db-up db-down db-run db-revert
.PHONY: dashboard build-dashboard

SERVICES = auth post moderation social social-feed gateway

# ─── Dev ──────────────────────────────────────────────────────────────────────

dev:
	docker compose -f docker-compose.dev.yaml up --build

dev-native:
	./scripts/dev-native.sh && tmux attach -t twomice

migrate:
	docker compose -f docker-compose.dev.yaml --profile migrate run --rm migrate

# ─── Build individual services ────────────────────────────────────────────────

build-auth:       ; cargo build --manifest-path services/auth/Cargo.toml
build-post:       ; cargo build --manifest-path services/post/Cargo.toml
build-moderation:  ; cargo build --manifest-path services/moderation/Cargo.toml
build-social:     ; cargo build --manifest-path services/social/Cargo.toml
build-feed:       ; cargo build --manifest-path services/social-feed/Cargo.toml
build-gateway:    ; cargo build --manifest-path services/gateway/Cargo.toml

build-all: $(addprefix build-,$(SERVICES)) build-frontend

# ─── Run individual services (dev, requires cargo-watch) ──────────────────────
# Each sets its own PORT (they all default to 8080) and loads DB URL from .env

dev-auth:
	set -a; . .env; set +a; PORT=8081 cargo watch -x run --manifest-path services/auth/Cargo.toml
dev-post:
	set -a; . .env; set +a; PORT=8082 cargo watch -x run --manifest-path services/post/Cargo.toml
dev-moderation:
	set -a; . .env; set +a; PORT=8083 cargo watch -x run --manifest-path services/moderation/Cargo.toml
dev-social:
	set -a; . .env; set +a; PORT=8084 cargo watch -x run --manifest-path services/social/Cargo.toml
dev-feed:
	set -a; . .env; set +a; PORT=8085 cargo watch -x run --manifest-path services/social-feed/Cargo.toml
dev-gateway:
	set -a; . .env; set +a; \
	  AUTH_SERVICE_URL=http://localhost:8081 \
	  POST_SERVICE_URL=http://localhost:8082 \
	  MODERATION_SERVICE_URL=http://localhost:8083 \
	  SOCIAL_SERVICE_URL=http://localhost:8084 \
	  FEED_SERVICE_URL=http://localhost:8085 \
	  cargo watch -x run --manifest-path services/gateway/Cargo.toml

# ─── Frontend (native) ────────────────────────────────────────────────────────

dev-frontend: ; VITE_API_PROXY=http://localhost:8080 npm run --prefix frontend dev
build-frontend:; cd frontend && npm run build

# ─── Git Dashboard ────────────────────────────────────────────────────────────

dashboard:      ; cargo run --manifest-path tools/git-dashboard/Cargo.toml
build-dashboard:; cargo build --manifest-path tools/git-dashboard/Cargo.toml

# ─── Database (local Postgres containers) ─────────────────────────────────────

db-up:    ; docker compose -f db/compose.yaml up -d
db-down:  ; docker compose -f db/compose.yaml down
db-run:   ; docker compose -f docker-compose.dev.yaml --profile migrate run --rm migrate
db-revert:; cargo run --manifest-path db/Cargo.toml revert
seed:     ; cargo run --manifest-path db/Cargo.toml seed
