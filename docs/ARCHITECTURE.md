# Card Game Scorer - Architecture Documentation

## Overview

Card Game Scorer is a full-stack web application for tracking scores in various card games. It consists of:
- **Backend**: Rust + Actix-web REST API
- **Frontend**: Vue 3 + TypeScript SPA
- **Database**: PostgreSQL
- **Infrastructure**: Kubernetes (K3s) with Traefik ingress

## System Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Kubernetes Cluster                          │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────┐     ┌─────────────┐     ┌─────────────────────┐  │
│  │   Traefik   │────▶│  Frontend   │     │      PostgreSQL     │  │
│  │   Ingress   │     │  (Nginx)    │     │   (postgres ns)     │  │
│  └─────────────┘     └─────────────┘     └─────────────────────┘  │
│         │                   │                       ▲              │
│         │                   │                       │              │
│         │            ┌──────▼──────┐               │              │
│         └───────────▶│   Backend   │───────────────┘              │
│           /api/*     │  (Actix)    │                              │
│                      └─────────────┘                              │
│                             │                                      │
│                      ┌──────▼──────┐                              │
│                      │    Brevo    │                              │
│                      │    SMTP     │                              │
│                      └─────────────┘                              │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
Cards/
├── backend/                 # Rust backend API
│   ├── src/
│   │   ├── main.rs         # Entry point, server setup
│   │   ├── db.rs           # Database operations
│   │   ├── models.rs       # Data structures
│   │   ├── routes.rs       # HTTP handlers
│   │   ├── notifications.rs # Email notifications
│   │   └── services/       # Game-specific scoring logic
│   │       ├── mod.rs
│   │       ├── hearts_service.rs
│   │       ├── king_service.rs
│   │       ├── double_king_service.rs
│   │       ├── color_whist_service.rs
│   │       └── game_service.rs
│   ├── migrations/         # SQL migrations
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend/               # Vue 3 frontend
│   ├── src/
│   │   ├── views/          # Page components
│   │   ├── components/     # Reusable components
│   │   ├── composables/    # Vue composables
│   │   ├── stores/         # Pinia stores
│   │   ├── api/            # API client
│   │   ├── styles/         # CSS files
│   │   ├── locales/        # i18n translations
│   │   ├── router/         # Vue Router
│   │   ├── types/          # TypeScript types
│   │   └── utils/          # Helper functions
│   ├── Dockerfile
│   └── nginx.conf
├── k8s/                    # Kubernetes manifests
├── grafana/                # Grafana dashboards
└── docs/                   # Documentation
```

## Data Flow

### Game Creation
1. User selects game type and enters player names
2. Frontend POSTs to `/api/games`
3. Backend creates game record and player records in PostgreSQL
4. Returns game ID for navigation

### Score Entry
1. User enters round scores
2. Frontend POSTs to `/api/games/{id}/round`
3. Backend validates scores via game-specific service
4. Calculates totals, updates database
5. Checks for game-over conditions
6. If game over, sends email notification (if enabled)

### Leaderboard
1. Frontend GETs `/api/leaderboard`
2. Backend aggregates all finished games
3. Calculates wins, points, placement stats per player
4. Returns sorted leaderboard data

## Game Types

| Game Type | Players | Scoring | Special Rules |
|-----------|---------|---------|---------------|
| Hearts | 3-4 | Low wins | Moon shot, 100 reset |
| King | 3-4 | 10 rounds, high wins | Mixed positive/negative |
| Double King | 4 | 10 games per player | Trump games for each player |
| Color Whist | 4 | Contract-based | Complex contract system |
