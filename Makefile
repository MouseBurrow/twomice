.PHONY: build-auth build-post build-moderation build-social build-feed build-gateway build-frontend build-all
.PHONY: dev-auth dev-post dev-moderation dev-social dev-feed dev-gateway dev-frontend
.PHONY: db-up db-down db-run db-revert

SERVICES = auth post moderation social social-feed gateway

# Build individual services
build-auth:       ; cargo build --manifest-path services/auth/Cargo.toml
build-post:       ; cargo build --manifest-path services/post/Cargo.toml
build-moderation:  ; cargo build --manifest-path services/moderation/Cargo.toml
build-social:     ; cargo build --manifest-path services/social/Cargo.toml
build-feed:       ; cargo build --manifest-path services/social-feed/Cargo.toml
build-gateway:    ; cargo build --manifest-path services/gateway/Cargo.toml

build-all: $(addprefix build-,$(SERVICES)) build-frontend

# Run individual services
dev-auth:         ; cargo run --manifest-path services/auth/Cargo.toml
dev-post:         ; cargo run --manifest-path services/post/Cargo.toml
dev-moderation:    ; cargo run --manifest-path services/moderation/Cargo.toml
dev-social:       ; cargo run --manifest-path services/social/Cargo.toml
dev-feed:         ; cargo run --manifest-path services/social-feed/Cargo.toml
dev-gateway:      ; cargo run --manifest-path services/gateway/Cargo.toml

# Frontend
dev-frontend: ; cd frontend && npm run dev
build-frontend: ; cd frontend && npm run build

# Git Dashboard
.PHONY: dashboard build-dashboard

dashboard:     ; cargo run --manifest-path tools/git-dashboard/Cargo.toml
build-dashboard:; cargo build --manifest-path tools/git-dashboard/Cargo.toml

# Database
db-up:    ; docker compose -f db/compose.yaml up -d
db-down:  ; docker compose -f db/compose.yaml down
db-run:   ; cargo run --manifest-path db/Cargo.toml run
db-revert:; cargo run --manifest-path db/Cargo.toml revert
