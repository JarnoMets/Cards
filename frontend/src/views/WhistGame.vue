<template>
  <div class="whist-game">
    <div v-if="loading" class="loading">{{ t('loading') }}</div>

    <div v-else-if="error" class="error-container">
      <h2>{{ t('error') }}</h2>
      <p>{{ error }}</p>
      <button class="primary" @click="goHome">{{ t('goHome') }}</button>
    </div>

    <div v-else-if="game" class="game-container">
      <div class="game-header">
        <button class="exit-button" @click="goHome" title="Exit game">←</button>
        <h1>{{ t('whistGame.title') }} - {{ t('round') }} {{ game.current_round + 1 }}</h1>
        <LanguageSelector class="header-lang-selector" />
      </div>

      <!-- Score Preview Banner -->
      <div v-if="hasActiveContract && previewScores.length > 0" class="score-preview card">
        <div class="preview-header">
          <span class="preview-icon">📊</span>
          <strong>{{ t('whistGame.scorePreview') }}</strong>
        </div>
        <div class="preview-scores">
          <div 
            v-for="(score, idx) in previewScores" 
            :key="idx" 
            class="preview-score"
            :class="[
              { positive: score > 0, negative: score < 0 },
              getPreviewTeamClass(idx)
            ]"
          >
            <span class="preview-player">{{ game.players[idx].name }}</span>
            <span class="preview-value">{{ score >= 0 ? '+' : '' }}{{ score }}</span>
          </div>
        </div>
        <div v-if="previewError" class="preview-error">{{ previewError }}</div>
      </div>

      <div class="scoreboard card">
        <table>
          <thead>
            <tr>
              <th class="round-col">{{ t('round') }}</th>
              <th class="contract-col">{{ t('whistGame.contract') }}</th>
              <th v-for="player in game.players" :key="player.id" class="player-col">
                {{ player.name }}
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="r in game.current_round" :key="r" class="history-row">
              <td class="round-col">{{ t('roundShort') }}{{ r }}</td>
              <td class="contract-col">
                <span class="contract-badge" :class="getContractClass(r - 1)">
                  {{ getContractDisplay(r - 1) }}
                </span>
              </td>
              <td 
                v-for="(player, playerIdx) in game.players" 
                :key="player.id" 
                class="score-cell"
                :class="[
                  { positive: (player.scores[r - 1] ?? 0) > 0, negative: (player.scores[r - 1] ?? 0) < 0 },
                  getHistoryTeamClass(r - 1, playerIdx)
                ]"
              >
                {{ formatScore(player.scores[r - 1]) }}
              </td>
            </tr>
            <!-- Totals row -->
            <tr class="totals-row">
              <td class="round-col"><strong>{{ t('whistGame.total') }}</strong></td>
              <td class="contract-col"></td>
              <td 
                v-for="player in game.players" 
                :key="player.id" 
                class="score-cell total-cell"
                :class="{ positive: player.total_score > 0, negative: player.total_score < 0 }"
              >
                <strong>{{ formatScore(player.total_score) }}</strong>
              </td>
            </tr>
          </tbody>
          <tfoot>
            <tr class="input-row">
              <td class="round-col"><strong>{{ t('roundShort') }}{{ game.current_round + 1 }}</strong></td>
              <td class="contract-col"></td>
              <td v-for="(player, index) in game.players" :key="player.id" class="input-cell" :class="getCurrentTeamClass(index)">
                <div class="whist-input">
                  <select 
                    v-model="selectedContracts[index]"
                    :class="{ 
                      'team-selected': isTeamContract(selectedContracts[index]) && getTeamPartnerCount(index) === 2,
                      'solo-selected': isSoloContract(selectedContracts[index]),
                      'misery-selected': isMiseryContract(selectedContracts[index]),
                      'opponent-auto': isOpponentOfContract(index)
                    }"
                  >
                    <option v-for="option in contractOptions" :key="option.key" :value="option.key">
                      {{ t(option.labelKey) }}
                    </option>
                  </select>
                  <!-- Show auto-calculated tricks for opponents of team contracts -->
                  <input
                    v-if="isOpponentOfContract(index)"
                    type="text"
                    :value="getOpponentTricks(index)"
                    disabled
                    class="auto-calculated"
                    :placeholder="t('whistGame.opponentTricks')"
                  />
                  <!-- Regular input for contracting players -->
                  <input
                    v-else
                    v-model.number="tricks[index]"
                    type="number"
                    min="0"
                    max="13"
                    :disabled="!requiresTricks(selectedContracts[index])"
                    :placeholder="t('whistGame.tricksPlaceholder')"
                    :class="{ synced: isSyncedTeamTrick(index) }"
                  />
                </div>
              </td>
            </tr>
            <tr>
              <td :colspan="game.players.length + 2" class="table-actions-cell">
                <div class="table-actions">
                  <button class="secondary" @click="passRound" :disabled="hasActiveContract">
                    {{ t('whistGame.passRound') }}
                  </button>
                  <div v-if="scoreError" class="error-message">{{ scoreError }}</div>
                  <button class="primary" @click="submitRound" :disabled="!canSubmit || submittingRound">
                    {{ t('submitRound') }}
                  </button>
                </div>
              </td>
            </tr>
          </tfoot>
        </table>
      </div>

      <!-- Contract Legend -->
      <div class="contract-legend card">
        <h3>{{ t('whistGame.contractLegend') }}</h3>
        <div class="legend-grid">
          <div class="legend-section">
            <h4 class="legend-title team">{{ t('whistGame.legendTeam') }}</h4>
            <div class="legend-items">
              <span class="legend-item">Accept 8-13, Trull</span>
            </div>
          </div>
          <div class="legend-section">
            <h4 class="legend-title solo">{{ t('whistGame.legendSolo') }}</h4>
            <div class="legend-items">
              <span class="legend-item">Solo 5-8, Abondance, Piccolo, Slam, Grand Slam</span>
            </div>
          </div>
          <div class="legend-section">
            <h4 class="legend-title misery">{{ t('whistGame.legendMisery') }}</h4>
            <div class="legend-items">
              <span class="legend-item">Small/Large/Open Misery</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import LanguageSelector from '@/components/LanguageSelector.vue'
