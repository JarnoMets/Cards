<template>
  <div class="app-shell" :class="animationClasses">
    <DemoBanner />
    <SettingsGear />
    <main class="app-content">
      <RouterView v-slot="{ Component }">
        <component :is="Component" />
      </RouterView>
    </main>
    <SiteFooter />
    <DemoHydrateOverlay :visible="hydrating" :phase="hydratePhase" :error="hydrateError" />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { RouterView } from 'vue-router'
import SiteFooter from './components/SiteFooter.vue'
import SettingsGear from './components/Settings.vue'
import DemoBanner from './components/DemoBanner.vue'
import DemoHydrateOverlay from './components/DemoHydrateOverlay.vue'
import { useAnimationSettings } from './composables/useAnimationSettings'
import { isDemoHost } from './composables/useDemoReadonly'
import { ensureCardsDemoHydrated } from './demo/hydrate'

const { animationClasses } = useAnimationSettings()
const hydrating = ref(false)
const hydratePhase = ref('Preparing…')
const hydrateError = ref<string | undefined>()

onMounted(async () => {
  if (!isDemoHost()) return
  hydrating.value = true
  try {
    await ensureCardsDemoHydrated((p) => {
      hydratePhase.value = p.phase
      hydrateError.value = p.error
    })
  } catch (e) {
    hydrateError.value = e instanceof Error ? e.message : 'Failed to prepare demo'
  } finally {
    hydrating.value = false
  }
})
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
