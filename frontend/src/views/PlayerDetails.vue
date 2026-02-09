<template>
  <div class="player-details-view">
    <div class="player-details-inner">
      <div class="page-header">
        <div class="page-title">
          <p class="eyebrow">{{ t('player.header.eyebrow') }}</p>
          <div class="player-name-row">
            <h1 v-if="!isEditingName">{{ displayName }}</h1>
            <div v-else class="name-edit-form">
              <input 
                type="text" 
                v-model="newPlayerName" 
                :placeholder="displayName"
                @keyup.enter="savePlayerName"
                @keyup.escape="cancelEditName"
                ref="nameInput"
              />
              <button class="primary" @click="savePlayerName" :disabled="renamingPlayer">
                {{ renamingPlayer ? '...' : t('player.admin.save') }}
              </button>
              <button class="secondary" @click="cancelEditName">{{ t('player.admin.cancel') }}</button>
            </div>
            <button 
              v-if="isAuthenticated && !isEditingName" 
              class="edit-name-btn" 
              @click="startEditName"
              :title="t('player.admin.rename')"
            >
              <svg viewBox="0 0 24 24" fill="currentColor" width="16" height="16">
                <path d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
              </svg>
            </button>
          </div>
          <p v-if="renameError" class="error-text">{{ renameError }}</p>
          <p v-if="renameSuccess" class="success-text">{{ renameSuccess }}</p>
          <p class="meta" v-if="profile">
            <span>{{ firstSeenLabel }}</span>
            <span v-if="lastSeenLabel">• {{ lastSeenLabel }}</span>
          </p>
        </div>
        <div class="header-actions">
          <button class="secondary" type="button" @click="goBack">{{ t('player.actions.back') }}</button>
          <button class="secondary" type="button" @click="openSearch">{{ t('player.actions.search') }}</button>
          <LanguageSelector />
        </div>
      </div>

      <div v-if="loading" class="card loading-card">{{ t('loading') }}</div>

      <div v-else-if="error" class="card error-card">
        <p>{{ error }}</p>
        <button class="primary" type="button" @click="loadProfile">{{ t('player.actions.refresh') }}</button>
      </div>

      <div v-else-if="profile" class="player-content">
        <section class="summary-grid">
          <div class="card summary-card" v-for="card in summaryCards" :key="card.label">
            <p class="summary-label">{{ card.label }}</p>
            <p class="summary-value">{{ card.value }}</p>
          </div>
        </section>

        <!-- Player Settings Section -->
        <section class="settings-section card">
          <div class="settings-header">
            <h3>{{ t('player.settings.title') }}</h3>
          </div>
          <div class="settings-form" v-if="!loadingSettings">
            <div class="settings-row">
              <label for="player-email">{{ t('player.settings.email') }}</label>
              <input 
                id="player-email"
                type="email" 
                v-model="playerEmail"
                :placeholder="t('player.settings.emailPlaceholder')"
                class="settings-input"
              />
            </div>
            <div class="settings-row checkbox-row">
              <label class="checkbox-label">
                <input 
                  type="checkbox" 
                  v-model="gameNotifications"
                  :disabled="!playerEmail.trim()"
                />
                <span>{{ t('player.settings.notifications') }}</span>
              </label>
              <p class="settings-hint">{{ t('player.settings.notificationsHint') }}</p>
            </div>
            <div class="settings-actions">
              <button 
                class="primary" 
                @click="savePlayerSettings"
                :disabled="savingSettings"
              >
                {{ savingSettings ? '...' : t('player.settings.save') }}
              </button>
            </div>
            <p v-if="settingsError" class="error-text">{{ settingsError }}</p>
            <p v-if="settingsSuccess" class="success-text">{{ settingsSuccess }}</p>
          </div>
          <div v-else class="settings-loading">{{ t('loading') }}</div>
        </section>

        <section class="charts-grid">
          <div class="card chart-card">
            <div class="chart-header">
              <div>
                <h2>{{ t('player.charts.eloTrend.title') }}</h2>
                <p>{{ t('player.charts.eloTrend.subtitle') }}</p>
              </div>
              <div class="chart-selectors">
                <div class="game-type-selector multi-select">
                  <button 
                    v-for="gt in availableGameTypes" 
                    :key="gt.value"
                    :class="{ active: selectedEloGameTypes.includes(gt.value) }"
                    :style="selectedEloGameTypes.includes(gt.value) ? { backgroundColor: getGameTypeColor(gt.value), borderColor: getGameTypeColor(gt.value) } : {}"
                    @click="toggleEloGameType(gt.value)"
                  >
                    {{ gt.label }}
                  </button>
                </div>
                <div class="time-range-selector">
                  <button 
                    v-for="range in timeRanges" 
                    :key="range.value"
                    :class="{ active: eloTimeRange === range.value }"
                    @click="setEloTimeRange(range.value)"
                  >
                    {{ range.label }}
                  </button>
                </div>
              </div>
            </div>
            <div v-if="loadingEloHistory" class="chart-loading">
              {{ t('loading') }}
            </div>
            <div v-else-if="eloTrendData" class="chart-wrapper with-legend">
              <div class="chart-canvas-container">
                <Line :data="eloTrendData" :options="eloTrendOptions" />
              </div>
              <div v-if="selectedEloGameTypes.length > 1" class="chart-legend">
                <div 
                  v-for="gameType in selectedEloGameTypes" 
                  :key="gameType" 
                  class="legend-item"
                >
                  <span class="legend-color" :style="{ backgroundColor: getGameTypeColor(gameType) }"></span>
                  <span class="legend-label">{{ gameType === 'overall' ? t('player.charts.eloTrend.overall') : gameTypeLabel(gameType) }}</span>
                </div>
              </div>
            </div>
            <p v-else class="empty-chart">{{ t('player.charts.eloTrend.noData') }}</p>
          </div>

          <div class="card chart-card">
            <div class="chart-header">
              <div>
                <h2>{{ t('player.charts.gameMix.title') }}</h2>
                <p>{{ t('player.charts.gameMix.subtitle') }}</p>
              </div>
            </div>
            <div v-if="gameMixData" class="chart-wrapper doughnut">
              <Doughnut :data="gameMixData" :options="doughnutOptions" />
            </div>
            <p v-else class="empty-chart">{{ t('player.summary.noGames') }}</p>
          </div>
        </section>

        <section class="card breakdown-card">
          <div class="section-header">
            <h2>{{ t('player.breakdown.title') }}</h2>
          </div>
          <p v-if="gameBreakdownCards.length === 0" class="empty-chart">{{ t('player.summary.noGames') }}</p>
          <div v-else class="breakdown-grid">
            <article 
              class="mode-card" 
              v-for="card in gameBreakdownCards" 
              :key="card.gameType"
              :class="{ 'mode-card--expanded': expandedBreakdowns[card.gameType] }"
            >
              <header class="mode-card-header" @click="toggleBreakdown(card.gameType)">
                <div class="mode-card-title">
                  <p class="eyebrow">{{ card.label }}</p>
                  <div class="games-pills">
                    <span class="games-pill">{{ card.finishedGames }} {{ t('player.breakdown.finished') }}</span>
                    <span v-if="card.unfinishedGames > 0" class="games-pill games-pill--unfinished">{{ card.unfinishedGames }} {{ t('player.breakdown.inProgress') }}</span>
                  </div>
                </div>
                <button class="expand-btn" :title="expandedBreakdowns[card.gameType] ? t('player.breakdown.collapse') : t('player.breakdown.expand')">
                  <span class="expand-icon">{{ expandedBreakdowns[card.gameType] ? '−' : '+' }}</span>
                </button>
              </header>
              <div class="mode-card-body">
                <div class="mode-stat">
                  <p class="stat-label">{{ t('player.breakdown.wins') }}</p>
                  <p class="stat-value">{{ card.wins }}</p>
                  <p class="stat-subvalue">{{ formatPercent(card.winRate) }} {{ t('player.breakdown.winRateNote') }}</p>
                </div>
                <div class="mode-stat">
                  <p class="stat-label">{{ t('player.breakdown.rounds') }}</p>
                  <p class="stat-value">{{ card.totalRounds }}</p>
                  <p v-if="card.gameType === 'hearts' && card.totalRounds > 0" class="stat-subvalue">
                    {{ formatAveragePoints(card.averagePointsPerRound) }} {{ t('player.breakdown.avgPerHand') }}
                  </p>
                </div>
                <div class="mode-stat">
                  <p class="stat-label">{{ t('player.breakdown.points') }}</p>
                  <p class="stat-value">{{ formatPoints(card.totalPoints) }}</p>
                </div>
                <div class="mode-stat elo-stat">
                  <p class="stat-label">{{ t('player.breakdown.elo') }}</p>
                  <p class="stat-value" :class="getEloClass(getEloForGameType(card.gameType))">
                    {{ getEloForGameType(card.gameType) }}
                  </p>
                  <p class="stat-subvalue" v-if="getEloGamesForGameType(card.gameType) > 0">
                    {{ getEloGamesForGameType(card.gameType) }} {{ t('player.breakdown.eloGames') }}
                  </p>
                </div>
              </div>
              <div v-if="card.hasPlacementInsights" class="mode-placement">
                <div>
                  <p class="stat-label">{{ t('player.breakdown.best') }}</p>
                  <p class="stat-value">
                    {{ formatPlacement(card.bestPlacement) }}
                    <span v-if="card.bestPlacementCount > 0" class="placement-count">(×{{ card.bestPlacementCount }})</span>
                  </p>
                </div>
                <div>
                  <p class="stat-label">{{ t('player.breakdown.worst') }}</p>
                  <p class="stat-value">
                    {{ formatPlacement(card.worstPlacement) }}
                    <span v-if="card.worstPlacementCount > 0" class="placement-count">(×{{ card.worstPlacementCount }})</span>
                  </p>
                </div>
                <div>
                  <p class="stat-label">{{ t('player.summary.averagePlacement') }}</p>
                  <p class="stat-value">{{ formatAveragePlacement(card.averagePlacement) }}</p>
                </div>
              </div>
              
              <!-- Expanded Details Section -->
              <div v-if="expandedBreakdowns[card.gameType]" class="mode-expanded-details">
                <!-- Hearts-specific stats -->
                <div v-if="card.gameType === 'hearts' && heartsStats" class="detailed-stats">
                  <h4 class="detail-heading">{{ t('player.breakdown.heartsDetails') }}</h4>
                  
                  <div class="special-stats">
                    <div class="special-stat">
                      <div>
                        <p class="stat-label">{{ t('player.breakdown.shootTheMoon') }}</p>
                        <p class="stat-value">{{ heartsStats.shootTheMoonCount }}</p>
                      </div>
                    </div>
                    <div class="special-stat">
                      <div>
                        <p class="stat-label">{{ t('player.breakdown.resetTo100') }}</p>
                        <p class="stat-value">{{ heartsStats.resetCount }}</p>
                      </div>
                    </div>
                    <div class="special-stat">
                      <div>
                        <p class="stat-label">{{ t('player.breakdown.queenOfSpades') }}</p>
                        <p class="stat-value">{{ formatPercent(heartsStats.queenOfSpadesRate) }}</p>
                      </div>
                    </div>
                  </div>

                  <h4 class="detail-heading">{{ t('player.breakdown.scoreDistribution') }}</h4>
                  <div class="score-distribution">
                    <div 
                      v-for="item in heartsStats.scoreDistribution" 
                      :key="item.score" 
                      class="score-bar-row"
                    >
                      <span class="score-label" :class="{ 'score-label--special': item.isSpecial }">
                        {{ item.displayLabel }}
                      </span>
                      <div class="score-bar-container">
                        <div 
                          class="score-bar" 
                          :class="{ 'score-bar--special': item.isSpecial }"
                          :style="{ width: item.percentage + '%' }"
                        ></div>
                      </div>
                      <span class="score-count">{{ item.count }} ({{ formatPercent(item.percentage / 100) }})</span>
                    </div>
                  </div>
                </div>

                <!-- King-specific stats -->
                <div v-else-if="card.gameType === 'king'" class="detailed-stats">
                  <h4 class="detail-heading">{{ t('player.breakdown.kingDetails') }}</h4>
                  <p class="stat-empty">{{ t('player.breakdown.comingSoon') }}</p>
                </div>

                <!-- Double King-specific stats -->
                <div v-else-if="card.gameType === 'double_king'" class="detailed-stats">
                  <h4 class="detail-heading">{{ t('player.breakdown.doubleKingDetails') }}</h4>
                  <p class="stat-empty">{{ t('player.breakdown.comingSoon') }}</p>
                </div>

                <!-- Color Whist-specific stats -->
                <div v-else-if="card.gameType === 'color_whist'" class="detailed-stats">
                  <h4 class="detail-heading">{{ t('player.breakdown.colorWhistDetails') }}</h4>
                  <p class="stat-empty">{{ t('player.breakdown.comingSoon') }}</p>
                </div>
              </div>
              
              <p v-if="!card.hasPlacementInsights && !expandedBreakdowns[card.gameType]" class="stat-empty">{{ t('player.breakdown.empty') }}</p>
            </article>
          </div>
        </section>

        <section class="recent-section">
          <div class="section-heading">
            <h2 class="scribble-underline">{{ t('player.recentGames.title') }}</h2>
          </div>
          
          <!-- Filter buttons -->
          <div class="games-filter">
            <button 
              v-for="filter in gameFilters" 
              :key="filter.value"
              :class="{ active: recentGamesFilter === filter.value }"
              @click="recentGamesFilter = filter.value"
            >
              {{ filter.label }}
            </button>
          </div>
          
          <p v-if="filteredRecentGames.length === 0" class="no-games">{{ t('player.recentGames.empty') }}</p>
          <template v-else>
            <div class="games-grid">
              <div 
                v-for="game in paginatedRecentGames" 
                :key="game.game_id" 
                class="game-card card"
                :class="{ 'game-card--win': game.did_win }"
              >
                <div class="game-card-header">
                  <div class="game-date">{{ formatTimestamp(game.occurred_at) }}</div>
                  <div class="header-badges">
                    <span 
                      v-if="game.elo_change !== undefined && game.elo_change !== null" 
                      class="elo-change-badge"
                      :class="{ 
                        'elo-positive': game.elo_change > 0, 
                        'elo-negative': game.elo_change < 0,
                        'elo-neutral': game.elo_change === 0
                      }"
                    >
                      {{ game.elo_change >= 0 ? '+' : '' }}{{ game.elo_change }}
                    </span>
                    <span class="placement-badge" :class="{ 'placement-badge--win': game.did_win }">
                      #{{ game.placement }}/{{ game.total_players }}
                    </span>
                  </div>
                </div>
                <div class="game-card-body">
                  <div class="game-type-label">
                    <p class="eyebrow">{{ gameTypeLabel(game.game_type) }}</p>
                  </div>
                  <div class="scores-list">
                    <div 
                      v-for="participant in sortedParticipants(game)" 
                      :key="participant.name" 
                      class="player-score-row"
                      :class="{ 'player-score-row--self': participant.is_target }"
                    >
                      <RouterLink
                        class="player-name-link"
                        :class="{ 'player-name-link--self': participant.is_target }"
                        :to="playerLink(participant.name)"
                      >
                        {{ participant.name }}
                      </RouterLink>
                      <span class="score-value">{{ participant.total_score }}</span>
                    </div>
                  </div>
                </div>
                <div class="game-card-actions">
                  <button class="primary" type="button" @click="openGame(game.game_type, game.game_id)">
                    {{ t('player.actions.openGame') }}
                  </button>
                  <button 
                    v-if="isAuthenticated" 
                    class="danger delete-btn" 
                    type="button" 
                    @click="confirmDeleteGame(game.game_id, game.game_type)"
                    :disabled="deletingGameId === game.game_id"
                    :title="t('player.admin.deleteGame')"
                  >
                    ×
                  </button>
                </div>
              </div>
            </div>
            
            <!-- Pagination -->
            <div v-if="totalRecentPages > 1" class="pagination">
              <button 
                class="pagination-btn" 
                :disabled="recentGamesPage === 1"
                @click="recentGamesPage--"
              >
                ←
              </button>
              <span class="pagination-info">
                {{ t('player.recentGames.page', { current: recentGamesPage, total: totalRecentPages }) }}
              </span>
              <button 
                class="pagination-btn" 
                :disabled="recentGamesPage === totalRecentPages"
                @click="recentGamesPage++"
              >
                →
              </button>
            </div>
          </template>
        </section>
      </div>
    </div>

    <!-- Delete Confirmation Modal -->
    <Teleport to="body">
      <div v-if="showDeleteModal" class="modal-overlay" @click.self="showDeleteModal = false">
        <div class="modal-content delete-modal">
          <h2>{{ t('player.admin.confirmDeleteTitle') }}</h2>
          <p>{{ t('player.admin.confirmDeleteMessage') }}</p>
          <div class="modal-actions">
            <button class="secondary" @click="showDeleteModal = false">
              {{ t('player.admin.cancel') }}
            </button>
            <button class="danger" @click="deleteGame" :disabled="deletingGameId !== null">
              {{ deletingGameId ? '...' : t('player.admin.delete') }}
            </button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch, nextTick } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import LanguageSelector from '@/components/LanguageSelector.vue'
