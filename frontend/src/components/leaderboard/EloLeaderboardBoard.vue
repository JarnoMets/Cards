<template>
  <div class="card board-card">
    <div class="board-card-header">
      <h3>{{ title }}</h3>
    </div>
    <div v-if="rows.length > 0" class="table-wrapper">
      <table class="board-table">
        <thead>
          <tr>
            <th>#</th>
            <th 
              class="sortable" 
              @click="$emit('sort', 'player')"
            >
              {{ t('leaderboard.tableHeaders.player') }}
              <span v-if="sortColumn === 'player'" class="sort-indicator">
                {{ sortDirection === 'asc' ? '↑' : '↓' }}
              </span>
            </th>
            <th 
              class="sortable" 
              @click="$emit('sort', 'elo')"
            >
              {{ t('leaderboard.tableHeaders.elo') }}
              <span v-if="sortColumn === 'elo'" class="sort-indicator">
                {{ sortDirection === 'asc' ? '↑' : '↓' }}
              </span>
            </th>
            <th>{{ t('leaderboard.tableHeaders.change7d') }}</th>
            <th 
              class="sortable" 
              @click="$emit('sort', 'games')"
            >
              {{ t('leaderboard.tableHeaders.games') }}
              <span v-if="sortColumn === 'games'" class="sort-indicator">
                {{ sortDirection === 'asc' ? '↑' : '↓' }}
              </span>
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(row, index) in displayedRows" :key="row.player_name">
            <td class="rank-cell">{{ index + 1 }}</td>
            <td>
              <a 
                class="player-link" 
                :href="`/${locale}/players/${encodeURIComponent(row.player_name)}`"
                :title="row.player_name"
              >
                {{ formatPlayerName(row.player_name) }}
              </a>
            </td>
            <td :class="getEloClass(row.elo)">{{ row.elo }}</td>
            <td class="elo-change-cell">
              <span 
                v-if="row.elo_change !== null && row.elo_change !== undefined" 
                :class="getEloChangeClass(row.elo_change)"
              >
                <span v-if="row.elo_change > 0">↑ +{{ row.elo_change }}</span>
                <span v-else-if="row.elo_change < 0">↓ {{ row.elo_change }}</span>
                <span v-else>—</span>
              </span>
              <span v-else class="elo-change-na">—</span>
            </td>
            <td>{{ row.games_played }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <p v-else class="empty-board">{{ t('leaderboard.noData') }}</p>
    
    <div v-if="rows.length > topCount" class="board-card-footer">
      <p class="showing-label">
        {{ t('leaderboard.showing', { count: displayedRows.length, total: rows.length }) }}
      </p>
      <button class="text" type="button" @click="$emit('toggle-expand')">
        {{ expanded ? t('leaderboard.hideFull') : t('leaderboard.showFull') }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

interface EloRow {
  player_name: string
  elo: number
  games_played: number
  elo_change?: number | null
}

const props = defineProps<{
  title: string
  rows: EloRow[]
  sortColumn: 'player' | 'elo' | 'games' | null
  sortDirection: 'asc' | 'desc'
  expanded: boolean
  topCount: number
  locale: string
}>()

defineEmits<{
  (e: 'sort', column: 'player' | 'elo' | 'games'): void
  (e: 'toggle-expand'): void
}>()

const { t } = useI18n()

const displayedRows = computed(() => {
  if (props.expanded) return props.rows
  return props.rows.slice(0, props.topCount)
})

const formatPlayerName = (name: string): string => {
  if (name.length > 15) {
    return name.substring(0, 12) + '...'
  }
  return name
}

const getEloClass = (elo: number): string => {
  if (elo >= 1800) return 'elo-master'
  if (elo >= 1500) return 'elo-high'
  if (elo >= 1200) return 'elo-mid'
  return 'elo-low'
}

const getEloChangeClass = (change: number): string => {
  if (change > 0) return 'elo-up'
  if (change < 0) return 'elo-down'
  return 'elo-neutral'
}
</script>

<style scoped>
.board-card {
  overflow: hidden;
}

.board-card-header {
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--color-border);
}

.board-card-header h3 {
  margin: 0;
  font-size: 1.1rem;
}

.table-wrapper {
  overflow-x: auto;
}

.board-table {
  width: 100%;
  border-collapse: collapse;
}

.board-table th,
.board-table td {
  padding: 0.75rem 1rem;
  text-align: left;
}

.board-table th {
  font-weight: 600;
  color: var(--color-text-muted);
  font-size: 0.875rem;
  border-bottom: 1px solid var(--color-border);
}

.board-table td {
  border-bottom: 1px solid var(--color-border-light, var(--color-border));
}

.board-table tbody tr:hover {
  background: var(--color-surface-hover, var(--color-surface));
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
  width: 50px;
  color: var(--color-text-muted);
}

.player-link {
  color: var(--color-primary);
  text-decoration: none;
}

.player-link:hover {
  text-decoration: underline;
}

.elo-change-cell {
  white-space: nowrap;
}

.elo-up {
  color: var(--color-success, #22c55e);
}

.elo-down {
  color: var(--color-error, #ef4444);
}

.elo-neutral {
  color: var(--color-text-muted);
}

.elo-change-na {
  color: var(--color-text-muted);
}

.elo-master {
  color: var(--color-gold, #ffd700);
  font-weight: 600;
}

.elo-high {
  color: var(--color-success, #22c55e);
}

.elo-mid {
  color: var(--color-text);
}

.elo-low {
  color: var(--color-warning, #f59e0b);
}

.empty-board {
  padding: 2rem;
  text-align: center;
  color: var(--color-text-muted);
}

.board-card-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1.25rem;
  border-top: 1px solid var(--color-border);
  background: var(--color-surface);
}

.showing-label {
  font-size: 0.875rem;
  color: var(--color-text-muted);
  margin: 0;
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
</style>
