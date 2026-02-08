#!/bin/bash
set -e

# Configuration
REGISTRY="ghcr.io/danwritecode"
VERSION="${1:-latest}"

echo "Building multi-platform images..."

# Create buildx builder if it doesn't exist
docker buildx create --name multiplatform --use 2>/dev/null || docker buildx use multiplatform

# Build and push multi-platform backend image
docker buildx build \
  --platform linux/amd64 \
  -f backend/Dockerfile \
  -t $REGISTRY/bs-backend:$VERSION \
  -t $REGISTRY/bs-backend:latest \
  --push \
  .

# Build and push multi-platform frontend image
docker buildx build \
  --platform linux/amd64 \
  -f frontend/Dockerfile \
  -t $REGISTRY/bs-frontend:$VERSION \
  -t $REGISTRY/bs-frontend:latest \
  --build-arg NUXT_BACKEND_URL=http://bs-backend:50051 \
  --push \
  .

echo "Build and push complete!"
echo ""
echo "Images pushed:"
echo "  - $REGISTRY/bs-backend:$VERSION"
echo "  - $REGISTRY/bs-frontend:$VERSION"
echo ""
echo "Now SSH into your droplet and run: ./scripts/deploy.sh"
