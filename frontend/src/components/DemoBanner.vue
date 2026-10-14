<template>
  <div v-if="show" class="demo-banner" role="status">
    <span>Demo — changes stay in this browser only. Nothing is saved on the server.</span>
    <button type="button" class="demo-banner-reset" @click="resetLocal">Reset local data</button>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { isDemoHost } from '../composables/useDemoReadonly'
import { setDemoMeta } from '../demo/db'

const show = ref(false)
onMounted(() => {
  show.value = isDemoHost()
})

async function resetLocal() {
  await setDemoMeta({ hydrated: false, hydratedAt: null })
  window.location.reload()
}
</script>

<style scoped>
.demo-banner {
  position: sticky;
  top: 0;
  z-index: 4000;
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: center;
  gap: 0.75rem 1.25rem;
  padding: 0.55rem 1rem;
  background: #2c241c;
  color: #f4efe6;
  font-size: 0.85rem;
  border-bottom: 1px solid rgba(125, 73, 58, 0.35);
  text-align: center;
}
.demo-banner-reset {
  appearance: none;
  border: 1px solid #c4a484;
  background: transparent;
  color: #e8d5c0;
  border-radius: 999px;
  padding: 0.25rem 0.75rem;
  font: inherit;
  font-size: 0.8rem;
  cursor: pointer;
}
</style>
