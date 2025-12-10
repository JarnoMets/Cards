import type { Game } from '../types'

/**
 * Truncate display names to 12 characters (11 + dot) for UI-only display
 */
export const displayName = (name: string): string => {
  if (!name) return ''
  return name.length > 12 ? `${name.slice(0, 11)}.` : name
}

/**
 * Get the score delta (points added) for a player in a specific round
 */
export const getRoundDelta = (game: Game | null, playerIndex: number, roundIndex: number): number | null => {
  if (!game) return null
  const player = game.players[playerIndex]
  if (!player || !player.scores || roundIndex < 0 || roundIndex >= player.scores.length) {
    return null
  }
  return player.scores[roundIndex]
}

/**
 * Get the cumulative total score after a specific round
 * Optionally applies 100-point resets (Hearts-specific behavior)
 */
export const getTotalAfterRound = (game: Game | null, playerIndex: number, roundIndex: number, resetAt100: boolean = false): number | null => {
  if (!game) return null
  const player = game.players[playerIndex]
  if (!player || !player.scores || roundIndex < 0 || roundIndex >= player.scores.length) {
    return null
  }

  let total = 0
  for (let i = 0; i <= roundIndex; i++) {
    total += player.scores[i]
    if (resetAt100 && total === 100) {
      total = 0
    }
  }

  return total
}

/**
 * Check if a player has the lowest score in a specific round (for Hearts)
 */
export const isLowestInRound = (game: Game | null, playerIndex: number, roundIndex: number): boolean => {
  if (!game) return false

  const scores = game.players
    .map((_, idx) => ({ idx, total: getTotalAfterRound(game, idx, roundIndex) }))
    .filter(s => s.total !== null)

  if (scores.length === 0) return false

  const minScore = Math.min(...scores.map(s => s.total!))
  const playerScore = getTotalAfterRound(game, playerIndex, roundIndex)

  return playerScore !== null && playerScore === minScore
}

/**
 * Check if a player has the highest score in a specific round (for King/Double King)
 */
export const isHighestInRound = (game: Game | null, playerIndex: number, roundIndex: number): boolean => {
  if (!game) return false

  const scores = game.players
    .map((_, idx) => ({ idx, total: getTotalAfterRound(game, idx, roundIndex) }))
    .filter(s => s.total !== null)

  if (scores.length === 0) return false

  const maxScore = Math.max(...scores.map(s => s.total!))
  const playerScore = getTotalAfterRound(game, playerIndex, roundIndex)

  return playerScore !== null && playerScore === maxScore
}
