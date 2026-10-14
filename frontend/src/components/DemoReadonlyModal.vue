<template>
  <div
    v-if="visible"
    class="modal-overlay"
    role="dialog"
    aria-modal="true"
    aria-labelledby="demo-readonly-title"
    @click.self="dismissDemoReadonly"
  >
    <div class="modal-content demo-modal">
      <p class="demo-eyebrow">{{ t('demoReadonly.eyebrow') }}</p>
      <h2 id="demo-readonly-title">{{ t('demoReadonly.title') }}</h2>
      <p>{{ t('demoReadonly.body') }}</p>
      <div class="modal-actions">
        <button type="button" class="btn-primary" @click="dismissDemoReadonly">
          {{ t('demoReadonly.dismiss') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useDemoReadonly } from '../composables/useDemoReadonly'

const { t } = useI18n()
const { visible, dismissDemoReadonly } = useDemoReadonly()
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 3000;
  backdrop-filter: blur(2px);
  padding: 1rem;
}

.modal-content {
  background: var(--card-background, #fffdf8);
  border-radius: 16px;
  padding: 24px;
  max-width: 28rem;
  width: 100%;
  box-shadow: var(--card-shadow, 0 18px 40px rgba(0, 0, 0, 0.18));
  border: 1.5px solid var(--card-border, rgba(125, 73, 58, 0.18));
}

.demo-eyebrow {
  margin: 0 0 0.35rem;
  font-size: 0.75rem;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--ink-muted, #7a6a5c);
}

.demo-modal h2 {
  margin: 0 0 12px;
  font-family: 'Playfair Display', serif;
  color: var(--ink, #2c241c);
}

.demo-modal p {
  margin: 0 0 20px;
  color: var(--ink-muted, #7a6a5c);
  line-height: 1.5;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
}

.btn-primary {
  appearance: none;
  border: none;
  border-radius: 999px;
  padding: 0.65rem 1.25rem;
  background: #7d493a;
  color: #fffdf8;
  font: inherit;
  cursor: pointer;
}

.btn-primary:hover {
  background: #63392e;
}
</style>