import { gamesApi } from '@/api/games'
import { useAuth } from '@/stores/auth'
import type { Game, GameType, PlayerDetailResponse, PlayerGameSummary, PlayerEloRating, EloHistoryEntry } from '@/types'
import {
  Chart,
  LineElement,
  LineController,
  PointElement,
  CategoryScale,
  LinearScale,
  Tooltip,
  Filler,
  Legend,
  ArcElement
} from 'chart.js'
import type { ChartOptions } from 'chart.js'
import { Line, Doughnut } from 'vue-chartjs'

Chart.register(LineElement, LineController, PointElement, CategoryScale, LinearScale, Tooltip, Filler, Legend, ArcElement)

const GAME_ROUTE_BY_TYPE: Record<GameType, string> = {
  hearts: 'hearts-game',
  king: 'KingGame',
  double_king: 'DoubleKingGame',
  color_whist: 'ColorWhistGame',
  whist: 'WhistGame',
  manille: 'ManilleGame',
  press: 'PressGame'
}

const GAME_COLORS: Record<GameType, string> = {
  hearts: '#c41e3a',
  king: '#2f5c4c',
  double_king: '#8b6914',
  color_whist: '#1a1a1a',
  whist: '#34495e',
  manille: '#6b5b95',
  press: '#d35400'
}

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const { isAuthenticated } = useAuth()

