<template>
  <div class="page-container game-page hearts-game-page">
    <div class="content-container game-sheet">
      <div v-if="loading" class="loading">{{ t('loading') }}</div>

      <div v-else-if="error" class="error-container card">
        <h2>{{ t('error') }}</h2>
        <p>{{ error }}</p>
        <button class="primary" @click="goHome">{{ t('goHome') }}</button>
      </div>

      <div v-else-if="game" class="game-shell">
        <div class="game-header">
          <button class="exit-button" @click="goHome" title="Exit game">←</button>
          <h1>{{ t('heartsGame.title') }}</h1>
          <LanguageSelector class="header-lang-selector" />
        </div>

        <div v-if="passingDetails" class="passing-banner" :class="passingDetails.mode">
          <span class="passing-text">{{ passingDetails.headline }}</span>
          <span v-if="passingDetails.direction" class="passing-direction">{{ passingDetails.direction }}</span>
        </div>

        <div class="scoreboard card" ref="scoreTableRef">
          <table>
            <thead>
              <tr>
                <th>{{ t('round') }}</th>
                <th v-for="player in game.players" :key="player.id">
                  <div
                    class="player-header"
                    @mousedown="onPlayerHeaderPointerDown(player.id)"
                    @touchstart.prevent="onPlayerHeaderPointerDown(player.id)"
                    @mouseup="onPlayerHeaderPointerCancel"
                    @mouseleave="onPlayerHeaderPointerCancel"
                    @touchend="onPlayerHeaderPointerCancel"
                    @touchcancel="onPlayerHeaderPointerCancel"
                  >
                    <span>{{ truncateName(player.name) }}</span>
                  </div>
                </th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="r in game.current_round"
                :key="r"
                @mousedown="onRoundRowPointerDown(r - 1)"
                @touchstart.prevent="onRoundRowPointerDown(r - 1)"
                @mouseup="onRoundRowPointerCancel"
                @mouseleave="onRoundRowPointerCancel"
                @touchend="onRoundRowPointerCancel"
                @touchcancel="onRoundRowPointerCancel"
              >
                <td>{{ t('roundShort') }}{{ r }}</td>
                <td
                  v-for="(player, index) in game.players"
                  :key="player.id"
                  :class="{
                    lowest: isLowestInRound(index, r - 1),
                    'danger-score': (getTotalAfterRound(index, r - 1) != null && getTotalAfterRound(index, r - 1)! >= 75)
                  }"
                >
                  <template v-if="getTotalAfterRound(index, r - 1) !== null">
                    {{ getTotalAfterRound(index, r - 1) }}
                    (<span class="delta">{{ getRoundDelta(index, r - 1) !== null && getRoundDelta(index, r - 1)! >= 0 ? '+' : '' }}{{ getRoundDelta(index, r - 1) }}</span>)
                  </template>
                  <template v-else>-</template>
                </td>
              </tr>
            </tbody>
            <tfoot>
              <tr v-if="!(winners && winners.length)">
                <td><strong>{{ t('roundShort') }}{{ game.current_round + 1 }}</strong></td>
                <td v-for="(player, index) in game.players" :key="player.id">
                  <input
                    class="cell-input"
                    v-model.number="roundScores[index]"
                    type="number"
                    :min="0"
                    :max="26"
                    :placeholder="t('points')"
                  />
                </td>
              </tr>
              <tr v-if="!(winners && winners.length)">
                <td :colspan="game.players.length + 1" class="table-actions-cell">
                  <div class="table-actions">
                    <div v-if="scoreError" class="error-message" style="margin:0 12px 0 0">{{ scoreError }}</div>
                    <button class="primary" @click="submitRound" :disabled="!canSubmitRound || enteredTotal === 0 || submittingRound" style="margin-left:auto">
                      {{ submittingRound ? '...' : t('submitRound') }}
                    </button>
                  </div>
                </td>
              </tr>
              <tr v-else-if="winners && winners.length">
                <td><strong>{{ t('finalTally') }}</strong></td>
                <td v-for="player in game.players" :key="player.id">
                  <strong>{{ player.total_score }}</strong>
                </td>
              </tr>
            </tfoot>
          </table>
        </div>

        <div v-if="editingRoundIndex !== null && game" class="modal-overlay" @click="cancelRoundEdit">
          <div class="modal round-edit-modal" @click.stop>
            <h2>{{ t('roundEdit.title') }}</h2>
            <p class="modal-subtitle">{{ t('roundEdit.roundLabel', { round: editingRoundIndex + 1 }) }}</p>
            <p class="modal-hint">{{ t('roundEdit.hint') }}</p>
            <div class="round-edit-grid">
              <div class="round-edit-field" v-for="(player, idx) in game.players" :key="player.id">
                <label :for="`hearts-edit-${player.id}`">{{ player.name }}</label>
                <input
                  :id="`hearts-edit-${player.id}`"
                  type="number"
                  min="0"
                  max="26"
                  v-model.number="editingRoundScores[idx]"
                />
              </div>
            </div>
            <div v-if="editingRoundError" class="error-message">{{ editingRoundError }}</div>
            <div class="modal-actions">
              <button class="secondary" type="button" @click="cancelRoundEdit">{{ t('roundEdit.cancel') }}</button>
              <button class="primary" type="button" @click="saveRoundEdit" :disabled="savingRound">{{ t('roundEdit.save') }}</button>
            </div>
          </div>
        </div>

        <div v-if="editingPlayerId" class="modal-overlay" @click="cancelPlayerEdit">
          <div class="modal player-edit-modal" @click.stop>
            <h2>{{ t('playerEdit.title') }}</h2>
            <input
              type="text"
              v-model="playerNameDraft"
              :placeholder="t('playerEdit.label')"
            />
            <div v-if="playerEditError" class="error-message">{{ playerEditError }}</div>
            <div class="modal-actions">
              <button class="secondary" type="button" @click="cancelPlayerEdit">{{ t('playerEdit.cancel') }}</button>
              <button class="primary" type="button" @click="savePlayerName" :disabled="savingPlayer">{{ t('playerEdit.save') }}</button>
            </div>
          </div>
        </div>

        <WinnerModal
          :visible="showWinnerModal"
          :winners="winners || []"
          :elo-changes="eloChanges"
          @close="showWinnerModal = false"
        />
      </div>

      <DeleteConfirmModal
        :visible="showDeleteConfirm"
        @confirm="deleteGame"
        @cancel="showDeleteConfirm = false"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { gamesApi } from '../api/games'
