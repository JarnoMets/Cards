<template>
  <div class="leaderboard-view">
    <div class="leaderboard-inner">
      <div class="page-header">
        <div class="header-top-row">
          <button class="secondary" type="button" @click="goHome">
            {{ t('leaderboard.backButton') }}
          </button>
          <div class="header-right">
            <button class="primary search-players-btn" type="button" @click="goToPlayerSearch">
              {{ t('leaderboard.searchPlayers') }}
            </button>
            <LanguageSelector class="header-lang-selector" />
          </div>
        </div>
        <h1>{{ t('leaderboard.title') }}</h1>
      </div>

      <div v-if="loading" class="loading-skeleton">
        <div class="skeleton-tabs">
          <div class="skeleton-tab" v-for="i in 3" :key="i"></div>
        </div>
        <div class="skeleton-filter-card">
          <div class="skeleton-title"></div>
          <div class="skeleton-filters">
            <div class="skeleton-filter"></div>
            <div class="skeleton-filter"></div>
          </div>
        </div>
        <div class="skeleton-table-card">
          <div class="skeleton-table">
            <div class="skeleton-row" v-for="j in 10" :key="j"></div>
          </div>
        </div>
      </div>

      <div v-else-if="error" class="error-container">
        <h2>{{ t('error') }}</h2>
        <p>{{ error }}</p>
        <button class="primary" type="button" @click="loadLeaderboard">{{ t('leaderboard.retry') }}</button>
      </div>

      <div v-else-if="leaderboard" class="leaderboard-content">
        <div v-if="leaderboard.players.length === 0" class="card empty-state">
          {{ t('leaderboard.noData') }}
        </div>

        <div v-else class="leaderboard-main">
          <!-- Tabs -->
          <div class="tabs-container">
            <button
              v-for="tab in tabs"
              :key="tab.id"
              class="tab-button"
              :class="{ active: activeTab === tab.id }"
              @click="activeTab = tab.id"
            >
              {{ t(tab.labelKey) }}
            </button>
          </div>

          <!-- Filters -->
          <div class="card filter-card">
            <div class="filter-card-header">
              <h2>{{ t('leaderboard.filters.title') }}</h2>
              <button class="link" type="button" :disabled="!hasActiveFilters" @click="clearFilters">
                {{ t('leaderboard.filters.clear') }}
              </button>
            </div>
            <div class="filters-grid">
              <!-- Game Type Filter -->
              <label class="filter-field">
                <span>{{ t('leaderboard.gameFilter.label') }}</span>
                <select v-model="gameTypeFilter">
                  <option value="all">{{ t('leaderboard.gameFilter.all') }}</option>
                  <option v-for="gt in gameTypes" :key="gt.value" :value="gt.value">
                    {{ t(gt.labelKey) }}
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
            </div>
          </div>

          <!-- Wins Tab Content -->
          <div v-if="activeTab === 'wins'" class="tab-content">
            <div class="card board-card">
              <div class="board-card-header">
                <h3>{{ currentBoardTitle }}</h3>
              </div>
              <div v-if="filteredWinsRows.length > 0" class="table-wrapper">
                <table class="board-table">
                  <thead>
                    <tr>
                      <th>#</th>
                      <th class="sortable" @click="setWinsSortColumn('player')">
                        {{ t('leaderboard.tableHeaders.player') }}
                        <span class="sort-indicator" v-if="winsSortState.column === 'player'">{{ winsSortState.direction === 'asc' ? '↑' : '↓' }}</span>
                      </th>
                      <th class="sortable" @click="setWinsSortColumn('games')">
                        {{ t('leaderboard.tableHeaders.games') }}
                        <span class="sort-indicator" v-if="winsSortState.column === 'games'">{{ winsSortState.direction === 'asc' ? '↑' : '↓' }}</span>
                      </th>
                      <th class="sortable" @click="setWinsSortColumn('wins')">
                        {{ t('leaderboard.tableHeaders.wins') }}
                        <span class="sort-indicator" v-if="winsSortState.column === 'wins'">{{ winsSortState.direction === 'asc' ? '↑' : '↓' }}</span>
                      </th>
                      <th class="sortable" @click="setWinsSortColumn('winRate')">
                        {{ t('leaderboard.tableHeaders.winRate') }}
                        <span class="sort-indicator" v-if="winsSortState.column === 'winRate'">{{ winsSortState.direction === 'asc' ? '↑' : '↓' }}</span>
                      </th>
                      <th v-if="gameTypeFilter !== 'all'" class="sortable" @click="setWinsSortColumn('average')">
                        {{ t('leaderboard.tableHeaders.average') }}
                        <span class="sort-indicator" v-if="winsSortState.column === 'average'">{{ winsSortState.direction === 'asc' ? '↑' : '↓' }}</span>
                      </th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="(row, index) in displayedWinsRows" :key="row.playerName">
                      <td class="rank-cell">{{ index + 1 }}</td>
                      <td>
                        <a class="player-link" :href="`/${localeParam}/players/${encodeURIComponent(row.playerName)}`" :title="row.playerName">
                          {{ formatPlayerName(row.playerName) }}
                        </a>
                      </td>
                      <td>{{ row.games }}</td>
                      <td>{{ row.wins }}</td>
                      <td>{{ formatWinRate(row.winRate) }}</td>
                      <td v-if="gameTypeFilter !== 'all'">{{ formatAverage(row.averageScore) }}</td>
                    </tr>
                  </tbody>
                </table>
              </div>
              <p v-else class="empty-board">{{ t('leaderboard.noData') }}</p>
              <div class="board-card-footer" v-if="filteredWinsRows.length > TOP_COUNT">
                <p class="showing-label">
                  {{ t('leaderboard.showing', { count: displayedWinsRows.length, total: filteredWinsRows.length }) }}
                </p>
                <button class="text" type="button" @click="winsExpanded = !winsExpanded">
                  {{ winsExpanded ? t('leaderboard.hideFull') : t('leaderboard.showFull') }}
                </button>
              </div>
            </div>
          </div>

          <!-- ELO Tab Content -->
          <div v-if="activeTab === 'elo'" class="tab-content">
            <div class="card board-card">
              <div class="board-card-header">
                <h3>{{ eloTitle }}</h3>
              </div>
              <div v-if="filteredEloRows.length > 0" class="table-wrapper">
                <table class="board-table">
                  <thead>
                    <tr>
                      <th>#</th>
                      <th class="sortable" @click="setEloSortColumn('player')">
                        {{ t('leaderboard.tableHeaders.player') }}
                        <span class="sort-indicator" v-if="eloSortState.column === 'player'">{{ eloSortState.direction === 'asc' ? '↑' : '↓' }}</span>
                      </th>
                      <th class="sortable" @click="setEloSortColumn('elo')">
                        {{ t('leaderboard.tableHeaders.elo') }}
                        <span class="sort-indicator" v-if="eloSortState.column === 'elo'">{{ eloSortState.direction === 'asc' ? '↑' : '↓' }}</span>
                      </th>
                      <th>{{ t('leaderboard.tableHeaders.change7d') }}</th>
                      <th class="sortable" @click="setEloSortColumn('games')">
                        {{ t('leaderboard.tableHeaders.games') }}
                        <span class="sort-indicator" v-if="eloSortState.column === 'games'">{{ eloSortState.direction === 'asc' ? '↑' : '↓' }}</span>
                      </th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="(row, index) in displayedEloRows" :key="row.player_name">
                      <td class="rank-cell">{{ index + 1 }}</td>
                      <td>
                        <a class="player-link" :href="`/${localeParam}/players/${encodeURIComponent(row.player_name)}`" :title="row.player_name">
                          {{ formatPlayerName(row.player_name) }}
                        </a>
                      </td>
                      <td :class="getEloClass(row.elo)">{{ row.elo }}</td>
                      <td class="elo-change-cell">
                        <span v-if="row.elo_change !== null && row.elo_change !== undefined" :class="getEloChangeClass(row.elo_change)">
                          <span v-if="row.elo_change > 0">↑ +{{ row.elo_change }}</span>
                          <span v-else-if="row.elo_change < 0">↓ {{ row.elo_change }}</span>
                          <span v-else>+0</span>
                        </span>
                        <span v-else class="elo-change-na">—</span>
                      </td>
                      <td>{{ row.games_played }}</td>
                    </tr>
                  </tbody>
                </table>
              </div>
              <p v-else class="empty-board">{{ t('leaderboard.noData') }}</p>
              <div class="board-card-footer" v-if="filteredEloRows.length > TOP_COUNT">
                <p class="showing-label">
                  {{ t('leaderboard.showing', { count: displayedEloRows.length, total: filteredEloRows.length }) }}
                </p>
                <button class="text" type="button" @click="eloExpanded = !eloExpanded">
                  {{ eloExpanded ? t('leaderboard.hideFull') : t('leaderboard.showFull') }}
                </button>
              </div>
            </div>
          </div>

          <!-- Records Tab Content -->
          <div v-if="activeTab === 'records'" class="tab-content">
            <div v-if="filteredRecordTables.length === 0" class="card empty-state">
              <p>{{ t('leaderboard.records.noRecord') }}</p>
            </div>
            <div v-else class="record-board-grid">
              <div class="card record-board-card" v-for="record in filteredRecordTables" :key="record.key">
                <div class="record-board-header">
                  <h3>{{ t(record.titleKey) }}</h3>
                  <span class="badge" v-if="!record.isExpanded && record.hasMore">
                    {{ t('leaderboard.topBadge', { count: TOP_COUNT }) }}
                  </span>
                </div>

                <div v-if="record.rows.length > 0" class="table-wrapper mini">
                  <table class="record-table">
                    <thead>
                      <tr>
                        <th>#</th>
                        <th class="sortable" @click="setRecordSortColumn(record.key, 'player')">
                          {{ t('leaderboard.tableHeaders.player') }}
                          <span class="sort-indicator" v-if="getRecordSortState(record.key).column === 'player'">{{ getRecordSortState(record.key).direction === 'asc' ? '↑' : '↓' }}</span>
                        </th>
                        <th class="sortable" @click="setRecordSortColumn(record.key, 'score')">
                          {{ t('leaderboard.records.scoreColumn') }}
                          <span class="sort-indicator" v-if="getRecordSortState(record.key).column === 'score'">{{ getRecordSortState(record.key).direction === 'asc' ? '↑' : '↓' }}</span>
                        </th>
                        <th class="sortable" @click="setRecordSortColumn(record.key, 'date')">
                          {{ t('leaderboard.records.dateColumn') }}
                          <span class="sort-indicator" v-if="getRecordSortState(record.key).column === 'date'">{{ getRecordSortState(record.key).direction === 'asc' ? '↑' : '↓' }}</span>
                        </th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr v-for="(row, index) in record.visibleRows" :key="`${record.key}-${row.game_id}`">
                        <td>{{ index + 1 }}</td>
                        <td>
                          <a class="player-link" :href="`/${localeParam}/players/${encodeURIComponent(row.player_name)}`" :title="row.player_name">
                            {{ formatPlayerName(row.player_name) }}
                          </a>
                        </td>
                        <td>{{ row.score }}</td>
                        <td>{{ formatTimestamp(row.occurred_at) }}</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
                <p v-else class="empty-board">{{ t('leaderboard.records.noRecord') }}</p>

                <div class="board-card-footer">
                  <p class="showing-label">
                    {{ t('leaderboard.showing', { count: record.visibleCount, total: record.total }) }}
                  </p>
                  <div class="board-card-actions">
                    <button
                      class="text"
                      type="button"
                      :disabled="!record.hasMore && !record.isExpanded"
                      @click="toggleRecordVisibility(record.key)"
                    >
                      {{ record.isExpanded ? t('leaderboard.hideFull') : t('leaderboard.showFull') }}
                    </button>
                    <button class="secondary" type="button" @click="viewRecordDetails(record.mode, record.scope)">
                      {{ t('leaderboard.records.viewDetails') }}
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
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
import type { LeaderboardResponse, EloLeaderboardResponse, OverallEloLeaderboardResponse, GameType } from '../types'
import {
  buildLeaderboardRows,
  leaderboardBoardMeta,
  sortLeaderboardRows,
  type LeaderboardBoardKey,
} from '../utils/leaderboard'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const localeParam = computed(() => (route.params.locale as string) || 'en')

