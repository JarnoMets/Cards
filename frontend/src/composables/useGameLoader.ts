import { ref, nextTick } from 'vue'
import type { Ref } from 'vue'
import { gamesApi } from '@/api/games'
import type { Game } from '@/types'

export interface GameLoaderOptions {
  maxRetries?: number
  onLoaded?: (game: Game) => void
  scrollToBottom?: Ref<HTMLElement | null>
}

/**
 * Composable for loading a game with retry logic and error handling.
 * Consolidates the common game loading pattern used across game views.
 */
export const useGameLoader = (gameId: string, options: GameLoaderOptions = {}) => {
  const { maxRetries = 3, onLoaded, scrollToBottom } = options

  const game = ref<Game | null>(null)
  const loading = ref(true)
  const error = ref('')

  const loadGame = async (attempt: number = 1): Promise<void> => {
    try {
      loading.value = true
      error.value = ''

      game.value = await gamesApi.getGame(gameId)

      // Call optional callback after successful load
      if (onLoaded && game.value) {
        onLoaded(game.value)
      }

      await nextTick()

      // Auto-scroll scoreboard to bottom if ref provided
      if (scrollToBottom?.value) {
        scrollToBottom.value.scrollTop = scrollToBottom.value.scrollHeight
      }

      loading.value = false
    } catch (err: any) {
      console.error(`Error loading game (attempt ${attempt}/${maxRetries}):`, err)

      if (attempt < maxRetries) {
        // Exponential backoff: 500ms, 1s, 2s
        const delay = Math.pow(2, attempt - 1) * 500
        setTimeout(() => loadGame(attempt + 1), delay)
      } else {
        error.value =
          err.response?.data?.error ||
          err.message ||
          'Failed to load game after multiple attempts'
        loading.value = false
      }
    }
  }

  const reload = () => loadGame(1)

  return {
    game,
    loading,
    error,
    loadGame,
    reload
  }
}