const profile = ref<PlayerDetailResponse | null>(null)
const loading = ref(true)
const error = ref('')

// ELO ratings
const eloRatings = ref<PlayerEloRating[]>([])
const loadingElo = ref(false)

// ELO history for trend chart - now supports multiple game types
const eloHistoryByType = ref<Record<string, EloHistoryEntry[]>>({})
const loadingEloHistory = ref(false)
const eloTimeRange = ref<'month' | 'year' | 'all'>('month')
const selectedEloGameTypes = ref<Array<GameType | 'overall'>>(['overall'])

const timeRanges = [
  { value: 'month' as const, label: '1M' },
  { value: 'year' as const, label: '1Y' },
  { value: 'all' as const, label: 'All' }
]

// Recent games pagination and filter
const GAMES_PER_PAGE = 8 // 2 rows of 4
type GameFilterType = 'all' | 'finished' | 'unfinished' | 'won'
const recentGamesFilter = ref<GameFilterType>('all')
const recentGamesPage = ref(1)

// Admin: Rename player
const isEditingName = ref(false)
const newPlayerName = ref('')
const renamingPlayer = ref(false)
const renameError = ref('')
const renameSuccess = ref('')
const nameInput = ref<HTMLInputElement | null>(null)

// Player settings (email preferences)
const playerEmail = ref('')
const gameNotifications = ref(false)
const loadingSettings = ref(false)
const savingSettings = ref(false)
const settingsError = ref('')
const settingsSuccess = ref('')

// Admin: Delete game
const showDeleteModal = ref(false)
const gameToDelete = ref<{ id: string; type: GameType } | null>(null)
const deletingGameId = ref<string | null>(null)

// Expandable breakdown state
const expandedBreakdowns = ref<Record<GameType, boolean>>({
  hearts: false,
  king: false,
  double_king: false,
  color_whist: false,
  whist: false,
  manille: false,
  press: false
})

// Hearts detailed statistics
type HeartsScoreDistributionItem = {
  score: number
  displayLabel: string
  count: number
  percentage: number
  isSpecial: boolean
}

type HeartsDetailedStats = {
  shootTheMoonCount: number
  resetCount: number
  totalRounds: number
  queenOfSpadesRate: number
  scoreDistribution: HeartsScoreDistributionItem[]
}

const loadingHeartsStats = ref(false)

const localeParam = computed(() => (route.params.locale as string) || 'en')
const playerParam = computed(() => route.params.playerName as string)

type SummaryCard = {
  label: string
  value: string
}

type PlacementInsights = {
  best: number | null
  worst: number | null
  bestCount: number
  worstCount: number
  total: number
  count: number
}

type BreakdownCard = {
  gameType: GameType
  label: string
  games: number
  finishedGames: number
  unfinishedGames: number
  wins: number
  winRate: number
  totalRounds: number
  totalPoints: number
  averagePointsPerRound: number
  bestPlacement: number | null
  worstPlacement: number | null
  bestPlacementCount: number
  worstPlacementCount: number
  averagePlacement: number | null
  hasPlacementInsights: boolean
}

const doughnutOptions: ChartOptions<'doughnut'> = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: 'bottom'
    }
  }
}

const decodePlayerName = (value: string) => {
  try {
    return decodeURIComponent(value)
  } catch (err) {
    return value
  }
}

const displayName = computed(() => {
  if (profile.value) return profile.value.player_name
  return decodePlayerName(playerParam.value || '')
})

const formatTimestamp = (value: string | null) => {
  if (!value) return ''
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return value
  return date.toLocaleString()
}

const firstSeenLabel = computed(() => {
  if (!profile.value || !profile.value.first_played_at) {
    return t('player.header.neverPlayed')
  }
  return t('player.header.since', { date: formatTimestamp(profile.value.first_played_at) })
})

const lastSeenLabel = computed(() => {
  if (!profile.value || !profile.value.last_played_at) return ''
  return t('player.header.lastSeen', { date: formatTimestamp(profile.value.last_played_at) })
})

const loadEloHistory = async () => {
  if (!profile.value) return
  
  loadingEloHistory.value = true
  try {
    // Load history for all selected game types
    const newHistory: Record<string, EloHistoryEntry[]> = {}
    
    for (const gameType of selectedEloGameTypes.value) {
      const gameTypeParam = gameType === 'overall' ? undefined : gameType
      const response = await gamesApi.getPlayerEloHistory(
        profile.value.player_name,
        gameTypeParam,
        eloTimeRange.value
      )
      newHistory[gameType] = response.history
    }
    
    eloHistoryByType.value = newHistory
  } catch {
    eloHistoryByType.value = {}
  } finally {
    loadingEloHistory.value = false
  }
}

const setEloTimeRange = (range: 'month' | 'year' | 'all') => {
  eloTimeRange.value = range
  loadEloHistory()
}

const toggleEloGameType = (gameType: GameType | 'overall') => {
  const idx = selectedEloGameTypes.value.indexOf(gameType)
  if (idx === -1) {
    // Add the game type
    selectedEloGameTypes.value = [...selectedEloGameTypes.value, gameType]
  } else if (selectedEloGameTypes.value.length > 1) {
    // Remove it only if there's at least one left
    selectedEloGameTypes.value = selectedEloGameTypes.value.filter(t => t !== gameType)
  }
  loadEloHistory()
}

// Get color for a game type (for chart and legend)
const getGameTypeColor = (gameType: GameType | 'overall'): string => {
  if (gameType === 'overall') return '#2f5c4c'
  return GAME_COLORS[gameType] || '#2f5c4c'
}

// Player settings functions
const loadPlayerSettings = async () => {
  if (!profile.value) return
  
  loadingSettings.value = true
  settingsError.value = ''
  try {
    const settings = await gamesApi.getPlayerSettings(profile.value.player_name)
    playerEmail.value = settings.email || ''
    gameNotifications.value = settings.game_notifications
  } catch {
    // Settings don't exist yet, use defaults
    playerEmail.value = ''
    gameNotifications.value = false
  } finally {
    loadingSettings.value = false
  }
}

const savePlayerSettings = async () => {
  if (!profile.value) return
  
  savingSettings.value = true
  settingsError.value = ''
  settingsSuccess.value = ''
  
  try {
    await gamesApi.updatePlayerSettings(
      profile.value.player_name,
      playerEmail.value.trim() || null,
      gameNotifications.value
    )
    settingsSuccess.value = t('player.settings.saved')
    setTimeout(() => { settingsSuccess.value = '' }, 3000)
  } catch (err: any) {
    settingsError.value = err?.response?.data?.error || t('player.settings.error')
  } finally {
    savingSettings.value = false
  }
}

const loadProfile = async () => {
  try {
    loading.value = true
    error.value = ''
    const decoded = decodePlayerName(playerParam.value || '')
    profile.value = await gamesApi.getPlayerProfile(decoded)
    
    // Load ELO ratings
    loadingElo.value = true
    try {
      eloRatings.value = await gamesApi.getPlayerElo(decoded)
    } catch {
      eloRatings.value = []
    } finally {
      loadingElo.value = false
    }
    
    // Load ELO history for trend chart
    await loadEloHistory()
    
    // Load player settings
    await loadPlayerSettings()
  } catch (err: any) {
    profile.value = null
    error.value = err?.response?.data?.error || err?.message || t('player.errors.notFound')
  } finally {
    loading.value = false
  }
}

onMounted(loadProfile)

watch(
  () => route.params.playerName,
  () => {
    loadProfile()
  }
)

// Admin: Rename player functions
const startEditName = () => {
  isEditingName.value = true
  newPlayerName.value = displayName.value
  renameError.value = ''
  renameSuccess.value = ''
  nextTick(() => {
    nameInput.value?.focus()
    nameInput.value?.select()
  })
}

const cancelEditName = () => {
  isEditingName.value = false
  newPlayerName.value = ''
  renameError.value = ''
}

const savePlayerName = async () => {
  if (!newPlayerName.value.trim() || newPlayerName.value.trim() === displayName.value) {
    cancelEditName()
    return
  }

  renamingPlayer.value = true
  renameError.value = ''
  renameSuccess.value = ''

  try {
    const result = await gamesApi.renamePlayerGlobally(displayName.value, newPlayerName.value.trim())
    renameSuccess.value = t('player.admin.renameSuccess', { count: result.games_updated })
    isEditingName.value = false
    
    // Navigate to the new player profile
    router.replace({
      name: 'PlayerDetails',
      params: { locale: localeParam.value, playerName: newPlayerName.value.trim() }
    })
  } catch (err: any) {
    renameError.value = err?.response?.data?.error || err?.message || t('player.admin.renameFailed')
  } finally {
    renamingPlayer.value = false
  }
}

// Admin: Delete game functions
const confirmDeleteGame = (gameId: string, gameType: GameType) => {
  gameToDelete.value = { id: gameId, type: gameType }
  showDeleteModal.value = true
}

const deleteGame = async () => {
  if (!gameToDelete.value) return

  deletingGameId.value = gameToDelete.value.id

  try {
    await gamesApi.deleteGame(gameToDelete.value.id)
    showDeleteModal.value = false
    gameToDelete.value = null
    // Reload the profile to reflect the deleted game
    await loadProfile()
  } catch (err: any) {
    console.error('Failed to delete game:', err)
  } finally {
    deletingGameId.value = null
  }
}

