# Deployment Guide

## Prerequisites

- Docker installed
- k3s cluster running
- kubectl configured for your k3s cluster

## Local Development

### Quick Start with Docker Compose

```bash
docker-compose up
```

This will start both backend and frontend:
- Backend API: http://localhost:8080
- Frontend: http://localhost:3000

### Manual Development Setup

#### Backend

```bash
cd backend
cargo run
```

The API will be available at http://localhost:8080

#### Frontend

```bash
cd frontend
npm install
npm run dev
```

The app will be available at http://localhost:3000

## Deploy to k3s Homelab

### 1. Build Docker Images

```bash
make build-images
```

Or manually:

```bash
# Build images
docker build -t card-scorer-backend:latest ./backend
docker build -t card-scorer-frontend:latest ./frontend

# Import to k3s
docker save card-scorer-backend:latest | sudo k3s ctr images import -
docker save card-scorer-frontend:latest | sudo k3s ctr images import -
```

### 2. Deploy to Kubernetes

```bash
make deploy
```

Or manually:

```bash
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/backend-deployment.yaml
kubectl apply -f k8s/frontend-deployment.yaml
kubectl apply -f k8s/ingress.yaml
```

### 3. Configure DNS

Add to your `/etc/hosts` or configure your DNS:

```
<your-k3s-node-ip> cards.homelab.local
```

### 4. Access the Application

Open http://cards.homelab.local in your browser

## Configuration

### Ingress Hostname

Edit `k8s/ingress.yaml` and change the hostname:

```yaml
spec:
  rules:
  - host: cards.homelab.local  # Change to your domain
```

### Enable HTTPS

If you have cert-manager installed:

1. Uncomment the TLS section in `k8s/ingress.yaml`
2. Update the cert-manager annotation with your cluster issuer

## Monitoring

Check deployment status:

```bash
kubectl get pods -n card-scorer
kubectl get services -n card-scorer
kubectl get ingress -n card-scorer
```

View logs:

```bash
# Backend logs
kubectl logs -n card-scorer -l app=backend -f

# Frontend logs
kubectl logs -n card-scorer -l app=frontend -f
```

## Cleanup

Remove all resources:

```bash
make clean
```

Or manually:

```bash
kubectl delete namespace card-scorer
```

## Troubleshooting

### Pods not starting

```bash
kubectl describe pod -n card-scorer <pod-name>
```

### Image pull errors

Make sure images are properly imported to k3s:

```bash
sudo k3s ctr images ls | grep card-scorer
```

### Can't access the application

1. Check ingress is running: `kubectl get ingress -n card-scorer`
2. Verify DNS/hosts file configuration
3. Check if Traefik is running: `kubectl get pods -n kube-system | grep traefik`
