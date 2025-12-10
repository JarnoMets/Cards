<template>
  <div class="press-game">
    <div v-if="loading" class="loading">{{ t('loading') }}</div>

    <div v-else-if="error" class="error-container">
      <h2>{{ t('error') }}</h2>
      <p>{{ error }}</p>
      <button class="primary" @click="goHome">{{ t('goHome') }}</button>
    </div>

    <div v-else-if="game" class="game-container">
      <div class="game-header">
        <button class="exit-button" @click="goHome" title="Exit game">←</button>
        <h1>{{ t('pressGame.title') }} - {{ t('round') }} {{ game.current_round + 1 }}</h1>
        <LanguageSelector class="header-lang-selector" />
      </div>

      <!-- Team Scores Banner -->
      <div class="team-scores-banner card">
        <div class="team-score team-1">
          <div class="team-label">{{ t('pressGame.team1') }}</div>
          <div class="team-players">{{ game.players[0].name }} & {{ game.players[2].name }}</div>
          <div class="team-total" :class="{ winning: team1Score > team2Score }">{{ team1Score }}</div>
        </div>
        <div class="vs">vs</div>
        <div class="team-score team-2">
          <div class="team-label">{{ t('pressGame.team2') }}</div>
          <div class="team-players">{{ game.players[1].name }} & {{ game.players[3].name }}</div>
          <div class="team-total" :class="{ winning: team2Score > team1Score }">{{ team2Score }}</div>
        </div>
      </div>

      <!-- Score Preview -->
      <div v-if="bid !== null && tricksWon !== null && previewScores.length > 0" class="score-preview card">
        <div class="preview-header">
          <span class="preview-icon">📊</span>
          <strong>{{ t('pressGame.scorePreview') }}</strong>
        </div>
        <div class="preview-scores">
          <div class="preview-team team-1">
            <span>{{ t('pressGame.team1') }}</span>
            <span class="preview-value" :class="{ positive: previewScores[0] > 0, negative: previewScores[0] < 0 }">
              {{ previewScores[0] >= 0 ? '+' : '' }}{{ previewScores[0] }}
            </span>
          </div>
          <div class="preview-team team-2">
            <span>{{ t('pressGame.team2') }}</span>
            <span class="preview-value" :class="{ positive: previewScores[1] > 0, negative: previewScores[1] < 0 }">
              {{ previewScores[1] >= 0 ? '+' : '' }}{{ previewScores[1] }}
            </span>
          </div>
        </div>
        <div class="preview-result" :class="{ success: bidMade, failed: !bidMade }">
          {{ bidMade ? t('pressGame.success') : t('pressGame.failed') }}
        </div>
      </div>

      <!-- Score History Table -->
      <div class="scoreboard card">
        <table>
          <thead>
            <tr>
              <th class="round-col">{{ t('round') }}</th>
              <th class="trump-col">{{ t('pressGame.trumpLabel') }}</th>
              <th class="bid-col">{{ t('pressGame.bid') }}</th>
              <th class="team-col team-1">{{ t('pressGame.team1') }}</th>
              <th class="team-col team-2">{{ t('pressGame.team2') }}</th>
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
              <td class="bid-col">
                <span class="bid-info">
                  {{ getRoundBid(r - 1) }} (T{{ getRoundBiddingTeam(r - 1) }})
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
              <td class="round-col"><strong>{{ t('pressGame.total') }}</strong></td>
              <td class="trump-col"></td>
              <td class="bid-col"></td>
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
                  <option value="hearts">{{ t('pressGame.trumpSuits.hearts') }}</option>
                  <option value="diamonds">{{ t('pressGame.trumpSuits.diamonds') }}</option>
                  <option value="clubs">{{ t('pressGame.trumpSuits.clubs') }}</option>
                  <option value="spades">{{ t('pressGame.trumpSuits.spades') }}</option>
                </select>
              </td>
              <td class="bid-col" colspan="3">
                <div class="press-input">
                  <!-- Bidding Team Selection -->
                  <div class="input-group">
                    <label>{{ t('pressGame.biddingTeamLabel') }}</label>
                    <div class="team-buttons">
                      <button 
                        :class="{ active: biddingTeam === 1 }" 
                        @click="biddingTeam = 1"
                        class="team-btn team-1-btn"
                      >
                        {{ t('pressGame.team1') }}
                      </button>
                      <button 
                        :class="{ active: biddingTeam === 2 }" 
                        @click="biddingTeam = 2"
                        class="team-btn team-2-btn"
                      >
                        {{ t('pressGame.team2') }}
                      </button>
                    </div>
                  </div>

                  <!-- Bid Input -->
                  <div class="input-group">
                    <label>{{ t('pressGame.bidLabel') }}</label>
                    <input
                      v-model.number="bid"
                      type="number"
                      min="1"
                      max="8"
                      :placeholder="t('pressGame.bidPlaceholder')"
                    />
                  </div>

                  <!-- Tricks Won Input -->
                  <div class="input-group">
                    <label>{{ t('pressGame.tricksWonLabel') }}</label>
                    <input
                      v-model.number="tricksWon"
                      type="number"
                      min="0"
                      max="8"
                      :placeholder="t('pressGame.tricksWonPlaceholder')"
                    />
                  </div>
                </div>
              </td>
            </tr>
            <tr>
              <td colspan="5" class="table-actions-cell">
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

      <!-- Card Ranking Legend -->
      <div class="card-values-legend card">
        <h3>{{ t('pressGame.cardRanking.title') }}</h3>
        <div class="values-grid">
          <span>{{ t('pressGame.cardRanking.rightBower') }}</span>
          <span>{{ t('pressGame.cardRanking.leftBower') }}</span>
          <span>{{ t('pressGame.cardRanking.restOrder') }}</span>
        </div>
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
import type { Game, PressTrump, PressRoundRequest, GameEloChange } from '../types'
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
const bid = ref<number | null>(null)
const tricksWon = ref<number | null>(null)
const biddingTeam = ref<1 | 2>(1)
const selectedTrump = ref<PressTrump>('hearts')

