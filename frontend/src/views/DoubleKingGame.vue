<template>
  <div class="double-king-game">
    <div v-if="loading" class="loading">{{ t('loading') }}</div>
    
    <div v-else-if="error" class="error-container">
      <h2>{{ t('error') }}</h2>
      <p>{{ error }}</p>
      <button class="primary" @click="goHome">{{ t('goHome') }}</button>
    </div>

    <div v-else-if="game" class="game-container">
      <div class="game-header">
        <button class="exit-button" @click="goHome" title="Exit game">←</button>
        <h1>{{ t('doubleKingGame.title') }}</h1>
        <LanguageSelector class="header-lang-selector" />
      </div>

      <!-- Game Selection Overview -->
      <div class="game-selection-overview card">
        <h2>{{ t('doubleKingGame.gameSelection') }}</h2>
        <div class="game-selection-grid">
          <button 
            v-for="(gameItem, index) in gameSelectionItems" 
            :key="index"
            class="game-selection-btn"
            :class="{ 
              'selected': gameItem.index === selectedGameIndex,
              'completed': gameItem.completed
            }"
            @click="selectGame(gameItem.index)"
            :disabled="gameItem.completed"
          >
            <span class="game-name">
              <span class="game-label">{{ gameItem.baseLabel }}</span>
              <span class="game-score" :class="gameItem.scoreClass">{{ gameItem.score }}</span>
              <span class="game-remaining">({{ gameItem.remainingPlays }}x)</span>
            </span>
            <span v-if="gameItem.completed" class="checkmark">✓</span>
          </button>
        </div>
      </div>

      <RoundDetailCard
        v-if="selectedGameDetails"
        :icon="selectedGameDetails.icon"
        :eyebrow="t('doubleKingGame.gameDetails.title')"
        :title="selectedGameDetails.title"
        :hint="selectedGameDetails.remainingLabel"
        :description="selectedGameDetails.description"
        :stats="selectedGameDetails.stats"
      />
      <RoundDetailCard
        v-else
        :eyebrow="t('doubleKingGame.gameDetails.title')"
        :title="t('doubleKingGame.title')"
        :description="t('doubleKingGame.gameDetails.unselected')"
        muted
      />

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
              <td v-for="(player, index) in game.players" :key="player.id"
                  :class="{
                    'col-alt': index % 2 === 1,
                    'highest': isHighestInRound(index, r - 1),
                    'positive-score': (getTotalAfterRound(index, r - 1) != null && getTotalAfterRound(index, r - 1)! >= 0),
                    'negative-score': (getTotalAfterRound(index, r - 1) != null && getTotalAfterRound(index, r - 1)! < 0)
                  }">
                <template v-if="getTotalAfterRound(index, r - 1) !== null">
                  {{ getTotalAfterRound(index, r - 1) }} (<span class="delta">{{ getRoundDelta(index, r - 1) !== null && getRoundDelta(index, r - 1)! >= 0 ? '+' : '' }}{{ getRoundDelta(index, r - 1) }}</span>)
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
                  <button class="primary" @click="submitRound" :disabled="!canSubmitRound || enteredTotal === 0 || submittingRound" style="margin-left:auto">{{ t('submitRound') }}</button>
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
        <!--ScoreSummaryChips :players="game.players" :display-name="truncateName" /-->
      </div>

        <WinnerModal
          :visible="showWinnerModal"
          :winners="winners || []"
          :elo-changes="eloChanges"
          @close="showWinnerModal = false"
        />
      </div>

    <div v-if="editingRoundIndex !== null && game" class="modal-overlay" @click="cancelRoundEdit">
      <div class="modal round-edit-modal" @click.stop>
        <h2>{{ t('roundEdit.title') }}</h2>
        <p class="modal-subtitle">{{ t('roundEdit.roundLabel', { round: editingRoundIndex + 1 }) }}</p>
        <p class="modal-hint">
          <span v-if="editingGameDetails">{{ editingGameDetails.baseLabel }}</span>
          <span v-else>{{ t('doubleKingGame.gameDetails.unselected') }}</span>
        </p>
        <div class="round-edit-grid">
          <div class="round-edit-field" v-for="(player, idx) in game.players" :key="player.id">
            <label :for="`double-king-edit-${player.id}`">{{ player.name }}</label>
            <input
              :id="`double-king-edit-${player.id}`"
              type="number"
              min="0"
              :max="editingRoundLimits.maxScore"
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

    <DeleteConfirmModal
      :visible="showDeleteConfirm"
      @confirm="deleteGame"
      @cancel="showDeleteConfirm = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { gamesApi } from '../api/games'
