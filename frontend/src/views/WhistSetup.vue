<template>
  <div class="page-container whist-setup">
    <div class="content-container setup-container">
      <SetupHeader 
        :title="t('whistGame.title')" 
        @back="goToGameSelection" 
      />

      <!-- Game type selection (contracts) -->
      <div class="info-section card">
        <h1 class="scribble-underline">{{ t('whistRules.about') }}</h1>
        <p>{{ t('whistRules.contractSelectionHint') }}</p>
        <div class="contract-checkboxes">
          <label v-for="contract in contracts" :key="contract.key" class="checkbox-row">
            <input
              type="checkbox"
              v-model="enabledContracts"
              :value="contract.key"
            />
            <span>{{ t(contract.labelKey) }}</span>
          </label>
        </div>
        
        <!-- Game Settings -->
        <div class="game-settings">
          <h3>{{ t('whistSetup.settings') }}</h3>
          <label class="checkbox-row setting-row">
            <input
              type="checkbox"
              v-model="passRoundDoubling"
            />
            <span>{{ t('whistSetup.passRoundDoubling') }}</span>
          </label>
          <p class="setting-hint">{{ t('whistSetup.passRoundDoublingHint') }}</p>
        </div>
      </div>

      <PlayerInputSection
        :players="players"
        :error="error"
        :can-start="canStartGame"
        :recently-selected-index="recentlySelectedIndex"
        :allow-add-remove="false"
        :fixed-count="4"
        @validate="validatePlayers"
        @open-picker="openPlayerPicker"
        @start="startGame"
      />

      <PreviousGamesSection
        :paged-games="pagedGames"
        :total-games="games.length"
        :loading="loadingGames"
        :page="page"
        :page-size="pageSize"
        :total-pages="totalPages"
        :is-authenticated="isAuthenticated"
        loading-key="loadingGames"
        no-games-key="noGames"
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
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSetupLogic } from '@/composables/useSetupLogic'
import { SetupHeader, PlayerInputSection, PreviousGamesSection, GameDeleteModal } from '@/components/setup'
import PlayerPickerModal from '@/components/PlayerPickerModal.vue'
import { getGameConfig } from '@/config/games'

const { t } = useI18n()
const gameConfig = getGameConfig('whist')

// Contract keys; these will be sent to the game page via route query later if needed
const contracts = [
  { key: 'accept', labelKey: 'whistGame.contracts.accept' },
  { key: 'solo', labelKey: 'whistGame.contracts.solo' },
  { key: 'smallMisery', labelKey: 'whistGame.contracts.smallMisery' },
  { key: 'piccolo', labelKey: 'whistGame.contracts.piccolo' },
  { key: 'abondance', labelKey: 'whistGame.contracts.abondance' },
  { key: 'trull', labelKey: 'whistGame.contracts.trull' },
  { key: 'largeMisery', labelKey: 'whistGame.contracts.largeMisery' },
  { key: 'openMisery', labelKey: 'whistGame.contracts.openMisery' },
  { key: 'slam', labelKey: 'whistGame.contracts.slam' },
  { key: 'grandSlam', labelKey: 'whistGame.contracts.grandSlam' },
]

// All but small misery selected by default
const enabledContracts = ref<string[]>(contracts.map(c => c.key).filter(k => k !== 'smallMisery'))

// Game settings
const passRoundDoubling = ref(false)

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
  startGame: baseStartGame,
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
  gameType: 'whist',
  minPlayers: gameConfig.minPlayers,
  maxPlayers: gameConfig.maxPlayers,
  defaultPlayers: gameConfig.defaultPlayers,
  pageSize: 5,
})

const startGame = async () => {
  // For now we just start the game; later we can persist enabledContracts
  await baseStartGame()
}

onMounted(() => {
  loadGames()
})
</script>

<style scoped>
@import '@/styles/shared.css';
@import '@/styles/setup.css';

.contract-checkboxes {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-top: 8px;
}

.checkbox-row {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 0.95rem;
}

.game-settings {
  margin-top: 24px;
  padding-top: 16px;
  border-top: 1px dashed rgba(45, 31, 26, 0.15);
}

.game-settings h3 {
  font-size: 1rem;
  margin-bottom: 12px;
  color: var(--ink);
}

.setting-row {
  margin-bottom: 4px;
}

.setting-hint {
  font-size: 0.85rem;
  color: var(--ink-muted);
  margin: 4px 0 0 24px;
  font-style: italic;
}
</style>
