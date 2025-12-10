<template>
  <div class="settings-view">
    <div class="settings-inner">
      <div class="page-header">
        <div>
          <h1>{{ t('settings.title') }}</h1>
        </div>
        <div class="header-actions">
          <button class="secondary" type="button" @click="goBack">
            {{ t('settings.backButton') }}
          </button>
          <LanguageSelector />
        </div>
      </div>

      <!-- Login Form (when not authenticated) -->
      <div v-if="!isAuthenticated" class="card login-card">
        <h2>{{ t('settings.loginTitle') }}</h2>
        <form @submit.prevent="handleLogin">
          <div class="form-group">
            <label for="password">{{ t('settings.password') }}</label>
            <input 
              type="password" 
              id="password" 
              v-model="password" 
              :placeholder="t('settings.passwordPlaceholder')"
              autocomplete="current-password"
            />
          </div>
          <p v-if="loginError" class="error-message">{{ t('settings.invalidPassword') }}</p>
          <div class="form-actions">
            <button type="submit" class="primary">
              {{ t('settings.login') }}
            </button>
          </div>
        </form>
      </div>

      <!-- Settings Content (when authenticated) -->
      <div v-else class="settings-content">
        <div class="settings-grid">
          <!-- Email Notifications Section -->
          <section class="card settings-section">
            <h3>{{ t('settings.notifications.title') }}</h3>
            <div class="setting-item">
              <label class="toggle-label">
                <span>{{ t('settings.notifications.emailOnGameEnd') }}</span>
                <input type="checkbox" v-model="settings.emailNotifications" />
                <span class="toggle-slider"></span>
              </label>
            </div>
            <div v-if="settings.emailNotifications" class="setting-item">
              <label for="emailAddress">{{ t('settings.notifications.emailAddress') }}</label>
              <input 
                type="email" 
                id="emailAddress" 
                v-model="settings.emailAddress"
                :placeholder="t('settings.notifications.emailPlaceholder')"
              />
            </div>
            <div class="setting-item test-email-row">
              <div class="test-email-info">
                <span class="email-status" :class="{ enabled: emailEnabled, disabled: !emailEnabled }">
                  {{ emailEnabled ? t('settings.notifications.serverEnabled') : t('settings.notifications.serverDisabled') }}
                </span>
              </div>
              <button 
                class="secondary" 
                @click="handleTestEmail" 
                :disabled="testEmailLoading || !emailEnabled"
              >
                {{ testEmailLoading ? t('settings.notifications.sending') : t('settings.notifications.testEmail') }}
              </button>
            </div>
            <div v-if="emailDailyStats" class="setting-item email-stats">
              <div class="email-stats-label">{{ t('settings.notifications.dailyStats') }}</div>
              <div class="email-stats-values">
                <span>{{ emailDailyStats.emails_sent }} / {{ emailDailyStats.daily_limit }}</span>
                <span class="email-stats-remaining" :class="{ 'low': emailDailyStats.remaining < 20 }">
                  ({{ emailDailyStats.remaining }} {{ t('settings.notifications.remaining') }})
                </span>
              </div>
            </div>
            <p v-if="testEmailSuccess" class="success-message">{{ t('settings.notifications.testEmailSuccess') }}</p>
            <p v-if="testEmailError" class="error-message">{{ testEmailError }}</p>
          </section>

          <!-- Security Section -->
          <section class="card settings-section">
            <h3>{{ t('settings.security.title') }}</h3>
            <div class="setting-item">
              <label for="currentPassword">{{ t('settings.security.currentPassword') }}</label>
              <input 
                type="password" 
                id="currentPassword" 
                v-model="passwordChange.current"
                :placeholder="t('settings.security.currentPasswordPlaceholder')"
              />
            </div>
            <div class="setting-item">
              <label for="newPassword">{{ t('settings.security.newPassword') }}</label>
              <input 
                type="password" 
                id="newPassword" 
                v-model="passwordChange.new"
                :placeholder="t('settings.security.newPasswordPlaceholder')"
              />
            </div>
            <div class="setting-item">
              <label for="confirmPassword">{{ t('settings.security.confirmPassword') }}</label>
              <input 
                type="password" 
                id="confirmPassword" 
                v-model="passwordChange.confirm"
                :placeholder="t('settings.security.confirmPasswordPlaceholder')"
              />
            </div>
            <p v-if="passwordError" class="error-message">{{ passwordError }}</p>
            <p v-if="passwordSuccess" class="success-message">{{ t('settings.security.passwordChanged') }}</p>
            <button 
              class="secondary" 
              @click="handleChangePassword"
              :disabled="!passwordChange.current || !passwordChange.new || !passwordChange.confirm"
            >
              {{ t('settings.security.changePassword') }}
            </button>
          </section>

          <!-- Application Section -->
          <section class="card settings-section">
            <h3>{{ t('settings.application.title') }}</h3>
            <div class="setting-item">
              <label for="defaultLocale">{{ t('settings.application.defaultLanguage') }}</label>
              <select id="defaultLocale" v-model="settings.defaultLocale">
                <option value="nl">Nederlands</option>
                <option value="en">English</option>
              </select>
            </div>
          </section>

          <!-- Animations Section -->
          <section class="card settings-section">
            <h3>{{ t('settings.animations.title') }}</h3>
            <div class="setting-item">
              <label class="toggle-label">
                <span>{{ t('settings.animations.hoverEffects') }}</span>
                <input type="checkbox" v-model="animationSettings.hoverEffects" />
                <span class="toggle-slider"></span>
              </label>
            </div>
            <div class="setting-item">
              <label class="toggle-label">
                <span>{{ t('settings.animations.cardFlip') }}</span>
                <input type="checkbox" v-model="animationSettings.cardFlip" />
                <span class="toggle-slider"></span>
              </label>
            </div>
            <div class="setting-item">
              <label class="toggle-label">
                <span>{{ t('settings.animations.pageTransitions') }}</span>
                <input type="checkbox" v-model="animationSettings.pageTransitions" />
                <span class="toggle-slider"></span>
              </label>
            </div>
            <div class="setting-item">
              <label class="toggle-label">
                <span>{{ t('settings.animations.buttonPress') }}</span>
                <input type="checkbox" v-model="animationSettings.buttonPress" />
                <span class="toggle-slider"></span>
              </label>
            </div>
          </section>

          <!-- Danger Zone -->
          <section class="card settings-section danger-zone">
            <h3>{{ t('settings.dangerZone.title') }}</h3>
            <p class="danger-warning">{{ t('settings.dangerZone.warning') }}</p>
            <button class="danger" @click="handleClearData">
              {{ t('settings.dangerZone.clearLocalData') }}
            </button>
          </section>

          <!-- Admin Section -->
          <section class="card settings-section admin-section">
            <h3>{{ t('settings.admin.title') }}</h3>
            <p class="admin-description">{{ t('settings.admin.description') }}</p>
            
            <div class="admin-item">
              <div class="admin-item-info">
                <span class="admin-item-title">{{ t('settings.admin.recalculateElo') }}</span>
                <span class="admin-item-desc">{{ t('settings.admin.recalculateEloDesc') }}</span>
              </div>
              <button 
                class="secondary" 
                @click="handleRecalculateElo"
                :disabled="recalculateEloLoading"
              >
                {{ recalculateEloLoading ? t('settings.admin.recalculating') : t('settings.admin.recalculateButton') }}
              </button>
            </div>
            <p v-if="recalculateEloSuccess" class="success-message">
              {{ t('settings.admin.recalculateSuccess', { games: recalculateResult.games, players: recalculateResult.players }) }}
            </p>
            <p v-if="recalculateEloError" class="error-message">{{ recalculateEloError }}</p>

            <!-- ELO Config Section -->
            <div class="elo-config-section" v-if="eloConfig">
              <h4>{{ t('settings.admin.eloConfig.title') }}</h4>
              
              <div class="elo-config-grid">
                <div class="setting-item">
                  <label for="startingElo">{{ t('settings.admin.eloConfig.startingElo') }}</label>
                  <input type="number" id="startingElo" v-model.number="eloConfig.starting_elo" min="100" max="2000" />
                </div>
                
                <div class="setting-item">
                  <label for="targetRating">{{ t('settings.admin.eloConfig.targetRating') }}</label>
                  <input type="number" id="targetRating" v-model.number="eloConfig.target_rating" min="500" max="3000" />
                </div>
                
                <div class="setting-item">
                  <label for="floorElo">{{ t('settings.admin.eloConfig.floorElo') }}</label>
                  <input type="number" id="floorElo" v-model.number="eloConfig.floor_elo" min="0" max="500" />
                </div>
                
                <div class="setting-item">
                  <label for="kFactorNew">{{ t('settings.admin.eloConfig.kFactorNew') }}</label>
                  <input type="number" id="kFactorNew" v-model.number="eloConfig.k_factor_new" min="10" max="100" />
                </div>
                
                <div class="setting-item">
                  <label for="kFactorMid">{{ t('settings.admin.eloConfig.kFactorMid') }}</label>
                  <input type="number" id="kFactorMid" v-model.number="eloConfig.k_factor_mid" min="10" max="100" />
                </div>
                
                <div class="setting-item">
                  <label for="kFactorEstablished">{{ t('settings.admin.eloConfig.kFactorEstablished') }}</label>
                  <input type="number" id="kFactorEstablished" v-model.number="eloConfig.k_factor_established" min="10" max="100" />
                </div>
                
                <div class="setting-item">
                  <label for="gamesUntilMid">{{ t('settings.admin.eloConfig.gamesUntilMid') }}</label>
                  <input type="number" id="gamesUntilMid" v-model.number="eloConfig.games_until_mid" min="1" max="50" />
                </div>
                
                <div class="setting-item">
                  <label for="gamesUntilEstablished">{{ t('settings.admin.eloConfig.gamesUntilEstablished') }}</label>
                  <input type="number" id="gamesUntilEstablished" v-model.number="eloConfig.games_until_established" min="1" max="100" />
                </div>
              </div>
              
              <div class="elo-config-actions">
                <button class="secondary" @click="handleSaveEloConfig" :disabled="savingEloConfig">
                  {{ savingEloConfig ? t('settings.admin.eloConfig.saving') : t('settings.admin.eloConfig.save') }}
                </button>
              </div>
              <p v-if="eloConfigSuccess" class="success-message">{{ t('settings.admin.eloConfig.saveSuccess') }}</p>
              <p v-if="eloConfigError" class="error-message">{{ eloConfigError }}</p>
            </div>
          </section>
        </div>

        <div class="settings-footer">
          <button class="secondary" @click="handleLogout">
            {{ t('settings.logout') }}
          </button>
          <button class="primary" @click="handleSaveSettings">
            {{ t('settings.save') }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useAuth } from '../stores/auth'
