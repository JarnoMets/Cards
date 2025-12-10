export interface Player {
  id: string
  name: string
  scores: number[]
  game_indices: number[]
  total_score: number
}

export type GameType = 'hearts' | 'king' | 'double_king' | 'color_whist' | 'whist' | 'manille' | 'press'

export interface Game {
  id: string
  game_type: GameType
  players: Player[]
  created_at: string
  current_round: number
}

export interface CreateGameRequest {
  game_type: GameType
  players: string[]
}

export type ColorWhistContract =
  | 'accept_8'
  | 'accept_9'
  | 'accept_10'
  | 'accept_11'
  | 'accept_12'
  | 'accept_13'
  | 'solo_5'
  | 'solo_6'
  | 'solo_7'
  | 'solo_8'
  | 'abondance_9'
  | 'abondance_10'
  | 'abondance_11'
  | 'abondance_12'
  | 'trull'
  | 'piccolo'
  | 'small_misery'
  | 'large_misery'
  | 'open_misery'
  | 'grand_slam'

export interface ColorWhistSelectionPayload {
  player_index: number
  contract: ColorWhistContract
  tricks: number
}

export interface ColorWhistRoundRequest {
  selections: ColorWhistSelectionPayload[]
}

export interface AddRoundRequest {
  scores?: number[]
  game_index?: number  // For Double King games, indicates which game was selected
  colorWhistRound?: ColorWhistRoundRequest
  whistRound?: WhistRoundRequest
  manilleRound?: ManilleRoundRequest
  pressRound?: PressRoundRequest
}

export type WhistContract =
  | 'accept_8'
  | 'accept_9'
  | 'accept_10'
  | 'accept_11'
  | 'accept_12'
  | 'accept_13'
  | 'solo_5'
  | 'solo_6'
  | 'solo_7'
  | 'solo_8'
  | 'abondance_9'
  | 'abondance_10'
  | 'abondance_11'
  | 'abondance_12'
  | 'trull'
  | 'piccolo'
  | 'small_misery'
  | 'large_misery'
  | 'open_misery'
  | 'grand_slam'
  | 'slam'

export interface WhistSelectionPayload {
  player_index: number
  contract: WhistContract
  tricks: number
}

export interface WhistRoundRequest {
  selections: WhistSelectionPayload[]
}

// Manille types
export type ManilleTrump = 'hearts' | 'diamonds' | 'clubs' | 'spades' | 'no_trump'

export interface ManilleRoundRequest {
  team1_points: number
  trump: ManilleTrump
  multiplier: 1 | 2 | 4
}

// Press (Pressen/Zetten) types
export type PressTrump = 'hearts' | 'diamonds' | 'clubs' | 'spades'

export interface PressRoundRequest {
  bid: number           // 1-8 tricks
  bidding_team: 1 | 2   // Which team made the bid
  tricks_won: number    // 0-8 tricks won by bidding team
  trump: PressTrump
}

export interface LeaderboardGameStat {
  game_type: GameType
  wins: number
  games_played: number
  average_score: number
  win_rate: number
  highest_score: number | null
  lowest_score: number | null
}

export interface LeaderboardPlayerEntry {
  player_name: string
  total_wins: number
  stats: LeaderboardGameStat[]
}

export interface LeaderboardRecordEntry {
  game_type: GameType
  player_name: string
  score: number
  game_id: string
  occurred_at: string
}

export interface LeaderboardRecords {
  king_highest: LeaderboardRecordEntry[]
  king_lowest: LeaderboardRecordEntry[]
  double_king_highest: LeaderboardRecordEntry[]
  double_king_lowest: LeaderboardRecordEntry[]
}

export interface LeaderboardResponse {
  generated_at: string
  players: LeaderboardPlayerEntry[]
  records: LeaderboardRecords
}

export interface PlayerDetailGameStat {
  game_type: GameType
  games_played: number
  finished_games: number
  unfinished_games: number
  total_rounds: number
  wins: number
  win_rate: number
  average_score: number
  highest_score: number | null
  lowest_score: number | null
  total_points: number
}

export interface PlayerScorePoint {
  game_id: string
  game_type: GameType
  occurred_at: string
  total_score: number
  placement: number
  score_margin: number
  did_win: boolean
}

export interface GameParticipantSummary {
  name: string
  total_score: number
  placement: number
  is_target: boolean
}

export interface PlayerGameSummary {
  game_id: string
  game_type: GameType
  occurred_at: string
  player_score: number
  placement: number
  total_players: number
  did_win: boolean
  score_margin: number
  players: GameParticipantSummary[]
  elo_change?: number
}

export interface PlayerDetailResponse {
  player_name: string
  total_games: number
  finished_games: number
  unfinished_games: number
  total_wins: number
  total_points: number
  total_rounds: number
  win_rate: number
  first_played_at: string | null
  last_played_at: string | null
  game_type_stats: PlayerDetailGameStat[]
  score_timeline: PlayerScorePoint[]
  recent_games: PlayerGameSummary[]
}

export interface PlayerSearchResult {
  player_name: string
  games_played: number
  last_played_at: string | null
}

// ELO types
export interface PlayerEloRating {
  player_name: string
  game_type: GameType
  elo: number
  games_played: number
  updated_at: string
}

export interface EloLeaderboardResponse {
  generated_at: string
  ratings: PlayerEloRating[]
}

export interface OverallEloRating {
  player_name: string
  elo: number
  games_played: number
  updated_at: string
}

export interface OverallEloLeaderboardResponse {
  generated_at: string
  ratings: OverallEloRating[]
}

// ELO history types
export interface EloHistoryEntry {
  player_name: string
  game_type: string
  elo: number
  game_id: string
  recorded_at: string
}

export interface EloHistoryResponse {
  player_name: string
  game_type: string | null
  time_range: string
  history: EloHistoryEntry[]
}

export interface EloComparisonResponse {
  player_name: string
  current_elo: number
  elo_7_days_ago: number | null
  elo_change: number | null
}

// ELO changes for a completed game
export interface GameEloChange {
  player_name: string
  elo_before: number
  elo_after: number
  elo_change: number
}

// Skill rating configuration
export interface EloConfig {
  starting_elo: number
  target_rating: number
  k_factor_new: number
  k_factor_mid: number
  k_factor_established: number
  games_until_mid: number
  games_until_established: number
  floor_elo: number
}

// Player settings for email preferences
export interface PlayerSettings {
  player_name: string
  email: string | null
  game_notifications: boolean
}

// Daily email statistics
export interface EmailDailyStats {
  date: string
  emails_sent: number
  daily_limit: number
  remaining: number
}

// Type guards for game types
export const isHeartsGame = (game: Game): boolean => game.game_type === 'hearts'
export const isKingGame = (game: Game): boolean => game.game_type === 'king'
export const isDoubleKingGame = (game: Game): boolean => game.game_type === 'double_king'
export const isColorWhistGame = (game: Game): boolean => game.game_type === 'color_whist'
export const isWhistGame = (game: Game): boolean => game.game_type === 'whist'
export const isManilleGame = (game: Game): boolean => game.game_type === 'manille'
export const isPressGame = (game: Game): boolean => game.game_type === 'press'

// Game type display names
export const GAME_TYPE_NAMES: Record<GameType, string> = {
  hearts: 'Hearts',
  king: 'King',
  double_king: 'Double King',
  color_whist: 'Color Whist',
  whist: 'Whist',
  manille: 'Manille',
  press: 'Press'
}
