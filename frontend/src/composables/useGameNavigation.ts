import { useRouter, useRoute } from 'vue-router'
import { gamesApi } from '@/api/games'

export interface GameNavigationConfig {
  /** The route path to navigate to on exit (e.g., '/hearts/setup') */
  homeRoute?: string
  /** Whether to include locale in the route */
  includeLocale?: boolean
}

/**
 * Composable for game navigation (exit, delete, etc.)
 */
export const useGameNavigation = (config: GameNavigationConfig = {}) => {
  const router = useRouter()
  const route = useRoute()

  const getLocale = () => (route.params.locale as string) || 'en'

  const goHome = () => {
    const locale = getLocale()
    const homePath = config.homeRoute ?? ''
    
    if (config.includeLocale !== false) {
      router.push(`/${locale}${homePath}`)
    } else {
      router.push(homePath || '/')
    }
  }

  const deleteGameAndGoHome = async (gameId: string) => {
    try {
      await gamesApi.deleteGame(gameId)
      goHome()
    } catch (error) {
      console.error('Failed to delete game:', error)
      throw error
    }
  }

  const navigateToGame = (gameType: string, gameId: string) => {
    const locale = getLocale()
    router.push(`/${locale}/${gameType}/${gameId}`)
  }

  return {
    goHome,
    deleteGameAndGoHome,
    navigateToGame,
    getLocale,
  }
}
