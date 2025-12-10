<template>
  <div class="page-container game-selection">
    <div class="content-container selection-layout">
      <div class="selection-controls">
        <button class="secondary ghost" type="button" @click="openLeaderboard">
          {{ t('gameSelection.leaderboardButton') }}
        </button>
        <LanguageSelector class="header-lang-selector" />
      </div>

      <div ref="gamesGridRef" class="games-grid" role="list">
        <button type="button" class="game-card hearts" role="listitem" @click="startHearts">
          <div class="playing-card playing-card--red">
            <span class="card-suit card-suit--top-left">♥</span>
            <h2 class="card-title">{{ t('gameSelection.heartsTitle') }}</h2>
            <span class="card-suit card-suit--bottom-right">♥</span>
          </div>
        </button>

        <button type="button" class="game-card king" role="listitem" @click="startKing">
          <div class="playing-card">
            <span class="card-suit card-suit--top-left">♠</span>
            <h2 class="card-title">{{ t('gameSelection.kingTitle') }}</h2>
            <span class="card-suit card-suit--bottom-right">♠</span>
          </div>
        </button>

        <button type="button" class="game-card double-king" role="listitem" @click="startDoubleKing">
          <div class="playing-card playing-card--red">
            <span class="card-suit card-suit--top-left">♦</span>
            <h2 class="card-title">{{ t('gameSelection.doubleKingTitle') }}</h2>
            <span class="card-suit card-suit--bottom-right">♦</span>
          </div>
        </button>

        <button type="button" class="game-card whist" role="listitem" @click="startWhist">
          <div class="playing-card">
            <span class="card-suit card-suit--top-left">♣</span>
            <h2 class="card-title">{{ t('gameSelection.whistTitle') }}</h2>
            <span class="card-suit card-suit--bottom-right">♣</span>
          </div>
        </button>

        <button type="button" class="game-card color-whist" role="listitem" @click="startColorWhist">
          <div class="playing-card playing-card--red">
            <span class="card-suit card-suit--top-left">♥</span>
            <h2 class="card-title">{{ t('gameSelection.colorWhistTitle') }}</h2>
            <span class="card-suit card-suit--bottom-right">♥</span>
          </div>
        </button>

        <button type="button" class="game-card manille" role="listitem" @click="startManille">
          <div class="playing-card">
            <span class="card-suit card-suit--top-left">♠</span>
            <h2 class="card-title">{{ t('gameSelection.manilleTitle') }}</h2>
            <span class="card-suit card-suit--bottom-right">♠</span>
          </div>
        </button>

        <button type="button" class="game-card press" role="listitem" @click="startPress">
          <div class="playing-card playing-card--red">
            <span class="card-suit card-suit--top-left">♦</span>
            <h2 class="card-title">{{ t('gameSelection.pressTitle') }}</h2>
            <span class="card-suit card-suit--bottom-right">♦</span>
          </div>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import LanguageSelector from '@/components/LanguageSelector.vue'

const router = useRouter()
const route = useRoute()
const { t } = useI18n()

const gamesGridRef = ref<HTMLElement | null>(null)

// Handle horizontal scrolling with mouse wheel
const handleWheel = (event: WheelEvent) => {
  if (!gamesGridRef.value) return
  
  // Prevent default vertical scroll
  event.preventDefault()
  
  // Scroll horizontally based on vertical wheel movement
  const scrollAmount = event.deltaY
  gamesGridRef.value.scrollBy({
    left: scrollAmount,
    behavior: 'smooth'
  })
}

onMounted(() => {
  if (gamesGridRef.value) {
    gamesGridRef.value.addEventListener('wheel', handleWheel, { passive: false })
  }
})

onUnmounted(() => {
  if (gamesGridRef.value) {
    gamesGridRef.value.removeEventListener('wheel', handleWheel)
  }
})

const startHearts = () => {
  const locale = (route.params.locale as string) || 'en'
  router.push(`/${locale}/hearts/setup`)
}

const startKing = () => {
  const locale = (route.params.locale as string) || 'en'
  router.push(`/${locale}/king/setup`)
}

const startDoubleKing = () => {
  const locale = (route.params.locale as string) || 'en'
  router.push(`/${locale}/double-king/setup`)
}

const startColorWhist = () => {
  const locale = (route.params.locale as string) || 'en'
  router.push(`/${locale}/color-whist/setup`)
}

const startWhist = () => {
  const locale = (route.params.locale as string) || 'en'
  router.push(`/${locale}/whist/setup`)
}

const startManille = () => {
  const locale = (route.params.locale as string) || 'en'
  router.push(`/${locale}/manille`)
}

const startPress = () => {
  const locale = (route.params.locale as string) || 'en'
  router.push(`/${locale}/press`)
}

const openLeaderboard = () => {
  const locale = (route.params.locale as string) || 'en'
  router.push(`/${locale}/leaderboard`)
}
</script>

<style scoped>
.selection-layout {
  display: flex;
  flex-direction: column;
  min-height: calc(100vh - 120px);
  justify-content: center;
  overflow: hidden;
}

.selection-controls {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  padding: 24px 0 32px;
  position: absolute;
  top: 0;
  right: 24px;
  z-index: 10;
}

.games-grid {
  display: flex;
  gap: 40px;
  padding: 60px max(calc((100vw - (280px * 4.5)) / 2), 40px);
  overflow-x: auto;
  overflow-y: hidden;
  scroll-snap-type: x mandatory;
  scroll-behavior: smooth;
  -webkit-overflow-scrolling: touch;
  scrollbar-width: thin;
  scrollbar-color: rgba(47, 92, 76, 0.3) transparent;
}

