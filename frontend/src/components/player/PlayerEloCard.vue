<template>
  <div class="elo-card">
    <div class="card-header">
      <h3>{{ t('player.elo.title') }}</h3>
      <span v-if="loading" class="loading-indicator">{{ t('loading') }}</span>
    </div>
    
    <div v-if="!loading && ratings.length === 0" class="empty-state">
      {{ t('player.elo.noRatings') }}
    </div>
    
    <div v-else class="elo-grid">
      <div
        v-for="rating in ratings"
        :key="rating.game_type"
        class="elo-item"
      >
        <span class="game-type">{{ formatGameType(rating.game_type) }}</span>
        <span class="elo-value" :class="getEloClass(rating.elo)">
          {{ rating.elo }}
        </span>
        <span class="games-count">{{ t('player.elo.gamesPlayed', { count: rating.games_played }) }}</span>
      </div>
    </div>
    
    <div v-if="overallElo !== null" class="overall-elo">
      <span class="label">{{ t('player.elo.overall') }}</span>
      <span class="value" :class="getEloClass(overallElo)">{{ overallElo }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

interface EloRating {
  player_name: string
  game_type: string
  elo: number
  games_played: number
  updated_at: string
}

const props = defineProps<{
  ratings: EloRating[]
  loading: boolean
}>()

const { t } = useI18n()

const overallElo = computed(() => {
  if (props.ratings.length === 0) return null
  const totalElo = props.ratings.reduce((sum, r) => sum + r.elo, 0)
  return Math.round(totalElo / props.ratings.length)
})

const formatGameType = (type: string): string => {
  const keyMap: Record<string, string> = {
    hearts: 'leaderboard.boards.hearts',
    king: 'leaderboard.boards.king',
    double_king: 'leaderboard.boards.doubleKing',
    color_whist: 'leaderboard.boards.colorWhist',
    whist: 'leaderboard.boards.whist',
    manille: 'leaderboard.boards.manille'
  }
  return t(keyMap[type] || type)
}

const getEloClass = (elo: number): string => {
  if (elo >= 1800) return 'elo-master'
  if (elo >= 1500) return 'elo-high'
  if (elo >= 1200) return 'elo-mid'
  return 'elo-low'
}
</script>

<style scoped>
.elo-card {
  background: var(--color-card);
  border-radius: var(--radius-lg);
  padding: 1.5rem;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.card-header h3 {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
}

.loading-indicator {
  font-size: 0.875rem;
  color: var(--color-text-muted);
}

.empty-state {
  color: var(--color-text-muted);
  text-align: center;
  padding: 1rem;
}

.elo-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 1rem;
}

.elo-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  padding: 0.75rem;
  background: var(--color-surface);
  border-radius: var(--radius-md);
}

.game-type {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--color-text);
}

.elo-value {
  font-size: 1.25rem;
  font-weight: 700;
}

.games-count {
  font-size: 0.75rem;
  color: var(--color-text-muted);
}

.overall-elo {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--color-border);
}

.overall-elo .label {
  font-weight: 500;
}

.overall-elo .value {
  font-size: 1.5rem;
  font-weight: 700;
}

/* ELO tier colors */
.elo-master {
  color: var(--color-gold, #ffd700);
}

.elo-high {
  color: var(--color-success, #22c55e);
}

.elo-mid {
  color: var(--color-text);
}

.elo-low {
  color: var(--color-warning, #f59e0b);
}
</style>
