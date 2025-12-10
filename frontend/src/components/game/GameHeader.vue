<template>
  <div class="game-header">
    <button class="exit-button" @click="$emit('exit')" :title="t('common.exit')">←</button>
    <h1>
      {{ title }}
      <template v-if="showRound"> - {{ t('round') }} {{ currentRound }}</template>
    </h1>
    <LanguageSelector class="header-lang-selector" />
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import LanguageSelector from '@/components/LanguageSelector.vue'

withDefaults(defineProps<{
  title: string
  showRound?: boolean
  currentRound?: number
}>(), {
  showRound: false,
  currentRound: 1,
})

defineEmits<{
  exit: []
}>()

const { t } = useI18n()
</script>

<style scoped>
.game-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 24px;
}

.game-header h1 {
  flex: 1;
  text-align: center;
  font-size: clamp(1.5rem, 4vw, 2rem);
  margin: 0;
}

.exit-button {
  width: 44px;
  height: 44px;
  border-radius: 50%;
  border: none;
  background: rgba(45, 31, 26, 0.08);
  font-size: 1.5rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.exit-button:hover {
  background: rgba(45, 31, 26, 0.15);
  transform: scale(1.05);
}
</style>
