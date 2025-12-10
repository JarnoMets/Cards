<template>
  <div v-if="visible" class="modal-overlay" @click="$emit('cancel')">
    <div class="modal" @click.stop>
      <h2>{{ title || t('deleteGameConfirm') }}</h2>
      <p>{{ message || t('deleteGameWarning') }}</p>
      <div class="modal-actions">
        <button class="secondary" @click="$emit('cancel')">
          {{ cancelText || t('cancel') }}
        </button>
        <button class="danger" @click="$emit('confirm')">
          {{ confirmText || t('delete') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

defineProps<{
  visible: boolean
  title?: string
  message?: string
  confirmText?: string
  cancelText?: string
}>()

defineEmits<{
  confirm: []
  cancel: []
}>()

const { t } = useI18n()
</script>

<style scoped>
/* Styles are inherited from shared.css via parent component */
</style>