// Expandable breakdown toggle
const toggleBreakdown = async (gameType: GameType) => {
  expandedBreakdowns.value[gameType] = !expandedBreakdowns.value[gameType]
  
  // Load player's games if expanding any breakdown and not yet loaded
  if (expandedBreakdowns.value[gameType] && allPlayerGames.value.length === 0) {
    await loadPlayerGames()
  }
}

// Cache for all player games (loaded once, used for all breakdowns)
const allPlayerGames = ref<Game[]>([])

// Load all games for this player using the dedicated endpoint (single query)
const loadPlayerGames = async () => {
  if (!profile.value) return
  
  loadingHeartsStats.value = true
  try {
    // Single efficient query - backend returns only this player's games
    allPlayerGames.value = await gamesApi.getPlayerGames(profile.value.player_name)
  } catch (err) {
    console.error('Failed to load player games:', err)
  } finally {
    loadingHeartsStats.value = false
  }
}

// Filter games by type (uses cached data)
const heartsGames = computed(() => 
  allPlayerGames.value.filter(g => g.game_type === 'hearts')
)

// Compute Hearts detailed statistics
const heartsStats = computed<HeartsDetailedStats | null>(() => {
  if (!profile.value || heartsGames.value.length === 0) return null
  
  const playerName = profile.value.player_name.toLowerCase().trim()
  let shootTheMoonCount = 0
  let resetCount = 0
  let queenOfSpadesCount = 0
  let totalRounds = 0
  
  // Score distribution: 0-26 for normal scores, plus "shot moon" category
  // Use -1 as key for "shot the moon" to keep it separate
  const SHOT_MOON_KEY = -1
  const scoreFrequency: Record<number, number> = {}
  
  // Initialize all possible scores (0-26) with 0 count
  for (let i = 0; i <= 26; i++) {
    scoreFrequency[i] = 0
  }
  scoreFrequency[SHOT_MOON_KEY] = 0 // Shot the moon category
  
  for (const game of heartsGames.value) {
    // Find this player in the game
    const playerIndex = game.players.findIndex(
      p => p.name.toLowerCase().trim() === playerName
    )
    if (playerIndex === -1) continue
    
    const player = game.players[playerIndex]
    const scores = player.scores
    
    // Calculate per-round scores (differences between cumulative scores)
    let previousTotal = 0
    for (let i = 0; i < scores.length; i++) {
      const currentTotal = scores[i]
      const roundScore = currentTotal - previousTotal
      totalRounds++
      
      // Check if someone shot the moon this round
      // When someone shoots the moon, they get 0 (or -26) and others get 26 (or 0)
      const allRoundScores = game.players.map(p => {
        const prevScore = i > 0 ? p.scores[i - 1] : 0
        return p.scores[i] - prevScore
      })
      
      // Someone shot the moon if one player has 0 and all others have 26
      // OR if one player has -26 and all others have 0
      const someoneShotMoon = allRoundScores.some((score, idx) => {
        const others = allRoundScores.filter((_, i) => i !== idx)
        return (score === 0 && others.every(s => s === 26)) || 
               (score === -26 && others.every(s => s === 0))
      })
      
      // Did THIS player shoot the moon?
      const playerShotMoon = (roundScore === 0 && allRoundScores.filter((_, idx) => idx !== playerIndex).every(s => s === 26)) ||
                              (roundScore === -26 && allRoundScores.filter((_, idx) => idx !== playerIndex).every(s => s === 0))
      
      if (playerShotMoon) {
        shootTheMoonCount++
        // Count under "shot the moon" category
        scoreFrequency[SHOT_MOON_KEY]++
      } else {
        // Normal round or someone else shot the moon
        // Count the actual score (usually 0-26, but could be -26 if shooter)
        const normalizedScore = Math.max(0, Math.min(26, roundScore))
        scoreFrequency[normalizedScore] = (scoreFrequency[normalizedScore] || 0) + 1
      }
      
      // Check for Queen of Spades (13+ points in a normal round)
      if (!someoneShotMoon && roundScore >= 13) {
        queenOfSpadesCount++
      }
      
      // Check for reset: score was >= 100 but now it's less (reset happened)
      // In Hearts, hitting exactly 100 resets you to 0
      if (previousTotal >= 100 && currentTotal < previousTotal) {
        resetCount++
      }
      
      previousTotal = currentTotal
    }
  }
  
  // Build score distribution - include ALL scores (even with 0 count)
  // Then sort by frequency (count) descending
  const distribution: HeartsScoreDistributionItem[] = []
  
  // Add "Shot the Moon" first (always include)
  distribution.push({
    score: SHOT_MOON_KEY,
    displayLabel: 'Shot the Moon',
    count: scoreFrequency[SHOT_MOON_KEY],
    percentage: totalRounds > 0 ? (scoreFrequency[SHOT_MOON_KEY] / totalRounds) * 100 : 0,
    isSpecial: true
  })
  
  // Add all scores 0-26 (always include, even with 0 count)
  for (let score = 0; score <= 26; score++) {
    distribution.push({
      score,
      displayLabel: score.toString(),
      count: scoreFrequency[score],
      percentage: totalRounds > 0 ? (scoreFrequency[score] / totalRounds) * 100 : 0,
      isSpecial: false
    })
  }
  
  // Sort by count descending (most frequent first), keeping Shot the Moon at top
  const shotTheMoon = distribution[0]
  const regularScores = distribution.slice(1)
  regularScores.sort((a, b) => b.count - a.count)
  
  const sortedDistribution = [shotTheMoon, ...regularScores]
  
  return {
    shootTheMoonCount,
    resetCount,
    totalRounds,
    queenOfSpadesRate: totalRounds > 0 ? queenOfSpadesCount / totalRounds : 0,
    scoreDistribution: sortedDistribution
  }
})

const gameTypeLabel = (type: GameType) => {
  const keyMap: Record<GameType, string> = {
    hearts: 'leaderboard.boards.hearts',
    king: 'leaderboard.boards.king',
    double_king: 'leaderboard.boards.doubleKing',
    color_whist: 'leaderboard.boards.colorWhist',
    whist: 'leaderboard.boards.whist',
    manille: 'leaderboard.boards.manille',
    press: 'leaderboard.boards.press'
  }
  return t(keyMap[type])
}

const formatPercent = (value: number) => `${(value * 100).toFixed(1)}%`
const formatPlacement = (value: number | null) => (value === null ? '—' : `#${value}`)
const formatAveragePlacement = (value: number | null) => (value === null ? '—' : value.toFixed(1))
const formatPoints = (value: number) => value.toLocaleString()
const formatAveragePoints = (value: number) => value.toFixed(1)

const favoriteMode = computed(() => {
  if (!profile.value) return null
  return profile.value.game_type_stats.reduce((best, stat) => {
    if (stat.games_played === 0) return best
    if (!best || stat.games_played > best.games_played) return stat
    return best
  }, null as PlayerDetailResponse['game_type_stats'][number] | null)
})

const bestMode = computed(() => {
  if (!profile.value) return null
  return profile.value.game_type_stats
    .filter((stat) => stat.games_played > 0)
    .reduce((best, stat) => {
      if (!best) return stat
      if (stat.win_rate === best.win_rate) {
        return stat.games_played > best.games_played ? stat : best
      }
      return stat.win_rate > best.win_rate ? stat : best
    }, null as PlayerDetailResponse['game_type_stats'][number] | null)
})

const averagePlacement = computed(() => {
  if (!profile.value || profile.value.score_timeline.length === 0) return null
  const total = profile.value.score_timeline.reduce((sum, point) => sum + point.placement, 0)
  return total / profile.value.score_timeline.length
})

const placementInsightsByMode = computed<Record<GameType, PlacementInsights>>(() => {
  const insights: Record<GameType, PlacementInsights> = {
    hearts: { best: null, worst: null, bestCount: 0, worstCount: 0, total: 0, count: 0 },
    king: { best: null, worst: null, bestCount: 0, worstCount: 0, total: 0, count: 0 },
    double_king: { best: null, worst: null, bestCount: 0, worstCount: 0, total: 0, count: 0 },
    color_whist: { best: null, worst: null, bestCount: 0, worstCount: 0, total: 0, count: 0 },
    whist: { best: null, worst: null, bestCount: 0, worstCount: 0, total: 0, count: 0 },
    manille: { best: null, worst: null, bestCount: 0, worstCount: 0, total: 0, count: 0 },
    press: { best: null, worst: null, bestCount: 0, worstCount: 0, total: 0, count: 0 }
  }

  if (!profile.value) return insights

  // First pass: find best and worst placements
  profile.value.score_timeline.forEach((point) => {
    const entry = insights[point.game_type]
    if (entry.best === null || point.placement < entry.best) {
      entry.best = point.placement
    }
    if (entry.worst === null || point.placement > entry.worst) {
      entry.worst = point.placement
    }
    entry.total += point.placement
    entry.count += 1
  })

  // Second pass: count occurrences of best and worst placements
  profile.value.score_timeline.forEach((point) => {
    const entry = insights[point.game_type]
    if (point.placement === entry.best) {
      entry.bestCount++
    }
    if (point.placement === entry.worst) {
      entry.worstCount++
    }
  })

  return insights
})