// Data
const leaderboard = ref<LeaderboardResponse | null>(null)
const eloData = ref<EloLeaderboardResponse | null>(null)
const overallEloData = ref<OverallEloLeaderboardResponse | null>(null)
const loading = ref(true)
const error = ref('')
const TOP_COUNT = 25
const MIN_GAME_OPTION_VALUES = [0, 5, 10, 20]

// Tab state
type TabId = 'wins' | 'elo' | 'records'
const activeTab = ref<TabId>('elo')
const tabs = [
  { id: 'elo' as const, labelKey: 'leaderboard.tabs.elo' },
  { id: 'wins' as const, labelKey: 'leaderboard.tabs.wins' },
  { id: 'records' as const, labelKey: 'leaderboard.tabs.records' }
]

// Filters
const gameTypeFilter = ref<'all' | GameType>('all')
const minGamesFilter = ref(0)
const searchTerm = ref('')

const gameTypes: { value: GameType; labelKey: string }[] = [
  { value: 'hearts', labelKey: 'leaderboard.boards.hearts' },
  { value: 'king', labelKey: 'leaderboard.boards.king' },
  { value: 'double_king', labelKey: 'leaderboard.boards.doubleKing' },
  { value: 'color_whist', labelKey: 'leaderboard.boards.colorWhist' },
  { value: 'whist', labelKey: 'leaderboard.boards.whist' },
  { value: 'manille', labelKey: 'leaderboard.boards.manille' }
]

