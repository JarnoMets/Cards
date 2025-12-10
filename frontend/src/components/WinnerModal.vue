<template>
  <div
    v-if="visible && winners.length > 0"
    class="winner-modal-overlay"
    @click.self="$emit('close')"
  >
    <div class="winner-modal">
      <button class="close-modal" @click="$emit('close')">×</button>
      <div class="winner-modal-content">
        <div class="winner-trophy">
          <svg viewBox="0 0 24 24" fill="currentColor" width="64" height="64">
            <path d="M19 5h-2V3H7v2H5c-1.1 0-2 .9-2 2v1c0 2.55 1.92 4.63 4.39 4.94.63 1.5 1.98 2.63 3.61 2.96V19H7v2h10v-2h-4v-3.1c1.63-.33 2.98-1.46 3.61-2.96C19.08 12.63 21 10.55 21 8V7c0-1.1-.9-2-2-2zM5 8V7h2v3.82C5.84 10.4 5 9.3 5 8zm14 0c0 1.3-.84 2.4-2 2.82V7h2v1z"/>
          </svg>
        </div>
        <div class="winner-title">{{ t('gameOver') }}!</div>
        <div class="winner-names">
          <span v-for="(winner, index) in winners" :key="winner.id">
            <strong>{{ winner.name }}</strong>
            <span v-if="index < winners.length - 1"> & </span>
          </span>
        </div>
        <div class="winner-score">
          {{ t('winsWith') }}
          <strong>{{ winners[0].total_score }} {{ t('points') }}</strong>
        </div>

        <!-- Rating changes -->
        <div v-if="eloChanges.length > 0" class="rating-changes">
          <div class="rating-changes-title">{{ t('ratingChanges') }}</div>
          <div class="rating-changes-list">
            <div 
              v-for="change in sortedEloChanges" 
              :key="change.player_name" 
              class="rating-change-row"
            >
              <span class="player-name">{{ change.player_name }}</span>
              <span class="rating-values">
                {{ change.elo_before }} → {{ change.elo_after }}
                <span :class="getRatingChangeClass(change.elo_change)">
                  ({{ formatRatingChange(change.elo_change) }})
                </span>
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Player, GameEloChange } from '@/types'

const props = defineProps<{
  visible: boolean
  winners: Player[]
  eloChanges?: GameEloChange[]
}>()

defineEmits<{
  close: []
}>()

const { t } = useI18n()

// Default to empty array if not provided
const eloChanges = computed(() => props.eloChanges || [])

// Sort ELO changes: winners first (by rating change descending)
const sortedEloChanges = computed(() => {
  return [...eloChanges.value].sort((a, b) => b.elo_change - a.elo_change)
})

const formatRatingChange = (change: number): string => {
  return change >= 0 ? `+${change}` : `${change}`
}

const getRatingChangeClass = (change: number): string => {
  if (change > 0) return 'positive'
  if (change < 0) return 'negative'
  return 'neutral'
}
</script>

<style scoped>
.winner-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
}

.winner-modal {
  background: linear-gradient(180deg, rgba(251, 249, 242, 0.98), rgba(248, 244, 235, 0.98));
  padding: 40px 48px;
  border-radius: 16px;
  max-width: 400px;
  width: 90%;
  text-align: center;
  position: relative;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.3);
  border: 2px dashed rgba(45, 31, 26, 0.2);
}

.winner-modal-content {
  display: flex;
  flex-direction: column;
  gap: 12px;
  align-items: center;
}

.close-modal {
  position: absolute;
  top: 12px;
  right: 16px;
  background: none;
  border: none;
  font-size: 1.5rem;
  color: var(--ink-secondary, #6b5a52);
  cursor: pointer;
  font-weight: 400;
  transition: color 0.2s ease;
  padding: 0;
  line-height: 1;
}

.close-modal:hover {
  color: var(--ink, #2d1f1a);
}

.winner-trophy {
  color: var(--accent-gold, #c99c5d);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 8px;
}

.winner-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--ink, #2d1f1a);
  font-family: var(--font-heading, 'Playfair Display', serif);
}

.winner-names {
  font-size: 1.2rem;
  font-weight: 600;
  color: var(--ink, #2d1f1a);
}

.winner-score {
  font-size: 1rem;
  color: var(--ink-secondary, #6b5a52);
  margin-top: 4px;
}

.winner-score strong {
  color: var(--accent-emerald, #2f5c4c);
}

.rating-changes {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px dashed rgba(45, 31, 26, 0.2);
  width: 100%;
}

.rating-changes-title {
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--ink-secondary, #6b5a52);
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.rating-changes-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.rating-change-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.9rem;
  padding: 4px 0;
}

.rating-change-row .player-name {
  color: var(--ink, #2d1f1a);
  font-weight: 500;
}

.rating-change-row .rating-values {
  color: var(--ink-secondary, #6b5a52);
  font-family: var(--font-mono, monospace);
  font-size: 0.85rem;
}

.rating-change-row .positive {
  color: var(--accent-emerald, #2f5c4c);
  font-weight: 600;
}

.rating-change-row .negative {
  color: var(--accent-crimson, #a33c3c);
  font-weight: 600;
}

.rating-change-row .neutral {
  color: var(--ink-secondary, #6b5a52);
}

/* Mobile */
@media (max-width: 480px) {
  .winner-modal {
    padding: 32px 24px;
    max-width: 320px;
  }

  .winner-trophy svg {
    width: 48px;
    height: 48px;
  }

  .winner-title {
    font-size: 1.25rem;
  }

  .winner-names {
    font-size: 1rem;
  }

  .winner-score {
    font-size: 0.9rem;
  }
}
</style>
