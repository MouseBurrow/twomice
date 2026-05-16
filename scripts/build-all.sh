#!/usr/bin/env bash
set -euo pipefail

for service in auth post gateway; do
    echo "=== Building $service ==="
    cargo build --manifest-path "services/$service/Cargo.toml" "$@"
    echo ""
done