// Wins sorting
type WinsSortColumn = 'player' | 'games' | 'wins' | 'winRate' | 'average'
type SortDirection = 'asc' | 'desc'
const winsSortState = ref<{ column: WinsSortColumn; direction: SortDirection }>({
  column: 'wins',
  direction: 'desc'
})
const winsExpanded = ref(false)

// ELO sorting
type EloSortColumn = 'player' | 'elo' | 'games'
const eloSortState = ref<{ column: EloSortColumn; direction: SortDirection }>({
  column: 'elo',
  direction: 'desc'
})
const eloExpanded = ref(false)

// Records
type RecordTableKey = 'king_highest' | 'king_lowest' | 'double_king_highest' | 'double_king_lowest'
type RecordScope = 'highest' | 'lowest'
type RecordMode = 'king' | 'double_king'
type RecordSortColumn = 'player' | 'score' | 'date'
type RecordSortState = { column: RecordSortColumn | null; direction: SortDirection }

const recordVisibility = ref<Record<RecordTableKey, boolean>>({
  king_highest: false,
  king_lowest: false,
  double_king_highest: false,
  double_king_lowest: false
})

const recordSortState = ref<Record<RecordTableKey, RecordSortState>>({
  king_highest: { column: 'score', direction: 'desc' },
  king_lowest: { column: 'score', direction: 'asc' },
  double_king_highest: { column: 'score', direction: 'desc' },
  double_king_lowest: { column: 'score', direction: 'asc' }
})

