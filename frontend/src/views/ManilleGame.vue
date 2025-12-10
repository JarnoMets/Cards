<template>
  <div class="manille-game">
    <div v-if="loading" class="loading">{{ t('loading') }}</div>

    <div v-else-if="error" class="error-container">
      <h2>{{ t('error') }}</h2>
      <p>{{ error }}</p>
      <button class="primary" @click="goHome">{{ t('goHome') }}</button>
    </div>

    <div v-else-if="game" class="game-container">
      <div class="game-header">
        <button class="exit-button" @click="goHome" title="Exit game">←</button>
        <h1>{{ t('manilleGame.title') }} - {{ t('round') }} {{ game.current_round + 1 }}</h1>
        <LanguageSelector class="header-lang-selector" />
      </div>

      <!-- Team Scores Banner -->
      <div class="team-scores-banner card">
        <div class="team-score team-1">
          <div class="team-label">{{ t('manilleGame.team1') }}</div>
          <div class="team-players">{{ game.players[0].name }} & {{ game.players[2].name }}</div>
          <div class="team-total" :class="{ winning: team1Score > team2Score }">{{ team1Score }}</div>
        </div>
        <div class="vs">vs</div>
        <div class="team-score team-2">
          <div class="team-label">{{ t('manilleGame.team2') }}</div>
          <div class="team-players">{{ game.players[1].name }} & {{ game.players[3].name }}</div>
          <div class="team-total" :class="{ winning: team2Score > team1Score }">{{ team2Score }}</div>
        </div>
      </div>

      <!-- Score Preview -->
      <div v-if="team1Points !== null && previewScores.length > 0" class="score-preview card">
        <div class="preview-header">
          <span class="preview-icon">📊</span>
          <strong>{{ t('manilleGame.scorePreview') }}</strong>
        </div>
        <div class="preview-scores">
          <div class="preview-team team-1">
            <span>{{ t('manilleGame.team1') }}</span>
            <span class="preview-value" :class="{ positive: previewScores[0] > 0, negative: previewScores[0] < 0 }">
              {{ previewScores[0] >= 0 ? '+' : '' }}{{ previewScores[0] }}
            </span>
          </div>
          <div class="preview-team team-2">
            <span>{{ t('manilleGame.team2') }}</span>
            <span class="preview-value" :class="{ positive: previewScores[1] > 0, negative: previewScores[1] < 0 }">
              {{ previewScores[1] >= 0 ? '+' : '' }}{{ previewScores[1] }}
            </span>
          </div>
        </div>
        <div class="preview-multiplier" v-if="effectiveMultiplier > 1">
          ×{{ effectiveMultiplier }} multiplier active
        </div>
      </div>

      <!-- Score History Table -->
      <div class="scoreboard card">
        <table>
          <thead>
            <tr>
              <th class="round-col">{{ t('round') }}</th>
              <th class="trump-col">{{ t('manilleGame.trumpLabel') }}</th>
              <th class="team-col team-1">{{ t('manilleGame.team1') }}</th>
              <th class="team-col team-2">{{ t('manilleGame.team2') }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="r in game.current_round" :key="r" class="history-row">
              <td class="round-col">{{ t('roundShort') }}{{ r }}</td>
              <td class="trump-col">
                <span class="trump-badge" :class="getRoundTrump(r - 1)">
                  {{ formatTrump(getRoundTrump(r - 1)) }}
                </span>
              </td>
              <td class="score-cell team-1" :class="{ positive: game.players[0].scores[r - 1] > 0, negative: game.players[0].scores[r - 1] < 0 }">
                {{ formatScore(game.players[0].scores[r - 1]) }}
              </td>
              <td class="score-cell team-2" :class="{ positive: game.players[1].scores[r - 1] > 0, negative: game.players[1].scores[r - 1] < 0 }">
                {{ formatScore(game.players[1].scores[r - 1]) }}
              </td>
            </tr>
            <!-- Totals row -->
            <tr class="totals-row">
              <td class="round-col"><strong>{{ t('manilleGame.total') }}</strong></td>
              <td class="trump-col"></td>
              <td class="score-cell team-1 total-cell">
                <strong>{{ team1Score }}</strong>
              </td>
              <td class="score-cell team-2 total-cell">
                <strong>{{ team2Score }}</strong>
              </td>
            </tr>
          </tbody>
          <tfoot v-if="!gameOver">
            <tr class="input-row">
              <td class="round-col"><strong>{{ t('roundShort') }}{{ game.current_round + 1 }}</strong></td>
              <td class="trump-col">
                <select v-model="selectedTrump" class="trump-select">
                  <option value="hearts">{{ t('manilleGame.trumpSuits.hearts') }}</option>
                  <option value="diamonds">{{ t('manilleGame.trumpSuits.diamonds') }}</option>
                  <option value="clubs">{{ t('manilleGame.trumpSuits.clubs') }}</option>
                  <option value="spades">{{ t('manilleGame.trumpSuits.spades') }}</option>
                  <option value="no_trump">{{ t('manilleGame.trumpSuits.no_trump') }}</option>
                </select>
              </td>
              <td class="input-cell team-1" colspan="2">
                <div class="manille-input">
                  <label>{{ t('manilleGame.team1') }} {{ t('manilleGame.pointsPlaceholder') }}</label>
                  <input
                    v-model.number="team1Points"
                    type="number"
                    min="0"
                    max="60"
                    :placeholder="t('manilleGame.pointsPlaceholder')"
                  />
                  <div class="team2-auto">
                    {{ t('manilleGame.team2') }}: {{ team2Points !== null ? team2Points : '-' }}
                  </div>
                </div>
              </td>
            </tr>
            <tr class="multiplier-row">
              <td colspan="4">
                <div class="multiplier-selector">
                  <label>{{ t('manilleGame.multiplierLabel') }}:</label>
                  <div class="multiplier-buttons">
                    <button 
                      :class="{ active: multiplier === 1 }" 
                      @click="multiplier = 1"
                      :disabled="selectedTrump === 'no_trump' && multiplier > 1"
                    >
                      {{ t('manilleGame.multipliers.normal') }}
                    </button>
                    <button 
                      :class="{ active: multiplier === 2 }" 
                      @click="multiplier = 2"
                      :disabled="selectedTrump === 'no_trump'"
                    >
                      {{ t('manilleGame.multipliers.along') }}
                    </button>
                    <button 
                      :class="{ active: multiplier === 4 }" 
                      @click="multiplier = 4"
                      :disabled="selectedTrump === 'no_trump'"
                    >
                      {{ t('manilleGame.multipliers.against') }}
                    </button>
                  </div>
                </div>
              </td>
            </tr>
            <tr>
              <td colspan="4" class="table-actions-cell">
                <div class="table-actions">
                  <div v-if="scoreError" class="error-message">{{ scoreError }}</div>
                  <button class="primary" @click="submitRound" :disabled="!canSubmit">
                    {{ t('submitRound') }}
                  </button>
                </div>
              </td>
            </tr>
          </tfoot>
        </table>
      </div>

      <!-- Card Values Legend -->
      <div class="card-values-legend card">
        <h3>{{ t('manilleGame.cardValues.title') }}</h3>
        <div class="values-grid">
          <span>{{ t('manilleGame.cardValues.manille') }}</span>
          <span>{{ t('manilleGame.cardValues.ace') }}</span>
          <span>{{ t('manilleGame.cardValues.king') }}</span>
          <span>{{ t('manilleGame.cardValues.queen') }}</span>
          <span>{{ t('manilleGame.cardValues.jack') }}</span>
          <span>{{ t('manilleGame.cardValues.others') }}</span>
        </div>
        <div class="total-note">{{ t('manilleGame.cardValues.total') }}</div>
      </div>

      <WinnerModal
        :visible="showWinnerModal"
        :winners="winners || []"
        :elo-changes="eloChanges"
        @close="showWinnerModal = false"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { gamesApi } from '../api/games'
import type { Game, ManilleTrump, ManilleRoundRequest, GameEloChange } from '../types'
import { useI18n } from 'vue-i18n'
import LanguageSelector from '@/components/LanguageSelector.vue'
import WinnerModal from '@/components/WinnerModal.vue'

const router = useRouter()
const route = useRoute()
const { t } = useI18n()

const game = ref<Game | null>(null)
const loading = ref(true)
const error = ref('')
const scoreError = ref('')
const showWinnerModal = ref(false)
const eloChanges = ref<GameEloChange[]>([])

const gameId = route.params.id as string

// Round input state
const team1Points = ref<number | null>(null)
const selectedTrump = ref<ManilleTrump>('hearts')
const multiplier = ref<1 | 2 | 4>(1)

// Track trump per round for history display
const roundTrumps = ref<ManilleTrump[]>([])

// Computed
const team1Score = computed(() => game.value?.players[0]?.total_score ?? 0)
const team2Score = computed(() => game.value?.players[1]?.total_score ?? 0)

const team2Points = computed(() => {
  if (team1Points.value === null || team1Points.value < 0 || team1Points.value > 60) return null
  return 60 - team1Points.value
})

const effectiveMultiplier = computed(() => {
  const baseMultiplier = selectedTrump.value === 'no_trump' ? 2 : 1
  // Max multiplier is 4
  return Math.min(baseMultiplier * multiplier.value, 4)
})

const previewScores = computed(() => {
  if (team1Points.value === null || team2Points.value === null) return []
  
  const diff = team1Points.value - team2Points.value
  const score = diff * effectiveMultiplier.value
  
  return [score, -score]
})

const canSubmit = computed(() => {
  return team1Points.value !== null && 
         team1Points.value >= 0 && 
         team1Points.value <= 60 &&
         game.value !== null
})

const gameOver = computed(() => {
  if (!game.value) return false
  return team1Score.value >= 101 || team2Score.value >= 101
})

const winners = computed(() => {
  if (!game.value || !gameOver.value) return null
  
  // In Manille, highest team score wins
  if (team1Score.value > team2Score.value) {
    return [game.value.players[0], game.value.players[2]]
  } else {
    return [game.value.players[1], game.value.players[3]]
  }
})

watch(winners, async (val) => {
  if (val && val.length > 0) {
    showWinnerModal.value = true
    try {
      eloChanges.value = await gamesApi.getGameEloChanges(gameId)
    } catch (e) {
      console.error('Failed to fetch ELO changes:', e)
      eloChanges.value = []
    }
  }
})

// When trump changes to no_trump, reset multiplier if it's > 2
watch(selectedTrump, (val) => {
  if (val === 'no_trump' && multiplier.value > 1) {
    multiplier.value = 1
  }
})

const getRoundTrump = (roundIndex: number): ManilleTrump => {
  return roundTrumps.value[roundIndex] || 'hearts'
}

const formatTrump = (trump: ManilleTrump): string => {
  const symbols: Record<ManilleTrump, string> = {
    hearts: '♥',
    diamonds: '♦',
    clubs: '♣',
    spades: '♠',
    no_trump: 'NT',
  }
  return symbols[trump] || trump
}

const formatScore = (score: number | undefined): string => {
  if (score === undefined || score === null) return '-'
  return score >= 0 ? `+${score}` : `${score}`
}

const loadGame = async () => {
  try {
    loading.value = true
    error.value = ''
    game.value = await gamesApi.getGame(gameId)
    loading.value = false
  } catch (err: any) {
    error.value = err.response?.data?.error || err.message || 'Failed to load game'
    loading.value = false
  }
}

const submitRound = async () => {
  if (!game.value || !canSubmit.value) return
  
  scoreError.value = ''
  
  const manilleRound: ManilleRoundRequest = {
    team1_points: team1Points.value!,
    trump: selectedTrump.value,
    multiplier: multiplier.value,
  }
  
  try {
    const updated = await gamesApi.addRound(gameId, { manilleRound })
    game.value = updated
    
    // Track the trump for this round
    roundTrumps.value.push(selectedTrump.value)
    
    // Reset input for next round
    team1Points.value = null
    selectedTrump.value = 'hearts'
    multiplier.value = 1
  } catch (err: any) {
    console.error('Failed to submit round:', err)
    const errorData = err.response?.data
    if (errorData?.error) {
      scoreError.value = errorData.error
    } else {
      scoreError.value = t('manilleGame.errors.submitFailed')
    }
  }
}

const goHome = () => {
  const locale = (route.params.locale as string) || 'en'
  router.push(`/${locale}/manille`)
}

onMounted(() => {
  loadGame()
})
</script>

<style scoped>
@import '../styles/shared.css';
@import '../styles/game.css';

.manille-game {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.game-container {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.game-header {
  display: flex;
  align-items: center;
  gap: 16px;
}

.game-header h1 {
  flex: 1;
  margin: 0;
  font-size: 1.5rem;
}

.exit-button {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  padding: 8px 16px;
  font-size: 1.2rem;
  cursor: pointer;
}

/* Team Scores Banner */
.team-scores-banner {
  display: flex;
  align-items: center;
  justify-content: space-around;
  padding: 20px;
  gap: 20px;
}

.team-score {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.team-label {
  font-weight: bold;
  font-size: 1.1rem;
}

.team-players {
  font-size: 0.85rem;
  color: var(--color-text-muted);
}

.team-total {
  font-size: 2rem;
  font-weight: bold;
}

.team-total.winning {
  color: var(--color-success);
}

.team-1 .team-label,
.team-1 .team-total {
  color: var(--color-primary);
}

.team-2 .team-label,
.team-2 .team-total {
  color: var(--color-accent);
}

.vs {
  font-size: 1.2rem;
  font-weight: bold;
  color: var(--color-text-muted);
}

/* Score Preview */
.score-preview {
  padding: 16px;
}

.preview-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}

.preview-scores {
  display: flex;
  justify-content: space-around;
}

.preview-team {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.preview-value {
  font-size: 1.5rem;
  font-weight: bold;
}

.preview-value.positive {
  color: var(--color-success);
}

.preview-value.negative {
  color: var(--color-danger);
}

.preview-multiplier {
  text-align: center;
  margin-top: 12px;
  font-size: 0.9rem;
  color: var(--color-accent);
}

/* Scoreboard */
.scoreboard {
  overflow-x: auto;
}

.scoreboard table {
  width: 100%;
  border-collapse: collapse;
}

.scoreboard th,
.scoreboard td {
  padding: 12px;
  text-align: center;
  border-bottom: 1px solid var(--color-border);
}

.round-col {
  width: 80px;
}

.trump-col {
  width: 80px;
}

.team-col {
  width: 120px;
}

.trump-badge {
  display: inline-block;
  padding: 4px 8px;
  border-radius: 4px;
  font-weight: bold;
}

.trump-badge.hearts {
  color: #dc3545;
}

.trump-badge.diamonds {
  color: #dc3545;
}

.trump-badge.clubs {
  color: #333;
}

.trump-badge.spades {
  color: #333;
}

.trump-badge.no_trump {
  background: var(--color-accent);
  color: white;
}

.score-cell.positive {
  color: var(--color-success);
}

.score-cell.negative {
  color: var(--color-danger);
}

.totals-row {
  background: var(--color-surface);
}

.total-cell {
  font-size: 1.2rem;
}

/* Input */
.manille-input {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px;
}

.manille-input label {
  font-weight: 500;
}

.manille-input input {
  width: 120px;
  padding: 8px;
  text-align: center;
  font-size: 1.2rem;
  border: 1px solid var(--color-border);
  border-radius: 8px;
}

.team2-auto {
  font-size: 0.9rem;
  color: var(--color-text-muted);
}

.trump-select {
  padding: 8px;
  border-radius: 8px;
  border: 1px solid var(--color-border);
  font-size: 1rem;
}

/* Multiplier */
.multiplier-row td {
  padding: 16px;
}

.multiplier-selector {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.multiplier-buttons {
  display: flex;
  gap: 8px;
}

.multiplier-buttons button {
  padding: 8px 16px;
  border: 1px solid var(--color-border);
  border-radius: 8px;
  background: var(--color-surface);
  cursor: pointer;
  transition: all 0.2s;
}

.multiplier-buttons button.active {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.multiplier-buttons button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Actions */
.table-actions-cell {
  padding: 16px;
}

.table-actions {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  gap: 16px;
}

.error-message {
  color: var(--color-danger);
}

/* Card Values Legend */
.card-values-legend {
  padding: 16px;
}

.card-values-legend h3 {
  margin: 0 0 12px 0;
}

.values-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 8px;
  font-size: 0.9rem;
}

.total-note {
  margin-top: 12px;
  font-weight: bold;
  text-align: center;
}

/* Loading/Error */
.loading,
.error-container {
  text-align: center;
  padding: 40px;
}

.error-container button {
  margin-top: 16px;
}
</style>