import type { Game, GameEloChange } from '../types'
import { useI18n } from 'vue-i18n'
import LanguageSelector from '@/components/LanguageSelector.vue'
import WinnerModal from '@/components/WinnerModal.vue'
import DeleteConfirmModal from '@/components/DeleteConfirmModal.vue'
import { displayName as truncateName, getTotalAfterRound as getSharedTotalAfterRound, getRoundDelta as getSharedRoundDelta, isLowestInRound as checkIsLowestInRound } from '../utils/gameHelpers'
import { useLongPress } from '@/composables/useLongPress'
import { usePlayerNameEditor } from '@/composables/usePlayerNameEditor'

const router = useRouter()
const route = useRoute()
const { t } = useI18n()

const game = ref<Game | null>(null)
const loading = ref(true)
const error = ref('')
const scoreError = ref('')
const roundScores = ref<number[]>([])
const showDeleteConfirm = ref(false)
const showWinnerModal = ref(false)
const eloChanges = ref<GameEloChange[]>([])
const scoreTableRef = ref<HTMLElement | null>(null)
const retryCount = ref(0)
const maxRetries = 3
const gameId = route.params.id as string
const editingRoundIndex = ref<number | null>(null)
const editingRoundScores = ref<number[]>([])
const editingRoundError = ref('')
const savingRound = ref(false)
const submittingRound = ref(false)

const {
  editingPlayerId,
  playerNameDraft,
  playerEditError,
  savingPlayer,
  startEditingPlayer,
  cancelPlayerEdit,
  savePlayerName
} = usePlayerNameEditor(gameId, game)

const { start: startRoundLongPress, cancel: cancelRoundLongPress } = useLongPress<number>((roundIdx) => {
  openRoundEditor(roundIdx)
})

const { start: startPlayerLongPress, cancel: cancelPlayerLongPress } = useLongPress<string>((playerId) => {
  startEditingPlayer(playerId)
})

type PassingDirection = 'left' | 'right' | 'across' | 'positions'
type PassingInstruction = {
  type: 'keep' | 'pass'
  cards?: number
  direction?: PassingDirection
  positions?: number
}

type PassingCardDetails = {
  mode: 'keep' | 'pass'
  badge: string
  headline: string
  description: string
  cards?: number
  direction?: string
}

const canSubmitRound = computed(() => {
  return roundScores.value.every(score => 
    score !== null && score !== undefined && score >= 0 && score <= 26
  )
})

const enteredTotal = computed(() => {
  return roundScores.value.reduce((sum, score) => sum + (Number.isFinite(score) ? score : 0), 0)
})

const winners = computed(() => {
  if (!game.value) return null
  // Check if game over: any player at or above 100
  const maxScore = Math.max(...game.value.players.map(p => p.total_score))
  if (maxScore >= 100) {
    // In Hearts, lowest score wins. Find lowest score and all players with it
    const lowestScore = Math.min(...game.value.players.map(p => p.total_score))
    const winners = game.value.players.filter(p => p.total_score === lowestScore)
    return winners
  }
  return null
})

