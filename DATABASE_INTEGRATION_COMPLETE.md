# PostgreSQL Database Integration - Complete

## ✅ Implementation Complete

Your Card Game Scorer application now has **full PostgreSQL persistence**! Games and scores are saved to your Postgres pod and will be retained across restarts.

## What Was Done

### 1. Backend Changes

**Files Modified:**
- `backend/Cargo.toml` - Added `sqlx` for PostgreSQL with async support
- `backend/src/main.rs` - Initializes database connection on startup
- `backend/src/models.rs` - Updated AppState to use Database instead of Vec
- `backend/src/routes.rs` - All endpoints now use async database calls
- `backend/src/db.rs` - **NEW** - Complete database module with CRUD operations
- `backend/Dockerfile` - Updated to include migrations

**Key Features:**
- Connection pooling (5 connections by default)
- Automatic schema creation on first run
- Full error handling with `thiserror`
- Indexes for performance on game queries
- JSON storage for variable-length score arrays

### 2. Database Schema

```sql
CREATE TABLE games (
    id TEXT PRIMARY KEY,
    game_type TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    current_round INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE players (
    id TEXT PRIMARY KEY,
    game_id TEXT NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    position INTEGER NOT NULL,
    scores JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 3. Docker Compose Setup

**File:** `docker-compose.yaml`

```yaml
services:
  postgres:
    image: postgres:15
    ports: [5432:5432]
    volumes: [postgres_data:/var/lib/postgresql/data]
    healthcheck: checks if DB is ready
  
  backend:
    depends_on: [postgres]
    environment: DATABASE_URL=postgres://postgresuser:postgrespwd@postgres:5432/postgresdb
```

### 4. Kubernetes Deployment

**Files Updated:**
- `k8s/namespace.yaml` - Changed to `cards` namespace (consistent naming)
- `k8s/postgres-deployment.yaml` - Complete PostgreSQL setup with:
  - PVC for persistent storage (10GB)
  - Secrets for passwords
  - Health checks
  - Service for internal communication
  
- `k8s/backend-deployment.yaml` - Updated to:
  - Connect to PostgreSQL via DATABASE_URL env var
  - Add initContainer to wait for DB
  - Add health checks
  - Single replica (can scale horizontally now)

- `k8s/frontend-deployment.yaml` - Updated namespace to `cards`
- `k8s/ingress.yaml` - Updated namespace to `cards`

## Quick Start

### Local Development with Docker Compose

```bash
cd /home/jarno/Homelab/Cards

# First time setup (clean)
docker-compose down -v
docker-compose up --build

# Subsequent runs
docker-compose up
```

Then:
- Open http://localhost:3000
- Create a game and play
- Restart docker: `docker-compose restart`
- **Games will still be there!** ✨

### Kubernetes Deployment

```bash
# 1. Create namespace
kubectl apply -f k8s/namespace.yaml

# 2. Deploy PostgreSQL
kubectl apply -f k8s/postgres-deployment.yaml

# 3. Deploy backend (waits for DB to be ready)
kubectl apply -f k8s/backend-deployment.yaml

# 4. Deploy frontend
kubectl apply -f k8s/frontend-deployment.yaml
kubectl apply -f k8s/ingress.yaml

# Verify everything is running
kubectl -n cards get pods
```

## Data Persistence

### What's Persisted

✅ **Persisted:**
- All games created
- Player names
- All round scores
- Current round numbers
- Timestamps

### How Long Data Lasts

**Docker Compose:**
- Survives container restarts
- Survives `docker-compose up/down`
- Lost only with `docker-compose down -v` (removes volume)

**Kubernetes:**
- Survives pod restarts
- Survives node failures (if cluster-aware storage)
- Survives deployment updates
- Only lost if PVC is deleted

## Checking Your Data

### Docker Compose

```bash
# Access PostgreSQL
docker-compose exec postgres psql -U postgresuser -d postgresdb

# See all games
SELECT id, game_type, created_at, current_round FROM games;

# See players for a game
SELECT * FROM players WHERE game_id = 'YOUR_GAME_ID';

# See all scores for a player
SELECT scores FROM players WHERE id = 'PLAYER_ID';
```

### Kubernetes

```bash
# Access PostgreSQL
kubectl -n cards exec -it deployment/postgres -- psql -U postgresuser -d postgresdb