const getRecordSortState = (recordKey: RecordTableKey) => recordSortState.value[recordKey]

const setWinsSortColumn = (column: WinsSortColumn) => {
  if (winsSortState.value.column === column) {
    winsSortState.value.direction = winsSortState.value.direction === 'asc' ? 'desc' : 'asc'
  } else {
    winsSortState.value = {
      column,
      direction: column === 'player' ? 'asc' : 'desc'
    }
  }
}

const setEloSortColumn = (column: EloSortColumn) => {
  if (eloSortState.value.column === column) {
    eloSortState.value.direction = eloSortState.value.direction === 'asc' ? 'desc' : 'asc'
  } else {
    eloSortState.value = {
      column,
      direction: column === 'player' ? 'asc' : 'desc'
    }
  }
}

const setRecordSortColumn = (recordKey: RecordTableKey, column: RecordSortColumn) => {
  const current = recordSortState.value[recordKey]
  if (current.column === column) {
    recordSortState.value[recordKey] = {
      column,
      direction: current.direction === 'asc' ? 'desc' : 'asc'
    }
  } else {
    recordSortState.value[recordKey] = {
      column,
      direction: column === 'player' ? 'asc' : 'desc'
    }
  }
}

const loadLeaderboard = async () => {
  try {
    loading.value = true
    error.value = ''
    const [leaderboardData, eloResponse, overallEloResponse] = await Promise.all([
      gamesApi.getLeaderboard(),
      gamesApi.getEloLeaderboard().catch(() => null),
      gamesApi.getOverallEloLeaderboard().catch(() => null)
    ])
    leaderboard.value = leaderboardData
    eloData.value = eloResponse
    overallEloData.value = overallEloResponse
    
    // Fetch ELO comparisons for visible players (top players first)
    const playerNames = overallEloResponse?.ratings?.slice(0, TOP_COUNT)?.map(r => r.player_name) || []
    if (playerNames.length > 0) {
      fetchEloComparisons(playerNames)
    }
  } catch (err: any) {
    error.value = err.response?.data?.error || err.message || 'Failed to load leaderboard'
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  loadLeaderboard()
})

