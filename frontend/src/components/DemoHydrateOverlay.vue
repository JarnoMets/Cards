<template>
  <div v-if="visible" class="hydrate-overlay">
    <div class="hydrate-card">
      <p class="eyebrow">Local demo</p>
      <h2>Preparing cards demo</h2>
      <p>{{ phase }}</p>
      <p v-if="error" class="err">{{ error }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{ visible: boolean; phase: string; error?: string }>()
</script>

<style scoped>
.hydrate-overlay {
  position: fixed;
  inset: 0;
  z-index: 5000;
  display: grid;
  place-items: center;
  background: rgba(44, 36, 28, 0.65);
  backdrop-filter: blur(3px);
  padding: 1rem;
}
.hydrate-card {
  background: #fffdf8;
  border: 1.5px solid rgba(125, 73, 58, 0.18);
  border-radius: 16px;
  padding: 1.5rem;
  max-width: 24rem;
  width: 100%;
  color: #2c241c;
}
.eyebrow {
  margin: 0 0 0.35rem;
  font-size: 0.75rem;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: #7a6a5c;
}
h2 { margin: 0 0 0.5rem; font-family: 'Playfair Display', serif; }
p { margin: 0; color: #7a6a5c; }
.err { margin-top: 0.75rem; color: #a33; }
</style>