import { gamesApi } from '../api/games'
import type {
  Game,
  WhistContract,
  WhistRoundRequest,
  WhistSelectionPayload,
} from '../types'

type TrickValue = number | null
type ContractKey = 'none' | WhistContract

interface ContractOption {
  key: ContractKey
  labelKey: string
}

const contractOptions: ContractOption[] = [
  { key: 'none', labelKey: 'whistGame.contracts.none' },
  { key: 'accept_8', labelKey: 'whistGame.contracts.accept8' },
  { key: 'accept_9', labelKey: 'whistGame.contracts.accept9' },
  { key: 'accept_10', labelKey: 'whistGame.contracts.accept10' },
  { key: 'accept_11', labelKey: 'whistGame.contracts.accept11' },
  { key: 'accept_12', labelKey: 'whistGame.contracts.accept12' },
  { key: 'accept_13', labelKey: 'whistGame.contracts.accept13' },
  { key: 'solo_5', labelKey: 'whistGame.contracts.solo5' },
  { key: 'solo_6', labelKey: 'whistGame.contracts.solo6' },
  { key: 'solo_7', labelKey: 'whistGame.contracts.solo7' },
  { key: 'solo_8', labelKey: 'whistGame.contracts.solo8' },
  { key: 'abondance_9', labelKey: 'whistGame.contracts.abondance9' },
  { key: 'abondance_10', labelKey: 'whistGame.contracts.abondance10' },
  { key: 'abondance_11', labelKey: 'whistGame.contracts.abondance11' },
  { key: 'abondance_12', labelKey: 'whistGame.contracts.abondance12' },
  { key: 'trull', labelKey: 'whistGame.contracts.trull' },
  { key: 'piccolo', labelKey: 'whistGame.contracts.piccolo' },
  { key: 'small_misery', labelKey: 'whistGame.contracts.smallMisery' },
  { key: 'large_misery', labelKey: 'whistGame.contracts.largeMisery' },
  { key: 'open_misery', labelKey: 'whistGame.contracts.openMisery' },
  { key: 'slam', labelKey: 'whistGame.contracts.slam' },
  { key: 'grand_slam', labelKey: 'whistGame.contracts.grandSlam' },
]

const TEAM_CONTRACTS = new Set<ContractKey>([
  'accept_8',
  'accept_9',
  'accept_10',
  'accept_11',
  'accept_12',
  'accept_13',
  'trull',
])

const SOLO_CONTRACTS = new Set<ContractKey>([
  'solo_5',
  'solo_6',
  'solo_7',
  'solo_8',
  'abondance_9',
  'abondance_10',
  'abondance_11',
  'abondance_12',
  'piccolo',
  'slam',
  'grand_slam',
])

