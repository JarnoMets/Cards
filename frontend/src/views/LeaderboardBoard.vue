<template>
  <div class="leaderboard-board-view">
    <div class="leaderboard-board-inner">
      <div class="page-header">
        <button class="secondary" type="button" @click="goBack">{{ t('leaderboard.backToOverview') }}</button>
        <button class="primary" type="button" @click="goToPlayerSearch">{{ t('leaderboard.searchPlayers') }}</button>
        <LanguageSelector />
      </div>

      <div class="card board-summary" v-if="leaderboard">
        <h1>{{ t('leaderboard.fullRankingTitle', { board: t(currentBoardMeta.labelKey) }) }}</h1>
      </div>

      <div class="card filters-card">
        <div class="filters-grid">
          <label class="filter-field">
            <span>{{ t('leaderboard.filters.board') }}</span>
            <select :value="currentBoardKey" @change="onBoardChange">
              <option v-for="option in boardOptions" :key="option.key" :value="option.key">
                {{ option.label }}
              </option>
            </select>
          </label>
          <label class="filter-field">
            <span>{{ t('leaderboard.filters.minGames') }}</span>
            <select v-model.number="minGamesFilter">
              <option v-for="option in minGameOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </label>
          <label class="filter-field search-field">
            <span>{{ t('leaderboard.filters.search') }}</span>
            <input
              type="search"
              v-model.trim="searchTerm"
              :placeholder="t('leaderboard.filters.searchPlaceholder')"
            />
          </label>
          <div class="filter-actions">
            <button class="link" type="button" :disabled="!hasActiveFilters" @click="clearFilters">
              {{ t('leaderboard.filters.clear') }}
            </button>
          </div>
        </div>
      </div>

      <div v-if="loading" class="state-card">
        <div class="card">{{ t('loading') }}</div>
      </div>
      <div v-else-if="error" class="state-card">
        <div class="card error-card">
          <p>{{ error }}</p>
          <button class="primary" type="button" @click="loadLeaderboard">{{ t('leaderboard.retry') }}</button>
        </div>
      </div>
      <div v-else class="card table-card">
        <div class="table-meta">
          <span>{{ t('leaderboard.showing', { count: filteredRows.length, total: totalRows }) }}</span>
        </div>
        <div class="table-wrapper mini">
          <table class="board-table">
            <thead>
              <tr>
                <th class="sortable" @click="setSortColumn('player')">
                  {{ t('leaderboard.tableHeaders.player') }}
                  <span class="sort-indicator" v-if="sortState.column === 'player'">{{ sortState.direction === 'asc' ? '↑' : '↓' }}</span>
                </th>
                <th class="sortable" @click="setSortColumn('games')">
                  {{ t('leaderboard.tableHeaders.games') }}
                  <span class="sort-indicator" v-if="sortState.column === 'games'">{{ sortState.direction === 'asc' ? '↑' : '↓' }}</span>
                </th>
                <th class="sortable" @click="setSortColumn('wins')">
                  {{ t('leaderboard.tableHeaders.wins') }}
                  <span class="sort-indicator" v-if="sortState.column === 'wins'">{{ sortState.direction === 'asc' ? '↑' : '↓' }}</span>
                </th>
                <th class="sortable" @click="setSortColumn('winRate')">
                  {{ t('leaderboard.tableHeaders.winRate') }}
                  <span class="sort-indicator" v-if="sortState.column === 'winRate'">{{ sortState.direction === 'asc' ? '↑' : '↓' }}</span>
                </th>
                <th v-if="hasAverageColumn" class="sortable" @click="setSortColumn('average')">
                  {{ t('leaderboard.tableHeaders.average') }}
                  <span class="sort-indicator" v-if="sortState.column === 'average'">{{ sortState.direction === 'asc' ? '↑' : '↓' }}</span>
                </th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="row in filteredRows" :key="row.playerName">
                <td>
                  <button class="player-link" type="button" :title="row.playerName" @click="openPlayer(row.playerName)">
                    {{ formatPlayerName(row.playerName) }}
                  </button>
                </td>
                <td>{{ row.games }}</td>
                <td>{{ row.wins }}</td>
                <td>{{ formatWinRate(row.winRate) }}</td>
                <td v-if="hasAverageColumn">{{ formatAverage(row.averageScore) }}</td>
              </tr>
            </tbody>
          </table>
          <p v-if="filteredRows.length === 0" class="empty-board">{{ t('leaderboard.noData') }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import LanguageSelector from '@/components/LanguageSelector.vue'
import { gamesApi } from '../api/games'
import type { LeaderboardResponse } from '../types'
import {
  buildLeaderboardRows,
  leaderboardBoardMeta,
  leaderboardBoardOrder,
  sortLeaderboardRows,
  type LeaderboardBoardKey
} from '../utils/leaderboard'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const leaderboard = ref<LeaderboardResponse | null>(null)
const loading = ref(true)
const error = ref('')
const minGamesFilter = ref(0)
const searchTerm = ref('')
const MIN_GAME_OPTION_VALUES = [0, 5, 10, 20]

// Sorting state
type SortColumn = 'player' | 'games' | 'wins' | 'winRate' | 'average'
type SortDirection = 'asc' | 'desc'
const sortState = ref<{ column: SortColumn | null; direction: SortDirection }>({
  column: 'winRate',
  direction: 'desc'
})

const setSortColumn = (column: SortColumn) => {
  if (sortState.value.column === column) {
    sortState.value.direction = sortState.value.direction === 'asc' ? 'desc' : 'asc'
  } else {
    sortState.value = {
      column,
      direction: column === 'player' ? 'asc' : 'desc'
    }
  }
}

const localeParam = computed(() => (route.params.locale as string) || 'en')

const isBoardKey = (value: string | undefined): value is LeaderboardBoardKey => {
  if (!value) return false
  return (leaderboardBoardOrder as LeaderboardBoardKey[]).includes(value as LeaderboardBoardKey)
}

const currentBoardKey = computed<LeaderboardBoardKey>(() =>
  isBoardKey(route.params.board as string) ? (route.params.board as LeaderboardBoardKey) : 'overall'
)

watch(
  () => route.params.board,
  (value) => {
    if (!isBoardKey(value as string)) {
      router.replace(`/${localeParam.value}/leaderboard/overall`)
    }
  },
  { immediate: true }
)

const currentBoardMeta = computed(() => leaderboardBoardMeta[currentBoardKey.value])

const boardOptions = computed(() =>
  leaderboardBoardOrder.map((key) => ({
    key,
    label: t(leaderboardBoardMeta[key].labelKey)
  }))
)

const minGameOptions = computed(() =>
  MIN_GAME_OPTION_VALUES.map((value) => ({
    value,
    label:
      value === 0
        ? t('leaderboard.filters.options.none')
        : t('leaderboard.filters.options.atLeast', { count: value })
  }))
)

const hasActiveFilters = computed(() => minGamesFilter.value !== 0 || searchTerm.value.trim().length > 0)

const clearFilters = () => {
  minGamesFilter.value = 0
  searchTerm.value = ''
}

const loadLeaderboard = async () => {
  try {
    loading.value = true
    error.value = ''
    leaderboard.value = await gamesApi.getLeaderboard()
  } catch (err: any) {
    error.value = err.response?.data?.error || err.message || 'Failed to load leaderboard'
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadLeaderboard()
})

const rowsMap = computed(() => buildLeaderboardRows(leaderboard.value))

const sortedRows = computed(() => sortLeaderboardRows(rowsMap.value[currentBoardKey.value]))

const filteredRows = computed(() => {
  const search = searchTerm.value.trim().toLowerCase()
  const minGames = minGamesFilter.value
  let rows = sortedRows.value.filter((row) => {
    if (minGames > 0 && row.games < minGames) return false
    if (search && !row.playerName.toLowerCase().includes(search)) return false
    return true
  })
  
  // Apply custom sorting
  if (sortState.value.column) {
    rows = [...rows].sort((a, b) => {
      let cmp = 0
      switch (sortState.value.column) {
        case 'player':
          cmp = a.playerName.localeCompare(b.playerName)
          break
        case 'games':
          cmp = a.games - b.games
          break
        case 'wins':
          cmp = a.wins - b.wins
          break
        case 'winRate':
          cmp = a.winRate - b.winRate
          break
        case 'average':
          cmp = (a.averageScore ?? 0) - (b.averageScore ?? 0)
          break
      }
      return sortState.value.direction === 'asc' ? cmp : -cmp
    })
  }
  
  return rows
})

const totalRows = computed(() => sortedRows.value.length)

const hasAverageColumn = computed(() => currentBoardKey.value !== 'overall')

const formatAverage = (value: number | null) => {
  if (value === null || Number.isNaN(value)) return '—'
  return value.toFixed(1)
}

const formatWinRate = (value: number) => `${(value * 100).toFixed(1)}%`

const formatPlayerName = (name: string) => {
  const maxLength = 18
  if (name.length <= maxLength) return name
  return `${name.slice(0, maxLength - 1).trimEnd()}…`
}

const goBack = () => {
  router.push(`/${localeParam.value}/leaderboard`)
}

const goToPlayerSearch = () => {
  router.push(`/${localeParam.value}/players`)
}

const openPlayer = (name: string) => {
  router.push(`/${localeParam.value}/players/${encodeURIComponent(name)}`)
}

const onBoardChange = (event: Event) => {
  const value = (event.target as HTMLSelectElement).value
  if (!isBoardKey(value)) return
  router.replace(`/${localeParam.value}/leaderboard/${value}`)
}
</script>

<style scoped>
.leaderboard-board-view {
  min-height: 100vh;
  padding: 72px clamp(16px, 5vw, 48px) 56px;
  background: radial-gradient(circle at top, rgba(255, 255, 255, 0.35), rgba(255, 255, 255, 0)),
    linear-gradient(180deg, rgba(201, 156, 93, 0.07), rgba(255, 255, 255, 0)),
    var(--texture-noise),
    var(--paper-cream);
  background-size: auto, auto, 320px 320px, auto;
  color: var(--ink);
}

.leaderboard-board-inner {
  max-width: min(1600px, 96vw);
  margin: 0 auto;
  background: rgba(255, 255, 255, 0.7);
  border-radius: 32px;
  padding: clamp(20px, 3vw, 48px);
  border: 1px solid rgba(45, 31, 26, 0.12);
  box-shadow: 0 30px 80px rgba(33, 22, 12, 0.15);
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px dashed rgba(45, 31, 26, 0.18);
}

.board-summary {
  margin-bottom: 20px;
  padding: 0;
}

.board-summary h1 {
  margin-bottom: 0;
  font-family: var(--font-heading);
  color: var(--ink);
}

.filters-card {
  margin-bottom: 20px;
  background: rgba(255, 255, 255, 0.4);
  border-radius: 16px;
  padding: 20px;
  border: 1px solid rgba(45, 31, 26, 0.15);
  box-shadow: none;
}

.filters-grid {
  display: grid;
  gap: 16px;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  align-items: end;
}

.filter-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 0.9rem;
}