import type { Game, GameEloChange } from '../types'
import { useI18n } from 'vue-i18n'
import LanguageSelector from '@/components/LanguageSelector.vue'
import RoundDetailCard from '@/components/RoundDetailCard.vue'
import WinnerModal from '@/components/WinnerModal.vue'
import DeleteConfirmModal from '@/components/DeleteConfirmModal.vue'
import { displayName as truncateName, getRoundDelta as getSharedRoundDelta, getTotalAfterRound as getSharedTotalAfterRound, isHighestInRound as checkIsHighestInRound } from '../utils/gameHelpers'
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
const selectedGameIndex = ref<number | null>(null)
const retryCount = ref(0)
const maxRetries = 3
const gameId = route.params.id as string
const editingRoundIndex = ref<number | null>(null)
const editingRoundScores = ref<number[]>([])
const editingRoundError = ref('')
const savingRound = ref(false)
const submittingRound = ref(false)
const editingGameIndex = ref<number | null>(null)
const editingRoundLimits = computed(() => getDoubleKingLimits(editingGameIndex.value))

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

type DoubleKingDetailKey =
  | 'noTricks'
  | 'noHearts'
  | 'noKingsJacks'
  | 'noQueens'
  | 'noKingHearts'
  | 'not7thLast'
  | 'winTricks'

type DoubleKingDetailConfig = {
  key: DoubleKingDetailKey
  icon: string
  max: number
  expectedTotal: number
}

type SelectedGameDetails = {
  icon: string
  title: string
  description: string
  stats: { label: string; value: string; valueClass?: string }[]
  remainingLabel: string
}

type DoubleKingLimits = {
  maxScore: number
  expectedTotal: number
}

const getDoubleKingLimits = (gameIndex: number | null): DoubleKingLimits => {
  if (gameIndex === null) {
    return { maxScore: 13, expectedTotal: 13 }
  }

  switch (gameIndex) {
    case 2:
      return { maxScore: 8, expectedTotal: 8 }
    case 3:
      return { maxScore: 4, expectedTotal: 4 }
    case 4:
      return { maxScore: 1, expectedTotal: 1 }
    case 5:
      return { maxScore: 2, expectedTotal: 2 }
    default:
      return { maxScore: 13, expectedTotal: 13 }
  }
}

const validateDoubleKingScores = (scores: number[], gameIndex: number | null): string | null => {
  if (!game.value) {
    return t('roundEdit.errors.missingGame') as string
  }

  if (gameIndex === null) {
    return t('doubleKingGame.errors.invalidScores') as string
  }

  if (scores.length !== game.value.players.length) {
    return t('doubleKingGame.errors.invalidScores') as string
  }

  const { maxScore, expectedTotal } = getDoubleKingLimits(gameIndex)

  if (
    scores.some(
      (score) => score === null || score === undefined || !Number.isFinite(score) || score < 0 || score > maxScore
    )
  ) {
    return t('doubleKingGame.errors.invalidScores') as string
  }

  const total = scores.reduce((sum, score) => sum + (Number.isFinite(score) ? score : 0), 0)
  if (total !== expectedTotal) {
    return t('doubleKingGame.errors.invalidTotal', { total, expected: expectedTotal }) as string
  }

  return null
}