const MISERY_CONTRACTS = new Set<ContractKey>([
  'small_misery',
  'large_misery',
  'open_misery',
])

const route = useRoute()
const router = useRouter()
const { t } = useI18n()

const game = ref<Game | null>(null)
const loading = ref(true)
const submittingRound = ref(false)
const error = ref('')
const scoreError = ref('')
const previewError = ref('')
const selectedContracts = ref<ContractKey[]>([])
const tricks = ref<TrickValue[]>([])

const gameId = route.params.id as string

const requiresTricks = (key: ContractKey) => key !== 'none'
const isTeamContract = (key: ContractKey) => TEAM_CONTRACTS.has(key)
const isSoloContract = (key: ContractKey) => SOLO_CONTRACTS.has(key)
const isMiseryContract = (key: ContractKey) => MISERY_CONTRACTS.has(key)

const hasActiveContract = computed(() => selectedContracts.value.some(key => key !== 'none'))

// Get the active team contract (if any)
const activeTeamContract = computed((): { contract: ContractKey; players: number[] } | null => {
  for (const contractKey of TEAM_CONTRACTS) {
    const players = selectedContracts.value
      .map((key, idx) => key === contractKey ? idx : -1)
      .filter(idx => idx !== -1)
    if (players.length === 2) {
      return { contract: contractKey, players }
    }
  }
  return null
})

// Get the active solo contract (if any)
const activeSoloContract = computed((): { contract: ContractKey; player: number } | null => {
  for (const contractKey of SOLO_CONTRACTS) {
    const playerIdx = selectedContracts.value.findIndex(key => key === contractKey)
    if (playerIdx !== -1) {
      return { contract: contractKey, player: playerIdx }
    }
  }
  return null
})

// Check if a player is an opponent (not part of the active contract)
const isOpponentOfContract = (index: number): boolean => {
  const team = activeTeamContract.value
  if (team) {
    return !team.players.includes(index)
  }
  const solo = activeSoloContract.value
  if (solo) {
    return solo.player !== index
  }
  return false
}

// Get opponent tricks (13 - contracting player(s) tricks)
const getOpponentTricks = (_index: number): string => {
  const team = activeTeamContract.value
  if (team) {
    const teamTricks = tricks.value[team.players[0]]
    if (typeof teamTricks !== 'number') return '—'
    return (13 - teamTricks).toString()
  }
  const solo = activeSoloContract.value
  if (solo) {
    const soloTricks = tricks.value[solo.player]
    if (typeof soloTricks !== 'number') return '—'
    return (13 - soloTricks).toString()
  }
  return ''
}

// Get the number of players with the same team contract
const getTeamPartnerCount = (index: number): number => {
  const contract = selectedContracts.value[index]
  if (!isTeamContract(contract)) return 0
  return selectedContracts.value.filter(c => c === contract).length
}

// Check if a player's trick input is synced with team partner
const isSyncedTeamTrick = (index: number): boolean => {
  const contract = selectedContracts.value[index]
  if (!isTeamContract(contract)) return false
  return getTeamPartnerCount(index) === 2
}

// Format score with sign
const formatScore = (score: number | undefined): string => {
  if (score === undefined || score === null) return '-'
  if (score === 0) return '0'
  return score > 0 ? `+${score}` : `${score}`
}

// Get contract display for a round from game history
const getContractDisplay = (roundIndex: number): string => {
  // We'll derive this from the scores pattern
  const scores = game.value?.players.map(p => p.scores[roundIndex] ?? 0) || []
  const nonZero = scores.filter(s => s !== 0)
  
  if (nonZero.length === 0) return t('whistGame.contracts.none') as string
  
  // Check for team pattern (2 positive, 2 negative of same absolute value)
  const positive = scores.filter(s => s > 0)
  const negative = scores.filter(s => s < 0)
  
  if (positive.length === 2 && negative.length === 2) {
    if (positive[0] === positive[1] && Math.abs(negative[0]) === positive[0]) {
      return t('whistGame.legendTeam') as string
    }
  }
  
  // Check for solo pattern (1 player with 3x score of others)
  if (positive.length === 1 && negative.length === 3) {
    return t('whistGame.legendSolo') as string
  }
  if (negative.length === 1 && positive.length === 3) {
    return t('whistGame.legendSolo') as string
  }
  
  // Check for misery pattern
  if (positive.length + negative.length <= 4) {
    return t('whistGame.legendMisery') as string
  }
  
  return '—'
}