watch(winners, async (val) => {
  if (val && val.length > 0) {
    showWinnerModal.value = true
    // Fetch ELO changes when game ends
    try {
      eloChanges.value = await gamesApi.getGameEloChanges(gameId)
    } catch (e) {
      console.error('Failed to fetch ELO changes:', e)
      eloChanges.value = []
    }
  }
})

const validateHeartsScores = (scores: number[]): string | null => {
  if (!game.value) {
    return t('roundEdit.errors.missingGame') as string
  }

  if (
    scores.some(
      (score) => score === null || score === undefined || !Number.isFinite(score) || score < 0 || score > 26
    )
  ) {
    return t('heartsGame.errors.scoreRange') as string
  }

  const total = scores.reduce((sum, score) => sum + score, 0)
  const numPlayers = game.value.players.length
  const validTotals = [26, 26 * (numPlayers - 1)]
  if (!validTotals.includes(total)) {
    return t('heartsGame.errors.invalidTotal', { total, validTotals: validTotals.join(' or ') }) as string
  }

  const maxScore = Math.max(...scores)
  if (maxScore < 13) {
    return t('heartsGame.errors.queenOfSpades') as string
  }

  return null
}

const loadGame = async (attempt: number = 1) => {
  try {
    loading.value = true
    error.value = ''
    console.log(`Loading game: ${gameId} (attempt ${attempt}/${maxRetries})`)
    
    // Quick health check to ensure backend is responsive
    try {
      await gamesApi.healthCheck()
    } catch (healthErr) {
      console.warn('Health check failed, but proceeding with game load attempt', healthErr)
    }
    
    game.value = await gamesApi.getGame(gameId)
    console.log('Game loaded:', game.value)
    roundScores.value = new Array(game.value.players.length).fill(0)
    retryCount.value = 0
    await nextTick()
    // scroll scoreboard to bottom after initial load
    if (scoreTableRef.value) {
      scoreTableRef.value.scrollTop = scoreTableRef.value.scrollHeight
    }
    // Stop the loading spinner now that we've successfully loaded the game
    loading.value = false
  } catch (err: any) {
    console.error(`Error loading game (attempt ${attempt}/${maxRetries}):`, err)
    
    // Retry logic for network errors
    if (attempt < maxRetries) {
      const delay = Math.pow(2, attempt - 1) * 500 // exponential backoff: 500ms, 1s, 2s
      console.log(`Retrying in ${delay}ms...`)
      setTimeout(() => {
        loadGame(attempt + 1)
      }, delay)
    } else {
      // All retries exhausted
      error.value = err.response?.data?.error || err.message || 'Failed to load game after multiple attempts'
      loading.value = false
    }
  }
}

const getPassingInstruction = (round: number, numPlayers: number): PassingInstruction => {
  if (numPlayers === 3) {
    const sequence: PassingInstruction[] = [
      { type: 'keep' },
      { type: 'pass', cards: 4, direction: 'left' },
      { type: 'pass', cards: 4, direction: 'right' }
    ]
    return sequence[round % sequence.length]
  } else if (numPlayers === 4) {
    const sequence: PassingInstruction[] = [
      { type: 'keep' },
      { type: 'pass', cards: 3, direction: 'left' },
      { type: 'pass', cards: 3, direction: 'right' },
      { type: 'pass', cards: 3, direction: 'across' }
    ]
    return sequence[round % sequence.length]
  } else if (numPlayers >= 5) {
    const sequence: PassingInstruction[] = [{ type: 'keep' }]
    for (let i = 1; i < numPlayers; i++) {
      sequence.push({ type: 'pass', cards: 2, direction: 'positions', positions: i })
    }
    return sequence[round % sequence.length]
  }

  return { type: 'keep' }
}

const translateDirection = (instruction: PassingInstruction) => {
  switch (instruction.direction) {
    case 'left':
      return t('heartsGame.passing.toLeft')
    case 'right':
      return t('heartsGame.passing.toRight')
    case 'across':
      return t('heartsGame.passing.across')
    case 'positions':
      return t('heartsGame.passing.positionsClockwise', { positions: instruction.positions ?? 0 })
    default:
      return t('heartsGame.passing.keep')
  }
}

