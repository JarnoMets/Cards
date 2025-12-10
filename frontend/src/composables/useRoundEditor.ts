import { ref, type Ref } from 'vue'
import { gamesApi } from '@/api/games'
import type { Game, AddRoundRequest } from '@/types'

export interface RoundEditorConfig {
  /** Get current scores for a round */
  getScoresForRound: (game: Game, roundIndex: number) => number[]
  /** Validate the edited scores (return error message or empty string) */
  validateScores?: (scores: number[], game: Game, roundIndex: number) => string
  /** Build the request object from scores (defaults to { scores }) */
  buildRequest?: (scores: number[], game: Game, roundIndex: number) => AddRoundRequest
  /** Input min value */
  inputMin?: number
  /** Input max value */
  inputMax?: number
}

/**
 * Composable for managing round editing modal and logic
 */
export const useRoundEditor = (
  game: Ref<Game | null>,
  gameId: string,
  config: RoundEditorConfig,
  onSuccess?: () => void
) => {
  const editingRoundIndex = ref<number | null>(null)
  const editingRoundScores = ref<number[]>([])
  const editingRoundError = ref('')
  const savingRound = ref(false)

  const openRoundEditor = (roundIndex: number) => {
    if (!game.value) return
    editingRoundIndex.value = roundIndex
    editingRoundScores.value = config.getScoresForRound(game.value, roundIndex)
    editingRoundError.value = ''
  }

  const cancelRoundEdit = () => {
    editingRoundIndex.value = null
    editingRoundScores.value = []
    editingRoundError.value = ''
  }

  const saveRoundEdit = async (newScores?: number[]) => {
    if (editingRoundIndex.value === null || !game.value) return
    
    const scores = newScores ?? editingRoundScores.value
    
    // Validate if validator provided
    if (config.validateScores) {
      const error = config.validateScores(scores, game.value, editingRoundIndex.value)
      if (error) {
        editingRoundError.value = error
        return
      }
    }

    savingRound.value = true
    editingRoundError.value = ''

    try {
      const request = config.buildRequest 
        ? config.buildRequest(scores, game.value, editingRoundIndex.value)
        : { scores }
      const updated = await gamesApi.updateRound(gameId, editingRoundIndex.value, request)
      game.value = updated
      cancelRoundEdit()
      onSuccess?.()
    } catch (err: any) {
      editingRoundError.value = err.response?.data?.error || err.message || 'Failed to update round'
    } finally {
      savingRound.value = false
    }
  }

  return {
    editingRoundIndex,
    editingRoundScores,
    editingRoundError,
    savingRound,
    openRoundEditor,
    cancelRoundEdit,
    saveRoundEdit,
    inputMin: config.inputMin ?? 0,
    inputMax: config.inputMax ?? 100,
  }
}