import { useAnimationSettings } from '../composables/useAnimationSettings'
import { gamesApi } from '../api/games'
import LanguageSelector from '@/components/LanguageSelector.vue'

const { t } = useI18n()
const route = useRoute()
const router = useRouter()
const { isAuthenticated, login, logout, changePassword } = useAuth()
const { settings: animationSettings } = useAnimationSettings()

const localeParam = computed(() => (route.params.locale as string) || 'en')

const password = ref('')
const loginError = ref(false)

// Password change state
const passwordChange = reactive({
  current: '',
  new: '',
  confirm: ''
})
const passwordError = ref('')
const passwordSuccess = ref(false)

// Test email state
const emailEnabled = ref(false)
const testEmailLoading = ref(false)
const testEmailSuccess = ref(false)
const testEmailError = ref('')

// Email daily stats
import type { EmailDailyStats } from '../types'
const emailDailyStats = ref<EmailDailyStats | null>(null)

// Recalculate ELO state
const recalculateEloLoading = ref(false)
const recalculateEloSuccess = ref(false)
const recalculateEloError = ref('')
const recalculateResult = reactive({ games: 0, players: 0 })

// ELO config state
import type { EloConfig } from '../types'
const eloConfig = ref<EloConfig | null>(null)
const savingEloConfig = ref(false)
const eloConfigSuccess = ref(false)
const eloConfigError = ref('')

