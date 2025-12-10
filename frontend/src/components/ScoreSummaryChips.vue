<template>
  <div class="scoreboard-summary" aria-live="polite">
    <div
      v-for="(player, index) in players"
      :key="player.id"
      class="score-chip"
      :class="{
        'score-chip--leader': isLeader(index),
        'score-chip--trailing': isTrailer(index)
      }"
    >
      <span class="score-chip__label">{{ displayNameFn(player.name) }}</span>
      <span class="score-chip__value">{{ player.total_score ?? 0 }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Player } from '@/types'

const props = withDefaults(defineProps<{
  players: Player[]
  displayName?: (name: string) => string
}>(), {
  players: () => [],
})

const totals = computed(() => props.players.map(player => player.total_score ?? 0))

const leaderIndexes = computed(() => {
  if (!totals.value.length) return []
  const max = Math.max(...totals.value)
  return totals.value
    .map((total, index) => (total === max ? index : -1))
    .filter(index => index !== -1)
})

const trailingIndexes = computed(() => {
  if (!totals.value.length) return []
  const max = Math.max(...totals.value)
  const min = Math.min(...totals.value)
  if (max === min) return []
  return totals.value
    .map((total, index) => (total === min ? index : -1))
    .filter(index => index !== -1)
})

const displayNameFn = (name: string) => props.displayName ? props.displayName(name) : name
const isLeader = (index: number) => leaderIndexes.value.includes(index)
const isTrailer = (index: number) => trailingIndexes.value.includes(index)
</script>
