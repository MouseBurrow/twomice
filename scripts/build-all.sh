#!/usr/bin/env bash
set -euo pipefail

for service in auth post moderation social social-feed gateway; do
    echo "=== Building $service ==="
    cargo build --manifest-path "services/$service/Cargo.toml" "$@"
    echo ""
done

echo "=== Building frontend ==="
npm --prefix frontend ci
npm --prefix frontend run build
echo ""