const gameBreakdownCards = computed<BreakdownCard[]>(() => {
  if (!profile.value) return []
  
  return profile.value.game_type_stats
    .filter((stat) => stat.games_played > 0)
    .map((stat) => {
      const placement = placementInsightsByMode.value[stat.game_type]
      const average = placement.count > 0 ? placement.total / placement.count : null
      const hasPlacementInsights = Boolean(
        placement.best !== null || placement.worst !== null || average !== null
      )
      
      // Calculate average points per round
      const averagePointsPerRound = stat.total_rounds > 0 
        ? stat.total_points / stat.total_rounds 
        : 0

      return {
        gameType: stat.game_type,
        label: gameTypeLabel(stat.game_type),
        games: stat.games_played,
        finishedGames: stat.finished_games,
        unfinishedGames: stat.unfinished_games,
        wins: stat.wins,
        winRate: stat.win_rate,
        totalRounds: stat.total_rounds,
        totalPoints: stat.total_points,
        averagePointsPerRound,
        bestPlacement: placement.best,
        worstPlacement: placement.worst,
        bestPlacementCount: placement.bestCount,
        worstPlacementCount: placement.worstCount,
        averagePlacement: average,
        hasPlacementInsights
      }
    })
})

// ELO helpers
const getEloForGameType = (gameType: GameType): number => {
  const rating = eloRatings.value.find(r => r.game_type === gameType)
  return rating?.elo ?? 1000
}

const getEloGamesForGameType = (gameType: GameType): number => {
  const rating = eloRatings.value.find(r => r.game_type === gameType)
  return rating?.games_played ?? 0
}

const getEloClass = (elo: number): string => {
  if (elo >= 1700) return 'elo-high'
  if (elo >= 1200) return 'elo-mid'
  return 'elo-low'
}

const overallElo = computed(() => {
  if (eloRatings.value.length === 0) return null
  const total = eloRatings.value.reduce((sum, r) => sum + r.elo, 0)
  const totalGames = eloRatings.value.reduce((sum, r) => sum + r.games_played, 0)
  return {
    elo: Math.round(total / eloRatings.value.length),
    games_played: totalGames
  }
})

const summaryCards = computed<SummaryCard[]>(() => {
  if (!profile.value) return []
  const bestModeLabel = bestMode.value ? gameTypeLabel(bestMode.value.game_type) : t('player.summary.noGames')
  const favoriteModeLabel = favoriteMode.value
    ? gameTypeLabel(favoriteMode.value.game_type)
    : t('player.summary.noGames')
  
  // ELO summary - show overall ELO
  const eloLabel = overallElo.value 
    ? `${overallElo.value.elo}`
    : '1000'
  
  // Layout: 4 rows of 2 tiles
  // Row 1: Finished Games | Win Rate (related: finished games and win rate)
  // Row 2: In Progress | Hands Played (games status)
  // Row 3: Rating | Average Placement (performance metrics)
  // Row 4: Best Mode | Favorite Mode (preferences)
  return [
    {
      label: t('player.summary.finishedGames'),
      value: profile.value.finished_games.toString()
    },
    {
      label: t('player.summary.winRate'),
      value: formatPercent(profile.value.win_rate)
    },
    {
      label: t('player.summary.inProgress'),
      value: profile.value.unfinished_games.toString()
    },
    {
      label: t('player.summary.handsPlayed'),
      value: profile.value.total_rounds.toLocaleString()
    },
    {
      label: t('player.summary.overallElo'),
      value: eloLabel
    },
    {
      label: t('player.summary.averagePlacement'),
      value: formatAveragePlacement(averagePlacement.value)
    },
    {
      label: t('player.summary.bestMode'),
      value: bestModeLabel
    },
    {
      label: t('player.summary.favoriteMode'),
      value: favoriteModeLabel
    }
  ]
})

// Format date key for grouping (YYYY-MM-DD for days, YYYY-WW for weeks)
const getDateKey = (date: Date, mode: 'day' | 'week'): string => {
  if (mode === 'day') {
    return date.toISOString().split('T')[0]
  } else {
    // Week number calculation
    const startOfYear = new Date(date.getFullYear(), 0, 1)
    const days = Math.floor((date.getTime() - startOfYear.getTime()) / (24 * 60 * 60 * 1000))
    const weekNum = Math.ceil((days + startOfYear.getDay() + 1) / 7)
    return `${date.getFullYear()}-W${weekNum.toString().padStart(2, '0')}`
  }
}

// Generate date labels for the chart based on time range
const generateDateLabels = (timeRange: 'month' | 'year' | 'all', startDate: Date | null): { labels: string[], mode: 'day' | 'week' } => {
  const now = new Date()
  const labels: string[] = []
  
  if (timeRange === 'month') {
    // 30 days, one point per day
    for (let i = 29; i >= 0; i--) {
      const date = new Date(now)
      date.setDate(date.getDate() - i)
      labels.push(getDateKey(date, 'day'))
    }
    return { labels, mode: 'day' }
  } else if (timeRange === 'year') {
    // 52 weeks, one point per week
    for (let i = 51; i >= 0; i--) {
      const date = new Date(now)
      date.setDate(date.getDate() - (i * 7))
      labels.push(getDateKey(date, 'week'))
    }
    return { labels, mode: 'week' }
  } else {
    // All time - use daily data, max one point per day
    if (!startDate) {
      return { labels: [], mode: 'day' }
    }
    const totalDays = Math.ceil((now.getTime() - startDate.getTime()) / (24 * 60 * 60 * 1000))
    // Limit to reasonable number of points by skipping days if needed
    const maxPoints = 365 // Max 1 year of daily points
    const dayStep = Math.max(1, Math.ceil(totalDays / maxPoints))
    const numPoints = Math.min(maxPoints, Math.ceil(totalDays / dayStep))
    
    for (let i = numPoints - 1; i >= 0; i--) {
      const date = new Date(now)
      date.setDate(date.getDate() - (i * dayStep))
      labels.push(getDateKey(date, 'day'))
    }
    return { labels, mode: 'day' }
  }
}

// Format label for display
const formatLabelForDisplay = (key: string, mode: 'day' | 'week'): string => {
  if (mode === 'day') {
    const date = new Date(key)
    return date.toLocaleDateString(undefined, { month: 'short', day: 'numeric' })
  } else {
    // For weeks, show the start of the week
    const [year, week] = key.split('-W')
    const jan1 = new Date(parseInt(year), 0, 1)
    const weekStart = new Date(jan1.getTime() + (parseInt(week) - 1) * 7 * 24 * 60 * 60 * 1000)
    return weekStart.toLocaleDateString(undefined, { month: 'short', day: 'numeric' })
  }
}

// Get available game types for the selector
const availableGameTypes = computed(() => {
  const types: Array<{ value: GameType | 'overall', label: string }> = [
    { value: 'overall', label: t('player.charts.eloTrend.overall') }
  ]
  
  // Add game types that the player has played
  if (profile.value) {
    for (const stat of profile.value.game_type_stats) {
      if (stat.games_played > 0) {
        types.push({
          value: stat.game_type,
          label: gameTypeLabel(stat.game_type)
        })
      }
    }
  }
  
  return types
})

// ELO trend chart data - supports multiple game types overlaid
const eloTrendData = computed(() => {
  // Check if we have any data
  const hasData = Object.values(eloHistoryByType.value).some(arr => arr.length > 0)
  if (!hasData) return null
  
  // Find earliest date across all selected game types
  let earliestDate: Date | null = null
  for (const history of Object.values(eloHistoryByType.value)) {
    for (const entry of history) {
      const date = new Date(entry.recorded_at)
      if (!earliestDate || date < earliestDate) {
        earliestDate = date
      }
    }
  }
  
  // Generate fixed date labels based on time range
  const { labels: dateKeys, mode } = generateDateLabels(eloTimeRange.value, earliestDate)
  if (dateKeys.length === 0) return null
  
  // Build datasets for each selected game type
  const datasets: Array<{
    label: string
    data: (number | null)[]
    borderColor: string
    backgroundColor: string
    fill: boolean
    tension: number
    pointRadius: number
    pointHoverRadius: number
    spanGaps: boolean
  }> = []
  
  // Track min/max for Y-axis scaling
  let globalMin = Infinity
  let globalMax = -Infinity
  
  for (const gameType of selectedEloGameTypes.value) {
    const history = eloHistoryByType.value[gameType] || []
    if (history.length === 0) continue
    
    // Sort by date
    const sorted = [...history].sort(
      (a, b) => new Date(a.recorded_at).getTime() - new Date(b.recorded_at).getTime()
    )
    
    // Group history entries by date key
    const historyByDate: Record<string, number[]> = {}
    for (const entry of sorted) {
      const date = new Date(entry.recorded_at)
      const key = getDateKey(date, mode)
      if (!historyByDate[key]) {
        historyByDate[key] = []
      }
      historyByDate[key].push(entry.elo)
    }
    
    // Get average value for a date
    const getValueForDate = (key: string): number | null => {
      const values = historyByDate[key]
      if (!values || values.length === 0) return null
      return Math.round(values.reduce((sum, v) => sum + v, 0) / values.length)
    }
    
    // Build data points with gap filling
    const dataPoints: (number | null)[] = []
    let lastValue: number | null = null
    
    for (const key of dateKeys) {
      const value = getValueForDate(key)
      if (value !== null) {
        lastValue = value
        dataPoints.push(value)
        globalMin = Math.min(globalMin, value)
        globalMax = Math.max(globalMax, value)
      } else if (lastValue !== null) {
        dataPoints.push(lastValue)
      } else {
        dataPoints.push(null)
      }
    }
    
    // Fill leading nulls with the first non-null value
    const firstNonNull = dataPoints.find(v => v !== null) ?? null
    if (firstNonNull !== null) {
      for (let i = 0; i < dataPoints.length; i++) {
        if (dataPoints[i] === null) {
          dataPoints[i] = firstNonNull
        } else {
          break
        }
      }
    }
    
    // Skip if all null
    if (dataPoints.every(v => v === null)) continue
    
    const color = getGameTypeColor(gameType)
    const label = gameType === 'overall' 
      ? t('player.charts.eloTrend.overall')
      : gameTypeLabel(gameType)
    
    datasets.push({
      label,
      data: dataPoints,
      borderColor: color,
      backgroundColor: `${color}1a`,
      fill: selectedEloGameTypes.value.length === 1, // Only fill if single dataset
      tension: 0.35,
      pointRadius: eloTimeRange.value === 'month' ? 3 : 1,
      pointHoverRadius: 6,
      spanGaps: true
    })
  }
  
  if (datasets.length === 0) return null
  
  // Format labels for display
  const displayLabels = dateKeys.map(key => formatLabelForDisplay(key, mode))
  
  return {
    labels: displayLabels,
    datasets,
    _yAxisRange: { min: globalMin, max: globalMax }
  }
})