interface Settings {
  emailNotifications: boolean
  emailAddress: string
  defaultLocale: string
}

const settings = reactive<Settings>({
  emailNotifications: false,
  emailAddress: '',
  defaultLocale: 'nl'
})

// Load settings from localStorage
const loadSettings = () => {
  try {
    const stored = localStorage.getItem('cards_settings')
    if (stored) {
      const parsed = JSON.parse(stored)
      Object.assign(settings, parsed)
    }
  } catch {
    // Ignore errors
  }
}

// Check email status from server
const checkEmailStatus = async () => {
  try {
    const status = await gamesApi.getEmailStatus()
    emailEnabled.value = status.enabled
  } catch {
    emailEnabled.value = false
  }
}

// Load email daily stats
const loadEmailDailyStats = async () => {
  try {
    emailDailyStats.value = await gamesApi.getEmailDailyStats()
  } catch {
    emailDailyStats.value = null
  }
}

// Load ELO config from server
const loadEloConfig = async () => {
  try {
    eloConfig.value = await gamesApi.getEloConfig()
  } catch {
    // Ignore errors, will use null
  }
}

onMounted(() => {
  loadSettings()
  checkEmailStatus()
  loadEmailDailyStats()
  loadEloConfig()
})

const goBack = () => {
  router.push(`/${localeParam.value}`)
}