// Watch for game type filter changes to fetch new comparisons
watch(gameTypeFilter, () => {
  // Clear cached comparisons when filter changes
  eloComparisons.value = {}
  
  // Fetch new comparisons for visible players
  const playerNames = displayedEloRows.value.slice(0, TOP_COUNT).map(r => r.player_name)
  if (playerNames.length > 0) {
    fetchEloComparisons(playerNames)
  }
})

const goHome = () => {
  router.push(`/${localeParam.value}`)
}

const goToPlayerSearch = () => {
  router.push(`/${localeParam.value}/players`)
}

const minGameOptions = computed(() =>
  MIN_GAME_OPTION_VALUES.map((value) => ({
    value,
    label:
      value === 0
        ? t('leaderboard.filters.options.none')
        : t('leaderboard.filters.options.atLeast', { count: value })
  }))
)

const hasActiveFilters = computed(() => {
  return gameTypeFilter.value !== 'all' || minGamesFilter.value !== 0 || searchTerm.value.trim().length > 0
})

const clearFilters = () => {
  gameTypeFilter.value = 'all'
  minGamesFilter.value = 0
  searchTerm.value = ''
}

// Computed: Board title based on game filter
const currentBoardTitle = computed(() => {
  if (gameTypeFilter.value === 'all') {
    return t('leaderboard.boards.overall')
  }
  const meta = leaderboardBoardMeta[gameTypeFilter.value as LeaderboardBoardKey]
  return meta ? t(meta.labelKey) : ''
})

const eloTitle = computed(() => {
  if (gameTypeFilter.value === 'all') {
    return t('leaderboard.tabs.elo') + ' - ' + t('leaderboard.boards.overall')
  }
  const meta = leaderboardBoardMeta[gameTypeFilter.value as LeaderboardBoardKey]
  return t('leaderboard.tabs.elo') + ' - ' + (meta ? t(meta.labelKey) : '')
})

// WINS TAB: Build rows based on game type filter
const boardRows = computed(() => buildLeaderboardRows(leaderboard.value))