// Get nice rounded Y-axis bounds
const getYAxisBounds = (min: number, max: number): { min: number, max: number } => {
  const range = max - min
  const padding = Math.max(range * 0.1, 20) // At least 20 points padding
  
  // Round to nice numbers (multiples of 25, 50, or 100 depending on range)
  let step = 25
  if (range > 200) step = 50
  if (range > 500) step = 100
  
  const niceMin = Math.floor((min - padding) / step) * step
  const niceMax = Math.ceil((max + padding) / step) * step
  
  return { min: niceMin, max: niceMax }
}

// ELO trend chart options - now computed for dynamic Y-axis
const eloTrendOptions = computed((): ChartOptions<'line'> => {
  const yAxisRange = eloTrendData.value?._yAxisRange || { min: 800, max: 1200 }
  const bounds = getYAxisBounds(yAxisRange.min, yAxisRange.max)
  
  return {
    responsive: true,
    maintainAspectRatio: false,
    interaction: {
      mode: 'index',
      intersect: false
    },
    plugins: {
      legend: { display: false }, // We'll use our own legend
      tooltip: { 
        mode: 'index', 
        intersect: false,
        callbacks: {
          label: (context) => `${context.dataset.label}: ${context.parsed.y}`
        }
      }
    },
    scales: {
      x: {
        ticks: { 
          color: '#666', 
          maxRotation: 45,
          maxTicksLimit: eloTimeRange.value === 'month' ? 10 : 12
        },
        grid: { display: false }
      },
      y: {
        ticks: { color: '#666' },
        grid: { color: 'rgba(0,0,0,0.08)' },
        min: bounds.min,
        max: bounds.max
      }
    }
  }
})

const gameMixData = computed(() => {
  if (!profile.value) return null
  const filtered = profile.value.game_type_stats.filter((stat) => stat.games_played > 0)
  if (filtered.length === 0) return null
  return {
    labels: filtered.map((stat) => gameTypeLabel(stat.game_type)),
    datasets: [
      {
        data: filtered.map((stat) => stat.games_played),
        backgroundColor: filtered.map((stat) => GAME_COLORS[stat.game_type]),
        hoverOffset: 8,
        borderWidth: 0
      }
    ]
  }
})

// Game filters for recent games
const gameFilters = computed(() => [
  { value: 'all' as GameFilterType, label: t('player.recentGames.filterAll') },
  { value: 'finished' as GameFilterType, label: t('player.recentGames.filterFinished') },
  { value: 'unfinished' as GameFilterType, label: t('player.recentGames.filterUnfinished') },
  { value: 'won' as GameFilterType, label: t('player.recentGames.filterWon') }
])

// Filtered recent games based on selected filter
const filteredRecentGames = computed(() => {
  if (!profile.value) return []
  
  const games = profile.value.recent_games
  
  switch (recentGamesFilter.value) {
    case 'finished':
      // For now, we check if it's a win or has placement info as proxy for finished
      // The backend should ideally send a game_over flag per game
      return games.filter(g => g.did_win || g.placement <= g.total_players)
    case 'unfinished':
      // This would need backend support - for now show none
      return []
    case 'won':
      return games.filter(g => g.did_win)
    default:
      return games
  }
})

// Reset page when filter changes
watch(recentGamesFilter, () => {
  recentGamesPage.value = 1
})

// Total pages for pagination
const totalRecentPages = computed(() => {
  return Math.max(1, Math.ceil(filteredRecentGames.value.length / GAMES_PER_PAGE))
})

// Paginated recent games
const paginatedRecentGames = computed(() => {
  const start = (recentGamesPage.value - 1) * GAMES_PER_PAGE
  const end = start + GAMES_PER_PAGE
  return filteredRecentGames.value.slice(start, end)
})

const goBack = () => {
  router.push(`/${localeParam.value}/leaderboard`)
}

const openSearch = () => {
  router.push(`/${localeParam.value}/players`)
}

const openGame = (gameType: GameType, gameId: string) => {
  const routeName = GAME_ROUTE_BY_TYPE[gameType]
  if (!routeName) return
  router.push({ name: routeName, params: { locale: localeParam.value, id: gameId } })
}

const playerLink = (playerName: string) => ({
  name: 'PlayerDetails',
  params: { locale: localeParam.value, playerName }
})

// Sort participants by score - Hearts: lowest first (best), others: highest first (best)
const sortedParticipants = (game: PlayerGameSummary) => {
  const sorted = [...game.players]
  if (game.game_type === 'hearts') {
    // Hearts: lower score is better
    sorted.sort((a, b) => a.total_score - b.total_score)
  } else {
    // King, Double King, Color Whist: higher score is better
    sorted.sort((a, b) => b.total_score - a.total_score)
  }
  return sorted
}
</script>

<style scoped>
.player-details-view {
  min-height: 100vh;
  padding: 60px 24px 120px;
  background: transparent;
  color: var(--ink);
}

.player-details-inner {
  width: min(1400px, 96vw);
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 24px;
  background: rgba(255, 255, 255, 0.85);
  border-radius: 36px;
  padding: clamp(24px, 4vw, 48px);
  border: 1px solid rgba(45, 31, 26, 0.12);
  box-shadow: 0 45px 90px rgba(33, 22, 12, 0.18);
  position: relative;
  overflow: hidden;
}

.player-details-inner::before,
.player-details-inner::after {
  content: '';
  position: absolute;
  inset: 18px;
  border: 1px dashed rgba(45, 31, 26, 0.14);
  border-radius: 28px;
  pointer-events: none;
}

.player-details-inner::after {
  inset: 32px;
  border-style: solid;
  opacity: 0.35;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 16px;
  padding-bottom: 24px;
  border-bottom: 1px dashed rgba(45, 31, 26, 0.16);
}

.page-title h1 {
  font-size: clamp(2rem, 4vw, 3rem);
  margin-bottom: 6px;
  font-family: var(--font-heading);
}

.page-title .meta {
  opacity: 0.85;
  font-size: 0.95rem;
  color: var(--ink-muted);
}

.eyebrow {
  text-transform: uppercase;
  letter-spacing: 0.1em;
  font-size: 0.75rem;
  opacity: 0.7;
  font-family: var(--font-script);
}

.header-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.loading-card,
.error-card {
  text-align: center;
  color: #222;
}

.error-card p {
  margin-bottom: 12px;
}

.player-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.summary-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}

@media (max-width: 640px) {
  .summary-grid {
    grid-template-columns: 1fr;
  }
}

.summary-card {
  background: linear-gradient(140deg, rgba(255, 255, 255, 0.95), rgba(249, 238, 217, 0.95));
  color: var(--ink);
  padding: 22px;
  border-radius: 24px;
  border: 1px solid rgba(45, 31, 26, 0.12);
  box-shadow: var(--card-shadow-soft);
}

.summary-label {
  font-size: 0.9rem;
  opacity: 0.75;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--ink-muted);
}

.summary-value {
  font-size: 2rem;
  font-weight: 700;
  font-family: var(--font-heading);
}

/* Settings Section */
.settings-section {
  background: linear-gradient(140deg, rgba(255, 255, 255, 0.95), rgba(249, 238, 217, 0.95));
  padding: 20px 24px;
  border-radius: 20px;
  margin-bottom: 16px;
}

.settings-header h3 {
  margin: 0 0 16px 0;
  font-family: var(--font-heading);
  font-size: 1.1rem;
}

.settings-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.settings-row {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.settings-row label {
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--ink-secondary);
}

.settings-input {
  padding: 10px 14px;
  border: 1px solid rgba(45, 31, 26, 0.2);
  border-radius: 10px;
  font-size: 1rem;
  background: rgba(255, 255, 255, 0.7);
  max-width: 320px;
}

.settings-input:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 2px rgba(201, 156, 93, 0.2);
}

.checkbox-row {
  margin-top: 4px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 10px;
  cursor: pointer;
  font-size: 0.95rem;
  color: var(--ink);
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--primary-color);
  cursor: pointer;
}

