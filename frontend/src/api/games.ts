import axios from 'axios'
import type {
  Game,
  CreateGameRequest,
  AddRoundRequest,
  LeaderboardResponse,
  PlayerDetailResponse,
  PlayerSearchResult,
  PlayerEloRating,
  EloLeaderboardResponse,
  OverallEloLeaderboardResponse,
  EloHistoryResponse,
  EloComparisonResponse,
  EloConfig,
  GameType,
  GameEloChange,
  PlayerSettings,
  EmailDailyStats
} from '../types'

const API_BASE_URL = import.meta.env.VITE_API_URL || '/api'

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json'
  }
})

export const gamesApi = {
  async getGames(): Promise<Game[]> {
    const response = await api.get('/games')
    return response.data
  },

  async getGame(id: string): Promise<Game> {
    const response = await api.get(`/games/${id}`)
    return response.data
  },

  async createGame(request: CreateGameRequest): Promise<Game> {
    const response = await api.post('/games', request)
    return response.data
  },

  async deleteGame(id: string): Promise<void> {
    await api.delete(`/games/${id}`)
  },

  async getGameEloChanges(id: string): Promise<GameEloChange[]> {
    const response = await api.get(`/games/${id}/elo-changes`)
    return response.data
  },

  async addRound(id: string, request: AddRoundRequest): Promise<Game> {
    const response = await api.post(`/games/${id}/round`, request)
    return response.data
  },

  async updateRound(id: string, roundIndex: number, request: AddRoundRequest): Promise<Game> {
    const response = await api.patch(`/games/${id}/round/${roundIndex}`, request)
    return response.data
  },

  async updatePlayer(id: string, playerId: string, name: string): Promise<Game> {
    const response = await api.patch(`/games/${id}/players/${playerId}`, { name })
    return response.data
  },

  async healthCheck(): Promise<{ status: string; service: string }> {
    const response = await api.get('/health')
    return response.data
  },

  async getLeaderboard(): Promise<LeaderboardResponse> {
    const response = await api.get('/leaderboard')
    return response.data
  },

  async searchPlayers(query: string, limit = 12): Promise<PlayerSearchResult[]> {
    const params = new URLSearchParams()
    params.set('limit', String(limit))
    if (query.trim().length > 0) {
      params.set('q', query.trim())
    }
    const response = await api.get(`/players/search?${params.toString()}`)
    return response.data
  },

  async getPlayerProfile(name: string): Promise<PlayerDetailResponse> {
    const response = await api.get(`/players/${encodeURIComponent(name)}`)
    return response.data
  },

  async getPlayerGames(name: string): Promise<Game[]> {
    const response = await api.get(`/players/${encodeURIComponent(name)}/games`)
    return response.data
  },

  // Admin endpoints
  async renamePlayerGlobally(oldName: string, newName: string): Promise<{ message: string; games_updated: number }> {
    const response = await api.post('/admin/rename-player', { old_name: oldName, new_name: newName })
    return response.data
  },

  async sendTestEmail(): Promise<{ message: string }> {
    const response = await api.post('/admin/test-email')
    return response.data
  },

  async getEmailStatus(): Promise<{ enabled: boolean }> {
    const response = await api.get('/admin/email-status')
    return response.data
  },

  // ELO endpoints
  async getPlayerElo(name: string): Promise<PlayerEloRating[]> {
    const response = await api.get(`/players/${encodeURIComponent(name)}/elo`)
    return response.data
  },

  async getEloLeaderboard(gameType?: GameType): Promise<EloLeaderboardResponse> {
    const params = gameType ? `?game_type=${gameType}` : ''
    const response = await api.get(`/elo${params}`)
    return response.data
  },

  async getOverallEloLeaderboard(): Promise<OverallEloLeaderboardResponse> {
    const response = await api.get('/elo/overall')
    return response.data
  },

  async getPlayerEloHistory(name: string, gameType?: GameType, timeRange: string = 'all'): Promise<EloHistoryResponse> {
    const params = new URLSearchParams()
    if (gameType) params.set('game_type', gameType)
    params.set('time_range', timeRange)
    const response = await api.get(`/players/${encodeURIComponent(name)}/elo/history?${params.toString()}`)
    return response.data
  },

  async getPlayerEloComparison(name: string, gameType?: GameType): Promise<EloComparisonResponse> {
    const params = gameType ? `?game_type=${gameType}` : ''
    const response = await api.get(`/players/${encodeURIComponent(name)}/elo/comparison${params}`)
    return response.data
  },

  async recalculateElo(): Promise<{ message: string; games_processed: number; players_updated: number }> {
    const response = await api.post('/admin/recalculate-elo')
    return response.data
  },

  async getEloConfig(): Promise<EloConfig> {
    const response = await api.get('/admin/elo-config')
    return response.data
  },

  async setEloConfig(config: EloConfig): Promise<{ message: string }> {
    const response = await api.put('/admin/elo-config', config)
    return response.data
  },

  // Player settings endpoints
  async getPlayerSettings(name: string): Promise<PlayerSettings> {
    const response = await api.get(`/players/${encodeURIComponent(name)}/settings`)
    return response.data
  },

  async updatePlayerSettings(name: string, email: string | null, gameNotifications: boolean): Promise<PlayerSettings> {
    const response = await api.put(`/players/${encodeURIComponent(name)}/settings`, {
      email,
      game_notifications: gameNotifications
    })
    return response.data
  },

  // Daily email stats for admin
  async getEmailDailyStats(): Promise<EmailDailyStats> {
    const response = await api.get('/admin/email-daily-stats')
    return response.data
  }
}