const getContractClass = (roundIndex: number): string => {
  const display = getContractDisplay(roundIndex)
  if (display.includes('Team') || display.includes('Ploeg')) return 'team'
  if (display.includes('Solo')) return 'solo'
  if (display.includes('Misery') || display.includes('Misère')) return 'misery'
  return ''
}

// Determine team membership for historical round based on score patterns
const getHistoryTeamClass = (roundIndex: number, playerIdx: number): string => {
  if (!game.value) return ''
  
  const scores = game.value.players.map(p => p.scores[roundIndex] ?? 0)
  const playerScore = scores[playerIdx]
  
  // Find who has matching scores (same team)
  const positive = scores.map((s, i) => ({ s, i })).filter(x => x.s > 0)
  const negative = scores.map((s, i) => ({ s, i })).filter(x => x.s < 0)
  
  // Team contract pattern: 2 positive, 2 negative of same value
  if (positive.length === 2 && negative.length === 2 && 
      positive[0].s === positive[1].s && Math.abs(negative[0].s) === positive[0].s) {
    // This is a team contract
    if (playerScore > 0) return 'team-winner'
    if (playerScore < 0) return 'team-opponent'
  }
  
  // Solo contract pattern: 1 vs 3
  if (positive.length === 1 && negative.length === 3) {
    if (playerScore > 0) return 'solo-winner'
    if (playerScore < 0) return 'solo-opponent'
  }
  if (negative.length === 1 && positive.length === 3) {
    if (playerScore < 0) return 'solo-loser'
    if (playerScore > 0) return 'solo-opponent-win'
  }
  
  return ''
}

// Determine team class for current input row
const getCurrentTeamClass = (playerIdx: number): string => {
  const contract = selectedContracts.value[playerIdx]
  
  if (isTeamContract(contract)) {
    // Check if this player has a team partner
    const partnerCount = getTeamPartnerCount(playerIdx)
    if (partnerCount === 2) return 'current-team-member'
  }
  
  if (isSoloContract(contract)) {
    return 'current-solo-player'
  }
  
  if (isMiseryContract(contract)) {
    return 'current-misery-player'
  }
  
  // Check if this player is an opponent
  if (isOpponentOfContract(playerIdx)) {
    const team = activeTeamContract.value
    if (team) return 'current-team-opponent'
    const solo = activeSoloContract.value
    if (solo) return 'current-solo-opponent'
  }
  
  return ''
}

// Determine team class for score preview
const getPreviewTeamClass = (playerIdx: number): string => {
  const team = activeTeamContract.value
  if (team) {
    return team.players.includes(playerIdx) ? 'preview-team-member' : 'preview-opponent'
  }
  
  const solo = activeSoloContract.value
  if (solo) {
    return solo.player === playerIdx ? 'preview-solo-player' : 'preview-opponent'
  }
  
  // Check if this player has a misery contract
  const contract = selectedContracts.value[playerIdx]
  if (isMiseryContract(contract)) {
    return 'preview-misery-player'
  }
  
  return ''
}

// Calculate preview scores based on current selections
const previewScores = computed((): number[] => {
  if (!game.value || !hasActiveContract.value) return []
  
  try {
    previewError.value = ''
    return calculatePreviewScores()
  } catch (e: any) {
    previewError.value = e.message || ''
    return []
  }
})

