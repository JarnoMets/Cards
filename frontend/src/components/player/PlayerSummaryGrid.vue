<template>
  <div class="summary-grid">
    <div class="card summary-card" v-for="card in summaryCards" :key="card.label">
      <p class="summary-label">{{ card.label }}</p>
      <p class="summary-value">{{ card.value }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
interface SummaryCard {
  label: string
  value: string | number
}

defineProps<{
  summaryCards: SummaryCard[]
}>()
</script>

<style scoped>
.summary-grid {
  display: grid;
  gap: 1rem;
  grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
}

.summary-card {
  padding: 1rem 1.25rem;
  text-align: center;
}

.summary-label {
  font-size: 0.875rem;
  color: var(--color-text-muted);
  margin-bottom: 0.25rem;
}

.summary-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--color-text);
}

@media (max-width: 640px) {
  .summary-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .summary-value {
    font-size: 1.25rem;
  }
}
</style>