.games-grid::-webkit-scrollbar {
  height: 8px;
}

.games-grid::-webkit-scrollbar-track {
  background: transparent;
  margin: 0 20px;
}

.games-grid::-webkit-scrollbar-thumb {
  background: rgba(47, 92, 76, 0.3);
  border-radius: 4px;
  transition: background 0.2s ease;
}

.games-grid::-webkit-scrollbar-thumb:hover {
  background: rgba(47, 92, 76, 0.5);
}

.game-card {
  border: none;
  background: transparent;
  padding: 0;
  cursor: pointer;
  transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
  flex: 0 0 auto;
  width: 280px;
  scroll-snap-align: center;
  animation: slideIn 0.6s ease-out backwards;
}

.game-card:nth-child(1) { animation-delay: 0.1s; }
.game-card:nth-child(2) { animation-delay: 0.2s; }
.game-card:nth-child(3) { animation-delay: 0.3s; }
.game-card:nth-child(4) { animation-delay: 0.4s; }
.game-card:nth-child(5) { animation-delay: 0.5s; }
.game-card:nth-child(6) { animation-delay: 0.6s; }
.game-card:nth-child(7) { animation-delay: 0.7s; }

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateX(100px) scale(0.8);
  }
  to {
    opacity: 1;
    transform: translateX(0) scale(1);
  }
}

.game-card:focus-visible {
  outline: 3px solid var(--accent-emerald);
  outline-offset: 6px;
}

.game-card:disabled,
.game-card:disabled .playing-card {
  cursor: not-allowed;
  opacity: 0.55;
}

.game-card:not(:disabled):hover {
  transform: translateY(-12px) scale(1.03);
}

.playing-card {
  background: linear-gradient(180deg, rgba(255, 255, 255, 0.98), rgba(250, 248, 245, 0.95));
  border-radius: 20px;
  border: 2px solid rgba(45, 31, 26, 0.12);
  box-shadow: 0 12px 32px rgba(33, 22, 12, 0.15), 0 4px 10px rgba(33, 22, 12, 0.1);
  padding: 36px 24px;
  aspect-ratio: 2.5 / 3.5;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  position: relative;
  isolation: isolate;
  transition: box-shadow 0.3s ease, transform 0.2s ease;
}

.game-card:not(:disabled):hover .playing-card {
  box-shadow: 0 20px 50px rgba(33, 22, 12, 0.25), 0 8px 16px rgba(33, 22, 12, 0.15);
  transform: scale(1.02);
}

.playing-card::after {
  content: '';
  position: absolute;
  inset: 10px;
  border: 1px solid rgba(45, 31, 26, 0.08);
  border-radius: 16px;
  pointer-events: none;
}

.playing-card--red .card-suit,
.playing-card--red .card-title {
  color: #c41e3a;
}

.card-suit {
  font-size: 3.5rem;
  z-index: 1;
  color: #1a1a1a;
  font-weight: 400;
  line-height: 1;
  position: absolute;
}

.card-suit--top-left {
  top: 16px;
  left: 16px;
}

.card-suit--top-right {
  top: 16px;
  right: 16px;
}

.card-suit--bottom-left {
  bottom: 16px;
  left: 16px;
  transform: rotate(180deg);
}

.card-suit--bottom-right {
  bottom: 16px;
  right: 16px;
  transform: rotate(180deg);
}

.suit-red {
  color: #c41e3a;
}

.suit-black {
  color: #1a1a1a;
}

.card-title {
  font-size: 2rem;
  font-family: var(--font-heading);
  color: var(--ink);
  margin: 0;
  z-index: 1;
}

@media (max-width: 1024px) {
  .games-grid {
    padding: 60px max(calc((100vw - (240px * 4.5)) / 2), 30px);
    gap: 30px;
  }

  .game-card {
    width: 240px;
  }

  .card-suit {
    font-size: 3rem;
  }

  .card-title {
    font-size: 1.8rem;
  }
}

@media (max-width: 768px) {
  .games-grid {
    padding: 60px max(calc((100vw - (200px * 4.5)) / 2), 20px);
    gap: 24px;
  }

  .game-card {
    width: 200px;
  }

  .playing-card {
    padding: 28px 20px;
  }

  .card-suit {
    font-size: 2.5rem;
  }

  .card-title {
    font-size: 1.5rem;
  }

  .selection-controls {
    position: static;
    justify-content: space-between;
    padding: 16px 0;
    margin-bottom: 16px;
  }
}

@media (max-width: 480px) {
  .games-grid {
    padding: 40px max(calc((100vw - (160px * 4.5)) / 2), 15px);
    gap: 20px;
  }

  .game-card {
    width: 160px;
  }

  .playing-card {
    padding: 24px 16px;
    border-radius: 16px;
  }

  .card-suit {
    font-size: 2rem;
  }

  .card-title {
    font-size: 1.2rem;
  }

  .card-suit--top-left,
  .card-suit--top-right {
    top: 12px;
  }

  .card-suit--top-left,
  .card-suit--bottom-left {
    left: 12px;
  }

  .card-suit--top-right,
  .card-suit--bottom-right {
    right: 12px;
  }

  .card-suit--bottom-left,
  .card-suit--bottom-right {
    bottom: 12px;
  }
}
</style>