const calculatePreviewScores = (): number[] => {
  if (!game.value) return []
  
  const scores = new Array(game.value.players.length).fill(0)
  const contracts = selectedContracts.value
  const trickValues = tricks.value
  
  // Find primary contract
  let primaryContract: ContractKey | null = null
  const primaryPlayers: number[] = []
  const miseryPlayers: number[] = []
  
  contracts.forEach((key, idx) => {
    if (key === 'none') return
    if (isMiseryContract(key)) {
      miseryPlayers.push(idx)
      return
    }
    if (!primaryContract) {
      primaryContract = key
    }
    if (key === primaryContract) {
      primaryPlayers.push(idx)
    }
  })
  
  // Handle team contracts
  if (primaryContract && isTeamContract(primaryContract) && primaryPlayers.length === 2) {
    const trickCount = trickValues[primaryPlayers[0]]
    if (typeof trickCount !== 'number') return []
    
    const value = computeTeamValue(primaryContract, trickCount)
    primaryPlayers.forEach(idx => scores[idx] = value)
    contracts.forEach((_, idx) => {
      if (!primaryPlayers.includes(idx)) scores[idx] = -value
    })
  }
  
  // Handle solo contracts
  if (primaryContract && isSoloContract(primaryContract) && primaryPlayers.length === 1) {
    const playerIdx = primaryPlayers[0]
    const trickCount = trickValues[playerIdx]
    if (typeof trickCount !== 'number') return []
    
    const baseValue = computeSoloValue(primaryContract, trickCount)
    scores[playerIdx] = baseValue * 3
    contracts.forEach((_, idx) => {
      if (idx !== playerIdx) scores[idx] = -baseValue
    })
  }
  
  // Handle misery contracts
  miseryPlayers.forEach(playerIdx => {
    const contract = contracts[playerIdx]
    const trickCount = trickValues[playerIdx]
    if (typeof trickCount !== 'number') return
    
    const value = getMiseryValue(contract)
    const success = trickCount === 0
    
    if (success) {
      scores[playerIdx] += value * 3
      contracts.forEach((_, idx) => {
        if (idx !== playerIdx) scores[idx] -= value
      })
    } else {
      scores[playerIdx] -= value * 3
      contracts.forEach((_, idx) => {
        if (idx !== playerIdx) scores[idx] += value
      })
    }
  })
  
  return scores
}

const computeTeamValue = (contract: ContractKey, tricks: number): number => {
  if (tricks === 13) return 30
  
  const targetMap: Record<string, number> = {
    accept_8: 8, accept_9: 9, accept_10: 10, accept_11: 11, accept_12: 12, accept_13: 13, trull: 8
  }
  const target = targetMap[contract] || 8
  
  if (contract === 'trull') {
    return tricks >= 8 ? 16 : -16
  }
  
  if (tricks >= target) {
    const base = target === 13 ? 30 : 8 + 3 * (target - 8)
    return base + 3 * (tricks - target)
  }
  
  const diff = target - tricks
  const basePenalty = 11 + 3 * (target - 8)
  return -(basePenalty + 3 * (diff - 1))
}

const computeSoloValue = (contract: ContractKey, tricks: number): number => {
  if (contract === 'piccolo') return tricks === 1 ? 8 : -8
  if (contract === 'grand_slam') return tricks === 13 ? 60 : -60
  if (contract === 'slam') return tricks === 13 ? 30 : -30
  
  if (contract.startsWith('abondance_')) {
    const target = parseInt(contract.split('_')[1])
    if (tricks < target) {
      return target === 9 ? -10 : target === 10 ? -15 : target === 11 ? -20 : -30
    }
    const values = target === 9 ? [10, 15, 20, 30] : target === 10 ? [15, 20, 30] : target === 11 ? [20, 30] : [30]
    return values[Math.min(tricks - target, values.length - 1)]
  }
  
  if (contract.startsWith('solo_')) {
    const target = parseInt(contract.split('_')[1])
    if (tricks >= target) {
      if (target === 8) return 7
      const capped = Math.min(tricks, 8)
      return 3 + (target - 5) + (capped - target)
    }
    const basePenalty = target === 8 ? 8 : target - 1
    return -(basePenalty + (target - tricks - 1))
  }
  
  return 0
}

const getMiseryValue = (contract: ContractKey): number => {
  if (contract === 'small_misery') return 6
  if (contract === 'large_misery') return 12
  if (contract === 'open_misery') return 24
  return 0
}

// Pass round (all players pass)
const passRound = async () => {
  if (!game.value) return
  
  // For now, we just skip - in future we could track pass rounds for doubling
  scoreError.value = ''
  resetSelections(game.value.players.length)
}

const canSubmit = computed(() => {
  if (!hasActiveContract.value) return false
  return selectedContracts.value.every((key, idx) => {
    if (!requiresTricks(key)) return true
    const value = tricks.value[idx]
    return typeof value === 'number' && value >= 0 && value <= 13
  })
})

const resetSelections = (length: number) => {
  selectedContracts.value = new Array(length).fill('none')
  tricks.value = new Array(length).fill(null)
}

const loadGame = async () => {
  try {
    loading.value = true
    error.value = ''
    const data = await gamesApi.getGame(gameId)
    if (data.players.length !== 4) {
      throw new Error(t('whistGame.errors.playerCount') as string)
    }
    game.value = data
    resetSelections(data.players.length)
  } catch (err: any) {
    error.value = err.response?.data?.error || err.message || 'Failed to load game'
  } finally {
    loading.value = false
  }
}

