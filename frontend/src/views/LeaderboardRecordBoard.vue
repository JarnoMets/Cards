<template>
  <div class="record-board-view">
    <div class="record-board-inner">
      <div class="page-header">
        <button class="secondary" type="button" @click="goBack">{{ t('leaderboard.records.detail.back') }}</button>
        <LanguageSelector />
      </div>

      <div class="card summary-card" v-if="leaderboard">
        <h1>{{ detailTitle }}</h1>
      </div>

      <div class="card filters-card">
        <div class="filters-grid">
          <label class="filter-field">
            <span>{{ t('leaderboard.filters.board') }}</span>
            <select :value="currentMode" @change="onModeChange">
              <option v-for="option in modeOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </label>
          <label class="filter-field">
            <span>{{ t('leaderboard.sort.label') }}</span>
            <select v-model="sortState.key">
              <option v-for="option in sortOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </label>
          <label class="filter-field">
            <span>{{ t('leaderboard.records.detail.search') }}</span>
            <input
              type="search"
              v-model.trim="searchTerm"
              :placeholder="t('leaderboard.records.detail.searchPlaceholder')"
            />
          </label>
          <label class="filter-field">
            <span>{{ t('leaderboard.records.scopeLabel') }}</span>
            <select :value="currentScope" @change="onScopeChange">
              <option v-for="option in scopeOptions" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </label>
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
          <span>{{ t('leaderboard.showing', { count: visibleRows.length, total: totalRows }) }}</span>
          <button class="text" type="button" @click="toggleDirection">
            {{ sortState.direction === 'asc' ? t('leaderboard.sort.directionAsc') : t('leaderboard.sort.directionDesc') }}
          </button>
        </div>
        <div class="table-wrapper">
          <table>
            <thead>
              <tr>
                <th>#</th>
                <th class="sortable" @click="setSort('player')">
                  {{ t('leaderboard.records.detail.player') }}
                  <span class="sort-indicator" v-if="sortState.key === 'player'">{{ sortState.direction === 'asc' ? '↑' : '↓' }}</span>
                </th>
                <th class="sortable" @click="setSort('score')">
                  {{ t('leaderboard.records.detail.score') }}
                  <span class="sort-indicator" v-if="sortState.key === 'score'">{{ sortState.direction === 'asc' ? '↑' : '↓' }}</span>
                </th>
                <th class="sortable" @click="setSort('date')">
                  {{ t('leaderboard.records.detail.date') }}
                  <span class="sort-indicator" v-if="sortState.key === 'date'">{{ sortState.direction === 'asc' ? '↑' : '↓' }}</span>
                </th>
                <th class="sortable" @click="setSort('game')">
                  {{ t('leaderboard.records.detail.game') }}
                  <span class="sort-indicator" v-if="sortState.key === 'game'">{{ sortState.direction === 'asc' ? '↑' : '↓' }}</span>
                </th>
              </tr>
            </thead>
            <tbody>
              <tr 
                v-for="(row, index) in visibleRows" 
                :key="`${row.game_id}-${index}`"
                class="clickable-row"
                @click="openGame(row.game_id)"
              >
                <td>{{ index + 1 }}</td>
                <td>{{ row.player_name }}</td>
                <td>{{ row.score }}</td>
                <td>{{ formatTimestamp(row.occurred_at) }}</td>
                <td class="game-id-cell">{{ row.game_id.slice(0, 8) }}...</td>
              </tr>
            </tbody>
          </table>
          <p v-if="visibleRows.length === 0" class="empty-board">{{ t('leaderboard.records.noRecord') }}</p>
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

type RecordMode = 'king' | 'double_king'
type RecordScope = 'highest' | 'lowest'
type RecordTableKey = `${RecordMode}_${RecordScope}`

type SortKey = 'player' | 'score' | 'date' | 'game'
type SortDirection = 'asc' | 'desc'

interface SortState {
  key: SortKey
  direction: SortDirection
}

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const leaderboard = ref<LeaderboardResponse | null>(null)
const loading = ref(true)
const error = ref('')
const searchTerm = ref('')
const sortState = ref<SortState>({ key: 'score', direction: 'desc' })

const localeParam = computed(() => (route.params.locale as string) || 'en')

const isRecordMode = (value: string | undefined): value is RecordMode =>
  value === 'king' || value === 'double_king'
const isRecordScope = (value: string | undefined): value is RecordScope =>
  value === 'highest' || value === 'lowest'

const currentMode = computed<RecordMode>(() =>
  isRecordMode(route.params.mode as string) ? (route.params.mode as RecordMode) : 'king'
)
const currentScope = computed<RecordScope>(() =>
  isRecordScope(route.params.scope as string) ? (route.params.scope as RecordScope) : 'highest'
)

watch(
  () => [route.params.mode, route.params.scope],
  ([mode, scope]) => {
    const validMode = isRecordMode(mode as string)
    const validScope = isRecordScope(scope as string)
    if (!validMode || !validScope) {
      router.replace(`/${localeParam.value}/leaderboard/records/king/highest`)
    }
  },
  { immediate: true }
)

const modeOptions = computed(() => [
  { value: 'king', label: t('leaderboard.records.modeLabels.king') },
  { value: 'double_king', label: t('leaderboard.records.modeLabels.double_king') }
])

const scopeOptions = computed(() => [
  { value: 'highest', label: t('leaderboard.records.scopeLabels.highest') },
  { value: 'lowest', label: t('leaderboard.records.scopeLabels.lowest') }
])