// Track round history for display
interface RoundHistory {
  trump: PressTrump
  bid: number
  biddingTeam: 1 | 2
  tricksWon: number
}
const roundHistory = ref<RoundHistory[]>([])

// Computed
const team1Score = computed(() => game.value?.players[0]?.total_score ?? 0)
const team2Score = computed(() => game.value?.players[1]?.total_score ?? 0)

const bidMade = computed(() => {
  if (bid.value === null || tricksWon.value === null) return false
  return tricksWon.value >= bid.value
})

const previewScores = computed(() => {
  if (bid.value === null || tricksWon.value === null || biddingTeam.value === null) return []
  
  const bidValue = bid.value
  const score = bidMade.value ? bidValue : -bidValue
  
  if (biddingTeam.value === 1) {
    return [score, 0]
  } else {
    return [0, score]
  }
})

const canSubmit = computed(() => {
  return bid.value !== null && 
         bid.value >= 1 && 
         bid.value <= 8 &&
         tricksWon.value !== null &&
         tricksWon.value >= 0 &&
         tricksWon.value <= 8 &&
         biddingTeam.value !== null &&
         game.value !== null
})

const gameOver = computed(() => {
  if (!game.value) return false
  return team1Score.value >= 42 || team2Score.value >= 42
})

const winners = computed(() => {
  if (!game.value || !gameOver.value) return null
  
  // Highest team score wins
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

const getRoundTrump = (roundIndex: number): PressTrump => {
  return roundHistory.value[roundIndex]?.trump || 'hearts'
}

const getRoundBid = (roundIndex: number): number => {
  return roundHistory.value[roundIndex]?.bid || 0
}

const getRoundBiddingTeam = (roundIndex: number): number => {
  return roundHistory.value[roundIndex]?.biddingTeam || 1
}

const formatTrump = (trump: PressTrump): string => {
  const symbols: Record<PressTrump, string> = {
    hearts: '♥',
    diamonds: '♦',
    clubs: '♣',
    spades: '♠',
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
  
  const pressRound: PressRoundRequest = {
    bid: bid.value!,
    bidding_team: biddingTeam.value,
    tricks_won: tricksWon.value!,
    trump: selectedTrump.value,
  }
  
  try {
    const updated = await gamesApi.addRound(gameId, { pressRound })
    game.value = updated
    
    // Track the round history
    roundHistory.value.push({
      trump: selectedTrump.value,
      bid: bid.value!,
      biddingTeam: biddingTeam.value,
      tricksWon: tricksWon.value!,
    })
    
    // Reset input for next round
    bid.value = null
    tricksWon.value = null
    biddingTeam.value = 1
    selectedTrump.value = 'hearts'
  } catch (err: any) {
    console.error('Failed to submit round:', err)
    const errorData = err.response?.data
    if (errorData?.error) {
      scoreError.value = errorData.error
    } else {
      scoreError.value = t('pressGame.errors.submitFailed')
    }
  }
}

const goHome = () => {
  const locale = (route.params.locale as string) || 'en'
  router.push(`/${locale}/press`)
}

onMounted(() => {
  loadGame()
})
</script>

<style scoped>
@import '../styles/shared.css';
@import '../styles/game.css';

.press-game {
  max-width: 900px;
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

.preview-result {
  text-align: center;
  margin-top: 12px;
  font-size: 1rem;
  font-weight: bold;
  padding: 8px;
  border-radius: 8px;
}

.preview-result.success {
  background: rgba(var(--color-success-rgb, 76, 175, 80), 0.15);
  color: var(--color-success);
}

.preview-result.failed {
  background: rgba(var(--color-danger-rgb, 244, 67, 54), 0.15);
  color: var(--color-danger);
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
  width: 70px;
}

.trump-col {
  width: 70px;
}

.bid-col {
  width: 100px;
}

.team-col {
  width: 100px;
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

.bid-info {
  font-size: 0.9rem;
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
.press-input {
  display: flex;
  flex-wrap: wrap;
  gap: 16px;
  justify-content: center;
  align-items: flex-end;
  padding: 12px;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.input-group label {
  font-weight: 500;
  font-size: 0.85rem;
}

.input-group input {
  width: 100px;
  padding: 8px;
  text-align: center;
  font-size: 1.1rem;
  border: 1px solid var(--color-border);
  border-radius: 8px;
}

.team-buttons {
  display: flex;
  gap: 8px;
}

.team-btn {
  padding: 8px 16px;
  border: 2px solid var(--color-border);
  border-radius: 8px;
  background: var(--color-surface);
  cursor: pointer;
  transition: all 0.2s;
  font-weight: 500;
}

.team-1-btn {
  border-color: var(--color-primary);
  color: var(--color-primary);
}

.team-1-btn.active {
  background: var(--color-primary);
  color: white;
}

.team-2-btn {
  border-color: var(--color-accent);
  color: var(--color-accent);
}

.team-2-btn.active {
  background: var(--color-accent);
  color: white;
}

.trump-select {
  padding: 8px;
  border-radius: 8px;
  border: 1px solid var(--color-border);
  font-size: 1rem;
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
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 8px;
  font-size: 0.9rem;
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

@media (max-width: 600px) {
  .press-input {
    flex-direction: column;
    align-items: stretch;
  }
  
  .input-group {
    width: 100%;
  }
  
  .input-group input {
    width: 100%;
  }
  
  .team-buttons {
    justify-content: center;
  }
}
</style>