const getContractLabel = (key: ContractKey) => {
  const option = contractOptions.find(option => option.key === key)
  return option ? (t(option.labelKey) as string) : key
}

const ensureTrickValue = (playerIndex: number, contractKey: ContractKey): number => {
  const raw = tricks.value[playerIndex]
  if (raw === null || Number.isNaN(raw)) {
    throw new Error(t('whistGame.errors.tricksRequired', {
      player: game.value!.players[playerIndex].name,
      contract: getContractLabel(contractKey),
    }) as string)
  }
  if (raw < 0 || raw > 13) {
    throw new Error(t('whistGame.errors.tricksRange', {
      player: game.value!.players[playerIndex].name,
    }) as string)
  }
  return Math.round(raw)
}

const ensureTeamTricks = (participants: number[], contractKey: ContractKey) => {
  const base = ensureTrickValue(participants[0], contractKey)
  for (let i = 1; i < participants.length; i++) {
    const value = ensureTrickValue(participants[i], contractKey)
    if (value !== base) {
      throw new Error(t('whistGame.errors.teamSync', {
        contract: getContractLabel(contractKey),
      }) as string)
    }
  }
  return base
}
const buildWhistRound = (): WhistRoundRequest => {
  if (!game.value) {
    throw new Error(t('whistGame.errors.missingGame') as string)
  }

  const selections: WhistSelectionPayload[] = []
  const primaryGroups = new Map<ContractKey, number[]>()
  const miseryPlayers: number[] = []

  selectedContracts.value.forEach((key, idx) => {
    if (key === 'none') return
    if (isMiseryContract(key)) {
      miseryPlayers.push(idx)
      return
    }
    if (!primaryGroups.has(key)) {
      primaryGroups.set(key, [])
    }
    primaryGroups.get(key)!.push(idx)
  })

  if (primaryGroups.size > 1) {
    throw new Error(t('whistGame.errors.multiplePrimary') as string)
  }

  const primaryEntry = primaryGroups.entries().next().value as [ContractKey, number[]] | undefined

  if (primaryEntry && miseryPlayers.length > 0) {
    throw new Error(t('whistGame.errors.miseryConflict') as string)
  }

  if (!primaryEntry && miseryPlayers.length === 0) {
    throw new Error(t('whistGame.errors.missingContract') as string)
  }

  if (primaryEntry) {
    const [key, participants] = primaryEntry
    const contract = key as WhistContract

    if (isTeamContract(key)) {
      if (participants.length !== 2) {
        throw new Error(t('whistGame.errors.teamCount') as string)
      }
      const teamTricks = ensureTeamTricks(participants, key)
      participants.forEach(idx => {
        selections.push({
          player_index: idx,
          contract,
          tricks: teamTricks,
        })
      })
    } else if (isSoloContract(key)) {
      if (participants.length !== 1) {
        throw new Error(t('whistGame.errors.soloCount') as string)
      }
      const playerIndex = participants[0]
      const trickCount = ensureTrickValue(playerIndex, key)
      selections.push({
        player_index: playerIndex,
        contract,
        tricks: trickCount,
      })
    } else {
      throw new Error(t('whistGame.errors.unsupportedContract') as string)
    }
  }

  miseryPlayers.forEach(playerIndex => {
    const key = selectedContracts.value[playerIndex]
    const contract = key as WhistContract
    const trickCount = ensureTrickValue(playerIndex, key)
    selections.push({
      player_index: playerIndex,
      contract,
      tricks: trickCount,
    })
  })

  return { selections }
}

const submitRound = async () => {
  if (!game.value || submittingRound.value) return

  scoreError.value = ''

  let whistRound: WhistRoundRequest
  try {
    whistRound = buildWhistRound()
  } catch (err: any) {
    scoreError.value = err?.message || (t('whistGame.errors.validation') as string)
    return
  }

  try {
    submittingRound.value = true
    const updated = await gamesApi.addRound(gameId, { whistRound })
    game.value = updated
    resetSelections(updated.players.length)
  } catch (err: any) {
    scoreError.value = err.response?.data?.error || (t('whistGame.errors.submitFailed') as string)
  } finally {
    submittingRound.value = false
  }
}

const goHome = () => {
  const locale = route.params.locale || 'en'
  router.push(`/${locale}`)
}