const filteredWinsRows = computed(() => {
  const search = searchTerm.value.trim().toLowerCase()
  const minGames = minGamesFilter.value
  const boardKey: LeaderboardBoardKey = gameTypeFilter.value === 'all' ? 'overall' : gameTypeFilter.value
  
  let rows = sortLeaderboardRows(boardRows.value[boardKey]).filter((row) => {
    if (minGames > 0 && row.games < minGames) return false
    if (search && !row.playerName.toLowerCase().includes(search)) return false
    return true
  })
  
  // Apply custom sorting
  rows = [...rows].sort((a, b) => {
    let cmp = 0
    switch (winsSortState.value.column) {
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
    return winsSortState.value.direction === 'asc' ? cmp : -cmp
  })
  
  return rows
})

const displayedWinsRows = computed(() => {
  return winsExpanded.value ? filteredWinsRows.value : filteredWinsRows.value.slice(0, TOP_COUNT)
})

// ELO TAB: Build rows based on game type filter
interface EloRow {
  player_name: string
  elo: number
  games_played: number
  elo_change: number | null
}

// Store for ELO comparison data
const eloComparisons = ref<Record<string, number | null>>({})

// Fetch comparison data for visible players
const fetchEloComparisons = async (playerNames: string[]) => {
  const gameType = gameTypeFilter.value === 'all' ? undefined : gameTypeFilter.value
  
  // Only fetch for players we don't have data for
  const toFetch = playerNames.filter(name => !(name in eloComparisons.value))
  
  // Fetch in parallel but limit concurrency
  const results = await Promise.allSettled(
    toFetch.map(name => gamesApi.getPlayerEloComparison(name, gameType))
  )
  
  results.forEach((result, index) => {
    const name = toFetch[index]
    if (result.status === 'fulfilled') {
      eloComparisons.value[name] = result.value.elo_change
    } else {
      eloComparisons.value[name] = null
    }
  })
}

const filteredEloRows = computed((): EloRow[] => {
  const search = searchTerm.value.trim().toLowerCase()
  const minGames = minGamesFilter.value
  
  let rows: EloRow[] = []
  
  if (gameTypeFilter.value === 'all') {
    // Use overall ELO data
    if (overallEloData.value) {
      rows = overallEloData.value.ratings.map(r => ({
        player_name: r.player_name,
        elo: r.elo,
        games_played: r.games_played,
        elo_change: eloComparisons.value[r.player_name] ?? null
      }))
    }
  } else {
    // Use per-game-type ELO data
    if (eloData.value) {
      rows = eloData.value.ratings
        .filter(r => r.game_type === gameTypeFilter.value)
        .map(r => ({
          player_name: r.player_name,
          elo: r.elo,
          games_played: r.games_played,
          elo_change: eloComparisons.value[r.player_name] ?? null
        }))
    }
  }
  
  // Apply filters
  rows = rows.filter((row) => {
    if (minGames > 0 && row.games_played < minGames) return false
    if (search && !row.player_name.toLowerCase().includes(search)) return false
    return true
  })
  
  // Apply sorting
  rows = [...rows].sort((a, b) => {
    let cmp = 0
    switch (eloSortState.value.column) {
      case 'player':
        cmp = a.player_name.localeCompare(b.player_name)
        break
      case 'elo':
        cmp = a.elo - b.elo
        break
      case 'games':
        cmp = a.games_played - b.games_played
        break
    }
    return eloSortState.value.direction === 'asc' ? cmp : -cmp
  })
  
  return rows
})

const displayedEloRows = computed(() => {
  return eloExpanded.value ? filteredEloRows.value : filteredEloRows.value.slice(0, TOP_COUNT)
})

// RECORDS TAB: Filter by game type (only king and double_king have records)
const recordTableConfig: Array<{
  key: RecordTableKey
  titleKey: string
  mode: RecordMode
  scope: RecordScope
}> = [
  { key: 'king_highest', titleKey: 'leaderboard.records.kingHighest', mode: 'king', scope: 'highest' },
  { key: 'king_lowest', titleKey: 'leaderboard.records.kingLowest', mode: 'king', scope: 'lowest' },
  { key: 'double_king_highest', titleKey: 'leaderboard.records.doubleKingHighest', mode: 'double_king', scope: 'highest' },
  { key: 'double_king_lowest', titleKey: 'leaderboard.records.doubleKingLowest', mode: 'double_king', scope: 'lowest' }
]

const filteredRecordTables = computed(() => {
  if (!leaderboard.value) return []
  
  // Filter config based on game type selection
  let config = recordTableConfig
  if (gameTypeFilter.value === 'king') {
    config = recordTableConfig.filter(c => c.mode === 'king')
  } else if (gameTypeFilter.value === 'double_king') {
    config = recordTableConfig.filter(c => c.mode === 'double_king')
  } else if (gameTypeFilter.value !== 'all') {
    // Hearts, Color Whist, Whist, Manille don't have records
    return []
  }
  
  const search = searchTerm.value.trim().toLowerCase()
  
  return config.map((configItem) => {
    let rows = leaderboard.value ? [...(leaderboard.value.records[configItem.key] ?? [])] : []
    
    // Apply search filter
    if (search) {
      rows = rows.filter(r => r.player_name.toLowerCase().includes(search))
    }
    
    const sortState = recordSortState.value[configItem.key]
    
    // Apply sorting
    if (sortState.column) {
      rows.sort((a, b) => {
        let cmp = 0
        switch (sortState.column) {
          case 'player':
            cmp = a.player_name.localeCompare(b.player_name)
            break
          case 'score':
            cmp = a.score - b.score
            break
          case 'date':
            cmp = new Date(a.occurred_at).getTime() - new Date(b.occurred_at).getTime()
            break
        }
        return sortState.direction === 'asc' ? cmp : -cmp
      })
    }
    
    const isExpanded = recordVisibility.value[configItem.key]
    const visibleRows = isExpanded ? rows : rows.slice(0, TOP_COUNT)
    return {
      ...configItem,
      rows,
      visibleRows,
      visibleCount: visibleRows.length,
      total: rows.length,
      isExpanded,
      hasMore: rows.length > TOP_COUNT
    }
  })
})

const toggleRecordVisibility = (key: RecordTableKey) => {
  recordVisibility.value[key] = !recordVisibility.value[key]
}

const viewRecordDetails = (mode: RecordMode, scope: RecordScope) => {
  router.push(`/${localeParam.value}/leaderboard/records/${mode}/${scope}`)
}

// Formatting helpers
const formatAverage = (value: number | null) => {
  if (value === null || Number.isNaN(value)) return '—'
  return value.toFixed(1)
}

const formatWinRate = (value: number) => {
  return `${(value * 100).toFixed(1)}%`
}

const getEloClass = (elo: number | null): string => {
  if (elo === null) return ''
  if (elo >= 1700) return 'elo-high'
  if (elo >= 1200) return 'elo-mid'
  return 'elo-low'
}

const getEloChangeClass = (change: number | null): string => {
  if (change === null || change === 0) return 'elo-change-neutral'
  return change > 0 ? 'elo-change-up' : 'elo-change-down'
}

const formatPlayerName = (name: string) => {
  const maxLength = 20
  if (name.length <= maxLength) return name
  return `${name.slice(0, maxLength - 1).trimEnd()}…`
}

const formatTimestamp = (value: string) => {
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) {
    return value
  }
  return date.toLocaleString()
}
</script>

