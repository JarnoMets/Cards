/**
 * Game configuration
 * Defines min/max players and default settings for each game type
 */

export interface GameConfig {
  minPlayers: number
  maxPlayers: number
  defaultPlayers: number
}

export const GAME_CONFIGS: Record<string, GameConfig> = {
  hearts: {
    minPlayers: 3,
    maxPlayers: 6,
    defaultPlayers: 4,
  },
  king: {
    minPlayers: 3,
    maxPlayers: 4,
    defaultPlayers: 4,
  },
  double_king: {
    minPlayers: 3,
    maxPlayers: 4,
    defaultPlayers: 4,
  },
  color_whist: {
    minPlayers: 4,
    maxPlayers: 4,
    defaultPlayers: 4,
  },
  whist: {
    minPlayers: 4,
    maxPlayers: 4,
    defaultPlayers: 4,
  },
  manille: {
    minPlayers: 4,
    maxPlayers: 4,
    defaultPlayers: 4,
  },
  press: {
    minPlayers: 4,
    maxPlayers: 4,
    defaultPlayers: 4,
  },
}

export const getGameConfig = (gameType: string): GameConfig => {
  return GAME_CONFIGS[gameType] || {
    minPlayers: 2,
    maxPlayers: 6,
    defaultPlayers: 4,
  }
}
