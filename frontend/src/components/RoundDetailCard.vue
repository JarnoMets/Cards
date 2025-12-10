<template>
  <div class="round-detail-card" :class="{ 'round-detail-card--muted': muted }">
    <div v-if="icon" class="round-detail-card__icon" aria-hidden="true">{{ icon }}</div>
    <div class="round-detail-card__body">
      <p v-if="eyebrow" class="round-detail-card__eyebrow">{{ eyebrow }}</p>
      <div class="round-detail-card__title-row">
        <strong>{{ title }}</strong>
        <span v-if="hint" class="round-detail-card__hint">{{ hint }}</span>
      </div>
      <p v-if="description" class="round-detail-card__description">{{ description }}</p>
      <div v-if="stats?.length" class="round-detail-card__stats">
        <div v-for="stat in stats" :key="stat.label" class="stat-block">
          <span class="stat-label">{{ stat.label }}</span>
          <span class="stat-value" :class="stat.valueClass">{{ stat.value }}</span>
        </div>
      </div>
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
type StatEntry = {
  label: string
  value: string | number
  valueClass?: string
}

defineProps<{
  icon?: string
  eyebrow?: string
  title: string
  hint?: string
  description?: string
  stats?: StatEntry[]
  muted?: boolean
}>()
</script>