<style scoped>
.leaderboard-view {
  min-height: 100vh;
  padding: 72px clamp(16px, 5vw, 48px) 56px;
  background: radial-gradient(circle at top, rgba(255, 255, 255, 0.35), rgba(255, 255, 255, 0)),
    linear-gradient(180deg, rgba(201, 156, 93, 0.07), rgba(255, 255, 255, 0)),
    var(--texture-noise),
    var(--paper-cream);
  background-size: auto, auto, 320px 320px, auto;
  color: var(--ink);
}

.leaderboard-inner {
  max-width: min(1200px, 96vw);
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
  flex-direction: column;
  gap: 12px;
  margin-bottom: 20px;
  padding-bottom: 20px;
  border-bottom: 1px dashed rgba(45, 31, 26, 0.18);
}

.header-top-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.page-header h1 {
  font-size: clamp(1.8rem, 3vw, 2.5rem);
  margin: 0;
  font-family: var(--font-heading);
  color: var(--ink);
}

/* Tabs */
.tabs-container {
  display: flex;
  gap: 4px;
  padding: 4px;
  background: rgba(45, 31, 26, 0.06);
  border-radius: 16px;
  margin-bottom: 8px;
}

.tab-button {
  flex: 1;
  padding: 12px 24px;
  border: none;
  background: transparent;
  color: var(--ink-secondary);
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  border-radius: 12px;
  transition: all 0.2s ease;
}

.tab-button:hover {
  color: var(--ink);
  background: rgba(255, 255, 255, 0.5);
}

