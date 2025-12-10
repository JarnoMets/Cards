<template>
  <div class="app-shell" :class="animationClasses">
    <SettingsGear />
    <main class="app-content">
      <RouterView v-slot="{ Component }">
        <component :is="Component" />
      </RouterView>
    </main>
    <SiteFooter />
  </div>
</template>

<script setup lang="ts">
import { RouterView } from 'vue-router'
import SiteFooter from './components/SiteFooter.vue'
import SettingsGear from './components/Settings.vue'
import { useAnimationSettings } from './composables/useAnimationSettings'

const { animationClasses } = useAnimationSettings()
</script>

<style scoped>
.app-shell {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background: radial-gradient(circle at top, rgba(255, 255, 255, 0.35), rgba(255, 255, 255, 0)),
    linear-gradient(180deg, rgba(125, 73, 58, 0.06), rgba(47, 92, 76, 0.08)),
    var(--texture-noise),
    var(--paper-cream);
}

.app-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}
</style>