const sortOptions = computed(() => [
  { value: 'score', label: t('leaderboard.records.detail.score') },
  { value: 'player', label: t('leaderboard.records.detail.player') },
  { value: 'date', label: t('leaderboard.records.detail.date') },
  { value: 'game', label: t('leaderboard.records.detail.game') }
])

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

const recordKey = computed<RecordTableKey>(() => `${currentMode.value}_${currentScope.value}` as RecordTableKey)

const rows = computed(() => {
  if (!leaderboard.value) return []
  return leaderboard.value.records[recordKey.value] ?? []
})

const totalRows = computed(() => rows.value.length)

const filteredRows = computed(() => {
  const search = searchTerm.value.trim().toLowerCase()
  if (!search) return rows.value
  return rows.value.filter((row) => row.player_name.toLowerCase().includes(search))
})

const compareRows = (a: any, b: any, key: SortKey) => {
  switch (key) {
    case 'player':
      return a.player_name.localeCompare(b.player_name, undefined, { sensitivity: 'base' })
    case 'score':
      return a.score - b.score
    case 'date':
      return new Date(a.occurred_at).getTime() - new Date(b.occurred_at).getTime()
    case 'game':
      return a.game_id.localeCompare(b.game_id)
    default:
      return 0
  }
}

const visibleRows = computed(() => {
  const rowsCopy = [...filteredRows.value]
  const factor = sortState.value.direction === 'asc' ? 1 : -1
  return rowsCopy.sort((a, b) => compareRows(a, b, sortState.value.key) * factor)
})

const detailTitle = computed(() =>
  t('leaderboard.records.detailTitle', {
    mode: t(`leaderboard.records.modeLabels.${currentMode.value}`),
    scope: t(`leaderboard.records.scopeLabels.${currentScope.value}`)
  })
)

const formatTimestamp = (value: string) => {
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleDateString()
}

const goBack = () => {
  router.push(`/${localeParam.value}/leaderboard`)
}

const openGame = (gameId: string) => {
  const gameType = currentMode.value === 'king' ? 'king' : 'double-king'
  router.push(`/${localeParam.value}/${gameType}/${gameId}`)
}

const onModeChange = (event: Event) => {
  const value = (event.target as HTMLSelectElement).value
  if (!isRecordMode(value)) return
  router.replace(`/${localeParam.value}/leaderboard/records/${value}/${currentScope.value}`)
}

const onScopeChange = (event: Event) => {
  const value = (event.target as HTMLSelectElement).value
  if (!isRecordScope(value)) return
  router.replace(`/${localeParam.value}/leaderboard/records/${currentMode.value}/${value}`)
}

const toggleDirection = () => {
  sortState.value = {
    ...sortState.value,
    direction: sortState.value.direction === 'asc' ? 'desc' : 'asc'
  }
}

const setSort = (key: SortKey) => {
  if (sortState.value.key === key) {
    sortState.value.direction = sortState.value.direction === 'asc' ? 'desc' : 'asc'
  } else {
    sortState.value = {
      key,
      direction: key === 'player' ? 'asc' : 'desc'
    }
  }
}
</script>

<style scoped>
.record-board-view {
  min-height: 100vh;
  padding: 72px clamp(16px, 5vw, 48px) 56px;
  background: radial-gradient(circle at top, rgba(255, 255, 255, 0.35), rgba(255, 255, 255, 0)),
    linear-gradient(180deg, rgba(201, 156, 93, 0.07), rgba(255, 255, 255, 0)),
    var(--texture-noise),
    var(--paper-cream);
  background-size: auto, auto, 320px 320px, auto;
  color: var(--ink);
}

.record-board-inner {
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
  padding-bottom: 16px;
  border-bottom: 1px dashed rgba(45, 31, 26, 0.18);
}

.summary-card,
.filters-card,
.table-card {
  background: transparent;
  border-radius: 16px;
  padding: 20px;
  border: 1px dashed rgba(45, 31, 26, 0.2);
  box-shadow: none;
}

.filters-card {
  background: rgba(255, 255, 255, 0.4);
  border-style: solid;
}

.summary-card {
  padding: 0;
  border: none;
}

.summary-card h1 {
  font-family: var(--font-heading);
  color: var(--ink);
}

.filters-grid {
  display: grid;
  gap: 16px;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
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

.state-card {
  width: 100%;
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
  align-items: center;
  font-size: 0.9rem;
  margin-bottom: 12px;
  color: var(--ink-muted);
}

.table-wrapper {
  overflow-x: auto;
}

.table-card table {
  width: 100%;
  border-collapse: collapse;
  min-width: 640px;
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

tbody tr:nth-child(even) {
  background: rgba(201, 156, 93, 0.06);
}

button.text {
  border: none;
  background: none;
  color: var(--accent-emerald);
  font-weight: 600;
  cursor: pointer;
}

button.text:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.empty-board {
  margin-top: 12px;
  font-style: italic;
  color: var(--ink-muted);
}

.clickable-row {
  cursor: pointer;
  transition: background 0.2s ease;
}

.clickable-row:hover {
  background: rgba(47, 92, 76, 0.1) !important;
}

.game-id-cell {
  font-family: monospace;
  font-size: 0.85rem;
  color: var(--ink-muted);
}

@media (max-width: 768px) {
  .page-header {
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
  }
}
</style>
