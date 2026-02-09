<template>
  <div class="games-section">
    <div class="section-heading">
      <h2 class="scribble-underline">{{ t('previousGames') }}</h2>
    </div>
    
    <div v-if="loading" class="loading-games">{{ t(loadingKey) }}</div>
    
    <div v-else-if="pagedGames.length === 0" class="no-games">{{ t(noGamesKey) }}</div>
    
    <div v-else class="games-grid">
      <div v-for="game in pagedGames" :key="game.id" class="game-card card">
        <div class="game-card-header">
          <div class="game-date">{{ formatDate(game.created_at) }}</div>
          <button 
            v-if="isAuthenticated" 
            class="delete-btn" 
            @click="$emit('confirmDelete', game.id)" 
            :title="t('delete')"
          >×</button>
        </div>
        <div class="game-card-body">
          <div class="game-players">
            <strong>{{ t('playersLabel') }}:</strong> {{ game.players.map(p => p.name).join(', ') }}
          </div>
          <div class="game-status">
            <strong>{{ t('round') || 'Round' }}:</strong> {{ game.current_round }}
          </div>
          <div class="game-scores">
            <strong>{{ t('finalScores') || 'Final Scores' }}:</strong>
            
            <!-- Team score display for Manille -->
            <div v-if="scoreMode === 'team'" class="scores-list team-scores">
              <div class="team-score">
                {{ t('manilleSetup.teamA') }}: 
                <span class="score-value">{{ getTeamScore(game, 0) }}</span>
              </div>
              <div class="team-score">
                {{ t('manilleSetup.teamB') }}: 
                <span class="score-value">{{ getTeamScore(game, 1) }}</span>
              </div>
            </div>
            
            <!-- Individual score display -->
            <div v-else class="scores-list">
              <div v-for="player in game.players" :key="player.id" class="player-score">
                <span 
                  class="player-score-elo" 
                  v-if="getEloChange(game, player.name) !== null" 
                  :class="{
                    'elo-positive': getEloChange(game, player.name)! > 0,
                    'elo-negative': getEloChange(game, player.name)! < 0,
                    'elo-neutral': getEloChange(game, player.name) === 0
                  }"
                >
                  {{ getEloChange(game, player.name)! >= 0 ? '+' : '' }}{{ getEloChange(game, player.name) }}
                </span>
                {{ player.name }}: <span class="score-value">{{ player.total_score }}</span>
              </div>
            </div>
          </div>
        </div>
        <div class="game-card-actions">
          <button class="primary" @click="$emit('resume', game.id)">{{ t('continuePlaying') }}</button>
        </div>
      </div>
    </div>
    
    <div v-if="totalGames > pageSize" class="pagination">
      <button :disabled="page === 1" @click="$emit('prevPage')">{{ t('paginationPrev') }}</button>
      <span>{{ t('paginationPage', { page, totalPages }) }}</span>
      <button :disabled="page === totalPages" @click="$emit('nextPage')">{{ t('paginationNext') }}</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import type { GameWithElo } from '@/composables/useSetupLogic'

withDefaults(defineProps<{
  pagedGames: GameWithElo[]
  totalGames: number
  loading: boolean
  page: number
  pageSize: number
  totalPages: number
  isAuthenticated: boolean
  loadingKey?: string
  noGamesKey?: string
  scoreMode?: 'individual' | 'team'
}>(), {
  loadingKey: 'loadingGames',
  noGamesKey: 'noGames',
  scoreMode: 'individual',
})

defineEmits<{
  confirmDelete: [id: string]
  resume: [id: string]
  prevPage: []
  nextPage: []
}>()

const { t } = useI18n()

const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  return date.toLocaleDateString(undefined, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const getEloChange = (game: GameWithElo, playerName: string): number | null => {
  if (!game.elo_changes) return null
  const searchName = playerName.trim().toLowerCase()
  const change = game.elo_changes.find(
    c => c.player_name.trim().toLowerCase() === searchName
  )
  return change?.elo_change ?? null
}

const getTeamScore = (game: GameWithElo, teamIndex: number): number => {
  // Team 0: players 0,1 - Team 1: players 2,3
  const startIdx = teamIndex * 2
  const teamPlayers = game.players.slice(startIdx, startIdx + 2)
  return teamPlayers.reduce((sum, p) => sum + (p.total_score || 0), 0)
}
</script>

<style scoped>
@import '@/styles/shared.css';
@import '@/styles/setup.css';

.team-scores {
  display: flex;
  gap: 16px;
}

.team-score {
  font-weight: 500;
}
</style>
