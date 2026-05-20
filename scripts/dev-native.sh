#!/bin/bash
set -e

SESSION="twomice"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PGDATA="$HOME/.local/share/twomice-pgdata"
PGPORT="5432"

cd "$ROOT"

# ── Prerequisites ──────────────────────────────────────────────────────────────

for cmd in tmux cargo docker; do
  if ! command -v "$cmd" &>/dev/null; then
    echo "ERROR: $cmd is required."
    exit 1
  fi
done

if ! command -v cargo-watch &>/dev/null && ! cargo watch --version &>/dev/null; then
  echo "cargo-watch not found. Install it once with:  cargo install cargo-watch"
  cargo install cargo-watch
fi

if [ ! -x "$ROOT/frontend/node_modules/.bin/vite" ]; then
  echo "Installing frontend dependencies..."
  (cd "$ROOT/frontend" && npm install)
fi

if ! command -v sqlx &>/dev/null; then
  echo "sqlx-cli not found. Install it once with:  cargo install sqlx-cli"
  cargo install sqlx-cli
fi

# ── Stop Docker DB containers (they conflict with native Postgres) ────────────

# Stop containers from any compose project that might hold port 5432
docker compose -f db/compose.yaml down 2>/dev/null || true
docker compose -f docker-compose.dev.yaml down 2>/dev/null || true
docker compose -f docker-compose.yaml down 2>/dev/null || true
# Also stop any remaining containers on our ports
for port in 5432 5433 5434 5435 5436; do
  cid=$(docker ps -q --filter "publish=$port" 2>/dev/null)
  [ -n "$cid" ] && docker stop $cid 2>/dev/null || true
done

# ── Start native Postgres ────────────────────────────────────────────────────

if ! pg_isready -h 127.0.0.1 -p "$PGPORT" &>/dev/null; then
  # Double-check port 5432 is free before starting native Postgres
  if ss -tlnp | grep -q ":$PGPORT "; then
    echo "ERROR: Port $PGPORT is in use. Can't start native PostgreSQL."
    echo "Try: kill \$(lsof -ti :$PGPORT)"
    exit 1
  fi

  echo "Starting native PostgreSQL..."

  if [ ! -d "$PGDATA" ]; then
    echo "  Initializing database cluster..."
    mkdir -p "$(dirname "$PGDATA")"
    initdb -D "$PGDATA" -U twomice 2>&1 | tail -2
    mkdir -p /tmp/pgsock
  fi

  # Ensure config has our settings (append only if missing)
  if ! grep -q "^unix_socket_directories" "$PGDATA/postgresql.conf" 2>/dev/null; then
    cat >> "$PGDATA/postgresql.conf" <<-CONF
unix_socket_directories = '/tmp/pgsock'
port = $PGPORT
listen_addresses = 'localhost'
CONF
  fi

  pg_ctl -D "$PGDATA" -l "$PGDATA/logfile" start 2>&1

  until pg_isready -h 127.0.0.1 -p "$PGPORT" &>/dev/null; do
    printf "."
    sleep 1
  done
  printf " ready\n"

  # Create databases if missing
  for db in auth post moderation social social_feed; do
    psql -h 127.0.0.1 -p "$PGPORT" -U twomice -d postgres \
      -tc "SELECT 1 FROM pg_database WHERE datname='$db'" 2>/dev/null \
      | grep -q 1 || psql -h 127.0.0.1 -p "$PGPORT" -U twomice -d postgres \
        -c "CREATE DATABASE $db OWNER twomice;" 2>&1 | tail -1
  done
else
  echo "PostgreSQL already running on port $PGPORT"
fi

# ── Run migrations ───────────────────────────────────────────────────────────

echo "Running migrations..."
for service in auth post moderation social; do
  printf "  %-15s" "$service"
  AUTH_DATABASE_URL="postgresql://twomice:twomice@127.0.0.1:$PGPORT/$service" \
  POST_DATABASE_URL="postgresql://twomice:twomice@127.0.0.1:$PGPORT/$service" \
  MODERATION_DATABASE_URL="postgresql://twomice:twomice@127.0.0.1:$PGPORT/$service" \
  SOCIAL_DATABASE_URL="postgresql://twomice:twomice@127.0.0.1:$PGPORT/$service" \
  FEED_DATABASE_URL="postgresql://twomice:twomice@127.0.0.1:$PGPORT/$service" \
    cargo run --manifest-path db/Cargo.toml run "$service" 2>&1 | tail -1
done
# social-feed needs special handling (env var name mismatch)
printf "  %-15s" "social-feed"
sqlx migrate run --source db/migrations/social-feed \
  --database-url "postgresql://twomice:twomice@127.0.0.1:$PGPORT/social_feed" 2>&1 | tail -1