const handleLogin = () => {
  if (login(password.value)) {
    loginError.value = false
    password.value = ''
  } else {
    loginError.value = true
  }
}

const handleLogout = () => {
  logout()
}

const handleTestEmail = async () => {
  testEmailLoading.value = true
  testEmailSuccess.value = false
  testEmailError.value = ''

  try {
    await gamesApi.sendTestEmail()
    testEmailSuccess.value = true
  } catch (err: any) {
    testEmailError.value = err?.response?.data?.error || err?.message || t('settings.notifications.testEmailFailed')
  } finally {
    testEmailLoading.value = false
  }
}

const handleChangePassword = () => {
  passwordError.value = ''
  passwordSuccess.value = false

  if (passwordChange.new !== passwordChange.confirm) {
    passwordError.value = t('settings.security.passwordMismatch')
    return
  }

  if (passwordChange.new.length < 4) {
    passwordError.value = t('settings.security.passwordTooShort')
    return
  }

  if (changePassword(passwordChange.current, passwordChange.new)) {
    passwordSuccess.value = true
    passwordChange.current = ''
    passwordChange.new = ''
    passwordChange.confirm = ''
  } else {
    passwordError.value = t('settings.security.incorrectCurrent')
  }
}

const handleSaveSettings = () => {
  localStorage.setItem('cards_settings', JSON.stringify(settings))
  router.push(`/${localeParam.value}`)
}

const handleClearData = () => {
  if (confirm(t('settings.dangerZone.confirmClear'))) {
    localStorage.clear()
    window.location.reload()
  }
}

const handleRecalculateElo = async () => {
  if (!confirm(t('settings.admin.recalculateConfirm'))) {
    return
  }
  
  recalculateEloLoading.value = true
  recalculateEloSuccess.value = false
  recalculateEloError.value = ''

  try {
    const result = await gamesApi.recalculateElo()
    recalculateResult.games = result.games_processed
    recalculateResult.players = result.players_updated
    recalculateEloSuccess.value = true
  } catch (err: any) {
    recalculateEloError.value = err?.response?.data?.error || err?.message || t('settings.admin.recalculateFailed')
  } finally {
    recalculateEloLoading.value = false
  }
}

const handleSaveEloConfig = async () => {
  if (!eloConfig.value) return
  
  savingEloConfig.value = true
  eloConfigSuccess.value = false
  eloConfigError.value = ''

  try {
    await gamesApi.setEloConfig(eloConfig.value)
    eloConfigSuccess.value = true
  } catch (err: any) {
    eloConfigError.value = err?.response?.data?.error || err?.message || t('settings.admin.eloConfig.saveFailed')
  } finally {
    savingEloConfig.value = false
  }
}
</script>

<style scoped>
.settings-view {
  min-height: 100vh;
  padding: 60px 24px 120px;
  background: transparent;
  color: var(--ink);
}

.settings-inner {
  width: min(900px, 96vw);
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 24px;
  background: rgba(255, 255, 255, 0.85);
  border-radius: 36px;
  padding: clamp(24px, 4vw, 48px);
  border: 1px solid rgba(45, 31, 26, 0.12);
  box-shadow: 0 45px 90px rgba(33, 22, 12, 0.18);
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
  font-size: clamp(1.8rem, 3vw, 2.5rem);
  margin: 0;
  font-family: var(--font-heading);
}

.header-actions {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.card {
  background: rgba(255, 255, 255, 0.6);
  border-radius: 16px;
  padding: 24px;
  border: 1px solid var(--border-color);
}

.login-card {
  max-width: 400px;
  margin: 40px auto;
}

.login-card h2 {
  margin: 0 0 20px 0;
  font-family: 'Playfair Display', serif;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-weight: 500;
  color: var(--ink-muted);
}

.form-group input {
  width: 100%;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: 20px;
}

.error-message {
  color: var(--danger-color);
  font-size: 0.875rem;
  margin: 8px 0;
}

.success-message {
  color: var(--success-color);
  font-size: 0.875rem;
  margin: 8px 0;
}

.settings-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.settings-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 20px;
}

.settings-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.settings-section h3 {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--ink);
  padding-bottom: 8px;
  border-bottom: 1px dashed var(--border-color);
}

