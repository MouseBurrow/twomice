#!/usr/bin/env bash
set -euo pipefail

# Usage: deploy.sh <service_name> <env>
# Example: ./deploy.sh auth test   → deploys auth to test stack
# Example: ./deploy.sh auth prod  → deploys auth to prod stack

SERVICE="$1"
ENV="${2:-test}"
COMPOSE_FILE="docker-compose.${ENV}.yaml"

if [ ! -f "$COMPOSE_FILE" ]; then
    echo "Error: $COMPOSE_FILE not found"
    exit 1
fi

echo "=== Deploying $SERVICE to $ENV ==="
docker compose -f "$COMPOSE_FILE" pull "$SERVICE"
docker compose -f "$COMPOSE_FILE" up -d "$SERVICE"
echo "=== Done ==="
