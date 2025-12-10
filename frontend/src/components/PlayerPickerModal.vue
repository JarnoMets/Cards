<template>
  <div v-if="visible" class="picker-overlay" @click.self="emit('close')">
    <div class="picker-modal">
      <header class="picker-header">
        <div>
          <p class="eyebrow">{{ t('playerPicker.topList') }}</p>
          <h2>{{ t('playerPicker.title') }}</h2>
        </div>
        <button class="close-btn" type="button" @click="emit('close')">×</button>
      </header>

      <div class="search-field">
        <label :for="searchId">{{ t('playerPicker.searchLabel') }}</label>
        <input
          :id="searchId"
          type="search"
          :placeholder="t('playerPicker.searchPlaceholder')"
          :value="search"
          @input="onSearch"
        />
      </div>

      <div class="picker-body">
        <div v-if="loading" class="loading">{{ t('loading') }}</div>
        <p v-else-if="players.length === 0" class="empty">{{ t('playerPicker.noResults') }}</p>
        <ul v-else class="player-list">
          <li v-for="player in players" :key="player.player_name">
            <button
              type="button"
              class="player-row"
              :class="{ selected: player.player_name === selected }"
              @click="emit('select', player.player_name)"
            >
              <span class="name">{{ player.player_name }}</span>
              <span class="meta">{{ t('leaderboard.gamesLabel', { count: player.games_played }) }}</span>
            </button>
          </li>
        </ul>
      </div>

      <footer class="picker-actions">
        <button class="secondary" type="button" @click="emit('close')">{{ t('cancel') }}</button>
        <button
          class="primary"
          type="button"
          :disabled="!selected"
          @click="emit('confirm')"
        >
          {{ t('playerPicker.confirm') }}
        </button>
      </footer>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { PlayerSearchResult } from '@/types'

const props = defineProps({
  visible: { type: Boolean, default: false },
  players: { type: Array as () => PlayerSearchResult[], default: () => [] },
  selected: { type: String, default: '' },
  search: { type: String, default: '' },
  loading: { type: Boolean, default: false }
})

const emit = defineEmits<{
  close: []
  confirm: []
  select: [name: string]
  search: [value: string]
}>()

const { t } = useI18n()
const searchId = computed(() => `player-picker-search-${Math.random().toString(36).slice(2)}`)

const onSearch = (event: Event) => {
  const value = (event.target as HTMLInputElement).value
  emit('search', value)
}
</script>

<style scoped>
.picker-overlay {
  position: fixed;
  inset: 0;
  background: rgba(45, 31, 26, 0.65);
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 20px;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.picker-modal {
  background: linear-gradient(135deg, #fffdf8, #f9f0dc);
  border-radius: 26px;
  width: min(480px, 96vw);
  max-height: 90vh;
  padding: 28px;
  display: flex;
  flex-direction: column;
  gap: 18px;
  box-shadow: 0 32px 80px rgba(33, 22, 12, 0.22);
  border: 1px solid rgba(45, 31, 26, 0.12);
  position: relative;
}

.picker-modal::before {
  content: '';
  position: absolute;
  inset: 14px;
  border: 1px dashed rgba(45, 31, 26, 0.14);
  border-radius: 20px;
  pointer-events: none;
}

.picker-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 16px;
}

.picker-header h2 {
  font-family: var(--font-heading, 'Playfair Display', serif);
  color: var(--ink, #2d1f1a);
  margin: 0;
  font-size: 1.5rem;
}

.close-btn {
  background: linear-gradient(140deg, rgba(255, 255, 255, 0.95), rgba(248, 234, 205, 0.95));
  border: 1px solid rgba(45, 31, 26, 0.2);
  border-radius: 50%;
  width: 36px;
  height: 36px;
  font-size: 1.4rem;
  line-height: 1;
  cursor: pointer;
  color: var(--ink, #2d1f1a);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: rgba(176, 42, 55, 0.1);
  border-color: rgba(176, 42, 55, 0.4);
  color: var(--danger-dark, #8b1a2b);
}

.search-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.search-field label {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--ink-secondary, #5a4a3f);
}

.search-field input {
  width: 100%;
  padding: 12px 16px;
  border: 1.5px solid rgba(45, 31, 26, 0.2);
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.92);
  color: var(--ink, #2d1f1a);
  font-size: 1rem;
  box-shadow: inset 0 2px 4px rgba(45, 31, 26, 0.08);
  transition: all 0.25s ease;
}

.search-field input:focus {
  outline: none;
  border-color: rgba(47, 92, 76, 0.65);
  box-shadow: inset 0 2px 4px rgba(45, 31, 26, 0.08), 0 0 0 3px rgba(201, 156, 93, 0.25);
}

.search-field input::placeholder {
  color: rgba(45, 31, 26, 0.4);
}

.picker-body {
  flex: 1;
  overflow: hidden;
}

.player-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-height: 320px;
  overflow-y: auto;
}

.player-row {
  width: 100%;
  border: 1.5px solid rgba(45, 31, 26, 0.12);
  border-radius: 14px;
  padding: 14px 16px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.95), rgba(250, 244, 233, 0.9));
  cursor: pointer;
  transition: all 0.2s ease;
}

.player-row:hover {
  border-color: rgba(47, 92, 76, 0.5);
  box-shadow: 0 4px 12px rgba(33, 22, 12, 0.1);
  transform: translateY(-1px);
}

.player-row.selected {
  border-color: var(--accent-emerald, #2f5c4c);
  background: linear-gradient(135deg, rgba(237, 249, 244, 0.95), rgba(210, 235, 224, 0.95));
  box-shadow: 0 4px 16px rgba(47, 92, 76, 0.15);
}

.player-row .name {
  font-weight: 600;
  color: var(--ink, #2d1f1a);
  font-family: var(--font-heading, 'Playfair Display', serif);
}

.player-row .meta {
  font-size: 0.85rem;
  color: var(--ink-muted, #8a7a6f);
}

.loading,
.empty {
  text-align: center;
  color: var(--ink-muted, #8a7a6f);
  padding: 24px;
  font-style: italic;
}

.picker-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding-top: 8px;
  border-top: 1px dashed rgba(45, 31, 26, 0.15);
}

.eyebrow {
  text-transform: uppercase;
  letter-spacing: 0.08em;
  font-size: 0.75rem;
  font-weight: 700;
  color: var(--accent-emerald, #2f5c4c);
  margin-bottom: 2px;
}

@media (max-width: 480px) {
  .picker-modal {
    padding: 20px;
    border-radius: 20px;
  }

  .picker-header h2 {
    font-size: 1.3rem;
  }

  .player-list {
    max-height: 260px;
  }

  .picker-actions {
    flex-direction: column;
  }

  .picker-actions button {
    width: 100%;
  }
}
</style>
