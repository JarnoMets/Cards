# Performance Optimizations

This document describes the performance optimizations implemented in the Card Game Scorer application.

## Database Optimizations

### Materialized Views for Leaderboard

**Problem:** The leaderboard endpoint was processing all games in memory on every request, which became slow with many games.

**Solution:** Created materialized views and database indexes to pre-calculate player statistics.

#### Migration: `20250126000000_leaderboard_optimization.sql`

This migration adds:

1. **Indexes for faster queries:**
   - `idx_players_name` - Index on player names
   - `idx_players_name_game_id` - Composite index for player lookups
   - `idx_games_created_at` - Games sorted by date

2. **Materialized View: `player_leaderboard_stats`**
   - Pre-calculates player statistics per game type
   - Includes: games_played, wins, average_score, highest_score, lowest_score
   - Updated with `refresh_leaderboard_stats()` function

3. **View: `leaderboard_records`**
   - Pre-calculates top 50 highest and lowest scores per game type
   - Uses window functions for efficient ranking

#### Usage

The leaderboard endpoint (`GET /api/leaderboard`) now:
1. First tries to use the optimized materialized view
2. Falls back to the legacy in-memory processing if the view doesn't exist
3. Automatically refreshes the cache after games complete (logged, not blocking)

#### Manual Cache Refresh

To manually refresh the leaderboard cache:

```sql
SELECT refresh_leaderboard_stats();
```

Or via the backend API (future enhancement):
```bash
POST /api/admin/refresh-leaderboard
```

#### Performance Impact

- **Before:** O(n) where n = total number of games (processes all games every time)
- **After:** O(1) - Direct query from pre-calculated materialized view
- **Expected improvement:** 10-100x faster for large datasets (>1000 games)

### When to Refresh the Cache

The materialized view is automatically refreshed:
- After a game is completed (via background task)
- Can be manually triggered via SQL command

For high-traffic scenarios, consider:
- Scheduled refresh (e.g., every 5 minutes via cron)
- Disable automatic refresh and use scheduled refresh only

## Frontend Optimizations

### 1. Horizontal Scroll with Mouse Wheel

**Feature:** Game selection cards can now be scrolled horizontally using the mouse wheel.

**Implementation:**
- Added wheel event listener to the games grid
- Prevents default vertical scroll
- Converts vertical wheel delta to horizontal scroll
- Smooth scroll behavior with `scroll-behavior: smooth`

**Files Modified:**
- `frontend/src/views/GameSelection.vue`

### 2. Skeleton Loading for Leaderboard

**Problem:** Leaderboard felt laggy during loading with just a "Loading..." text.

**Solution:** Added animated skeleton placeholders.

**Implementation:**
- Created skeleton components that mimic the actual leaderboard structure
- Uses shimmer animation (gradient moving left to right)
- Shows placeholders for: filters, board cards, tables, and rows

**Files Modified:**
- `frontend/src/views/Leaderboard.vue`

**CSS Features:**
- `@keyframes shimmer` - Animated gradient for loading effect
- Responsive skeleton grid matching actual layout
- Uses same color scheme as actual components

### 3. Optimized Data Flow

**Before:**
```
Frontend Request → Backend → Fetch ALL games → Process in memory → Response
```

**After:**
```
Frontend Request → Backend → Query materialized view → Response
                          ↓ (fallback if needed)
                   Fetch games → Process → Response
```

## Future Optimizations

### Backend
- [ ] Add Redis cache layer for leaderboard data
- [ ] Implement pagination for large leaderboards
- [ ] Add database connection pooling optimization
- [ ] Consider read replicas for leaderboard queries

### Frontend
- [ ] Implement virtual scrolling for large leaderboards
- [ ] Add progressive loading (load top 10, then rest)
- [ ] Optimize bundle size with code splitting
- [ ] Add service worker for offline caching

### Database
- [ ] Add partial indexes for active games only
- [ ] Consider partitioning by game type
- [ ] Optimize JSONB queries with GIN indexes
- [ ] Add database query caching

## Monitoring

To monitor query performance:

```sql
-- Check materialized view refresh time
EXPLAIN ANALYZE SELECT refresh_leaderboard_stats();

-- Check query performance
EXPLAIN ANALYZE SELECT * FROM player_leaderboard_stats;

-- Check index usage
SELECT schemaname, tablename, indexname, idx_scan
FROM pg_stat_user_indexes
WHERE schemaname = 'public'
ORDER BY idx_scan;
```

## Rollback

If issues occur, you can revert to legacy processing:

1. Drop the materialized view:
```sql
DROP MATERIALIZED VIEW IF EXISTS player_leaderboard_stats CASCADE;
```

2. The backend will automatically fall back to in-memory processing.

## Benchmarks

### Leaderboard Load Time (Example Dataset: 1000 games)

| Implementation | Load Time | Database Query Time | Frontend Render Time |
|---------------|-----------|---------------------|---------------------|
| Legacy (in-memory) | 2500ms | 2200ms | 300ms |
| Optimized (materialized view) | 150ms | 50ms | 100ms |
| **Improvement** | **94% faster** | **98% faster** | **67% faster** |

*Note: These are example benchmarks. Actual performance depends on hardware and data size.*

## Maintenance

### Regular Tasks

1. **Monitor materialized view size:**
   ```sql
   SELECT pg_size_pretty(pg_total_relation_size('player_leaderboard_stats'));
   ```

2. **Check last refresh time:**
   ```sql
   SELECT last_refresh FROM pg_stat_user_tables 
   WHERE relname = 'player_leaderboard_stats';
   ```

3. **Vacuum and analyze regularly:**
   ```sql
   VACUUM ANALYZE players;
   VACUUM ANALYZE games;
   ```

## Support

For issues or questions about these optimizations, see:
- Database migrations: `backend/migrations/`
- Backend code: `backend/src/db.rs` and `backend/src/routes.rs`
- Frontend code: `frontend/src/views/Leaderboard.vue`
