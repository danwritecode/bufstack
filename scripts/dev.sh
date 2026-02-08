#!/usr/bin/env bash
set -euo pipefail

cleanup() { kill $BACKEND_PID $FRONTEND_PID 2>/dev/null || true; }
trap cleanup EXIT

PORTFILE=$(mktemp)
FRONTEND_PID=""

# Start backend, pipe output through to find the port
echo "Starting backend..."
(cd backend/api && cargo run --bin grpc-server) 2>&1 | while IFS= read -r line; do
  echo "$line"
  if [[ "$line" =~ gRPC\ server\ listening\ on\ 0\.0\.0\.0:([0-9]+) ]]; then
    echo "${BASH_REMATCH[1]}" > "$PORTFILE"
  fi
done &
BACKEND_PID=$!

# Wait for the port to appear (up to 120s for cargo build + startup)
echo "Waiting for backend to be ready..."
for i in $(seq 1 120); do
  PORT=$(cat "$PORTFILE" 2>/dev/null || true)
  [ -n "$PORT" ] && break
  sleep 1
done
rm -f "$PORTFILE"

if [ -z "$PORT" ]; then
  echo "ERROR: Backend did not start within 120 seconds"
  exit 1
fi

echo "Backend ready on port $PORT"

# Start frontend with the discovered backend URL
export NUXT_BACKEND_URL="http://localhost:$PORT"
echo "Starting frontend with NUXT_BACKEND_URL=$NUXT_BACKEND_URL"
(cd frontend && bun dev) &
FRONTEND_PID=$!

wait
