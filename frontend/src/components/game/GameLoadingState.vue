<template>
  <div v-if="loading" class="loading">{{ t('loading') }}</div>

  <div v-else-if="error" class="error-container card">
    <h2>{{ t('error') }}</h2>
    <p>{{ error }}</p>
    <button class="primary" @click="$emit('goHome')">{{ t('goHome') }}</button>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

defineProps<{
  loading: boolean
  error: string
}>()

defineEmits<{
  goHome: []
}>()

const { t } = useI18n()
</script>

<style scoped>
.loading {
  text-align: center;
  padding: 48px;
  font-size: 1.2rem;
  color: var(--ink-muted);
}

.error-container {
  text-align: center;
  padding: 48px;
}

.error-container h2 {
  color: #c41e3a;
  margin-bottom: 12px;
}

.error-container p {
  color: var(--ink-muted);
  margin-bottom: 24px;
}
</style>
