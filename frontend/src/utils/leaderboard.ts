import type {
  GameType,
  LeaderboardResponse,
  LeaderboardPlayerEntry,
  LeaderboardGameStat
} from '../types'

export type LeaderboardBoardKey = 'overall' | GameType

export interface LeaderboardRow {
  playerName: string
  wins: number
  games: number
  winRate: number
  averageScore: number | null
  highestScore: number | null
  lowestScore: number | null
  elo?: number
}

export const sortLeaderboardRows = (rows: LeaderboardRow[]): LeaderboardRow[] =>
  [...rows].sort((a, b) => {
    if (b.wins !== a.wins) return b.wins - a.wins
    if (b.winRate !== a.winRate) return b.winRate - a.winRate
    const avgA = a.averageScore ?? -Infinity
    const avgB = b.averageScore ?? -Infinity
    if (avgB !== avgA) return avgB - avgA
    if (b.games !== a.games) return b.games - a.games
    return a.playerName.localeCompare(b.playerName, undefined, { sensitivity: 'base' })
  })

export const leaderboardBoardOrder: LeaderboardBoardKey[] = [
  'overall',
  'hearts',
  'king',
  'double_king',
  'color_whist',
  'whist',
  'manille'
]

export const leaderboardBoardMeta: Record<LeaderboardBoardKey, { labelKey: string; descriptionKey: string }> = {
  overall: {
    labelKey: 'leaderboard.boards.overall',
    descriptionKey: 'leaderboard.boardDescriptions.overall'
  },
  hearts: {
    labelKey: 'leaderboard.boards.hearts',
    descriptionKey: 'leaderboard.boardDescriptions.hearts'
  },
  king: {
    labelKey: 'leaderboard.boards.king',
    descriptionKey: 'leaderboard.boardDescriptions.king'
  },
  double_king: {
    labelKey: 'leaderboard.boards.doubleKing',
    descriptionKey: 'leaderboard.boardDescriptions.doubleKing'
  },
  color_whist: {
    labelKey: 'leaderboard.boards.colorWhist',
    descriptionKey: 'leaderboard.boardDescriptions.colorWhist'
  },
  whist: {
    labelKey: 'leaderboard.boards.whist',
    descriptionKey: 'leaderboard.boardDescriptions.whist'
  },
  manille: {
    labelKey: 'leaderboard.boards.manille',
    descriptionKey: 'leaderboard.boardDescriptions.manille'
  }
}

export type LeaderboardRowsMap = Record<LeaderboardBoardKey, LeaderboardRow[]>

const emptyRowsMap = (): LeaderboardRowsMap => ({
  overall: [],
  hearts: [],
  king: [],
  double_king: [],
  color_whist: [],
  whist: [],
  manille: []
})

export const buildLeaderboardRows = (
  leaderboard: LeaderboardResponse | null
): LeaderboardRowsMap => {
  if (!leaderboard) {
    return emptyRowsMap()
  }

  const rows = emptyRowsMap()

  leaderboard.players.forEach((player) => {
    const totalGames = player.stats.reduce((sum, stat) => sum + stat.games_played, 0)
    if (totalGames > 0) {
      const totalWins = player.total_wins
      rows.overall.push({
        playerName: player.player_name,
        wins: totalWins,
        games: totalGames,
        winRate: totalGames > 0 ? totalWins / totalGames : 0,
        averageScore: null,
        highestScore: null,
        lowestScore: null
      })
    }

    player.stats.forEach((stat) => {
      if (stat.games_played > 0) {
        rows[stat.game_type].push(mapStatToRow(player, stat))
      }
    })
  })

  return rows
}

const mapStatToRow = (
  player: LeaderboardPlayerEntry,
  stat: LeaderboardGameStat
): LeaderboardRow => ({
  playerName: player.player_name,
  wins: stat.wins,
  games: stat.games_played,
  winRate: stat.win_rate,
  averageScore: stat.average_score,
  highestScore: stat.highest_score,
  lowestScore: stat.lowest_score
})
