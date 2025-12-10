<template>
  <div class="players-section card">
    <div class="section-heading">
      <h2 class="scribble-underline">{{ t('playersLabel') }}</h2>
      <span class="score-badge score-badge--accent">{{ displayCount }}</span>
    </div>
    
    <!-- Team info for Manille -->
    <p v-if="showTeamInfo" class="team-info">
      {{ t('manilleSetup.teamInfo') }}
    </p>
    
    <div class="players-list">
      <div 
        v-for="(_, index) in players" 
        :key="index" 
        class="player-item" 
        :class="{ 
          'recently-selected': recentlySelectedIndex === index,
          'team-a': showTeamIndicators && index < 2,
          'team-b': showTeamIndicators && index >= 2
        }"
      >
        <span v-if="showTeamIndicators" class="team-indicator">
          {{ index < 2 ? t('manilleSetup.teamA') : t('manilleSetup.teamB') }}
        </span>
        <input
          v-model="players[index]"
          type="text"
          :placeholder="`${t('playerLabel')} ${index + 1}`"
          @input="$emit('validate')"
        />
        <div class="player-item-buttons">
          <button
            class="secondary select-btn"
            type="button"
            @click="$emit('openPicker', index)"
          >
            {{ t('selectPlayer') }}
          </button>
          <button
            v-if="allowAddRemove && players.length > minPlayers"
            class="danger"
            type="button"
            @click="$emit('remove', index)"
          >
            {{ t('removePlayer') }}
          </button>
        </div>
      </div>
    </div>
    
    <button 
      v-if="allowAddRemove" 
      class="secondary" 
      @click="$emit('add')" 
      style="width: 100%; margin-top: 20px;"
    >
      {{ t('addPlayer') }}
    </button>
    
    <div v-if="error" class="error-message">{{ error }}</div>
    
    <button
      class="primary"
      @click="$emit('start')"
      :disabled="!canStart"
      style="width: 100%; margin-top: 20px; font-size: 1.2rem; padding: 18px;"
    >
      {{ t('startGame') }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

const props = withDefaults(defineProps<{
  players: string[]
  error?: string
  canStart: boolean
  recentlySelectedIndex?: number | null
  allowAddRemove?: boolean
  minPlayers?: number
  fixedCount?: number
  showTeamInfo?: boolean
  showTeamIndicators?: boolean
}>(), {
  error: '',
  recentlySelectedIndex: null,
  allowAddRemove: true,
  minPlayers: 2,
  showTeamInfo: false,
  showTeamIndicators: false,
})

defineEmits<{
  validate: []
  openPicker: [index: number]
  remove: [index: number]
  add: []
  start: []
}>()

const { t } = useI18n()

const displayCount = computed(() => {
  return props.fixedCount ?? props.players.length
})
</script>

<style scoped>
@import '@/styles/shared.css';
@import '@/styles/setup.css';

.team-info {
  font-size: 0.9rem;
  color: var(--ink-secondary);
  margin-bottom: 12px;
  font-style: italic;
}

.team-indicator {
  font-size: 0.75rem;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: 4px;
  background: var(--primary-color);
  color: white;
}

.team-a .team-indicator {
  background: var(--primary-color);
}

.team-b .team-indicator {
  background: var(--accent-color);
}
</style>
