<template>
  <div class="settings-gear-container" @mouseenter="showGear = true" @mouseleave="showGear = false">
    <router-link 
      :to="`/${locale}/settings`"
      class="settings-gear-button" 
      :class="{ visible: showGear }"
      :title="t('settings.openSettings')"
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="3"></circle>
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
      </svg>
    </router-link>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t, locale } = useI18n()

const showGear = ref(false)
</script>

<style scoped>
.settings-gear-container {
  position: fixed;
  top: 0;
  right: 0;
  z-index: 1000;
  padding: 16px;
}

.settings-gear-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  padding: 8px;
  border-radius: 50%;
  background: transparent;
  border: none;
  opacity: 0;
  transition: opacity 0.3s ease, transform 0.3s ease, background-color 0.2s ease;
  cursor: pointer;
  box-shadow: none;
  text-decoration: none;
}

.settings-gear-button.visible {
  opacity: 0.6;
}

.settings-gear-button:hover {
  opacity: 1;
  background: rgba(45, 31, 26, 0.08);
  transform: rotate(45deg);
}

.settings-gear-button svg {
  width: 24px;
  height: 24px;
  color: var(--ink-muted);
}

/* Mobile responsive */
@media (max-width: 480px) {
  .settings-gear-container {
    padding: 12px;
  }

  .settings-gear-button {
    width: 36px;
    height: 36px;
  }

  .settings-gear-button svg {
    width: 20px;
    height: 20px;
  }
}
</style>
