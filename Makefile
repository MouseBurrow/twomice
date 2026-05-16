.PHONY: build-auth build-post build-gateway build-all dev-auth dev-post dev-gateway

# Build individual services
build-auth:
	cargo build --manifest-path services/auth/Cargo.toml

build-post:
	cargo build --manifest-path services/post/Cargo.toml

build-gateway:
	cargo build --manifest-path services/gateway/Cargo.toml

build-all: build-auth build-post build-gateway

# Run individual services
dev-auth:
	cargo run --manifest-path services/auth/Cargo.toml

dev-post:
	cargo run --manifest-path services/post/Cargo.toml

dev-gateway:
	cargo run --manifest-path services/gateway/Cargo.toml

# Database
db-up:
	docker compose -f db/compose.yaml up -d

db-down:
	docker compose -f db/compose.yaml down

db-run:
	cargo run --manifest-path db/Cargo.toml run

db-revert:
	cargo run --manifest-path db/Cargo.toml revert