// Game selection items for Double King - will be populated based on players
let gameItems: { labelKey?: string; playerName?: string; label?: string; count: number }[] = []

const initializeGameItems = () => {
  if (!game.value) return
  gameItems = [
    { labelKey: 'doubleKingGame.games.noTricks', count: 2 },
    { labelKey: 'doubleKingGame.games.noHearts', count: 2 },
    { labelKey: 'doubleKingGame.games.noKingsJacks', count: 2 },
    { labelKey: 'doubleKingGame.games.noQueens', count: 2 },
    { labelKey: 'doubleKingGame.games.noKingHearts', count: 2 },
    { labelKey: 'doubleKingGame.games.not7thLast', count: 2 },
  ]
  // Add Trump choice games with player names
  for (let i = 0; i < game.value.players.length; i++) {
    gameItems.push({ labelKey: 'doubleKingGame.games.winTricks', playerName: truncateName(game.value.players[i].name), count: 2 })
  }
}

const baseGameDetailConfigs: DoubleKingDetailConfig[] = [
  { key: 'noTricks', icon: '∅', max: 13, expectedTotal: 13 },
  { key: 'noHearts', icon: '♥', max: 13, expectedTotal: 13 },
  { key: 'noKingsJacks', icon: 'KJ', max: 8, expectedTotal: 8 },
  { key: 'noQueens', icon: 'Q', max: 4, expectedTotal: 4 },
  { key: 'noKingHearts', icon: '♥K', max: 1, expectedTotal: 1 },
  { key: 'not7thLast', icon: '7th', max: 2, expectedTotal: 2 },
]

const trumpGameDetailConfig: DoubleKingDetailConfig = {
  key: 'winTricks',
  icon: '♦',
  max: 13,
  expectedTotal: 13,
}

const getDetailConfig = (idx: number): DoubleKingDetailConfig => {
  if (idx < baseGameDetailConfigs.length) {
    return baseGameDetailConfigs[idx]
  }
  return trumpGameDetailConfig
}

const getTrumpOwnerName = (idx: number): string => {
  if (!game.value) return ''
  const playerIdx = idx - baseGameDetailConfigs.length
  return game.value.players[playerIdx]?.name || t('doubleKingGame.gameDetails.trumpPlayer')
}

const gameSelectionItems = computed(() => {
  return gameItems.map((item, idx) => {
    // Count how many full plays (rounds) of this game have been completed.
    // Each completed round appends the game_index to every player's `game_indices`.
    // To count completed rounds reliably we take the minimum number of occurrences
    // of this index across all players (only full rounds where all players have an entry).
    let timesPlayed = 0
    if (game.value && game.value.players.length > 0) {
      const counts = game.value.players.map(player => (player.game_indices ? player.game_indices.filter(gi => gi === idx).length : 0))
      timesPlayed = Math.min(...counts)
    }

    // A game is completed only if it has been played 2 times
    const completed = timesPlayed >= 2

    // Calculate remaining plays
    const remainingPlays = Math.max(0, 2 - timesPlayed)
    
    // Build label with localization and player name if applicable
    let label = item.labelKey ? t(item.labelKey) : (item.label || '')
    if (item.playerName) {
      label = `${label} (${item.playerName})`
    }
    const baseLabel = label

    // Determine score display and class
    let score = ''
    let scoreClass = ''
    if (idx < 6) {
      // penalty games
      switch (idx) {
        case 0:
          score = game.value ? (game.value.players.length === 3 ? '-15' : '-20') : '-20'
          break
        case 1:
          score = '-20'
          break
        case 2:
          score = '-30'
          break
        case 3:
          score = '-50'
          break
        case 4:
          score = '-160'
          break
        case 5:
          score = game.value ? (game.value.players.length === 3 ? '-80' : '-90') : '-90'
          break
      }
      scoreClass = 'penalty'
    } else {
      // trump games
      score = '+25'
      scoreClass = 'positive'
    }

    return {
      index: idx,
      baseLabel,
      score,
      scoreClass,
      remainingPlays,
      completed,
      timesPlayed
    }
  })
})