# Same queries as above
```

## Monitoring

### Check Backend Logs

```bash
# Docker Compose
docker-compose logs -f backend | grep -E "(connected|error|Game created)"

# Kubernetes
kubectl -n cards logs -f deployment/backend | grep -E "(connected|error|Game created)"
```

### Check Database Health

```bash
# Docker Compose
docker-compose exec postgres pg_isready -U postgresuser

# Kubernetes
kubectl -n cards exec deployment/postgres -- pg_isready -U postgresuser
```

## Architecture Benefits

| Feature | Before (In-Memory) | After (PostgreSQL) |
|---------|-------|-----------|
| Data Persistence | ❌ Lost on restart | ✅ Persists forever |
| Scalability | 🔴 Single instance only | 🟢 Multiple backends possible |
| Data Integrity | 🟠 Mutex protection | 🟢 ACID transactions |
| Query Performance | 🟠 Linear search | 🟢 Indexed queries |
| Connection Pooling | ❌ No pooling | ✅ 5-10 connections |
| Recovery | ❌ No recovery | ✅ Backupable |

## What Happens Next

### For You Right Now

1. **Test it**: Create some games and restart Docker
2. **Verify**: Games should reappear
3. **Enjoy**: No more losing game data!

### For Production

Consider adding:
- **Backups**: Daily SQL dumps to S3/NFS
- **Monitoring**: Database size, connection count, slow queries
- **Scaling**: Multiple backend replicas behind load balancer
- **Performance**: Add more indexes based on usage patterns
- **Security**: Rotate passwords regularly, use SSL for connections

## Troubleshooting

### Backend Says "Failed to connect to database"

1. Check PostgreSQL is running:
   ```bash
   # Docker
   docker-compose ps
   # Kubernetes
   kubectl -n cards get pods
   ```

2. Check DATABASE_URL environment variable is set correctly

3. Verify network connectivity:
   ```bash
   docker-compose exec backend nc -zv postgres 5432
   ```

### Games Not Loading After Restart

This is **intentional** - check the database:
```bash
docker-compose exec postgres psql -U postgresuser -d postgresdb -c "SELECT COUNT(*) FROM games;"
```

If games are in the database but not showing in UI, it's a frontend issue, not database issue.

### "Database is locked" or Connection Errors

PostgreSQL shouldn't give "database is locked" errors. If you see connection errors:
1. Restart PostgreSQL: `docker-compose restart postgres`
2. Increase connection pool in `backend/src/db.rs` (currently 5)
3. Check logs for details

## File Reference

```
backend/
  src/
    db.rs          ← NEW: Database module with all query operations
    main.rs        ← UPDATED: Initialize DB on startup
    models.rs      ← UPDATED: AppState now uses Database
    routes.rs      ← UPDATED: All endpoints now async + use DB
  migrations/
    20250101000000_init.sql  ← AUTO-RUN: Schema creation
  Cargo.toml       ← UPDATED: Added sqlx dependency
  Dockerfile       ← UPDATED: Copy migrations

docker-compose.yaml         ← UPDATED: Added postgres service

k8s/
  namespace.yaml             ← UPDATED: Changed to 'cards' namespace
  postgres-deployment.yaml   ← UPDATED: Complete PostgreSQL K8s setup
  backend-deployment.yaml    ← UPDATED: Connect to PostgreSQL
  frontend-deployment.yaml   ← UPDATED: Changed namespace
  ingress.yaml               ← UPDATED: Changed namespace

POSTGRESQL_SETUP.md          ← NEW: Full documentation
```

## Build & Deploy

### Rebuild Docker Image

```bash
docker-compose build --no-cache backend
docker-compose up -d
```

### Deploy to Kubernetes

```bash
# Build images (adjust registry as needed)
docker build -t jarn/card-scorer-backend:latest backend/
docker build -t jarn/card-scorer-frontend:latest frontend/
docker push jarn/card-scorer-backend:latest
docker push jarn/card-scorer-frontend:latest

# Deploy
kubectl apply -f k8s/
```

## Next Steps

1. ✅ Data is now persisted
2. 📋 Consider automating database backups
3. 📋 Add monitoring/alerting for database
4. 📋 Plan capacity based on your usage

That's it! Your games are now safe! 🎉
