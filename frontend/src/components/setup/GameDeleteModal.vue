<template>
  <Teleport to="body">
    <div v-if="visible" class="modal-overlay" @click="$emit('close')">
      <div class="modal" @click.stop>
        <h2>{{ t('deleteGameConfirm') }}</h2>
        <p>{{ t('deleteGameWarning') }}</p>
        <div class="modal-actions">
          <button class="secondary" @click="$emit('close')">{{ t('cancel') }}</button>
          <button class="danger" @click="$emit('confirm')">{{ t('delete') }}</button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

defineProps<{
  visible: boolean
}>()

defineEmits<{
  close: []
  confirm: []
}>()

const { t } = useI18n()
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.modal {
  background: white;
  padding: 32px;
  border-radius: 16px;
  max-width: 400px;
  width: 90%;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.modal h2 {
  margin: 0 0 12px;
  font-size: 1.5rem;
}

.modal p {
  margin: 0 0 24px;
  color: var(--ink-muted);
}

.modal-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}
</style>
