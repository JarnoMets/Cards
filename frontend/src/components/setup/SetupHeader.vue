<template>
  <div class="setup-header">
    <button class="back-button" @click="$emit('back')" :title="t('common.backToSelection')">←</button>
    <h1>{{ title }}</h1>
    <LanguageSelector class="header-lang-selector" />
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import LanguageSelector from '@/components/LanguageSelector.vue'

defineProps<{
  title: string
}>()

defineEmits<{
  back: []
}>()

const { t } = useI18n()
</script>

<style scoped>
@import '@/styles/shared.css';
</style>