.checkbox-label input[type="checkbox"]:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.settings-hint {
  font-size: 0.8rem;
  color: var(--ink-muted);
  margin: 4px 0 0 28px;
}

.settings-actions {
  margin-top: 4px;
}

.settings-actions button {
  min-width: 100px;
}

.settings-loading {
  padding: 20px;
  text-align: center;
  color: var(--ink-secondary);
}

.section-header {
  margin-bottom: 12px;
}

.section-header h2 {
  margin: 0;
}

.charts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 20px;
}

.chart-card {
  background: linear-gradient(160deg, rgba(255, 255, 255, 0.95), rgba(245, 233, 208, 0.95));
  color: var(--ink);
  min-height: 320px;
  border: 1px solid rgba(45, 31, 26, 0.12);
  border-radius: 32px;
  box-shadow: var(--card-shadow-soft);
  padding: 24px;
}

.chart-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
  flex-wrap: wrap;
  margin-bottom: 8px;
}

.chart-header h2 {
  margin: 0;
  font-family: var(--font-heading);
}

.chart-header p {
  margin: 4px 0 0 0;
  font-size: 0.85rem;
  color: var(--ink-muted);
}

.chart-selectors {
  display: flex;
  flex-direction: column;
  gap: 8px;
  align-items: flex-end;
}

.time-range-selector,
.game-type-selector {
  display: flex;
  gap: 4px;
  background: rgba(45, 31, 26, 0.06);
  border-radius: 8px;
  padding: 4px;
}

.time-range-selector button,
.game-type-selector button {
  background: transparent;
  border: none;
  padding: 6px 12px;
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--ink-muted);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: none;
  white-space: nowrap;
}

.time-range-selector button:hover,
.game-type-selector button:hover {
  color: var(--ink);
  background: rgba(255, 255, 255, 0.5);
  transform: none;
}

.time-range-selector button.active,
.game-type-selector button.active {
  background: #fff;
  color: var(--primary-color) !important;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

/* Multi-select game type buttons use their game color when active */
.game-type-selector.multi-select button.active {
  color: #fff !important;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.chart-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 260px;
  color: var(--ink-muted);
}

.chart-wrapper {
  height: 260px;
  padding: 12px;
}

.chart-wrapper.with-legend {
  display: flex;
  flex-direction: column;
  height: auto;
  min-height: 260px;
}

.chart-canvas-container {
  height: 220px;
  position: relative;
}

.chart-legend {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  gap: 12px 20px;
  padding: 12px 0 0 0;
  margin-top: 8px;
  border-top: 1px solid rgba(45, 31, 26, 0.08);
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.legend-color {
  width: 12px;
  height: 12px;
  border-radius: 3px;
  flex-shrink: 0;
}

.legend-label {
  font-size: 0.8rem;
  color: var(--ink-muted);
  font-weight: 500;
}

.chart-wrapper.doughnut {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 260px;
}

.empty-chart {
  text-align: center;
  color: var(--ink-muted);
  margin-top: 32px;
  font-family: var(--font-script);
}

.breakdown-card {
  background: linear-gradient(160deg, rgba(255, 255, 255, 0.95), rgba(248, 235, 214, 0.95));
  color: var(--ink);
  padding: 32px;
  border: 1px solid rgba(45, 31, 26, 0.12);
  border-radius: 32px;
  box-shadow: var(--card-shadow-soft);
}

.breakdown-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
  gap: 16px;
}

.mode-card {
  border: 1px solid rgba(45, 31, 26, 0.12);
  border-radius: 24px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  background: rgba(255, 255, 255, 0.92);
  box-shadow: 0 12px 30px rgba(33, 22, 12, 0.12);
}

.mode-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}

.games-pills {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.games-pill {
  background: rgba(47, 92, 76, 0.12);
  color: var(--primary-color);
  padding: 4px 12px;
  border-radius: 999px;
  font-size: 0.8rem;
  font-weight: 600;
}

.games-pill--unfinished {
  background: rgba(201, 156, 93, 0.15);
  color: #8b6914;
}

.mode-card-body {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
}

.mode-stat {
  flex: 1 1 120px;
  border-right: 1px dashed rgba(45, 31, 26, 0.12);
  padding-right: 12px;
}

.mode-stat:last-child {
  border-right: none;
  padding-right: 0;
}

.stat-label {
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-size: 0.75rem;
  color: var(--ink-muted);
  margin-bottom: 4px;
}

.stat-value {
  font-size: 1.4rem;
  font-weight: 700;
  color: var(--ink);
  font-family: var(--font-heading);
}

.stat-subvalue {
  font-size: 0.9rem;
  color: var(--ink-secondary);
}

.mode-placement {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 12px;
}

.placement-count {
  font-size: 0.75em;
  color: var(--ink-muted);
  font-weight: 500;
  margin-left: 4px;
}

.stat-empty {
  margin: 0;
  font-size: 0.9rem;
  color: var(--ink-secondary);
  font-style: italic;
}

.recent-card {
  background: linear-gradient(160deg, rgba(255, 255, 255, 0.95), rgba(248, 236, 214, 0.95));
  color: var(--ink);
  padding: 32px;
  border: 1px solid rgba(45, 31, 26, 0.12);
  border-radius: 32px;
  box-shadow: var(--card-shadow-soft);
}

.recent-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.recent-item {
  border: 1px dashed rgba(45, 31, 26, 0.2);
  border-radius: 16px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  background: transparent;
}

.recent-item.win {
  border-color: rgba(47, 92, 76, 0.5);
  border-style: solid;
  background: rgba(47, 92, 76, 0.04);
}

.recent-item-header {
  display: flex;
  justify-content: space-between;
  gap: 12px;
}

.placement-text {
  margin: 6px 0;
}

.placement-rank {
  font-size: 1.6rem;
  font-weight: 700;
  font-family: var(--font-heading);
  color: var(--ink);
}

.placement-suffix {
  font-size: 1rem;
  color: var(--ink-muted);
  margin-left: 2px;
}

.participants {
  list-style: none;
  padding: 0;
  margin: 0;
  border: 1px dashed rgba(45, 31, 26, 0.16);
  border-radius: 18px;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.participants li {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  padding-bottom: 8px;
  border-bottom: 1px dashed rgba(45, 31, 26, 0.12);
}

.participants li:last-child {
  padding-bottom: 0;
  border-bottom: none;
}

.player-chip {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  border: 1px solid rgba(47, 92, 76, 0.35);
  border-radius: 999px;
  padding: 6px 14px;
  font-weight: 600;
  color: var(--primary-color);
  background: transparent;
  text-decoration: none;
  transition: background 0.2s ease, color 0.2s ease;
}

.player-chip:hover {
  background: rgba(47, 92, 76, 0.12);
}

.player-chip.self {
  background: var(--primary-color);
  color: #fff;
  border-color: var(--primary-color);
}

.player-chip .you-indicator {
  font-size: 0.75em;
  opacity: 0.8;
  margin-left: 2px;
}

.participants li.current-player {
  background: rgba(47, 92, 76, 0.08);
  margin: -4px -8px;
  padding: 12px 16px;
  border-radius: 12px;
  border-bottom-color: transparent;
}

.player-score {
  font-weight: 600;
  color: var(--ink);
}

.recent-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

/* Admin: Player name editing */
.player-name-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.player-name-row h1 {
  margin: 0;
}

.edit-name-btn {
  background: transparent;
  border: none;
  padding: 4px 8px;
  cursor: pointer;
  opacity: 0.5;
  transition: opacity 0.2s ease;
  box-shadow: none;
  font-size: 1rem;
}

.edit-name-btn:hover {
  opacity: 1;
  transform: none;
}

.name-edit-form {
  display: flex;
  align-items: center;
  gap: 8px;
}

.name-edit-form input {
  font-size: 1.5rem;
  font-family: var(--font-heading);
  padding: 8px 12px;
  border-radius: 8px;
  min-width: 200px;
}

.error-text {
  color: var(--danger-color);
  font-size: 0.9rem;
  margin-top: 4px;
}

.success-text {
  color: var(--success-color);
  font-size: 0.9rem;
  margin-top: 4px;
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  backdrop-filter: blur(2px);
}

.modal-content {
  background: var(--card-background);
  border-radius: 16px;
  padding: 24px;
  max-width: 90vw;
  box-shadow: var(--card-shadow);
  border: 1.5px solid var(--card-border);
}

.delete-modal {
  width: 400px;
}

.delete-modal h2 {
  margin: 0 0 12px 0;
  font-family: 'Playfair Display', serif;
  color: var(--danger-color);
}

.delete-modal p {
  margin: 0 0 20px 0;
  color: var(--ink-muted);
}

.modal-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

/* ============================================================================
   RECENT GAMES SECTION - Styled like setup page
   ============================================================================ */

.recent-section {
  margin-top: 24px;
}

.section-heading {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  flex-wrap: wrap;
  margin-bottom: 16px;
}

.no-games {
  text-align: center;
  color: var(--ink-secondary);
  padding: 28px;
  font-style: italic;
}

/* Games filter - styled like time-range-selector */
.games-filter {
  display: flex;
  gap: 4px;
  margin-bottom: 16px;
  flex-wrap: wrap;
  background: rgba(45, 31, 26, 0.06);
  border-radius: 8px;
  padding: 4px;
  width: fit-content;
}

.games-filter button {
  background: transparent;
  border: none;
  padding: 6px 14px;
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--ink-muted);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: none;
  white-space: nowrap;
}

.games-filter button:hover {
  color: var(--ink);
  background: rgba(255, 255, 255, 0.5);
  transform: none;
}

.games-filter button.active {
  background: #fff;
  color: var(--primary-color) !important;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.games-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 15px;
  margin-bottom: 20px;
}

@media (max-width: 1200px) {
  .games-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}

@media (max-width: 900px) {
  .games-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 600px) {
  .games-grid {
    grid-template-columns: 1fr;
  }
}

/* Pagination */
.pagination {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  margin-top: 20px;
}

.pagination-btn {
  padding: 8px 16px;
  border: 1px solid rgba(45, 31, 26, 0.2);
  border-radius: 8px;
  background: transparent;
  color: var(--ink);
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: none;
}

.pagination-btn:hover:not(:disabled) {
  background: rgba(47, 92, 76, 0.08);
  border-color: var(--primary-color);
  transform: none;
}

.pagination-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.pagination-info {
  font-size: 0.9rem;
  color: var(--ink-muted);
}

.game-card {
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(45, 31, 26, 0.12);
  border-radius: 24px;
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  transition: transform 0.25s ease, box-shadow 0.25s ease;
  box-shadow: 0 20px 40px rgba(33, 22, 12, 0.12);
}

.game-card:hover {
  transform: translateY(-6px) rotate(-0.2deg);
  box-shadow: 0 30px 60px rgba(33, 22, 12, 0.18);
}

.game-card--win {
  border-color: rgba(201, 156, 93, 0.5);
  border-width: 2px;
  background: rgba(201, 156, 93, 0.08);
}

.game-card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  border-bottom: 1px dashed rgba(45, 31, 26, 0.12);
  padding-bottom: 12px;
}