const syncTeamSelections = () => {
  if (!tricks.value.length) return
  TEAM_CONTRACTS.forEach(contractKey => {
    const indices = selectedContracts.value
      .map((key, idx) => (key === contractKey ? idx : -1))
      .filter(idx => idx !== -1)
    if (indices.length === 2) {
      const base = tricks.value[indices[0]]
      indices.slice(1).forEach(idx => {
        if (tricks.value[idx] !== base) {
          tricks.value[idx] = base
        }
      })
    }
  })
}

watch(selectedContracts, (newVal) => {
  if (!tricks.value.length || !game.value) return
  newVal.forEach((key, idx) => {
    if (key === 'none') {
      tricks.value[idx] = null
    }
  })
  syncTeamSelections()
}, { deep: true })

watch(tricks, (newVal, oldVal) => {
  if (!oldVal) return
  newVal.forEach((value, idx) => {
    if (oldVal[idx] === value) return
    const key = selectedContracts.value[idx]
    if (isTeamContract(key)) {
      selectedContracts.value.forEach((otherKey, otherIdx) => {
        if (otherIdx !== idx && otherKey === key && tricks.value[otherIdx] !== value) {
          tricks.value[otherIdx] = value
        }
      })
    }
  })
}, { deep: true })

onMounted(() => {
  loadGame()
})
</script>

<style scoped>
@import '../styles/shared.css';
@import '../styles/game.css';

.whist-input {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.whist-input select,
.whist-input input {
  width: 100%;
  padding: 8px;
  border-radius: 8px;
  border: 1.5px solid rgba(45, 31, 26, 0.2);
  font-size: 0.9rem;
}

.whist-input select.team-selected {
  border-color: var(--accent-emerald);
  background-color: rgba(47, 92, 76, 0.08);
}

.whist-input select.solo-selected {
  border-color: #e67e22;
  background-color: rgba(230, 126, 34, 0.08);
}

.whist-input select.misery-selected {
  border-color: #9b59b6;
  background-color: rgba(155, 89, 182, 0.08);
}

.whist-input input.synced {
  border-color: var(--accent-emerald);
}

.whist-input input.auto-calculated {
  background: rgba(47, 92, 76, 0.08);
  color: var(--ink-secondary);
  text-align: center;
  font-weight: 500;
}

.whist-input select.opponent-auto {
  opacity: 0.6;
}

/* Score Preview */
.score-preview {
  margin-bottom: 16px;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.9), rgba(250, 248, 245, 0.9));
  border: 1px solid rgba(47, 92, 76, 0.2);
}

.preview-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
  color: var(--ink);
}

.preview-icon {
  font-size: 1.2rem;
}

.preview-scores {
  display: flex;
  gap: 16px;
  flex-wrap: wrap;
}

.preview-score {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 8px 16px;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(45, 31, 26, 0.1);
}

.preview-score.positive {
  background: rgba(46, 204, 113, 0.15);
  border-color: rgba(46, 204, 113, 0.3);
}

.preview-score.negative {
  background: rgba(231, 76, 60, 0.15);
  border-color: rgba(231, 76, 60, 0.3);
}

.preview-player {
  font-size: 0.8rem;
  color: var(--ink-secondary);
}

.preview-value {
  font-size: 1.2rem;
  font-weight: 700;
}

.preview-score.positive .preview-value {
  color: #27ae60;
}

.preview-score.negative .preview-value {
  color: #c0392b;
}

/* Preview team coloring */
.preview-score.preview-team-member {
  border: 2px solid var(--accent-emerald);
}

.preview-score.preview-solo-player {
  border: 2px solid #e67e22;
}

.preview-score.preview-misery-player {
  border: 2px solid #9b59b6;
}

.preview-score.preview-opponent {
  opacity: 0.8;
}

.preview-error {
  margin-top: 8px;
  font-size: 0.85rem;
  color: var(--ink-muted);
  font-style: italic;
}

/* Table styling */
.round-col {
  width: 60px;
}

.contract-col {
  width: 100px;
}

.player-col {
  min-width: 100px;
}

.score-cell {
  text-align: center;
  font-weight: 500;
}

.score-cell.positive {
  color: #27ae60;
}

.score-cell.negative {
  color: #c0392b;
}

.totals-row {
  background: rgba(201, 156, 93, 0.1);
  border-top: 2px solid rgba(45, 31, 26, 0.15);
}

.total-cell {
  font-size: 1.1rem;
}

.input-row {
  background: rgba(255, 255, 255, 0.6);
}

.input-cell {
  padding: 12px 8px;
}

