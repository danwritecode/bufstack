# Bufstack

Full-stack application template: **Rust gRPC backend + Nuxt 4 frontend**, connected via Protocol Buffers for end-to-end type safety.

Write your backend in Rust, your frontend in Vue/Nuxt, and get automatically generated TypeScript types from your `.proto` files.

## Architecture

- **Backend**: Rust with [Tonic](https://github.com/hyperium/tonic) gRPC + [SQLx](https://github.com/launchbadge/sqlx)/SQLite
- **Frontend**: [Nuxt 4](https://nuxt.com) + Vue 3 + [Tailwind CSS v4](https://tailwindcss.com)
- **Communication**: gRPC-Web via [ConnectRPC](https://connectrpc.com)
- **Auth**: [Clerk](https://clerk.com) (ready to enable)
- **Deployment**: Docker with [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) caching

## Prerequisites

- [Rust](https://rustup.rs/)
- [Bun](https://bun.sh)
- [Protocol Compiler (protoc)](https://grpc.io/docs/protoc-installation/)
- [Buf CLI](https://buf.build/docs/installation)

## Quick Start

```bash
# Install frontend dependencies
cd frontend && bun install && cd ..

# Generate TypeScript types from protos
cd frontend && bun run generate && cd ..

# Start both backend and frontend
bun run dev
```

The backend runs on `http://localhost:50051` (gRPC) and the frontend on `http://localhost:3000`.

## Project Structure

```
bufstack/
├── backend/
│   ├── api/          # gRPC server (Tonic, port 50051)
│   ├── data/         # Database layer (SQLx + SQLite)
│   ├── services/     # Business logic
│   ├── io/           # IO utilities
│   └── workers/      # Background workers
├── frontend/
│   ├── app/
│   │   ├── composables/useGrpc.ts  # gRPC client composable
│   │   ├── gen/      # Generated protobuf TypeScript
│   │   └── pages/    # Vue pages
│   └── server/api/rpc/  # gRPC proxy route
├── protos/           # Protocol Buffer definitions
└── scripts/          # Build & deploy scripts
```

## Adding a New Service

1. Define your service in `protos/your_service.proto`
2. Add the proto to `backend/api/build.rs`
3. Create `backend/api/src/services/your_service.rs`
4. Register in `backend/api/src/grpc.rs`
5. Regenerate types: `cd frontend && bun run generate`

## Enabling Auth

Clerk auth middleware is included but disabled by default. To enable:

1. Set `NUXT_CLERK_SECRET_KEY` and `NUXT_PUBLIC_CLERK_PUBLISHABLE_KEY` environment variables
2. Uncomment the auth interceptor lines in `backend/api/src/grpc.rs`

## Docker

```bash
# Development
docker compose up

# Production build & push
./scripts/build-and-push.sh

# Deploy
./scripts/deploy.sh
```
