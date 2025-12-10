import { ref, watch } from 'vue'
import type { Ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { gamesApi } from '@/api/games'
import type { Game } from '@/types'

export const usePlayerNameEditor = (gameId: string, game: Ref<Game | null>) => {
  const { t } = useI18n()
  const editingPlayerId = ref<string | null>(null)
  const playerNameDraft = ref('')
  const playerEditError = ref('')
  const savingPlayer = ref(false)

  const startEditingPlayer = (playerId: string) => {
    if (!game.value) return
    const player = game.value.players.find(p => p.id === playerId)
    if (!player) return
    editingPlayerId.value = playerId
    playerNameDraft.value = player.name
    playerEditError.value = ''
  }

  const cancelPlayerEdit = () => {
    editingPlayerId.value = null
    playerNameDraft.value = ''
    playerEditError.value = ''
  }

  const savePlayerName = async () => {
    if (!game.value || !editingPlayerId.value) return
    const trimmed = playerNameDraft.value.trim()
    if (!trimmed) {
      playerEditError.value = t('playerEdit.errors.emptyName') as string
      return
    }

    try {
      savingPlayer.value = true
      playerEditError.value = ''
      const updatedGame = await gamesApi.updatePlayer(gameId, editingPlayerId.value, trimmed)
      game.value = updatedGame
      cancelPlayerEdit()
    } catch (err: any) {
      playerEditError.value = err.response?.data?.error || err.message || (t('playerEdit.errors.generic') as string)
    } finally {
      savingPlayer.value = false
    }
  }

  watch(game, (newVal) => {
    if (!newVal || !editingPlayerId.value) return
    const player = newVal.players.find(p => p.id === editingPlayerId.value)
    if (player) {
      playerNameDraft.value = player.name
    }
  })

  return {
    editingPlayerId,
    playerNameDraft,
    playerEditError,
    savingPlayer,
    startEditingPlayer,
    cancelPlayerEdit,
    savePlayerName
  }
}
