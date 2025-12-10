<template>
  <Teleport to="body">
    <div v-if="visible" class="modal-overlay" @click="$emit('close')">
      <div class="modal player-edit-modal" @click.stop>
        <h2>{{ t('playerEdit.title') }}</h2>
        <input
          type="text"
          v-model="localName"
          :placeholder="t('playerEdit.label')"
          @keyup.enter="handleSave"
        />
        <div v-if="error" class="error-message">{{ error }}</div>
        <div class="modal-actions">
          <button class="secondary" @click="$emit('close')">{{ t('cancel') }}</button>
          <button class="primary" @click="handleSave" :disabled="saving">
            {{ saving ? '...' : t('save') }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'

const props = withDefaults(defineProps<{
  visible: boolean
  playerName: string
  error?: string
  saving?: boolean
}>(), {
  error: '',
  saving: false,
})

const emit = defineEmits<{
  close: []
  save: [name: string]
}>()

const { t } = useI18n()

const localName = ref('')

watch(() => props.playerName, (newName) => {
  localName.value = newName
}, { immediate: true })

const handleSave = () => {
  emit('save', localName.value)
}
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

.player-edit-modal {
  background: white;
  padding: 32px;
  border-radius: 16px;
  max-width: 400px;
  width: 90%;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.player-edit-modal h2 {
  margin: 0 0 16px;
  font-size: 1.5rem;
}

.player-edit-modal input {
  width: 100%;
  padding: 12px 16px;
  border: 1px solid rgba(45, 31, 26, 0.2);
  border-radius: 8px;
  font-size: 1rem;
  margin-bottom: 16px;
}

.error-message {
  color: #c41e3a;
  margin-bottom: 16px;
  padding: 8px 12px;
  background: rgba(196, 30, 58, 0.1);
  border-radius: 8px;
}

.modal-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}
</style>
