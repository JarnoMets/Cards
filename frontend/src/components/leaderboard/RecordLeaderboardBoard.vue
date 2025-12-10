<template>
  <div class="card record-board-card">
    <div class="record-board-header">
      <h3>{{ title }}</h3>
      <span class="badge" v-if="!expanded && hasMore">
        {{ t('leaderboard.topBadge', { count: topCount }) }}
      </span>
    </div>

    <div v-if="rows.length > 0" class="table-wrapper mini">
      <table class="record-table">
        <thead>
          <tr>
            <th>#</th>
            <th class="sortable" @click="$emit('sort', 'player')">
              {{ t('leaderboard.tableHeaders.player') }}
              <span v-if="sortColumn === 'player'" class="sort-indicator">
                {{ sortDirection === 'asc' ? '↑' : '↓' }}
              </span>
            </th>
            <th class="sortable" @click="$emit('sort', 'score')">
              {{ t('leaderboard.tableHeaders.score') }}
              <span v-if="sortColumn === 'score'" class="sort-indicator">
                {{ sortDirection === 'asc' ? '↑' : '↓' }}
              </span>
            </th>
            <th class="sortable" @click="$emit('sort', 'date')">
              {{ t('leaderboard.tableHeaders.date') }}
              <span v-if="sortColumn === 'date'" class="sort-indicator">
                {{ sortDirection === 'asc' ? '↑' : '↓' }}
              </span>
            </th>
          </tr>
        </thead>
        <tbody>
          <tr 
            v-for="(row, index) in displayedRows" 
            :key="`${row.player_name}-${row.game_id}`"
            class="clickable-row"
            @click="openGame(row.game_type, row.game_id)"
          >
            <td class="rank-cell">{{ index + 1 }}</td>
            <td>
              <a 
                class="player-link" 
                :href="`/${locale}/players/${encodeURIComponent(row.player_name)}`"
                :title="row.player_name"
                @click.stop
              >
                {{ formatPlayerName(row.player_name) }}
              </a>
            </td>
            <td :class="scoreClass">{{ row.score }}</td>
            <td class="date-cell">{{ formatDateCompact(row.occurred_at) }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <p v-else class="empty-board">{{ t('leaderboard.records.noRecord') }}</p>

    <div v-if="hasMore" class="record-board-footer">
      <button class="text" type="button" @click="$emit('toggle-expand')">
        {{ expanded ? t('leaderboard.hideFull') : t('leaderboard.showFull') }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import type { GameType } from '@/types'

interface RecordRow {
  player_name: string
  score: number
  game_id: string
  game_type: GameType
  occurred_at: string
}

const props = defineProps<{
  title: string
  rows: RecordRow[]
  sortColumn: 'player' | 'score' | 'date' | null
  sortDirection: 'asc' | 'desc'
  expanded: boolean
  topCount: number
  locale: string
  isHighScore?: boolean
}>()

defineEmits<{
  (e: 'sort', column: 'player' | 'score' | 'date'): void
  (e: 'toggle-expand'): void
}>()

const { t } = useI18n()
const router = useRouter()

const hasMore = computed(() => props.rows.length > props.topCount)

const displayedRows = computed(() => {
  if (props.expanded) return props.rows
  return props.rows.slice(0, props.topCount)
})

const scoreClass = computed(() => {
  return props.isHighScore ? 'score-high' : 'score-low'
})

const formatPlayerName = (name: string): string => {
  if (name.length > 15) {
    return name.substring(0, 12) + '...'
  }
  return name
}

const formatDateCompact = (dateString: string): string => {
  const date = new Date(dateString)
  // Compact format: "Nov 27" or "Nov '24" if older than this year
  const now = new Date()
  const sameYear = date.getFullYear() === now.getFullYear()
  
  if (sameYear) {
    return date.toLocaleDateString(undefined, {
      month: 'short',
      day: 'numeric'
    })
  } else {
    return date.toLocaleDateString(undefined, {
      month: 'short',
      year: '2-digit'
    })
  }
}

const openGame = (gameType: GameType, gameId: string) => {
  // Navigate to the game
  const gameRoutes: Record<GameType, string> = {
    hearts: 'hearts',
    king: 'king',
    double_king: 'double-king',
    color_whist: 'color-whist',
    whist: 'whist',
    manille: 'manille'
  }
  const routeName = gameRoutes[gameType] || 'king'
  router.push(`/${props.locale}/${routeName}/${gameId}`)
}
</script>

<style scoped>
.record-board-card {
  overflow: hidden;
}

.record-board-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--color-border);
}

.record-board-header h3 {
  margin: 0;
  font-size: 1rem;
}

.badge {
  font-size: 0.75rem;
  padding: 0.25rem 0.5rem;
  background: var(--color-primary);
  color: var(--color-primary-foreground, white);
  border-radius: var(--radius-sm, 4px);
}

.table-wrapper.mini {
  max-height: 400px;
  overflow-y: auto;
}

.record-table {
  width: 100%;
  border-collapse: collapse;
}

.record-table th,
.record-table td {
  padding: 0.5rem 0.75rem;
  text-align: left;
}

.record-table th {
  font-weight: 600;
  color: var(--color-text-muted);
  font-size: 0.8rem;
  border-bottom: 1px solid var(--color-border);
  position: sticky;
  top: 0;
  background: var(--color-card);
}

.record-table td {
  font-size: 0.875rem;
  border-bottom: 1px solid var(--color-border-light, var(--color-border));
}

.record-table tbody tr:hover {
  background: var(--color-surface-hover, var(--color-surface));
}

.record-table tbody tr.clickable-row {
  cursor: pointer;
  transition: background 0.15s ease;
}

.record-table tbody tr.clickable-row:hover {
  background: var(--color-surface-hover, rgba(0, 0, 0, 0.04));
}

.sortable {
  cursor: pointer;
  user-select: none;
}

.sortable:hover {
  color: var(--color-text);
}

.sort-indicator {
  margin-left: 0.25rem;
}

.rank-cell {
  width: 40px;
  color: var(--color-text-muted);
}

.player-link {
  color: var(--color-primary);
  text-decoration: none;
}

.player-link:hover {
  text-decoration: underline;
}

.date-cell {
  color: var(--color-text-muted);
  white-space: nowrap;
}

.score-high {
  color: var(--color-success, #22c55e);
  font-weight: 600;
}

.score-low {
  color: var(--color-warning, #f59e0b);
  font-weight: 600;
}

.empty-board {
  padding: 1.5rem;
  text-align: center;
  color: var(--color-text-muted);
}

.record-board-footer {
  padding: 0.5rem 1rem;
  text-align: center;
  border-top: 1px solid var(--color-border);
  background: var(--color-surface);
}

button.text {
  background: none;
  border: none;
  color: var(--color-primary);
  cursor: pointer;
  font-size: 0.875rem;
  padding: 0;
}

button.text:hover {
  text-decoration: underline;
}

/* Mobile responsiveness */
@media (max-width: 480px) {
  .record-table th,
  .record-table td {
    padding: 0.4rem 0.5rem;
    font-size: 0.8rem;
  }

  .record-table th {
    font-size: 0.7rem;
  }

  .rank-cell {
    width: 30px;
    padding-left: 0.4rem !important;
    padding-right: 0.3rem !important;
  }

  .date-cell {
    font-size: 0.75rem;
    padding-right: 0.4rem !important;
  }

  .record-board-header {
    padding: 0.75rem 1rem;
  }

  .record-board-header h3 {
    font-size: 0.9rem;
  }
}
</style>
