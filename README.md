# Cards – Card Game Scorer

A self-hosted card-game scoring and leaderboard platform for tracking results across multiple Belgian/Dutch card games.

## Features

- **Multi-game support** – Whist, Color Whist, King, Double King, Manille, Press, Hearts
- **ELO rating system** – Per-game-type ELO ratings with full history and comparison charts
- **Leaderboard** – Global and per-game-type leaderboards with win-rate, average score, and records
- **Player profiles** – Per-player game history, ELO graph, and stats
- **Email notifications** – Optional SMTP email alerts for game results and daily digests
- **Admin panel** – Rename players globally, recalculate ELO, clean up empty games, configure ELO weights
- **Background tasks** – Automatic cleanup of stale games and periodic ELO recalculation

## Tech Stack

| Layer | Technology |
|---|---|
| Backend | Rust · Actix-web 4 · SQLx |
| Database | PostgreSQL 15 |
| Frontend | Vue 3 · TypeScript · Vite · Chart.js |
| Deployment | Docker Compose / Kubernetes (k3s) |

## Quick Start

### Local development

```bash
./dev.sh
```

- Backend: http://localhost:8080
- Frontend: http://localhost:3000

### Docker Compose

```bash
docker-compose up --build
```

### Kubernetes (k3s)

```bash
make deploy
```

## Project Structure

```
Cards/
├── backend/            # Rust Actix-web API
│   ├── src/
│   │   ├── db/         # Database layer (games, players, ELO, leaderboard)
│   │   ├── routes/     # HTTP handlers (games, players, ELO, leaderboard, admin)
│   │   ├── services/   # Business logic (ELO engine, email)
│   │   ├── models.rs   # Shared data models
│   │   ├── tasks.rs    # Background tasks
│   │   └── main.rs
│   └── Cargo.toml
├── frontend/           # Vue 3 SPA
│   ├── src/
│   │   ├── views/      # Page components per game type + leaderboard + player
│   │   ├── api/        # Typed API client
│   │   ├── stores/     # Pinia state stores
│   │   ├── components/ # Shared UI components
│   │   └── locales/    # i18n (EN / NL)
│   └── package.json
├── k8s/                # Kubernetes manifests
├── docker-compose.yaml
├── Makefile
├── dev.sh
└── deploy.sh
```

## API Overview

| Method | Path | Description |
|---|---|---|
| GET | `/api/health` | Health check |
| GET/POST | `/api/games` | List / create games |
| GET/DELETE | `/api/games/:id` | Get / delete a game |
| POST | `/api/games/:id/round` | Add a round |
| PATCH | `/api/games/:id/round/:idx` | Update a round |
| GET | `/api/leaderboard` | Global leaderboard |
| GET | `/api/players/search` | Search players |
| GET | `/api/players/:name` | Player profile |
| GET | `/api/players/:name/elo` | Player ELO data |
| GET | `/api/elo` | ELO leaderboard |
| POST | `/api/admin/recalculate-elo` | Recalculate all ELO |
| PUT | `/api/admin/elo-config` | Update ELO configuration |

## Environment Variables

### Backend

| Variable | Default | Description |
|---|---|---|
| `DATABASE_URL` | local postgres | PostgreSQL connection string |
| `RUST_LOG` | `info` | Log level |
| `EMAIL_ENABLED` | `false` | Enable SMTP email notifications |
| `EMAIL_HOST` | – | SMTP host |
| `EMAIL_PORT` | – | SMTP port |
| `EMAIL_USER` | – | SMTP username |
| `EMAIL_PASSWORD` | – | SMTP password |
| `EMAIL_FROM` | – | Sender address |

### Frontend

| Variable | Default | Description |
|---|---|---|
| `VITE_API_URL` | `/api` | Backend API base URL |

## License

MIT
