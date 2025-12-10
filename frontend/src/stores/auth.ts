import { ref, computed } from 'vue'

const STORAGE_KEY = 'cards_admin_authenticated'
const PASSWORD_KEY = 'cards_admin_password'
const SESSION_DURATION = 24 * 60 * 60 * 1000 // 24 hours
const DEFAULT_PASSWORD = 'admin'

interface AuthSession {
  authenticated: boolean
  expiresAt: number
}

function loadSession(): boolean {
  try {
    const stored = localStorage.getItem(STORAGE_KEY)
    if (!stored) return false
    
    const session: AuthSession = JSON.parse(stored)
    if (Date.now() > session.expiresAt) {
      localStorage.removeItem(STORAGE_KEY)
      return false
    }
    return session.authenticated
  } catch {
    return false
  }
}

function saveSession(authenticated: boolean) {
  const session: AuthSession = {
    authenticated,
    expiresAt: Date.now() + SESSION_DURATION
  }
  localStorage.setItem(STORAGE_KEY, JSON.stringify(session))
}

function getStoredPassword(): string {
  return localStorage.getItem(PASSWORD_KEY) || DEFAULT_PASSWORD
}

const isAuthenticated = ref(loadSession())

export function useAuth() {
  const login = (password: string): boolean => {
    const currentPassword = getStoredPassword()
    
    if (password === currentPassword) {
      isAuthenticated.value = true
      saveSession(true)
      return true
    }
    return false
  }

  const logout = () => {
    isAuthenticated.value = false
    localStorage.removeItem(STORAGE_KEY)
  }

  const changePassword = (currentPassword: string, newPassword: string): boolean => {
    if (currentPassword !== getStoredPassword()) {
      return false
    }
    localStorage.setItem(PASSWORD_KEY, newPassword)
    return true
  }

  return {
    isAuthenticated: computed(() => isAuthenticated.value),
    login,
    logout,
    changePassword
  }
}
