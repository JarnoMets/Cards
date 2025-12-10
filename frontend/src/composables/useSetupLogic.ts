import { ref, computed, watch, onBeforeUnmount } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { gamesApi } from '../api/games'
import { useAuth } from '../stores/auth'
import type { Game, PlayerSearchResult, GameEloChange } from '../types'

// Enriched game with ELO changes
export interface GameWithElo extends Game {
  elo_changes?: GameEloChange[]
}

export interface SetupConfig {
  gameType: 'hearts' | 'king' | 'double_king' | 'color_whist' | 'whist' | 'manille' | 'press'
  minPlayers?: number
  maxPlayers?: number
  defaultPlayers?: number
  pageSize?: number
}

/**
 * Shared composable for game setup pages (Hearts, King, DoubleKing)
 * Handles player management, game loading, pagination, deletion
 */
export const useSetupLogic = (config: SetupConfig) => {
  const router = useRouter()
  const route = useRoute()
  const { t } = useI18n()
  const { isAuthenticated } = useAuth()

  // Default values
  const minPlayers = config.minPlayers ?? 2
  const maxPlayers = config.maxPlayers ?? 4
  const defaultPlayers = config.defaultPlayers ?? 4
  const pageSize = config.pageSize ?? 6

  // State
  const players = ref<string[]>(Array(defaultPlayers).fill(''))
  const error = ref<string>('')
  const games = ref<GameWithElo[]>([])
  const loadingGames = ref(false)
  const page = ref(1)
  const deleteConfirmGameId = ref<string | null>(null)
  const playerPickerVisible = ref(false)
  const playerPickerIndex = ref<number | null>(null)
  const playerPickerSearch = ref('')
  const playerPickerResults = ref<PlayerSearchResult[]>([])
  const playerPickerLoading = ref(false)
  const playerPickerSelected = ref('')
  const recentlySelectedIndex = ref<number | null>(null)
  let pickerSearchTimeout: ReturnType<typeof setTimeout> | null = null
  let recentlySelectedTimeout: ReturnType<typeof setTimeout> | null = null

  // Computed
  const totalPages = computed(() => Math.ceil(games.value.length / pageSize))
  const pagedGames = computed(() => {
    const start = (page.value - 1) * pageSize
    return games.value.slice(start, start + pageSize)
  })

  const canStartGame = computed(() => {
    const filledPlayers = players.value.filter(p => p.trim() !== '')
    return (
      filledPlayers.length >= minPlayers &&
      filledPlayers.length <= maxPlayers &&
      filledPlayers.length === players.value.length
    )
  })

  // Methods
  const validatePlayers = () => {
    error.value = ''
  }

  const addPlayer = () => {
    if (players.value.length < maxPlayers) {
      players.value.push('')
    }
  }

  const removePlayer = (index: number) => {
    if (players.value.length > minPlayers) {
      players.value.splice(index, 1)
    }
  }

  const sortPlayersByFrequency = (entries: PlayerSearchResult[]): PlayerSearchResult[] =>
    [...entries].sort((a, b) => b.games_played - a.games_played)

  const sortPlayersAlphabetically = (entries: PlayerSearchResult[]): PlayerSearchResult[] =>
    [...entries].sort((a, b) =>
      a.player_name.localeCompare(b.player_name, undefined, { sensitivity: 'base' })
    )

  const fetchPlayerSuggestions = async (term: string) => {
    playerPickerLoading.value = true
    try {
      const trimmed = term.trim()
      
      // Get all other players that are already filled in (excluding the one we're editing)
      const otherSelectedPlayers = players.value
        .map((p, idx) => idx !== playerPickerIndex.value ? p.trim().toLowerCase() : '')
        .filter(p => p !== '')
      
      if (trimmed) {
        // User is searching - get many results, sort alphabetically
        const suggestions = await gamesApi.searchPlayers(trimmed, 50)
        playerPickerResults.value = sortPlayersAlphabetically(suggestions)
      } else {
        // No search term - show top 10 by frequency, excluding already selected players
        const suggestions = await gamesApi.searchPlayers('', 100)
        const sorted = sortPlayersByFrequency(suggestions)
        
        // Filter out already selected players (except the current one being edited)
        const filtered = sorted.filter(s => {
          const nameLower = s.player_name.trim().toLowerCase()
          return !otherSelectedPlayers.includes(nameLower)
        })
        
        // Take top 10
        playerPickerResults.value = filtered.slice(0, 10)
      }
    } catch (error) {
      console.error('Failed to load player suggestions', error)
      playerPickerResults.value = []
    } finally {
      playerPickerLoading.value = false
    }
  }

  const openPlayerPicker = (index: number) => {
    playerPickerIndex.value = index
    playerPickerVisible.value = true
    playerPickerSearch.value = ''
    playerPickerSelected.value = players.value[index] || ''
    fetchPlayerSuggestions('')
  }

  const closePlayerPicker = () => {
    playerPickerVisible.value = false
    playerPickerIndex.value = null
    playerPickerSelected.value = ''
  }

  const selectPlayerOption = (name: string) => {
    playerPickerSelected.value = name
  }

  const confirmPlayerPicker = () => {
    if (playerPickerIndex.value === null || !playerPickerSelected.value) {
      closePlayerPicker()
      return
    }
    const selectedIndex = playerPickerIndex.value
    players.value[selectedIndex] = playerPickerSelected.value
    validatePlayers()
    closePlayerPicker()
    
    // Set recently selected for visual feedback
    recentlySelectedIndex.value = selectedIndex
    
    // Clear the highlight after 2 seconds
    if (recentlySelectedTimeout) {
      clearTimeout(recentlySelectedTimeout)
    }
    recentlySelectedTimeout = setTimeout(() => {
      recentlySelectedIndex.value = null
    }, 2000)
  }

  const setPlayerPickerSearch = (value: string) => {
    playerPickerSearch.value = value
  }

  watch(playerPickerSearch, value => {
    if (pickerSearchTimeout) {
      clearTimeout(pickerSearchTimeout)
    }
    pickerSearchTimeout = setTimeout(() => {
      fetchPlayerSuggestions(value)
    }, 250)
  })

  onBeforeUnmount(() => {
    if (pickerSearchTimeout) {
      clearTimeout(pickerSearchTimeout)
    }
    if (recentlySelectedTimeout) {
      clearTimeout(recentlySelectedTimeout)
    }
  })

  const formatDate = (dateString: string): string => {
    try {
      const date = new Date(dateString)
      const locale = (route.params.locale as string) || undefined

      let weekday = new Intl.DateTimeFormat(locale, { weekday: 'short' }).format(date)
      weekday = weekday.charAt(0).toUpperCase() + weekday.slice(1)
      if (!weekday.endsWith('.')) weekday += '.'

      const dd = String(date.getDate()).padStart(2, '0')
      const MM = String(date.getMonth() + 1).padStart(2, '0')
      const yy = String(date.getFullYear()).slice(-2)
      const HH = String(date.getHours()).padStart(2, '0')
      const mm = String(date.getMinutes()).padStart(2, '0')

      return `${weekday} ${dd}/${MM}/${yy} ${HH}:${mm}`
    } catch {
      return dateString
    }
  }

  const loadGames = async () => {
    loadingGames.value = true
    try {
      const allGames = await gamesApi.getGames()
      const filteredGames = allGames
        .filter((game: Game) => game.game_type === config.gameType)
        .sort((a: Game, b: Game) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
      
      // Load ELO changes for each game in parallel (only for displayed games on first page)
      const gamesWithElo: GameWithElo[] = await Promise.all(
        filteredGames.slice(0, pageSize * 2).map(async (game) => {
          try {
            const eloChanges = await gamesApi.getGameEloChanges(game.id)
            return { ...game, elo_changes: eloChanges }
          } catch {
            return { ...game, elo_changes: undefined }
          }
        })
      )
      
      // Add remaining games without ELO changes (will be loaded on pagination)
      games.value = [
        ...gamesWithElo,
        ...filteredGames.slice(pageSize * 2).map(g => ({ ...g, elo_changes: undefined }))
      ]
    } catch (err) {
      console.error('Failed to load games:', err)
    } finally {
      loadingGames.value = false
    }
  }

  const resumeGame = (gameId: string) => {
    const locale = (route.params.locale as string) || 'en'
    const gameTypeRoutes: Record<string, string> = {
      hearts: `/hearts/game`,
      king: `/king/game`,
      double_king: `/double-king/game`,
      color_whist: `/color-whist/game`,
      whist: `/whist/game`,
      manille: `/manille/game`,
      press: `/press/game`,
    }
    const routePath = gameTypeRoutes[config.gameType]
    router.push(`/${locale}${routePath}/${gameId}`)
  }

  const confirmDelete = (gameId: string) => {
    deleteConfirmGameId.value = gameId
  }

  const deleteGame = async () => {
    if (!deleteConfirmGameId.value) return
    try {
      await gamesApi.deleteGame(deleteConfirmGameId.value)
      games.value = games.value.filter(g => g.id !== deleteConfirmGameId.value)
      deleteConfirmGameId.value = null
    } catch (err) {
      console.error('Failed to delete game:', err)
    }
  }

  const setupNamespaces: Record<SetupConfig['gameType'], string> = {
    hearts: 'heartsSetup',
    king: 'kingSetup',
    double_king: 'doubleKingSetup',
    color_whist: 'colorWhistSetup',
    whist: 'whistSetup',
    manille: 'manilleSetup',
    press: 'pressSetup',
  }

  const startGame = async () => {
    const playerNames = players.value.filter(p => p.trim() !== '')

    if (playerNames.length < minPlayers || playerNames.length > maxPlayers) {
      const namespace = setupNamespaces[config.gameType]
      error.value = t(`${namespace}.errorMinPlayers`)
      return
    }

    // Check for duplicate names
    const uniqueNames = new Set(playerNames.map(n => n.toLowerCase()))
    if (uniqueNames.size !== playerNames.length) {
      const namespace = setupNamespaces[config.gameType]
      error.value = t(`${namespace}.errorUniqueNames`)
      return
    }

    try {
      error.value = ''
      const game = await gamesApi.createGame({
        game_type: config.gameType,
        players: playerNames,
      })

      // Small delay to ensure game is persisted on backend before navigating
      await new Promise(resolve => setTimeout(resolve, 100))

      const locale = (route.params.locale as string) || 'en'
      const gameTypeRoutes: Record<string, string> = {
        hearts: `/hearts/game`,
        king: `/king/game`,
        double_king: `/double-king/game`,
        color_whist: `/color-whist/game`,
        whist: `/whist/game`,
        manille: `/manille/game`,
        press: `/press/game`,
      }
      const routePath = gameTypeRoutes[config.gameType]
      router.push(`/${locale}${routePath}/${game.id}`)
    } catch (err: any) {
      error.value = err.response?.data?.error || 'Failed to create game'
    }
  }

  const goToGameSelection = () => {
    const locale = (route.params.locale as string) || 'en'
    router.push(`/${locale}`)
  }

  return {
    // State
    players,
    error,
    games,
    loadingGames,
    page,
    deleteConfirmGameId,
    pageSize,

    // Auth
    isAuthenticated,

    // Computed
    totalPages,
    pagedGames,
    canStartGame,

    // Methods
    validatePlayers,
    addPlayer,
    removePlayer,
    formatDate,
    loadGames,
    resumeGame,
    confirmDelete,
    deleteGame,
    startGame,
    goToGameSelection,

    // Player picker
    playerPickerVisible,
    playerPickerResults,
    playerPickerSearch,
    playerPickerLoading,
    playerPickerSelected,
    openPlayerPicker,
    closePlayerPicker,
    confirmPlayerPicker,
    selectPlayerOption,
    setPlayerPickerSearch,
    recentlySelectedIndex,
  }
}