.tab-button.active {
  background: white;
  color: var(--accent-emerald);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

/* Loading skeleton */
.loading-skeleton {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.skeleton-tabs {
  display: flex;
  gap: 8px;
  padding: 8px;
  background: rgba(45, 31, 26, 0.06);
  border-radius: 16px;
}

.skeleton-tab {
  flex: 1;
  height: 44px;
  background: linear-gradient(90deg, rgba(45, 31, 26, 0.08) 25%, rgba(45, 31, 26, 0.12) 50%, rgba(45, 31, 26, 0.08) 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: 12px;
}

.skeleton-filter-card {
  background: rgba(255, 255, 255, 0.4);
  border: 1px solid rgba(45, 31, 26, 0.2);
  border-radius: 16px;
  padding: 20px;
}

.skeleton-title {
  height: 24px;
  width: 150px;
  background: linear-gradient(90deg, rgba(45, 31, 26, 0.08) 25%, rgba(45, 31, 26, 0.12) 50%, rgba(45, 31, 26, 0.08) 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: 8px;
  margin-bottom: 16px;
}

.skeleton-filters {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 16px;
}

.skeleton-filter {
  height: 48px;
  background: linear-gradient(90deg, rgba(45, 31, 26, 0.08) 25%, rgba(45, 31, 26, 0.12) 50%, rgba(45, 31, 26, 0.08) 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: 12px;
}

.skeleton-table-card {
  background: transparent;
  border: 1px dashed rgba(45, 31, 26, 0.2);
  border-radius: 16px;
  padding: 20px;
}

.skeleton-table {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.skeleton-row {
  height: 40px;
  background: linear-gradient(90deg, rgba(45, 31, 26, 0.08) 25%, rgba(45, 31, 26, 0.12) 50%, rgba(45, 31, 26, 0.08) 75%);
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: 6px;
}

@keyframes shimmer {
  0% {
    background-position: -200% 0;
  }
  100% {
    background-position: 200% 0;
  }
}

.error-container,
.empty-state {
  text-align: center;
  padding: 40px;
}

.card {
  background: transparent;
  color: var(--ink);
  border-radius: 16px;
  padding: 20px;
  box-shadow: none;
  border: 1px dashed rgba(45, 31, 26, 0.2);
}

.leaderboard-main {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.leaderboard-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.filter-card {
  padding: 20px;
  background: rgba(255, 255, 255, 0.4);
  border-style: solid;
}

.filter-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.filter-card-header h2 {
  font-family: var(--font-heading);
  color: var(--ink);
  margin: 0;
}

.filters-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
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

/* Tab content */
.tab-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.board-card-header {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 12px;
  align-items: flex-start;
}

.board-card-header h3 {
  margin: 0;
  font-family: var(--font-heading);
  color: var(--ink);
}

.table-wrapper {
  overflow-x: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th,
td {
  padding: 12px;
  text-align: left;
  border-bottom: 1px solid rgba(45, 31, 26, 0.1);
}

th {
  font-size: 0.85rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--ink-muted);
  font-family: var(--font-family);
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

.rank-cell {
  width: 50px;
  text-align: center;
  font-weight: 600;
  color: var(--ink-muted);
}

.board-table,
.record-table {
  table-layout: auto;
}

.board-table tbody tr:nth-child(even),
.record-table tbody tr:nth-child(even) {
  background: rgba(201, 156, 93, 0.06);
}

.board-table th:not(:first-child),
.board-table td:not(:first-child):not(:nth-child(2)) {
  text-align: center;
}

.board-card-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 12px;
  margin-top: 20px;
}

.board-card-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.showing-label {
  font-size: 0.85rem;
  color: var(--ink-muted);
}

.empty-board {
  margin: 12px 0;
  color: var(--ink-muted);
  font-style: italic;
}

button.link,
button.text {
  border: none;
  background: none;
  color: var(--accent-emerald);
  font-weight: 600;
  cursor: pointer;
}

button.link:disabled,
button.text:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

/* Records grid */
.record-board-grid {
  display: grid;
  gap: 24px;
  grid-template-columns: repeat(auto-fit, minmax(360px, 1fr));
}

.record-board-card {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.record-board-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 12px;
}

.record-board-header h3 {
  margin: 0;
  font-family: var(--font-heading);
  color: var(--ink);
}

.badge {
  align-self: flex-start;
  background: linear-gradient(120deg, #2f5c4c, #1f3a31);
  color: #fff;
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 0.75rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.table-wrapper.mini table {
  font-size: 0.9rem;
}

.record-table th,
.record-table td {
  font-size: 0.85rem;
  padding: 10px;
  white-space: nowrap;
}

.record-table th:first-child,
.record-table td:first-child {
  width: 12%;
  text-align: center;
}

.record-table th:nth-child(2),
.record-table td:nth-child(2) {
  width: 38%;
  text-align: left;
}

.record-table th:nth-child(3),
.record-table td:nth-child(3) {
  width: 20%;
  text-align: center;
}

.record-table th:nth-child(4),
.record-table td:nth-child(4) {
  width: 30%;
  text-align: center;
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
  text-decoration: none;
}

.player-link:hover {
  text-decoration: underline;
}

/* ELO styling - all ratings use standard text color */
.elo-high,
.elo-mid,
.elo-low {
  color: var(--ink);
}

/* ELO change indicators */
.elo-change-cell {
  text-align: center;
  font-size: 0.85rem;
  font-weight: 600;
}

.elo-change-up {
  color: #2f8f5d;
}

.elo-change-down {
  color: #c41e3a;
}

.elo-change-neutral {
  color: var(--ink-muted);
}

.elo-change-na {
  color: var(--ink-muted);
  font-style: italic;
}

@media (max-width: 768px) {
  .header-top-row {
    width: 100%;
  }

  .search-players-btn {
    display: none;
  }

  .tabs-container {
    flex-direction: column;
  }

  .tab-button {
    text-align: center;
  }

  .filters-grid {
    grid-template-columns: 1fr;
  }

  .board-card-actions {
    width: 100%;
    flex-direction: column;
    align-items: stretch;
  }

  .record-board-grid {
    grid-template-columns: 1fr;
  }

  th,
  td {
    padding: 10px 8px;
  }
}
</style>
