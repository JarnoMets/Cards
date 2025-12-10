<template>
  <Teleport to="body">
    <div v-if="visible" class="modal-overlay" @click="$emit('close')">
      <div class="modal round-edit-modal" @click.stop>
        <h2>{{ t('roundEdit.title') }}</h2>
        <p class="modal-subtitle">{{ t('roundEdit.roundLabel', { round: roundNumber }) }}</p>
        <p v-if="hint" class="modal-hint">{{ hint }}</p>
        
        <div class="round-edit-grid">
          <div v-for="(player, index) in players" :key="player.id" class="round-edit-row">
            <label>{{ player.name }}</label>
            <input
              type="number"
              v-model.number="localScores[index]"
              :min="inputMin"
              :max="inputMax"
              :placeholder="placeholder"
            />
          </div>
        </div>
        
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

interface Player {
  id: string
  name: string
}

const props = withDefaults(defineProps<{
  visible: boolean
  roundNumber: number
  players: Player[]
  scores: number[]
  inputMin?: number
  inputMax?: number
  placeholder?: string
  hint?: string
  error?: string
  saving?: boolean
}>(), {
  inputMin: 0,
  inputMax: 100,
  placeholder: '0',
  hint: '',
  error: '',
  saving: false,
})

const emit = defineEmits<{
  close: []
  save: [scores: number[]]
}>()

const { t } = useI18n()

const localScores = ref<number[]>([])

watch(() => props.scores, (newScores) => {
  localScores.value = [...newScores]
}, { immediate: true })

const handleSave = () => {
  emit('save', localScores.value)
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

.round-edit-modal {
  background: white;
  padding: 32px;
  border-radius: 16px;
  max-width: 500px;
  width: 90%;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.round-edit-modal h2 {
  margin: 0 0 8px;
  font-size: 1.5rem;
}

.modal-subtitle {
  margin: 0 0 8px;
  color: var(--ink-muted);
}

.modal-hint {
  margin: 0 0 24px;
  padding: 12px;
  background: rgba(47, 92, 76, 0.1);
  border-radius: 8px;
  font-size: 0.9rem;
  color: var(--ink);
}

.round-edit-grid {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 24px;
}

.round-edit-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.round-edit-row label {
  font-weight: 500;
  flex: 1;
}

.round-edit-row input {
  width: 100px;
  padding: 8px 12px;
  border: 1px solid rgba(45, 31, 26, 0.2);
  border-radius: 8px;
  font-size: 1rem;
  text-align: center;
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
