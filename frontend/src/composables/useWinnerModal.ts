import { ref, computed, watch, type Ref } from 'vue'
import { gamesApi } from '@/api/games'
import type { Game, GameEloChange } from '@/types'

export interface WinnerConfig {
  /** Function to determine if game is complete and who won */
  getWinners: (game: Game) => string[] | null
}

/**
 * Composable for managing winner detection and winner modal display
 */
export const useWinnerModal = (game: Ref<Game | null>, config: WinnerConfig) => {
  const showWinnerModal = ref(false)
  const eloChanges = ref<GameEloChange[]>([])

  const winners = computed(() => {
    if (!game.value) return null
    return config.getWinners(game.value)
  })

  // Watch for winner changes and fetch ELO changes
  watch(winners, async (val) => {
    if (val && val.length > 0 && game.value) {
      showWinnerModal.value = true
      try {
        eloChanges.value = await gamesApi.getGameEloChanges(game.value.id)
      } catch (e) {
        console.error('Failed to fetch ELO changes:', e)
        eloChanges.value = []
      }
    }
  })

  const closeWinnerModal = () => {
    showWinnerModal.value = false
  }

  return {
    showWinnerModal,
    eloChanges,
    winners,
    closeWinnerModal,
  }
}

// Common win condition helpers
export const winConditions = {
  /** Hearts: First to 100, lowest score wins */
  hearts: (game: Game): string[] | null => {
    const hasReached100 = game.players.some(p => (p.total_score ?? 0) >= 100)
    if (!hasReached100) return null
    
    const minScore = Math.min(...game.players.map(p => p.total_score ?? 0))
    return game.players
      .filter(p => (p.total_score ?? 0) === minScore)
      .map(p => p.name)
  },

  /** King/Double King: After all rounds, highest score wins */
  king: (totalRounds: number) => (game: Game): string[] | null => {
    if (game.current_round < totalRounds) return null
    
    const maxScore = Math.max(...game.players.map(p => p.total_score ?? 0))
    return game.players
      .filter(p => (p.total_score ?? 0) === maxScore)
      .map(p => p.name)
  },

  /** Team game: First team to target score wins */
  teamGame: (targetScore: number) => (game: Game): string[] | null => {
    // Assuming teams are [0,1] and [2,3]
    const team1Score = (game.players[0]?.total_score ?? 0) + (game.players[1]?.total_score ?? 0)
    const team2Score = (game.players[2]?.total_score ?? 0) + (game.players[3]?.total_score ?? 0)
    
    if (team1Score >= targetScore) {
      return [game.players[0].name, game.players[1].name]
    }
    if (team2Score >= targetScore) {
      return [game.players[2].name, game.players[3].name]
    }
    return null
  },
}
