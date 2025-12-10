<template>
  <div class="page-container manille-setup">
    <div class="content-container setup-container">
      <SetupHeader 
        :title="t('manilleGame.title')" 
        @back="goToGameSelection" 
      />

      <!-- About Manille section -->
      <div class="info-section card">
        <h1 class="scribble-underline">{{ t('manilleRules.about') }}</h1>
        <p>{{ t('manilleRules.overview') }}</p>
        <p><strong>{{ t('manilleRules.cardRanking') }}</strong></p>
        
        <h3>{{ t('manilleGame.cardValues.title') }}</h3>
        <ul>
          <li>{{ t('manilleGame.cardValues.manille') }}</li>
          <li>{{ t('manilleGame.cardValues.ace') }}</li>
          <li>{{ t('manilleGame.cardValues.king') }}</li>
          <li>{{ t('manilleGame.cardValues.queen') }}</li>
          <li>{{ t('manilleGame.cardValues.jack') }}</li>
          <li>{{ t('manilleGame.cardValues.others') }}</li>
        </ul>
        <p><strong>{{ t('manilleGame.cardValues.total') }}</strong></p>
        
        <p>{{ t('manilleRules.trumpChoice') }}</p>
        <p>{{ t('manilleRules.goingAlong') }}</p>
        <p>{{ t('manilleRules.goingAgainst') }}</p>
        <p>{{ t('manilleRules.targetScore') }}</p>
      </div>

      <!-- Custom Player section for Manille (team-based) -->
      <div class="players-section card">
        <div class="section-heading">
          <h2 class="scribble-underline">{{ t('playersLabel') }}</h2>
          <span class="score-badge score-badge--accent">{{ players.length }}</span>
        </div>
        <p class="eyebrow">{{ t('manilleSetup.eyebrow') }}</p>
        
        <!-- Team display -->
        <div class="team-info" v-if="players.length === 4">
          <div class="team team-1">
            <strong>{{ t('manilleGame.team1') }}:</strong> {{ players[0] || 'Player 1' }} & {{ players[2] || 'Player 3' }}
          </div>
          <div class="team team-2">
            <strong>{{ t('manilleGame.team2') }}:</strong> {{ players[1] || 'Player 2' }} & {{ players[3] || 'Player 4' }}
          </div>
        </div>

        <div class="players-list">
          <div v-for="(_, index) in players" :key="index" class="player-item" :class="{ 'recently-selected': recentlySelectedIndex === index }">
            <div class="player-label">
              <span class="team-indicator" :class="index % 2 === 0 ? 'team-1' : 'team-2'">
                {{ index % 2 === 0 ? 'T1' : 'T2' }}
              </span>
            </div>
            <input
              v-model="players[index]"
              type="text"
              :placeholder="`${t('playerLabel')} ${index + 1}`"
              @input="validatePlayers"
            />
            <div class="player-item-buttons">
              <button
                class="secondary select-btn"
                type="button"
                @click="openPlayerPicker(index)"
              >
                {{ t('selectPlayer') }}
              </button>
            </div>
          </div>
        </div>

        <div v-if="error" class="error-message">{{ error }}</div>
        <button
          class="primary"
          @click="startGame"
          :disabled="!canStartGame"
          style="width: 100%; margin-top: 20px; font-size: 1.2rem; padding: 18px;"
        >
          {{ t('startGame') }}
        </button>
      </div>

      <PreviousGamesSection
        :paged-games="pagedGames"
        :total-games="games.length"
        :loading="loadingGames"
        :page="page"
        :page-size="pageSize"
        :total-pages="totalPages"
        :is-authenticated="isAuthenticated"
        loading-key="loading"
        no-games-key="manilleSetup.noGames"
        score-mode="team"
        @confirm-delete="confirmDelete"
        @resume="resumeGame"
        @prev-page="page--"
        @next-page="page++"
      />

      <GameDeleteModal
        :visible="!!deleteConfirmGameId"
        @close="deleteConfirmGameId = null"
        @confirm="deleteGame"
      />
    </div>

    <PlayerPickerModal
      :visible="playerPickerVisible"
      :players="playerPickerResults"
      :selected="playerPickerSelected"
      :search="playerPickerSearch"
      :loading="playerPickerLoading"
      @close="closePlayerPicker"
      @search="setPlayerPickerSearch"
      @select="selectPlayerOption"
      @confirm="confirmPlayerPicker"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useSetupLogic } from '@/composables/useSetupLogic'
import { useI18n } from 'vue-i18n'
import { SetupHeader, PreviousGamesSection, GameDeleteModal } from '@/components/setup'
import PlayerPickerModal from '@/components/PlayerPickerModal.vue'
import { getGameConfig } from '@/config/games'

const { t } = useI18n()
const gameConfig = getGameConfig('manille')

const {
  players,
  error,
  games,
  loadingGames,
  page,
  deleteConfirmGameId,
  pageSize,
  isAuthenticated,
  totalPages,
  pagedGames,
  canStartGame,
  validatePlayers,
  loadGames,
  resumeGame,
  confirmDelete,
  deleteGame,
  startGame,
  goToGameSelection,
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
} = useSetupLogic({
  gameType: 'manille',
  minPlayers: gameConfig.minPlayers,
  maxPlayers: gameConfig.maxPlayers,
  defaultPlayers: gameConfig.defaultPlayers,
  pageSize: 6,
})

onMounted(() => {
  loadGames()
})
</script>

<style scoped>
@import '@/styles/shared.css';
@import '@/styles/setup.css';

.team-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 16px;
  padding: 12px;
  background: var(--color-surface);
  border-radius: 8px;
}

.team {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.9rem;
}

.team-1 {
  color: var(--color-primary);
}

.team-2 {
  color: var(--color-accent);
}

.team-indicator {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 50%;
  font-size: 0.7rem;
  font-weight: bold;
  color: white;
}

.team-indicator.team-1 {
  background: var(--color-primary);
}

.team-indicator.team-2 {
  background: var(--color-accent);
}

.player-label {
  display: flex;
  align-items: center;
  margin-right: 8px;
}

.team-score {
  display: flex;
  gap: 8px;
  align-items: center;
}

.team-label {
  font-weight: 600;
  font-size: 0.85rem;
}

.eyebrow {
  font-size: 0.9rem;
  color: var(--color-text-muted);
  margin-bottom: 12px;
}
</style>
