import { ref, computed, watch } from 'vue'

export interface AnimationSettings {
  hoverEffects: boolean
  cardFlip: boolean
  pageTransitions: boolean
  buttonPress: boolean
  confetti: boolean
}

const STORAGE_KEY = 'cards_animation_settings'

const defaultSettings: AnimationSettings = {
  hoverEffects: true,
  cardFlip: true,
  pageTransitions: true,
  buttonPress: true,
  confetti: true,
}

// Global reactive state
const settings = ref<AnimationSettings>({ ...defaultSettings })
const isLoaded = ref(false)

// Load from localStorage
const loadSettings = () => {
  if (isLoaded.value) return
  
  try {
    const stored = localStorage.getItem(STORAGE_KEY)
    if (stored) {
      const parsed = JSON.parse(stored)
      settings.value = { ...defaultSettings, ...parsed }
    }
  } catch {
    // Ignore errors, use defaults
  }
  isLoaded.value = true
}

// Save to localStorage
const saveSettings = () => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings.value))
  } catch {
    // Ignore errors
  }
}

// Auto-save when settings change
watch(settings, saveSettings, { deep: true })

/**
 * Composable for managing animation settings across the app.
 * Settings are persisted to localStorage.
 */
export const useAnimationSettings = () => {
  // Load on first use
  loadSettings()

  const updateSetting = <K extends keyof AnimationSettings>(
    key: K,
    value: AnimationSettings[K]
  ) => {
    settings.value[key] = value
  }

  const resetToDefaults = () => {
    settings.value = { ...defaultSettings }
  }

  // Computed CSS class helper
  const animationClasses = computed(() => ({
    'animations-hover': settings.value.hoverEffects,
    'animations-flip': settings.value.cardFlip,
    'animations-transitions': settings.value.pageTransitions,
    'animations-press': settings.value.buttonPress,
    'animations-confetti': settings.value.confetti,
  }))

  return {
    settings,
    updateSetting,
    resetToDefaults,
    animationClasses,
  }
}
