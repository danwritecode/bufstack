#!/bin/bash
set -e

echo "Deploying Bufstack..."

# Navigate to app directory
cd ~/bufstack

# Pull latest code
echo "Pulling latest code from GitHub..."
git pull origin master

# Stop existing containers
echo "Stopping existing containers..."
docker compose -f docker-compose.prod.yml down

# Pull latest images from registry
echo "Pulling latest images from registry..."
docker compose -f docker-compose.prod.yml pull

# Start containers
echo "Starting containers..."
docker compose -f docker-compose.prod.yml up -d

# Show running containers
echo "Deployment complete! Running containers:"
docker compose -f docker-compose.prod.yml ps

# Show logs (last 50 lines)
echo ""
echo "Recent logs:"
docker compose -f docker-compose.prod.yml logs --tail=50