.setting-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.setting-item label {
  font-weight: 500;
  color: var(--ink-muted);
  font-size: 0.9rem;
}

.setting-item input[type="email"],
.setting-item input[type="password"],
.setting-item input[type="number"],
.setting-item select {
  width: 100%;
  padding: 10px;
  border: 1.5px solid rgba(45, 31, 26, 0.2);
  border-radius: 8px;
  font-size: 0.9rem;
  background: rgba(255, 255, 255, 0.9);
}

/* Toggle switch */
.toggle-label {
  display: flex;
  align-items: center;
  justify-content: space-between;
  cursor: pointer;
  flex-direction: row;
}

.toggle-label input {
  display: none;
}

.toggle-slider {
  width: 48px;
  height: 26px;
  background: rgba(45, 31, 26, 0.2);
  border-radius: 13px;
  position: relative;
  transition: background 0.2s ease;
  flex-shrink: 0;
}

.toggle-slider::after {
  content: '';
  position: absolute;
  width: 22px;
  height: 22px;
  background: white;
  border-radius: 50%;
  top: 2px;
  left: 2px;
  transition: transform 0.2s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle-label input:checked + .toggle-slider {
  background: var(--accent-emerald);
}

.toggle-label input:checked + .toggle-slider::after {
  transform: translateX(22px);
}

/* Test email row */
.test-email-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-direction: row;
  gap: 12px;
  padding-top: 8px;
  border-top: 1px dashed var(--border-color);
}

.test-email-info {
  flex: 1;
}

.email-status {
  font-size: 0.85rem;
  padding: 4px 10px;
  border-radius: 12px;
}

.email-status.enabled {
  background: rgba(47, 92, 76, 0.12);
  color: var(--success-color);
}

.email-status.disabled {
  background: rgba(176, 42, 55, 0.1);
  color: var(--danger-color);
}

/* Email daily stats */
.email-stats {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 8px;
  border-top: 1px dashed var(--border-color);
  margin-top: 4px;
}

.email-stats-label {
  font-size: 0.85rem;
  color: var(--ink-secondary);
}

.email-stats-values {
  font-size: 0.9rem;
  font-weight: 600;
}

.email-stats-remaining {
  font-weight: 400;
  color: var(--ink-secondary);
  margin-left: 4px;
}

.email-stats-remaining.low {
  color: var(--danger-color);
}

/* Danger zone */
.danger-zone {
  background: rgba(176, 42, 55, 0.05);
  border-color: rgba(176, 42, 55, 0.2);
}

.danger-zone h3 {
  color: var(--danger-color);
}

.danger-warning {
  font-size: 0.85rem;
  color: var(--ink-muted);
  margin: 0;
}

/* Admin section */
.admin-section {
  background: rgba(47, 92, 76, 0.05);
  border-color: rgba(47, 92, 76, 0.2);
}

.admin-section h3 {
  color: var(--accent-emerald);
}

.admin-description {
  font-size: 0.85rem;
  color: var(--ink-muted);
  margin: 0;
}

.admin-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  padding: 12px 0;
  border-top: 1px dashed var(--border-color);
}

.admin-item-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  flex: 1;
}

.admin-item-title {
  font-weight: 600;
  color: var(--ink);
}

.admin-item-desc {
  font-size: 0.8rem;
  color: var(--ink-muted);
}

/* ELO Config */
.elo-config-section {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px dashed var(--border-color);
}

.elo-config-section h4 {
  margin: 0 0 16px 0;
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--ink);
}

.elo-config-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 12px;
}

.elo-config-grid .setting-item {
  gap: 4px;
}

.elo-config-grid .setting-item label {
  font-size: 0.8rem;
}

.elo-config-grid .setting-item input[type="number"] {
  padding: 8px;
  font-size: 0.85rem;
}

.elo-config-actions {
  margin-top: 16px;
  display: flex;
  justify-content: flex-end;
}

/* Footer */
.settings-footer {
  display: flex;
  justify-content: space-between;
  padding-top: 20px;
  border-top: 1px dashed var(--border-color);
}

/* Mobile responsive */
@media (max-width: 480px) {
  .settings-view {
    padding: 40px 16px 80px;
  }

  .settings-inner {
    padding: 20px;
  }

  .settings-grid {
    grid-template-columns: 1fr;
  }

  .settings-footer {
    flex-direction: column;
    gap: 12px;
  }

  .settings-footer button {
    width: 100%;
  }

  .header-actions {
    width: 100%;
    flex-direction: column;
  }
}
</style>
