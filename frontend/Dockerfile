# Stage 1: Builder - Install dependencies and build application
FROM node:22-slim AS builder
WORKDIR /app

# Accept build argument for backend URL (defaults to Docker service name)
ARG NUXT_BACKEND_URL=http://bs-backend:50051
ENV NUXT_BACKEND_URL=${NUXT_BACKEND_URL}

# Install Bun for dependency management
RUN npm install -g bun

# Copy package files for dependency installation
COPY frontend/package.json ./

# Install dependencies (skip postinstall since nuxt.config.ts isn't copied yet)
RUN bun install

# Copy application source code and configuration
# Note: Generated protobuf code (app/gen/) is already committed to the repo
COPY frontend/ .

# Run nuxt prepare now that source files are available, then build
RUN npx nuxt build

# Stage 2: Runtime - Minimal runtime image
FROM node:22-slim AS runtime
WORKDIR /app

# Copy the built application from builder
COPY --from=builder /app/.output /app/.output

# Expose the application port
EXPOSE 3000

# Set environment to production
ENV NODE_ENV=production

# Run the Nuxt server with Node.js
CMD ["node", ".output/server/index.mjs"]
