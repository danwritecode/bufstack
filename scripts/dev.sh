#!/usr/bin/env bash
set -euo pipefail

LOGFILE=$(mktemp /tmp/bufstack-backend.XXXXXX.log)
cleanup() { rm -f "$LOGFILE"; kill $BACKEND_PID $FRONTEND_PID 2>/dev/null || true; }
trap cleanup EXIT

# Start backend, tee output to a log file so we can parse the port
echo "Starting backend..."
(cd backend/api && cargo run --bin grpc-server) 2>&1 | tee "$LOGFILE" &
BACKEND_PID=$!

# Wait for the backend to announce its port (up to 120s for cargo build + startup)
echo "Waiting for backend to be ready..."
PORT=""
for i in $(seq 1 120); do
  if PORT=$(grep -oE "gRPC server listening on 0\.0\.0\.0:[0-9]+" "$LOGFILE" 2>/dev/null | head -1 | grep -oE "[0-9]+$"); then
    [ -n "$PORT" ] && break
  fi
  PORT=""
  sleep 1
done

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
