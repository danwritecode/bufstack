#!/bin/bash
set -e

echo "Setting up DigitalOcean Droplet for Bufstack..."

# Update system
echo "Updating system packages..."
sudo apt-get update
sudo apt-get upgrade -y

# Install Docker
echo "Installing Docker..."
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER

# Install Docker Compose
echo "Installing Docker Compose..."
sudo apt-get install -y docker-compose-plugin

# Verify installations
echo "Verifying installations..."
docker --version
docker compose version

# Configure firewall (UFW)
echo "Configuring firewall..."
sudo ufw allow OpenSSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw allow 3000/tcp  # Frontend (optional, can remove if behind reverse proxy)
sudo ufw --force enable

# Create app directory
echo "Creating application directory..."
mkdir -p ~/bufstack
cd ~/bufstack

# Install git if not present
echo "Installing git..."
sudo apt-get install -y git

# Optional: Install fail2ban for security
echo "Installing fail2ban for security..."
sudo apt-get install -y fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban

echo ""
echo "Droplet setup complete!"
echo ""
echo "IMPORTANT: You need to log out and log back in for Docker group changes to take effect."
echo ""
echo "Next steps:"
echo "1. Log out and SSH back in"
echo "2. Clone your repository to ~/bufstack"
echo "3. Run the deployment"