/* Team coloring for history rows */
.score-cell.team-winner {
  background: linear-gradient(135deg, rgba(47, 92, 76, 0.12), rgba(47, 92, 76, 0.08));
  border-left: 3px solid var(--accent-emerald);
}

.score-cell.team-opponent {
  background: linear-gradient(135deg, rgba(192, 57, 43, 0.08), rgba(192, 57, 43, 0.04));
  border-left: 3px solid rgba(192, 57, 43, 0.4);
}

.score-cell.solo-winner {
  background: linear-gradient(135deg, rgba(230, 126, 34, 0.15), rgba(230, 126, 34, 0.08));
  border-left: 3px solid #e67e22;
}

.score-cell.solo-opponent {
  background: linear-gradient(135deg, rgba(52, 73, 94, 0.08), rgba(52, 73, 94, 0.04));
  border-left: 3px solid rgba(52, 73, 94, 0.3);
}

.score-cell.solo-loser {
  background: linear-gradient(135deg, rgba(230, 126, 34, 0.08), rgba(192, 57, 43, 0.08));
  border-left: 3px solid #c0392b;
}

.score-cell.solo-opponent-win {
  background: linear-gradient(135deg, rgba(46, 204, 113, 0.08), rgba(52, 73, 94, 0.04));
  border-left: 3px solid rgba(46, 204, 113, 0.4);
}

/* Team coloring for current input row */
.input-cell.current-team-member {
  background: linear-gradient(135deg, rgba(47, 92, 76, 0.15), rgba(47, 92, 76, 0.08));
  border-left: 4px solid var(--accent-emerald);
  box-shadow: inset 0 0 8px rgba(47, 92, 76, 0.1);
}

.input-cell.current-team-opponent {
  background: linear-gradient(135deg, rgba(52, 73, 94, 0.06), rgba(52, 73, 94, 0.02));
  border-left: 4px solid rgba(52, 73, 94, 0.25);
}

.input-cell.current-solo-player {
  background: linear-gradient(135deg, rgba(230, 126, 34, 0.15), rgba(230, 126, 34, 0.08));
  border-left: 4px solid #e67e22;
  box-shadow: inset 0 0 8px rgba(230, 126, 34, 0.1);
}

.input-cell.current-solo-opponent {
  background: linear-gradient(135deg, rgba(52, 73, 94, 0.06), rgba(52, 73, 94, 0.02));
  border-left: 4px solid rgba(52, 73, 94, 0.25);
}

.input-cell.current-misery-player {
  background: linear-gradient(135deg, rgba(155, 89, 182, 0.15), rgba(155, 89, 182, 0.08));
  border-left: 4px solid #9b59b6;
  box-shadow: inset 0 0 8px rgba(155, 89, 182, 0.1);
}

/* Contract badge */
.contract-badge {
  display: inline-block;
  padding: 4px 8px;
  border-radius: 6px;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

.contract-badge.team {
  background: rgba(47, 92, 76, 0.15);
  color: var(--accent-emerald);
}

.contract-badge.solo {
  background: rgba(230, 126, 34, 0.15);
  color: #d35400;
}

.contract-badge.misery {
  background: rgba(155, 89, 182, 0.15);
  color: #8e44ad;
}

/* Contract Legend */
.contract-legend {
  margin-top: 16px;
  padding: 16px;
}

.contract-legend h3 {
  margin: 0 0 12px;
  font-size: 1rem;
  color: var(--ink);
}

.legend-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 16px;
}

.legend-section {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.legend-title {
  font-size: 0.85rem;
  font-weight: 600;
  margin: 0;
  padding: 4px 8px;
  border-radius: 4px;
  display: inline-block;
  width: fit-content;
}

.legend-title.team {
  background: rgba(47, 92, 76, 0.15);
  color: var(--accent-emerald);
}

.legend-title.solo {
  background: rgba(230, 126, 34, 0.15);
  color: #d35400;
}

.legend-title.misery {
  background: rgba(155, 89, 182, 0.15);
  color: #8e44ad;
}

.legend-items {
  font-size: 0.8rem;
  color: var(--ink-secondary);
  padding-left: 8px;
}

.table-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  flex-wrap: wrap;
}

.table-actions .error-message {
  margin: 0;
}

@media (max-width: 768px) {
  .preview-scores {
    flex-direction: column;
  }
  
  .preview-score {
    flex-direction: row;
    justify-content: space-between;
    width: 100%;
  }
  
  .legend-grid {
    grid-template-columns: 1fr;
  }
  
  .contract-col {
    display: none;
  }
}
</style>
