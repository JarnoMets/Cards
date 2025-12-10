# Database Schema Documentation

## Overview

The application uses PostgreSQL with a simple schema. Games and players are stored in separate tables with a foreign key relationship.

## Tables

### `games`

Stores game metadata.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | TEXT | PRIMARY KEY | UUID string |
| `game_type` | TEXT | NOT NULL | One of: `hearts`, `king`, `double_king`, `color_whist` |
| `created_at` | TIMESTAMPTZ | NOT NULL | Game creation timestamp |
| `current_round` | INTEGER | NOT NULL, DEFAULT 0 | Current round number (0-indexed) |

### `players`

Stores player data for each game.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| `id` | TEXT | PRIMARY KEY | UUID string |
| `game_id` | TEXT | NOT NULL, FK → games(id) ON DELETE CASCADE | Parent game |
| `name` | TEXT | NOT NULL | Player display name |
| `position` | INTEGER | NOT NULL | Player position (0-indexed) |
| `scores` | JSONB | NOT NULL, DEFAULT '[]' | Array of round scores |
| `created_at` | TIMESTAMPTZ | NOT NULL, DEFAULT NOW() | Record creation time |

### Indexes

```sql
CREATE INDEX idx_players_game_id ON players(game_id);
CREATE INDEX idx_games_created_at ON games(created_at DESC);
```

## Data Examples

### Hearts Game Player Scores

```json
// Simple scores array - each element is one round's score
[0, 13, 5, 26, 0, -26]  // -26 = shot the moon
```

### King Game Player Scores

```json
// 10 rounds with different scoring rules
[-10, -20, -30, -50, -160, -90, 25, 25, 25, 25]
```

### Double King Game Player Scores

```json
// Complex format with game indices
// scores[i] corresponds to game_indices[i]
// Game index 0-5 = base games (no tricks, no hearts, etc.)
// Game index 6-9 = trump games for each player
```

### Color Whist Player Scores

```json
// Each round can have multiple contracts
// Scores calculated from contract success/failure
[40, -20, 100, 0, -80]
```

## Entity Relationships

```
┌─────────────┐       ┌─────────────┐
│    games    │       │   players   │
├─────────────┤       ├─────────────┤
│ id (PK)     │◄──────│ game_id (FK)│
│ game_type   │   1:N │ id (PK)     │
│ created_at  │       │ name        │
│ current_round│       │ position    │
└─────────────┘       │ scores      │
                      │ created_at  │
                      └─────────────┘
```

## Queries

### Get all games for leaderboard

```sql
SELECT g.id, g.game_type, g.current_round, g.created_at
FROM games g
ORDER BY g.created_at DESC;
```

### Get player games by name

```sql
SELECT DISTINCT g.*
FROM games g
JOIN players p ON p.game_id = g.id
WHERE LOWER(p.name) = LOWER($1)
ORDER BY g.created_at DESC;
```

### Search players

```sql
SELECT DISTINCT LOWER(name) as name
FROM players
WHERE LOWER(name) LIKE LOWER($1 || '%')
GROUP BY LOWER(name)
ORDER BY LOWER(name)
LIMIT 20;
```

### Rename player globally

```sql
UPDATE players
SET name = $2
WHERE LOWER(name) = LOWER($1);
```

## Migrations

Migrations are in `backend/migrations/`. They run automatically on backend startup.

### Running migrations manually

```bash
# Connect to postgres pod
kubectl exec -it -n postgres postgres-0 -- psql -U postgresuser -d postgresdb

# Or locally
psql postgres://postgresuser:postgrespwd@localhost:5432/postgresdb
```

## Backup & Restore

### Backup

```bash
kubectl exec -n postgres postgres-0 -- pg_dump -U postgresuser postgresdb > backup.sql
```

### Restore

```bash
kubectl exec -i -n postgres postgres-0 -- psql -U postgresuser postgresdb < backup.sql
```
