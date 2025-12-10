# Development Guide

## Quick Start

### Prerequisites

- Docker & Docker Compose
- Node.js 18+
- Rust 1.75+
- kubectl (for deployment)

### Local Development

```bash
# Start database
docker-compose up -d postgres

# Backend (terminal 1)
cd backend
cargo run

# Frontend (terminal 2)
cd frontend
npm install
npm run dev
```

Or use the dev script:

```bash
./dev.sh
```

### URLs

- Frontend: http://localhost:5173
- Backend API: http://localhost:8080/api
- PostgreSQL: localhost:5432

---

## Project Structure

```
├── backend/              # Rust API server
│   ├── src/
│   │   ├── main.rs       # Entry point, route setup
│   │   ├── routes.rs     # HTTP handlers
│   │   ├── db.rs         # Database operations
│   │   ├── models.rs     # Data structures
│   │   ├── notifications.rs  # Email system
│   │   └── services/     # Game-specific logic
│   └── migrations/       # SQL migrations
│
├── frontend/             # Vue 3 SPA
│   ├── src/
│   │   ├── views/        # Page components
│   │   ├── components/   # Shared components
│   │   ├── stores/       # Pinia stores
│   │   ├── api/          # Backend API calls
│   │   ├── types/        # TypeScript types
│   │   ├── locales/      # i18n translations
│   │   └── styles/       # CSS files
│   └── index.html
│
├── k8s/                  # Kubernetes manifests
├── grafana/              # Monitoring dashboards
└── docs/                 # Documentation
```

---

## Backend Development

### Adding a New Endpoint

1. Add route in `main.rs`:
```rust
.route("/games/{id}/my-endpoint", web::post().to(routes::my_handler))
```

2. Implement handler in `routes.rs`:
```rust
pub async fn my_handler(
    db: web::Data<Database>,
    path: web::Path<String>,
    body: web::Json<MyRequest>,
) -> impl Responder {
    // Implementation
}
```

3. Add database method in `db.rs` if needed.

### Running Tests

```bash
cd backend
cargo test
```

### Building Release

```bash
cd backend
cargo build --release
```

---

## Frontend Development

### Adding a New View

1. Create component in `views/`:
```vue
<script setup lang="ts">
import { useI18n } from 'vue-i18n'
const { t } = useI18n()
</script>

<template>
  <div class="container">
    <h1>{{ t('page.title') }}</h1>
  </div>
</template>
```

2. Add route in `router/index.ts`:
```typescript
{
  path: '/:lang/my-page',
  name: 'MyPage',
  component: () => import('../views/MyPage.vue')
}
```

3. Add translations in `locales/en.json` and `locales/nl.json`.

### Component Guidelines

- Use Composition API with `<script setup>`
- Use `useI18n()` for all text
- Responsive breakpoints: 480px, 768px, 1024px, 1920px
- Use CSS variables from `style.css`

### Building

```bash
cd frontend
npm run build  # Output in dist/
```

---

## Environment Variables

### Backend

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | - | PostgreSQL connection string |
| `PORT` | 8080 | Server port |
| `SMTP_HOST` | - | SMTP server hostname |
| `SMTP_PORT` | 587 | SMTP port |
| `SMTP_USER` | - | SMTP username |
| `SMTP_PASS` | - | SMTP password |
| `SMTP_FROM` | - | Email from address |
| `SMTP_USE_TLS` | true | Use STARTTLS |
| `BASE_URL` | - | Frontend URL for email links |

### Frontend

Configured in `vite.config.ts`:

| Variable | Description |
|----------|-------------|
| `VITE_API_URL` | Backend API base URL |

---

## CSS Theming

Theme variables in `style.css`:

```css
:root {
  --bg: #0e1b12;
  --paper: #1a2f20;
  --paper-darker: #152619;
  --ink: #d4e5d8;
  --ink-muted: #8fa897;
  --accent: #3d7a4a;
  --accent-hover: #4a9659;
  --accent-gold: #c9a227;
  --accent-gold-light: #e6c547;
  --border: #2d4a35;
  --error: #e74c3c;
  --success: #27ae60;
}
```

---

## Debugging

### Backend Logs

```bash
# Local
cargo run 2>&1 | tee backend.log

# Kubernetes
kubectl logs -f -n cards deployment/card-game-scorer-backend
```

### Database Inspection

```bash
# Connect to database
kubectl exec -it -n postgres postgres-0 -- psql -U postgresuser -d postgresdb

# Common queries
SELECT * FROM games ORDER BY created_at DESC LIMIT 5;
SELECT name, COUNT(*) FROM players GROUP BY name;
```

### Frontend Debug

Use Vue DevTools browser extension.

---

## Deployment

See [DEPLOYMENT.md](../DEPLOYMENT.md) for full deployment instructions.

Quick deploy:

```bash
./deploy.sh
```

This builds Docker images and applies k8s manifests.

---

## Testing Email Notifications

1. Set up Brevo account and verify sender domain
2. Configure k8s secrets with SMTP credentials
3. Send test email:

```bash
curl -X POST "https://cards.jarnomets.com/api/notifications/test?email=your@email.com"
```
