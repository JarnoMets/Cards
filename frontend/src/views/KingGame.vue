<template>
  <div class="page-container game-page king-game-page">
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
          <h1>{{ t('kingGame.title') }}</h1>
          <LanguageSelector class="header-lang-selector" />
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
                    highest: isHighestInRound(index, r - 1),
                    'positive-score': (getTotalAfterRound(index, r - 1) != null && getTotalAfterRound(index, r - 1)! >= 0),
                    'negative-score': (getTotalAfterRound(index, r - 1) != null && getTotalAfterRound(index, r - 1)! < 0)
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
                    :min="kingInputConfig[game.current_round]?.min || 0"
                    :max="kingInputConfig[game.current_round]?.max || 26"
                    :placeholder="kingInputConfig[game.current_round]?.label || t('points')"
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

        <RoundDetailCard
          v-if="currentRoundDetail"
          :icon="currentRoundDetail.icon"
          :eyebrow="t('kingGame.roundCard.title')"
          :title="currentRoundDetail.label"
          :hint="currentRoundDetail.hint"
          :stats="currentRoundDetail.stats"
        />

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
            <template v-if="editingRoundConfig">
              {{ editingRoundConfig.label }} · {{ t('kingGame.roundCard.rangeLabel') }} {{ editingRoundConfig.min }}–{{ editingRoundConfig.max }}
            </template>
            <template v-else>
              {{ t('roundEdit.hint') }}
            </template>
          </p>
          <div class="round-edit-grid">
            <div class="round-edit-field" v-for="(player, idx) in game.players" :key="player.id">
              <label :for="`king-edit-${player.id}`">{{ player.name }}</label>
              <input
                :id="`king-edit-${player.id}`"
                type="number"
                :min="editingRoundConfig?.min ?? 0"
                :max="editingRoundConfig?.max ?? 26"
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
import { displayName as truncateName, getTotalAfterRound as getSharedTotalAfterRound, getRoundDelta as getSharedRoundDelta, isHighestInRound as checkIsHighestInRound } from '../utils/gameHelpers'
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

type KingRoundConfig = {
  min: number
  max: number
  label: string
  hint?: string
  scoring4p: string
  scoring3p: string
  icon: string
}

const gameId = route.params.id as string
const editingRoundIndex = ref<number | null>(null)
const editingRoundScores = ref<number[]>([])
const editingRoundError = ref('')
const savingRound = ref(false)
const submittingRound = ref(false)

const editingRoundConfig = computed(() => {
  if (editingRoundIndex.value === null) return null
  return kingInputConfig[editingRoundIndex.value] ?? null
})

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

// King round configuration (10 rounds total)
const kingInputConfig: KingRoundConfig[] = [
  { 
    min: 0, max: 13, 
    label: t('kingRules.round1'), 
    hint: t('kingRules.inputHearts'),
    scoring4p: '-20',
    scoring3p: '-15',
    icon: '♥'
  },
  { 
    min: 0, max: 13, 
    label: t('kingRules.round2'), 
    hint: t('kingRules.inputLastTrick'),
    scoring4p: '-20',
    scoring3p: '-20',
    icon: '♠A'
  },
  { 
    min: 0, max: 8, 
    label: t('kingRules.round3'), 
    hint: t('kingRules.inputKings'),
    scoring4p: '-30',
    scoring3p: '-30',
    icon: 'K'
  },
  { 
    min: 0, max: 4, 
    label: t('kingRules.round4'), 
    hint: t('kingRules.inputQueens'),
    scoring4p: '-50',
    scoring3p: '-50',
    icon: 'Q'
  },
  { 
    min: 0, max: 1, 
    label: t('kingRules.round5'), 
    hint: t('kingRules.inputTricks'),
    scoring4p: '-160',
    scoring3p: '-160',
    icon: '∅'
  },
  { 
    min: 0, max: 2, 
    label: t('kingRules.round6'), 
    hint: t('kingRules.inputHeartsAny'),
    scoring4p: '-90',
    scoring3p: '-80',
    icon: '♥K'
  },
  { 
    min: 0, max: 13, 
    label: t('kingRules.round7'), 
    hint: t('kingRules.inputTricks'),
    scoring4p: '+25',
    scoring3p: '+25',
    icon: '★'
  },
  { 
    min: 0, max: 13, 
    label: t('kingRules.round8'), 
    hint: t('kingRules.inputTricks'),
    scoring4p: '+25',
    scoring3p: '+25',
    icon: '★'
  },
  { 
    min: 0, max: 13, 
    label: t('kingRules.round9'), 
    hint: t('kingRules.inputTricks'),
    scoring4p: '+25',
    scoring3p: '+25',
    icon: '★'
  },
  { 
    min: 0, max: 13, 
    label: t('kingRules.round10'), 
    hint: t('kingRules.inputTricks'),
    scoring4p: '+25',
    scoring3p: '+25',
    icon: '★'
  },
]

