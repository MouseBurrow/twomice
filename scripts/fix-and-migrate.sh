#!/bin/bash
set -e

cd "$(dirname "$0")/.."

echo "=== Sourcing env ==="
set -a
source deploy/.env
set +a

fix_password() {
    local container=$1
    local password=$2
    local db=$3

    echo "  Fixing password for $container ($db)..."
    if docker exec "$container" psql -U twomice -d "$db" -c "ALTER USER twomice PASSWORD '$password'" >/dev/null 2>&1; then
        echo "    OK"
    else
        echo "    WARN: could not fix password (already correct?)"
    fi
}

run_migrations() {
    local container=$1
    local db=$2
    local migrations_dir=$3

    echo "  Running migrations from $migrations_dir..."
    for f in "$migrations_dir"/*.up.sql; do
        [ -f "$f" ] || continue
        echo "    Applying $(basename $f)..."
        docker exec -i "$container" psql -U twomice -d "$db" < "$f"
    done
}

echo ""
echo "=== Step 1: Fix passwords ==="
fix_password twomice-prod-auth-db        "$AUTH_DB_PASSWORD"        auth
fix_password twomice-prod-post-db        "$POST_DB_PASSWORD"        post
fix_password twomice-prod-moderation-db  "$MODERATION_DB_PASSWORD"  moderation
fix_password twomice-prod-social-db      "$SOCIAL_DB_PASSWORD"      social
fix_password twomice-prod-social-feed-db "$FEED_DB_PASSWORD"        social_feed

echo ""
echo "=== Step 2: Run migrations ==="
run_migrations twomice-prod-auth-db        auth        db/migrations/auth
run_migrations twomice-prod-post-db        post        db/migrations/post
run_migrations twomice-prod-moderation-db  moderation  db/migrations/moderation
run_migrations twomice-prod-social-db      social      db/migrations/social
run_migrations twomice-prod-social-feed-db social_feed db/migrations/social-feed

echo ""
echo "=== Step 3: Restart services ==="
docker compose -f deploy/docker-compose.prod.yaml restart auth post moderation social social-feed

echo ""
echo "=== Done! Check logs: ==="
echo "  docker compose -f deploy/docker-compose.prod.yaml logs --tail 5 auth"
echo "  docker compose -f deploy/docker-compose.prod.yaml logs --tail 5 post"
echo "  docker compose -f deploy/docker-compose.prod.yaml logs --tail 5 moderation"
echo "  docker compose -f deploy/docker-compose.prod.yaml logs --tail 5 social"
echo "  docker compose -f deploy/docker-compose.prod.yaml logs --tail 5 social-feed"