.header-badges {
  display: flex;
  gap: 8px;
  align-items: center;
}

.elo-change-badge {
  padding: 4px 10px;
  border-radius: 999px;
  font-size: 0.8rem;
  font-weight: 700;
  font-family: var(--font-heading);
  background: rgba(45, 31, 26, 0.08);
  color: var(--ink-secondary);
}

.elo-change-badge.elo-positive {
  background: rgba(76, 175, 80, 0.15);
  color: #2e7d32;
}

.elo-change-badge.elo-negative {
  background: rgba(244, 67, 54, 0.12);
  color: #c62828;
}

.game-date {
  font-size: 0.9rem;
  color: var(--ink-secondary);
  font-weight: 500;
}

.placement-badge {
  background: rgba(45, 31, 26, 0.1);
  color: var(--ink);
  padding: 4px 12px;
  border-radius: 999px;
  font-size: 0.85rem;
  font-weight: 700;
  font-family: var(--font-heading);
}

.placement-badge--win {
  background: var(--primary-color);
  color: #fff;
}

.game-card-body {
  display: flex;
  flex-direction: column;
  gap: 10px;
  flex: 1 1 auto;
}

.game-type-label {
  margin-bottom: 4px;
}

.scores-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.player-score-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 6px 10px;
  border-radius: 8px;
  transition: background 0.15s ease;
}

.player-score-row:hover {
  background: rgba(45, 31, 26, 0.04);
}

.player-score-row--self {
  background: rgba(47, 92, 76, 0.1);
  border-radius: 8px;
}

.player-name-link {
  text-decoration: none;
  color: var(--primary-color);
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 6px;
  transition: color 0.15s ease;
}

.player-name-link:hover {
  color: var(--accent-emerald);
}

.player-name-link--self {
  font-weight: 700;
}

.you-badge {
  font-size: 0.7em;
  background: var(--primary-color);
  color: #fff;
  padding: 2px 6px;
  border-radius: 999px;
}

.score-value {
  font-weight: 600;
  color: var(--ink);
  font-family: var(--font-heading);
}

.game-card-actions {
  margin-top: auto;
  padding-top: 8px;
  display: flex;
  gap: 10px;
  align-items: center;
}

.game-card-actions .primary {
  flex: 1;
  padding: 10px 16px;
  font-size: 0.95rem;
}

.game-card-actions .delete-btn {
  background: transparent;
  border: 1px solid var(--danger-color);
  color: var(--danger-color);
  font-size: 1.4rem;
  padding: 4px 10px;
  line-height: 1;
  cursor: pointer;
  transition: background 0.2s ease, color 0.2s ease;
}

.game-card-actions .delete-btn:hover {
  background: var(--danger-color);
  color: #fff;
}

/* ============================================================================
   EXPANDABLE BREAKDOWN CARDS
   ============================================================================ */

.mode-card-header {
  cursor: pointer;
  user-select: none;
}

.mode-card-title {
  display: flex;
  align-items: center;
  gap: 12px;
}

.expand-btn {
  background: transparent;
  border: 1px solid rgba(45, 31, 26, 0.2);
  border-radius: 50%;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
  padding: 0;
  box-shadow: none;
}

.expand-btn:hover {
  background: rgba(47, 92, 76, 0.1);
  border-color: var(--primary-color);
  transform: none;
}

.expand-icon {
  font-size: 1.2rem;
  font-weight: 700;
  color: var(--ink);
  line-height: 1;
}

.mode-card--expanded {
  border-color: var(--primary-color);
  border-width: 2px;
}

.mode-expanded-details {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px dashed rgba(45, 31, 26, 0.15);
}

.detailed-stats {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.detail-heading {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--ink);
  margin: 0 0 8px 0;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.special-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 12px;
}

.special-stat {
  display: flex;
  align-items: flex-start;
  gap: 10px;
  padding: 12px;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 12px;
  border: 1px dashed rgba(45, 31, 26, 0.12);
}

.special-stat .stat-label {
  margin-bottom: 2px;
}

.special-stat .stat-value {
  font-size: 1.2rem;
}

/* Score Distribution */
.score-distribution {
  display: flex;
  flex-direction: column;
  gap: 6px;
  max-height: 300px;
  overflow-y: auto;
}

.score-bar-row {
  display: grid;
  grid-template-columns: 70px 1fr 100px;
  align-items: center;
  gap: 10px;
}

.score-label {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--ink);
  text-align: right;
}

.score-label--special {
  color: var(--primary-color);
}

.score-bar-container {
  height: 20px;
  background: rgba(45, 31, 26, 0.08);
  border-radius: 10px;
  overflow: hidden;
}

.score-bar {
  height: 100%;
  background: var(--accent-emerald, #2f5c4c);
  border-radius: 10px;
  min-width: 4px;
  transition: width 0.3s ease;
}

.score-bar--special {
  background: linear-gradient(90deg, #c41e3a, #e85d75);
}

.score-count {
  font-size: 0.8rem;
  color: var(--ink-secondary);
  text-align: left;
}

/* ELO styling */
.elo-stat .stat-value {
  font-weight: 700;
}

.elo-high {
  color: #2f8f5d;
}

.elo-mid {
  color: #6b5b4f;
}

.elo-low {
  color: #c41e3a;
}

.elo-positive {
  color: #2f8f5d;
}

.elo-negative {
  color: #c41e3a;
}

.elo-neutral {
  color: var(--ink-muted);
}

/* Scribble underline for headings */
.scribble-underline {
  position: relative;
  display: inline-block;
}

.scribble-underline::after {
  content: '';
  position: absolute;
  left: 0;
  right: 0;
  bottom: -4px;
  height: 2px;
  background: linear-gradient(90deg, var(--primary-color) 0%, transparent 100%);
  border-radius: 2px;
}

@media (max-width: 768px) {
  .player-details-view {
    padding: 40px 12px 80px;
  }

  .player-details-inner {
    width: 100%;
    padding: 16px;
    border-radius: 24px;
  }

  .recent-item-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .header-actions {
    width: 100%;
    flex-direction: column;
    align-items: stretch;
  }

  .player-name-row {
    flex-direction: column;
    align-items: flex-start;
  }

  .name-edit-form {
    flex-direction: column;
    width: 100%;
  }

  .name-edit-form input {
    width: 100%;
    min-width: auto;
  }

  .recent-actions {
    flex-direction: column;
  }

  .delete-modal {
    width: calc(100vw - 32px);
  }

  .games-grid {
    grid-template-columns: 1fr;
    gap: 12px;
  }

  .game-card {
    padding: 16px;
  }

  .game-card:hover {
    transform: none;
  }

  .score-bar-row {
    grid-template-columns: 50px 1fr 80px;
    gap: 6px;
  }

  .score-label {
    font-size: 0.75rem;
  }

  .score-count {
    font-size: 0.7rem;
  }

  .special-stats {
    grid-template-columns: 1fr;
  }

  .breakdown-cards {
    grid-template-columns: 1fr;
  }

  .section-heading {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
}

@media (max-width: 480px) {
  .player-header h1 {
    font-size: 1.5rem;
  }

  .game-card {
    padding: 14px;
    border-radius: 16px;
  }

  .game-card-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }

  .header-badges {
    width: 100%;
    justify-content: flex-start;
  }

  .game-date {
    font-size: 0.8rem;
  }

  .player-score-row {
    font-size: 0.85rem;
  }

  .mode-card {
    padding: 14px;
  }

  .mode-card-title {
    flex-wrap: wrap;
    gap: 8px;
  }

  .stat-item {
    padding: 8px;
  }

  .stat-value {
    font-size: 1.1rem;
  }

  .chart-placeholder {
    height: 180px;
  }
}
</style>