const selectedGameDetails = computed<SelectedGameDetails | null>(() => {
  if (!game.value || selectedGameIndex.value === null) return null
  const selectionMeta = gameSelectionItems.value.find(item => item.index === selectedGameIndex.value)
  if (!selectionMeta) return null
  const config = getDetailConfig(selectedGameIndex.value)
  const descriptionKey = `doubleKingGame.gameDetails.descriptions.${config.key}`
  const description = config.key === 'winTricks'
    ? t(descriptionKey, { player: getTrumpOwnerName(selectedGameIndex.value) })
    : t(descriptionKey)

  return {
    icon: config.icon,
    title: selectionMeta.baseLabel,
    description,
    remainingLabel: t('doubleKingGame.gameDetails.remaining', { plays: selectionMeta.remainingPlays }),
    stats: [
      {
        label: t('doubleKingGame.gameDetails.rangeLabel'),
        value: `0–${config.max}`
      },
      {
        label: t('doubleKingGame.gameDetails.totalLabel'),
        value: `${config.expectedTotal}`
      },
      {
        label: t('doubleKingGame.gameDetails.scoreLabel'),
        value: selectionMeta.score,
        valueClass: selectionMeta.scoreClass
      }
    ]
  }
})

const editingGameDetails = computed(() => {
  if (editingGameIndex.value === null) return null
  return gameSelectionItems.value.find((item) => item.index === editingGameIndex.value) ?? null
})

const canSubmitRound = computed(() => {
  if (!game.value || selectedGameIndex.value === null) return false
  return !validateDoubleKingScores(roundScores.value, selectedGameIndex.value)
})

const enteredTotal = computed(() => {
  return roundScores.value.reduce((sum, score) => sum + (Number.isFinite(score) ? score : 0), 0)
})


