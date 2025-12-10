<template>
  <div class="player-search-view">
    <div class="player-search-inner">
      <div class="page-header">
        <div>
          <h1>{{ t('player.searchTitle') }}</h1>
          <p>{{ t('player.search.title') }}</p>
        </div>
        <div class="header-actions">
          <button class="secondary" type="button" @click="goBack">{{ t('player.actions.back') }}</button>
          <LanguageSelector />
        </div>
      </div>

      <div class="card search-card">
        <label>
          <span>{{ t('player.search.helper') }}</span>
          <input
            type="search"
            v-model.trim="query"
            :placeholder="t('player.search.placeholder')"
            @keyup.enter="performSearch"
          />
        </label>
      </div>

      <div v-if="error" class="card error-card">
        <p>{{ error }}</p>
        <button class="primary" type="button" @click="performSearch">{{ t('player.actions.refresh') }}</button>
      </div>

      <div v-else class="card results-card">
        <div class="results-meta">
          <p>{{ resultsLabel }}</p>
          <span v-if="loading">{{ t('loading') }}</span>
        </div>
        <p v-if="!query" class="empty-state">{{ t('player.search.noQuery') }}</p>
        <p v-else-if="!loading && results.length === 0" class="empty-state">{{ t('player.search.noResults') }}</p>
        <div v-else class="results-grid">
          <article class="result-item" v-for="player in results" :key="player.player_name">
            <div>
              <h2>{{ player.player_name }}</h2>
              <p class="meta">
                {{ t('leaderboard.gamesLabel', { count: player.games_played }) }} ·
                <span>{{ player.last_played_at ? formatTimestamp(player.last_played_at) : '—' }}</span>
              </p>
            </div>
            <button class="primary" type="button" @click="openProfile(player.player_name)">
              {{ t('player.search.viewProfile') }}
            </button>
          </article>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, computed, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import LanguageSelector from '@/components/LanguageSelector.vue'
import { gamesApi } from '@/api/games'
import type { PlayerSearchResult } from '@/types'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()

const localeParam = computed(() => (route.params.locale as string) || 'en')
const query = ref('')
const loading = ref(false)
const error = ref('')
const results = ref<PlayerSearchResult[]>([])
let debounceHandle: number | null = null

const formatTimestamp = (value: string) => {
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleDateString()
}

const loadPlayers = async (term: string) => {
  try {
    loading.value = true
    error.value = ''
    results.value = await gamesApi.searchPlayers(term, 25)
  } catch (err: any) {
    error.value = err?.response?.data?.error || err?.message || t('player.errors.loadFailed')
  } finally {
    loading.value = false
  }
}

const performSearch = () => {
  if (debounceHandle) {
    window.clearTimeout(debounceHandle)
    debounceHandle = null
  }
  loadPlayers(query.value)
}

watch(
  () => query.value,
  (value) => {
    if (debounceHandle) {
      window.clearTimeout(debounceHandle)
    }
    debounceHandle = window.setTimeout(() => {
      loadPlayers(value)
    }, 250)
  }
)

onMounted(() => {
  loadPlayers('')
})

onBeforeUnmount(() => {
  if (debounceHandle) {
    window.clearTimeout(debounceHandle)
  }
})

const resultsLabel = computed(() => t('player.search.results', { count: results.value.length }))

const goBack = () => {
  router.push(`/${localeParam.value}/leaderboard`)
}

const openProfile = (name: string) => {
  router.push(`/${localeParam.value}/players/${encodeURIComponent(name)}`)
}
</script>

<style scoped>
.player-search-view {
  min-height: 100vh;
  padding: 60px 24px 120px;
  background: transparent;
  color: var(--ink);
}

.player-search-inner {
  width: min(1100px, 96vw);
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 24px;
  background: rgba(255, 255, 255, 0.85);
  border-radius: 36px;
  padding: clamp(24px, 4vw, 48px);
  border: 1px solid rgba(45, 31, 26, 0.12);
  box-shadow: 0 45px 90px rgba(33, 22, 12, 0.18);
  position: relative;
  overflow: hidden;
}

.player-search-inner::before,
.player-search-inner::after {
  content: '';
  position: absolute;
  inset: 18px;
  border: 1px dashed rgba(45, 31, 26, 0.14);
  border-radius: 28px;
  pointer-events: none;
}

.player-search-inner::after {
  inset: 32px;
  border-style: solid;
  opacity: 0.35;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 16px;
  padding-bottom: 24px;
  border-bottom: 1px dashed rgba(45, 31, 26, 0.16);
}

.page-header h1 {
  font-size: clamp(2rem, 4vw, 3rem);
  margin-bottom: 6px;
  font-family: var(--font-heading);
  color: var(--ink);
}

.page-header p {
  color: var(--ink-muted);
  font-size: 0.95rem;
}

.header-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.card {
  background: linear-gradient(160deg, rgba(255, 255, 255, 0.95), rgba(248, 235, 214, 0.95));
  color: var(--ink);
  border-radius: 24px;
  padding: 24px;
  border: 1px solid rgba(45, 31, 26, 0.12);
  box-shadow: var(--card-shadow-soft);
}

.search-card {
  background: linear-gradient(140deg, rgba(255, 255, 255, 0.95), rgba(249, 238, 217, 0.95));
}

.search-card label {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.search-card label span {
  color: var(--ink-muted);
  font-size: 0.9rem;
}

.results-card {
  min-height: 240px;
}

.results-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.results-grid {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.result-item {
  border: 1px solid rgba(45, 31, 26, 0.12);
  border-radius: 18px;
  padding: 20px;
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: center;
  background: rgba(255, 255, 255, 0.92);
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.result-item:hover {
  transform: translateY(-2px);
  box-shadow: 0 12px 24px rgba(33, 22, 12, 0.12);
}

.result-item h2 {
  margin-bottom: 6px;
  font-family: var(--font-heading);
  color: var(--ink);
}

.meta {
  color: var(--ink-muted);
  font-size: 0.9rem;
}

.empty-state {
  text-align: center;
  color: var(--ink-muted);
  font-style: italic;
  padding: 32px;
}

.error-card {
  text-align: center;
}

@media (max-width: 700px) {
  .result-item {
    flex-direction: column;
    align-items: flex-start;
  }

  .header-actions {
    width: 100%;
    flex-direction: column;
    align-items: stretch;
  }
}
</style>
