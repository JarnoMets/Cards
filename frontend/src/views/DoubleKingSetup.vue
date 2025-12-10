<template>
  <div class="page-container double-king-setup">
    <div class="content-container setup-container">
      <SetupHeader 
        :title="t('doubleKingGame.title')" 
        @back="goToGameSelection" 
      />

      <!-- About Double King section -->
      <div class="info-section card">
        <h1 class="scribble-underline">{{ t('doubleKingRules.about') }}</h1>
        <ul>
          <li>{{ t('doubleKingRules.rule1') }}</li>
          <li>{{ t('doubleKingRules.rule2') }}</li>
          <li>{{ t('doubleKingRules.rule3') }}</li>
          <li>{{ t('doubleKingRules.rule4') }}</li>
          <li>{{ t('doubleKingRules.rule5') }}</li>
          <li>{{ t('doubleKingRules.rule6') }}</li>
          <li>{{ t('doubleKingRules.rule7') }}</li>
          <li>{{ t('doubleKingRules.rule8') }}</li>
          <li>{{ t('doubleKingRules.rule9') }}</li>
        </ul>
      </div>

      <PlayerInputSection
        :players="players"
        :error="error"
        :can-start="canStartGame"
        :recently-selected-index="recentlySelectedIndex"
        :allow-add-remove="true"
        :min-players="2"
        @validate="validatePlayers"
        @open-picker="openPlayerPicker"
        @remove="removePlayer"
        @add="addPlayer"
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
import { onMounted } from 'vue'
import { useSetupLogic } from '@/composables/useSetupLogic'
import { useI18n } from 'vue-i18n'
import { SetupHeader, PlayerInputSection, PreviousGamesSection, GameDeleteModal } from '@/components/setup'
import PlayerPickerModal from '@/components/PlayerPickerModal.vue'
import { getGameConfig } from '@/config/games'

const { t } = useI18n()
const gameConfig = getGameConfig('double_king')

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
  addPlayer,
  removePlayer,
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
  gameType: 'double_king',
  minPlayers: gameConfig.minPlayers,
  maxPlayers: gameConfig.maxPlayers,
  defaultPlayers: gameConfig.defaultPlayers,
  pageSize: 5,
})

onMounted(() => {
  loadGames()
})
</script>

<style scoped>
@import '@/styles/shared.css';
@import '@/styles/setup.css';
</style>