.filter-field select,
.filter-field input {
  padding: 10px 12px;
  border-radius: 12px;
  border: 1.5px solid rgba(45, 31, 26, 0.2);
  font-size: 1rem;
  background: rgba(255, 255, 255, 0.92);
  color: var(--ink);
}

.filter-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
}

.table-card {
  background: transparent;
  border-radius: 16px;
  padding: 20px;
  border: 1px dashed rgba(45, 31, 26, 0.2);
  box-shadow: none;
}

.state-card {
  margin-top: 20px;
}

.error-card {
  display: flex;
  flex-direction: column;
  gap: 12px;
  color: var(--danger-color);
}

.table-meta {
  display: flex;
  justify-content: space-between;
  font-size: 0.9rem;
  margin-bottom: 12px;
  color: var(--ink-muted);
}

.table-wrapper {
  overflow-x: auto;
}

.table-wrapper.mini table {
  font-size: 0.9rem;
}

table {
  width: 100%;
  border-collapse: collapse;
  table-layout: fixed;
}

th,
td {
  padding: 12px;
  text-align: left;
  border-bottom: 1px solid rgba(45, 31, 26, 0.1);
  white-space: nowrap;
}

th {
  font-size: 0.85rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--ink-muted);
}

th.sortable {
  cursor: pointer;
  user-select: none;
  transition: color 0.2s ease;
}

