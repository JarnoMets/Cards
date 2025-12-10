# API Reference

Base URL: `/api`

## Games

### List All Games

```
GET /games
```

Returns all games ordered by creation date (newest first).

**Response:**
```json
[
  {
    "id": "uuid",
    "game_type": "hearts",
    "created_at": "2025-01-15T12:00:00Z",
    "current_round": 5,
    "players": [
      {
        "id": "uuid",
        "game_id": "uuid",
        "name": "Alice",
        "position": 0,
        "scores": [0, 13, 5, 26, 0]
      }
    ]
  }
]
```

---

### Get Single Game

```
GET /games/{id}
```

**Response:** Same as list item above.

**Errors:**
- `404` - Game not found

---

### Create Game

```
POST /games
```

**Request Body:**
```json
{
  "game_type": "hearts",
  "players": ["Alice", "Bob", "Charlie", "Dave"]
}
```

**Response:**
```json
{
  "id": "uuid",
  "game_type": "hearts",
  "created_at": "2025-01-15T12:00:00Z",
  "current_round": 0,
  "players": [...]
}
```

---

### Delete Game

```
DELETE /games/{id}
```

**Response:**
```json
{
  "message": "Game deleted successfully"
}
```

---

## Hearts-Specific

### Submit Hearts Round

```
POST /games/{id}/hearts/round
```

**Request Body:**
```json
{
  "scores": [0, 13, 0, 13]  // Must sum to 26 (or contain -26 for moon shot)
}
```

---

### Undo Hearts Round

```
POST /games/{id}/hearts/undo
```

Removes the last round.

---

## King-Specific

### Submit King Round

```
POST /games/{id}/king/round
```

**Request Body:**
```json
{
  "scores": [-10, -20, -30, -50]
}
```

---

### Undo King Round

```
POST /games/{id}/king/undo
```

---

## Double King-Specific

### Submit Double King Round

```
POST /games/{id}/double-king/round
```

**Request Body:**
```json
{
  "game_index": 0,
  "scores": [-10, -20, -30, -50]
}
```

---

### Undo Double King Round

```
POST /games/{id}/double-king/undo
```

---

## Color Whist-Specific

### Submit Color Whist Round

```
POST /games/{id}/color-whist/round
```

**Request Body:**
```json
{
  "contracts": [...],
  "scores": [40, -20, 100, 0]
}
```

---

### Undo Color Whist Round

```
POST /games/{id}/color-whist/undo
```

---

## Leaderboard

### Get Leaderboard Data

```
GET /leaderboard
```

**Response:**
```json
{
  "players": [
    {
      "name": "Alice",
      "games_played": 15,
      "wins": 8,
      "total_points": 1250
    }
  ],
  "game_type_stats": {...}
}
```

---

## Players

### Search Players

```
GET /players/search?q={query}
```

Returns player names matching the query prefix.

**Response:**
```json
["Alice", "Alex", "Amanda"]
```

---

### Get Player Details

```
GET /players/{name}
```

**Response:**
```json
{
  "name": "Alice",
  "games": [...],
  "stats": {
    "total_games": 25,
    "wins": 12,
    "games_by_type": {...}
  }
}
```

---

### Rename Player

```
POST /players/rename
```

**Request Body:**
```json
{
  "old_name": "Bob",
  "new_name": "Robert"
}
```

Updates all games where this player participated.

---

## Notifications

### Send Test Email

```
POST /notifications/test
```

**Query Params:**
- `email` - Recipient email address

Sends a test notification with the most recent game.

---

### Send Game Email

```
POST /notifications/send
```

**Request Body:**
```json
{
  "game_id": "uuid",
  "email": "user@example.com"
}
```

Sends game summary email.

---

## Health

### Health Check

```
GET /health
```

**Response:**
```json
{
  "status": "ok",
  "database": "connected"
}
```

---

## Error Responses

All errors follow this format:

```json
{
  "error": "Error message description"
}
```

Common status codes:
- `400` - Bad request (validation error)
- `404` - Resource not found
- `500` - Internal server error