# ── Build libs once ──────────────────────────────────────────────────────────

echo "Building shared libs..."
(cd "$ROOT/libs" && cargo build --workspace 2>&1 | tail -3)

# ── Services configuration ────────────────────────────────────────────────────

declare -A PORTS=(
  [gateway]=8080 [auth]=8081 [post]=8082
  [moderation]=8083 [social]=8084 [social-feed]=8085
)

declare -A DIRS=(
  [gateway]=services/gateway [auth]=services/auth [post]=services/post
  [moderation]=services/moderation [social]=services/social
  [social-feed]=services/social-feed
)

declare -A DB_VARS=(
  [auth]=AUTH_DATABASE_URL [post]=POST_DATABASE_URL
  [moderation]=MODERATION_DATABASE_URL [social]=SOCIAL_DATABASE_URL
  [social-feed]=FEED_DATABASE_URL
)

declare -A DB_URLS=(
  [auth]="postgresql://twomice:twomice@127.0.0.1:$PGPORT/auth"
  [post]="postgresql://twomice:twomice@127.0.0.1:$PGPORT/post"
  [moderation]="postgresql://twomice:twomice@127.0.0.1:$PGPORT/moderation"
  [social]="postgresql://twomice:twomice@127.0.0.1:$PGPORT/social"
  [social-feed]="postgresql://twomice:twomice@127.0.0.1:$PGPORT/social_feed"
)

GW_AUTH="http://localhost:8081"
GW_POST="http://localhost:8082"
GW_MOD="http://localhost:8083"
GW_SOC="http://localhost:8084"
GW_FEED="http://localhost:8085"

# ── Build command for a service pane ──────────────────────────────────────────

service_cmd() {
  local name="$1"
  local dir="$ROOT/${DIRS[$name]}"
  local port="${PORTS[$name]}"
  local dbvar="${DB_VARS[$name]}"
  local dburl="${DB_URLS[$name]}"

  local cmd="cd '$dir'"

  if [ "$name" = "gateway" ]; then
    cmd+=" && export AUTH_SERVICE_URL='$GW_AUTH'"
    cmd+=" && export POST_SERVICE_URL='$GW_POST'"
    cmd+=" && export MODERATION_SERVICE_URL='$GW_MOD'"
    cmd+=" && export SOCIAL_SERVICE_URL='$GW_SOC'"
    cmd+=" && export FEED_SERVICE_URL='$GW_FEED'"
  else
    cmd+=" && export PORT='$port'"
    cmd+=" && export ${dbvar}='${dburl}'"
  fi

  cmd+=" && echo '── $name (port ${port}) ──'"
  cmd+=" && exec cargo watch -x run"

  echo "$cmd"
}

# ── Kill existing processes on service ports ─────────────────────────────────

echo "Cleaning up old processes..."
for port in 8080 8081 8082 8083 8084 8085 5173; do
  pids=$(ss -tlpn 'sport = :'"$port" 2>/dev/null | grep -oP 'pid=\K\d+' || true)
  if [ -n "$pids" ]; then
    echo "$pids" | xargs -r kill 2>/dev/null || true
    echo "  Killed processes on port $port"
  fi
done
sleep 1

# ── Kill existing session ────────────────────────────────────────────────────

tmux kill-session -t "$SESSION" 2>/dev/null || true

# ── Create windows ───────────────────────────────────────────────────────────

echo "Starting services..."

tmux new-session -d -s "$SESSION" -n "startup" "echo Starting services..."
tmux new-window -t "$SESSION" -n "gateway" "$(service_cmd gateway)"
tmux new-window -t "$SESSION" -n "auth" "$(service_cmd auth)"
tmux new-window -t "$SESSION" -n "post" "$(service_cmd post)"
tmux new-window -t "$SESSION" -n "moderation" "$(service_cmd moderation)"
tmux new-window -t "$SESSION" -n "social" "$(service_cmd social)"
tmux new-window -t "$SESSION" -n "social-feed" "$(service_cmd social-feed)"

tmux kill-window -t "$SESSION:startup" 2>/dev/null || true
tmux select-window -t "$SESSION:gateway"

echo "Starting frontend..."

tmux new-window -t "$SESSION" -n "frontend" \
  "cd '$ROOT/frontend' && VITE_API_PROXY='http://localhost:8080' npm run dev || bash"

# ── Done ──────────────────────────────────────────────────────────────────────

echo ""
echo "  Development session started!"
echo ""
echo "  Attach:  tmux attach -t $SESSION"
echo "  Detach:  Ctrl+B D"
echo ""
echo "  Services are on port 8080-8085 (native)"
echo "  PostgreSQL is on port $PGPORT (native)"
echo "  Frontend is on port 5173  (http://localhost:5173)"
echo ""
echo "  First compile will take a minute or two per service."