const winners = computed(() => {
  if (!game.value) return null
  if (game.value.current_round >= 6 + game.value.players.length) {
      const highestScore = Math.max(...game.value.players.map(p => p.total_score))
      const winners = game.value.players.filter(p => p.total_score === highestScore)
      return winners
  }
  return null
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

const loadGame = async (attempt: number = 1) => {
  try {
    loading.value = true
    error.value = ''
    
    game.value = await gamesApi.getGame(gameId)
    initializeGameItems()
    roundScores.value = new Array(game.value.players.length).fill(0)
    
    // Select first uncompleted game
    const firstUncompleted = gameSelectionItems.value.find(g => !g.completed)
    if (firstUncompleted) {
      selectedGameIndex.value = firstUncompleted.index
    }
    
    retryCount.value = 0
    await nextTick()
    if (scoreTableRef.value) {
      scoreTableRef.value.scrollTop = scoreTableRef.value.scrollHeight
    }
    loading.value = false
  } catch (err: any) {
    if (attempt < maxRetries) {
      const delay = Math.pow(2, attempt - 1) * 500
      setTimeout(() => {
        loadGame(attempt + 1)
      }, delay)
    } else {
      error.value = err.response?.data?.error || err.message || 'Failed to load game after multiple attempts'
      loading.value = false
    }
  }
}

const submitRound = async () => {
  if (!game.value || submittingRound.value) return

  scoreError.value = ''
  const validationError = validateDoubleKingScores(roundScores.value, selectedGameIndex.value)
  if (validationError) {
    scoreError.value = validationError
    return
  }

  try {
    submittingRound.value = true
    game.value = await gamesApi.addRound(gameId, { scores: roundScores.value, game_index: selectedGameIndex.value ?? undefined })
    roundScores.value = new Array(game.value.players.length).fill(0)
    
    // Auto-select next uncompleted game
    const nextUncompleted = gameSelectionItems.value.find(g => !g.completed)
    if (nextUncompleted) {
      selectedGameIndex.value = nextUncompleted.index
    }
    
    await nextTick()
    if (scoreTableRef.value) {
      scoreTableRef.value.scrollTop = scoreTableRef.value.scrollHeight
    }
  } catch (err: any) {
    scoreError.value = err.response?.data?.error || t('doubleKingGame.errors.submitFailed')
  } finally {
    submittingRound.value = false
  }
}

const getRoundGameIndexFromGame = (targetGame: Game, roundIndex: number): number | null => {
  for (const player of targetGame.players) {
    const entry = player.game_indices?.[roundIndex]
    if (entry !== undefined && entry !== null) {
      return entry
    }
  }
  return null
}

const populateEditingRoundState = (roundIndex: number) => {
  if (!game.value) return
  editingRoundScores.value = game.value.players.map((player) => player.scores[roundIndex] ?? 0)
  editingGameIndex.value = getRoundGameIndexFromGame(game.value, roundIndex)
}

const openRoundEditor = (roundIndex: number) => {
  if (!game.value) return
  if (roundIndex < 0 || roundIndex >= game.value.current_round) return
  editingRoundIndex.value = roundIndex
  populateEditingRoundState(roundIndex)
  editingRoundError.value = ''
}

const cancelRoundEdit = () => {
  editingRoundIndex.value = null
  editingRoundScores.value = []
  editingRoundError.value = ''
  editingGameIndex.value = null
  savingRound.value = false
}

const saveRoundEdit = async () => {
  if (!game.value || editingRoundIndex.value === null) return

  const validationError = validateDoubleKingScores(editingRoundScores.value, editingGameIndex.value)
  if (validationError) {
    editingRoundError.value = validationError
    return
  }

  try {
    savingRound.value = true
    const updated = await gamesApi.updateRound(gameId, editingRoundIndex.value, {
      scores: editingRoundScores.value,
      game_index: editingGameIndex.value ?? undefined
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

const selectGame = (gameIndex: number) => {
  selectedGameIndex.value = gameIndex
}

watch(game, (g) => {
  if (!g) return
  roundScores.value = new Array(g.players.length).fill(0)
  if (editingRoundIndex.value !== null) {
    const currentIndex = editingRoundIndex.value
    if (currentIndex >= g.current_round) {
      cancelRoundEdit()
    } else {
      editingRoundScores.value = g.players.map((player) => player.scores[currentIndex] ?? 0)
      editingGameIndex.value = getRoundGameIndexFromGame(g, currentIndex)
    }
  }
})

const getRoundDelta = (playerIndex: number, roundIndex: number): number | null => {
  return getSharedRoundDelta(game.value, playerIndex, roundIndex)
}

const getTotalAfterRound = (playerIndex: number, roundIndex: number): number | null => {
  return getSharedTotalAfterRound(game.value, playerIndex, roundIndex)
}

const isHighestInRound = (playerIndex: number, roundIndex: number): boolean => {
  return checkIsHighestInRound(game.value, playerIndex, roundIndex)
}

const goHome = () => {
  const locale = route.params.locale || 'en'
  router.push(`/${locale}`)
}

const deleteGame = async () => {
  try {
    await gamesApi.deleteGame(gameId)
    goHome()
  } catch (err: any) {
    error.value = err.response?.data?.error || 'Failed to delete game'
  }
  showDeleteConfirm.value = false
}

onMounted(() => {
  loadGame()
  
  const loadingTimeout = setTimeout(() => {
    if (loading.value && error.value) {
      loading.value = false
    }
  }, 5000)
  
  return () => clearTimeout(loadingTimeout)
})
</script>

<style scoped>
@import '../styles/shared.css';
@import '../styles/game.css';
@import '../styles/double-king-game.css';

.game-score.penalty {
  color: #c0392b; /* red */
  margin-left: 6px;
}
.game-score.positive {
  color: #27ae60; /* green */
  margin-left: 6px;
}
.game-label { margin-right: 6px; }
.game-remaining { margin-left: 6px; color: #555; }
</style>
