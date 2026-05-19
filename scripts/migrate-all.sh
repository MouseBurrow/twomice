#!/bin/bash
set -e

SERVICES="auth post moderation social social-feed"

for service in $SERVICES; do
  echo "=== Running migrations for $service ==="
  case $service in
    social-feed) db_url="$FEED_DATABASE_URL" ;;
    *)
      var="${service}_DATABASE_URL"
      var="${var^^}"
      db_url="${!var}"
      ;;
  esac
  if [ -z "$db_url" ]; then
    echo "ERROR: $service database URL is not set"
    exit 1
  fi
  cargo run --manifest-path "/workspace/db/Cargo.toml" run "$service"
done

echo "=== All migrations complete! ==="
