# Bufstack

**Rails for Rust.** An opinionated full-stack application template: **Rust gRPC backend + Nuxt 4 frontend**, connected via Protocol Buffers for end-to-end type safety.

Write your backend in Rust, your frontend in Vue/Nuxt, and get automatically generated TypeScript types from your `.proto` files. Bufstack makes strong choices so you don't have to -- just build your app.

## Philosophy

Bufstack is deliberately opinionated. Instead of giving you a blank canvas and infinite choices, we pick the best tools and wire them together so you can focus on building features:

- **Protocol Buffers** for schema definition -- your `.proto` files are the single source of truth for types across the entire stack
- **gRPC** (via Tonic) for backend services -- strongly typed, fast, streaming-capable RPCs instead of hand-rolled REST endpoints
- **SQLx** with SQLite for data access -- compile-time checked SQL queries with zero overhead, no ORM magic
- **Clerk** for authentication -- drop-in auth that handles JWTs, sessions, and user management so you never roll your own
- **Tailwind CSS v4** for styling -- utility-first CSS that keeps your frontend moving fast
- **ConnectRPC** to bridge gRPC to the browser -- type-safe RPC calls from Vue components, generated from the same protos as the backend
- **Docker** with cargo-chef for deployment -- reproducible builds with excellent layer caching

## Architecture

```
protos/*.proto          <-- Single source of truth for types
        |
   ┌────┴────┐
   ▼         ▼
Backend    Frontend
(Rust)     (Nuxt 4)
Tonic      ConnectRPC
SQLx       Vue 3
Clerk      Clerk
           Tailwind
```

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
│   │   ├── migrations/   # SQLx migration files
│   │   ├── models/       # Rust data models
│   │   └── repositories/ # Database access layer
│   ├── services/     # Business logic
│   ├── io/           # IO utilities
│   └── workers/      # Background workers
├── frontend/
│   ├── app/
│   │   ├── composables/useGrpc.ts  # gRPC client composable
│   │   ├── gen/      # Generated protobuf TypeScript
│   │   └── pages/    # Vue pages
│   └── server/api/rpc/  # gRPC proxy route
├── protos/           # Protocol Buffer definitions (source of truth)
├── scripts/          # Build & deploy scripts
└── .claude/skills/   # Claude Code skills (e.g. scaffold-entity)
```

## Adding a New Entity (Scaffolding)

**New entities should be scaffolded using the `/scaffold-entity` Claude Code skill.** This is the recommended way to add new database-backed entities with full CRUD operations.

The workflow:

1. Create a migration file in `backend/data/migrations/`:
   ```sql
   -- 20260208000000_your_entity.up.sql
   CREATE TABLE your_entity (
       id INTEGER PRIMARY KEY AUTOINCREMENT,
       user_id TEXT NOT NULL,
       name TEXT NOT NULL,
       description TEXT,
       created_at TEXT NOT NULL DEFAULT (datetime('now')),
       updated_at TEXT NOT NULL DEFAULT (datetime('now'))
   );
   ```

2. Run `/scaffold-entity` in Claude Code. It will automatically generate:
   - **Proto definition** (`protos/your_entity.proto`) -- gRPC service with Create/Get/List/Update/Delete RPCs
   - **Rust model** (`backend/data/src/models/your_entity.rs`) -- SQLx-compatible struct with `FromRow`
   - **Repository** (`backend/data/src/repositories/your_entity_repository.rs`) -- Full CRUD database operations
   - **gRPC service** (`backend/api/src/services/your_entity_service.rs`) -- Tonic service implementation
   - **Auto-registration** in `grpc.rs`, `build.rs`, and all `mod.rs` files
   - **Frontend test UI** on `_testing.vue` (dev-only CRUD page)

3. Run `cargo check` to validate, then `cd frontend && bun run generate` for TypeScript types.

## Adding a New Service (Manual)

If you need a service that doesn't follow the standard entity CRUD pattern:

1. Define your service in `protos/your_service.proto`
2. Add the proto to `backend/api/build.rs`:
   ```rust
   tonic_prost_build::compile_protos("../../protos/your_service.proto")?;
   ```
3. Create `backend/api/src/services/your_service.rs`
4. Register in `backend/api/src/grpc.rs` (add import, init, and `.add_service()`)
5. Regenerate types: `cd frontend && bun run generate`

## Enabling Auth

Clerk auth middleware is included but disabled by default. To enable:

1. Set `NUXT_CLERK_SECRET_KEY` and `NUXT_PUBLIC_CLERK_PUBLISHABLE_KEY` environment variables
2. Uncomment the auth interceptor lines in `backend/api/src/grpc.rs`

The auth middleware extracts `user_id` from Clerk JWTs and injects it into gRPC request metadata, making it available to all service implementations.

## Docker & Deployment

### Development

```bash
docker compose up
```

This starts both `bs-backend` (port 50051) and `bs-frontend` (port 3000) with a shared `bs-network` bridge and a `bs-data` volume for the SQLite database.

### Adding a New Service to Docker Compose

To add a new service (e.g., a worker, a cache, a separate microservice):

1. **Add to `docker-compose.yml`** (development):
   ```yaml
   services:
     # ... existing services ...

     bs-your-service:
       build:
         context: .
         dockerfile: path/to/Dockerfile
       container_name: bs-your-service
       environment:
         - RUST_LOG=info
         - DATABASE_URL=sqlite:///app/data/bufstack.db
       volumes:
         - bs-data:/app/data    # Share the database volume if needed
       networks:
         - bs-network           # Same network so services can talk to each other
       depends_on:
         - bs-backend           # If it depends on the backend
       restart: unless-stopped
   ```

2. **Add to `docker-compose.prod.yml`** (production):
   ```yaml
   bs-your-service:
     image: ghcr.io/danwritecode/bs-your-service:latest
     env_file:
       - "/env/bufstack.env"
     container_name: bs-your-service
     networks:
       - bs-network
     restart: unless-stopped
   ```

3. **Update `scripts/build-and-push.sh`** to build and push the new image:
   ```bash
   docker buildx build \
     --platform linux/amd64 \
     -f path/to/Dockerfile \
     -t $REGISTRY/bs-your-service:$VERSION \
     -t $REGISTRY/bs-your-service:latest \
     --push \
     .
   ```

4. **If it needs a new port exposed**, update `scripts/setup-droplet.sh` to allow it through the firewall:
   ```bash
   sudo ufw allow YOUR_PORT/tcp
   ```

### Production Build & Deploy

```bash
# Build images and push to GitHub Container Registry
./scripts/build-and-push.sh

# SSH into your server, then:
./scripts/deploy.sh
```

### Server Setup (DigitalOcean)

```bash
# Run once on a fresh droplet
./scripts/setup-droplet.sh
```

This installs Docker, configures the firewall (SSH, HTTP, HTTPS, port 3000), and sets up fail2ban.