th.sortable:hover {
  color: var(--accent-emerald);
}

.sort-indicator {
  margin-left: 4px;
  font-size: 0.75rem;
  color: var(--accent-emerald);
}

.board-table {
  table-layout: fixed;
}

.board-table th:first-child,
.board-table td:first-child {
  width: 25%;
  overflow: hidden;
  text-overflow: ellipsis;
}

.board-table th:not(:first-child),
.board-table td:not(:first-child) {
  text-align: center;
}

.board-table tbody tr:nth-child(even) {
  background: rgba(201, 156, 93, 0.06);
}

.player-name {
  display: inline-block;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  vertical-align: middle;
}

.player-link {
  background: none;
  border: none;
  color: var(--accent-emerald);
  font-weight: 600;
  padding: 0;
  display: inline-flex;
  max-width: 100%;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  cursor: pointer;
  text-align: left;
}

.player-link:hover {
  text-decoration: underline;
}

.board-table th:nth-child(2),
.board-table td:nth-child(2) {
  width: 18%;
}

.board-table th:nth-child(3),
.board-table td:nth-child(3) {
  width: 18%;
}

.board-table th:nth-child(4),
.board-table td:nth-child(4) {
  width: 19%;
}

.board-table th:nth-child(5),
.board-table td:nth-child(5) {
  width: 20%;
}

.empty-board {
  margin-top: 12px;
  font-style: italic;
  color: var(--ink-muted);
}

button.link {
  border: none;
  background: none;
  color: var(--accent-emerald);
  font-weight: 600;
  cursor: pointer;
}

button.link:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }

  .filters-grid {
    grid-template-columns: 1fr;
  }
}
</style>