const currentRoundDetail = computed(() => {
  if (!game.value || game.value.current_round >= kingInputConfig.length) return null
  const config = kingInputConfig[game.value.current_round]
  const playerCount = game.value.players.length
  const scoring = playerCount === 3 ? config.scoring3p : config.scoring4p
  return {
    ...config,
    scoring,
    stats: [
      {
        label: t('kingGame.roundCard.rangeLabel'),
        value: `${config.min}–${config.max}`
      },
      {
        label: t('kingGame.roundCard.scoreLabel'),
        value: scoring,
        valueClass: scoring.includes('+') ? 'positive' : 'penalty'
      }
    ]
  }
})

const canSubmitRound = computed(() => {
  if (!game.value) return false
  const playerCount = game.value.players.length
  if (roundScores.value.length !== playerCount) {
    return false
  }
  const targetRoundIndex = Math.min(game.value.current_round, kingInputConfig.length - 1)
  return !validateKingScores(roundScores.value, targetRoundIndex)
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

const validateKingScores = (scores: number[], roundIndex: number): string | null => {
  if (!game.value) {
    return t('roundEdit.errors.missingGame') as string
  }

  if (scores.some(score => score === null || score === undefined || !Number.isFinite(score))) {
    return t('kingGame.errors.invalidScores') as string
  }

  if (roundIndex >= 0 && roundIndex < kingInputConfig.length) {
    const config = kingInputConfig[roundIndex]
    if (scores.some(score => score < config.min || score > config.max)) {
      return t('kingGame.errors.scoreRange', { min: config.min, max: config.max }) as string
    }
  }

  return null
}

const loadGame = async (attempt: number = 1) => {
  try {
    loading.value = true
    error.value = ''
    console.log(`Loading King game: ${gameId} (attempt ${attempt}/${maxRetries})`)
    
    // Quick health check to ensure backend is responsive
    try {
      await gamesApi.healthCheck()
    } catch (healthErr) {
      console.warn('Health check failed, but proceeding with game load attempt', healthErr)
    }
    
    game.value = await gamesApi.getGame(gameId)
    console.log('King game loaded:', game.value)
    roundScores.value = new Array(game.value.players.length).fill(0)
    retryCount.value = 0
    await nextTick()
    if (scoreTableRef.value) {
      scoreTableRef.value.scrollTop = scoreTableRef.value.scrollHeight
    }
    loading.value = false
  } catch (err: any) {
    console.error(`Error loading King game (attempt ${attempt}/${maxRetries}):`, err)
    
    if (attempt < maxRetries) {
      const delay = Math.pow(2, attempt - 1) * 500
      console.log(`Retrying in ${delay}ms...`)
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
  const targetRoundIndex = Math.min(game.value.current_round, kingInputConfig.length - 1)
  const validationError = validateKingScores(roundScores.value, targetRoundIndex)
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
    scoreError.value = err.response?.data?.error || t('kingGame.errors.submitFailed')
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
  const validationError = validateKingScores(editingRoundScores.value, editingRoundIndex.value)
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
</style>
