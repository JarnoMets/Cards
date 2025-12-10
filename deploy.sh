#!/bin/zsh
set -e

# Compile backend (Rust)
echo "Compiling backend..."
(cd backend && cargo build --release)

# Compile frontend (Vite)
echo "Compiling frontend..."
(cd frontend && npm install && npm run build)

# Build Docker images
echo "Building backend Docker image..."
sudo docker build -t card-scorer-backend:latest backend
echo "Building frontend Docker image..."
sudo docker build -t card-scorer-frontend:latest frontend

# Check if logged in to Docker
if ! sudo docker info 2>&1 | grep -q 'Username:'; then
  echo "Not logged in to Docker. Please log in."
  sudo docker login
else
  echo "Already logged in to Docker."
fi

# Tag images
sudo docker tag card-scorer-backend:latest jarn/card-scorer-backend:latest
sudo docker tag card-scorer-frontend:latest jarn/card-scorer-frontend:latest

# Push images
sudo docker push jarn/card-scorer-backend:latest
sudo docker push jarn/card-scorer-frontend:latest

echo "Deployment complete!"