const passingDetails = computed<PassingCardDetails | null>(() => {
  if (!game.value) return null
  const instruction = getPassingInstruction(game.value.current_round, game.value.players.length)

  if (instruction.type === 'keep') {
    return {
      mode: 'keep',
      badge: t('heartsGame.passingCard.keepBadge'),
      headline: t('heartsGame.passingCard.keepHeadline'),
      description: t('heartsGame.passingCard.keepDescription'),
    }
  }

  const cards = instruction.cards ?? 0
  const direction = translateDirection(instruction)

  return {
    mode: 'pass',
    badge: t('heartsGame.passingCard.passBadge'),
    headline: t('heartsGame.passingCard.passHeadline', { cards }, cards),
    description: t('heartsGame.passingCard.passDescription', { cards, direction }, cards),
    cards,
    direction
  }
})

const submitRound = async () => {
  if (!game.value || submittingRound.value) return

  scoreError.value = ''
  const validationError = validateHeartsScores(roundScores.value)
  if (validationError) {
    scoreError.value = validationError
    return
  }

  try {
    submittingRound.value = true
    game.value = await gamesApi.addRound(gameId, { scores: roundScores.value })
    roundScores.value = new Array(game.value.players.length).fill(0)
    await nextTick()
    if (scoreTableRef.value) {
      scoreTableRef.value.scrollTop = scoreTableRef.value.scrollHeight
    }
  } catch (err: any) {
    scoreError.value = err.response?.data?.error || t('heartsGame.errors.submitFailed')
  } finally {
    submittingRound.value = false
  }
}

const openRoundEditor = (roundIndex: number) => {
  if (!game.value) return
  if (roundIndex < 0 || roundIndex >= game.value.current_round) return
  editingRoundIndex.value = roundIndex
  editingRoundScores.value = game.value.players.map((player) => player.scores[roundIndex] ?? 0)
  editingRoundError.value = ''
}

const cancelRoundEdit = () => {
  editingRoundIndex.value = null
  editingRoundScores.value = []
  editingRoundError.value = ''
  savingRound.value = false
}

const saveRoundEdit = async () => {
  if (!game.value || editingRoundIndex.value === null) return
  const validationError = validateHeartsScores(editingRoundScores.value)
  if (validationError) {
    editingRoundError.value = validationError
    return
  }

  try {
    savingRound.value = true
    const updated = await gamesApi.updateRound(gameId, editingRoundIndex.value, {
      scores: editingRoundScores.value,
    })
    game.value = updated
    cancelRoundEdit()
  } catch (err: any) {
    editingRoundError.value = err.response?.data?.error || (t('roundEdit.errors.generic') as string)
  } finally {
    savingRound.value = false
  }
}

const onRoundRowPointerDown = (roundIndex: number) => {
  if (roundIndex < 0) return
  startRoundLongPress(roundIndex)
}

const onRoundRowPointerCancel = () => {
  cancelRoundLongPress()
}

const onPlayerHeaderPointerDown = (playerId: string) => {
  startPlayerLongPress(playerId)
}

const onPlayerHeaderPointerCancel = () => {
  cancelPlayerLongPress()
}

// ensure roundScores length matches players when game updates
watch(game, (g) => {
  if (!g) return
  roundScores.value = new Array(g.players.length).fill(0)
  const currentEditIndex = editingRoundIndex.value
  if (currentEditIndex !== null) {
    if (currentEditIndex >= g.current_round) {
      cancelRoundEdit()
    } else {
      editingRoundScores.value = g.players.map((player) => player.scores[currentEditIndex] ?? 0)
    }
  }
})

// Get the delta (points scored) for a specific player in a specific round
const getRoundDelta = (playerIndex: number, roundIndex: number): number | null => {
  return getSharedRoundDelta(game.value, playerIndex, roundIndex)
}

// Get the total score after a specific round (accounting for resets at 100)
const getTotalAfterRound = (playerIndex: number, roundIndex: number): number | null => {
  return getSharedTotalAfterRound(game.value, playerIndex, roundIndex, true)
}

// Check if a player has the lowest score in a specific round
const isLowestInRound = (playerIndex: number, roundIndex: number): boolean => {
  return checkIsLowestInRound(game.value, playerIndex, roundIndex)
}

// Navigate back to game selection
const goHome = () => {
  const locale = route.params.locale || 'en'
  router.push(`/${locale}/hearts/setup`)
}

// Delete the current game
const deleteGame = async () => {
  try {
    await gamesApi.deleteGame(gameId)
    goHome()
  } catch (err: any) {
    error.value = err.response?.data?.error || 'Failed to delete game'
  }
  showDeleteConfirm.value = false
}

// Load game on mount
onMounted(() => {
  loadGame()
  
  // Set loading to false if retries complete with error
  const loadingTimeout = setTimeout(() => {
    if (loading.value && error.value) {
      loading.value = false
    }
  }, 5000) // 5 second safety timeout
  
  return () => clearTimeout(loadingTimeout)
})

// King-specific logic - removed; use KingGame.vue instead

</script>

<style scoped>
@import '../styles/shared.css';
@import '../styles/game.css';
</style>
