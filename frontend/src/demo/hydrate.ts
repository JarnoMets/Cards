import axios from 'axios'
import type { Game } from '../types'
import { isDemoHost } from '../composables/useDemoReadonly'
import { isDemoHydrated, setDemoMeta } from './db'
import { saveCardsDemoState, type CardsDemoState } from './handlers'

function resolveApiBase(): string {
  if (import.meta.env.VITE_API_URL) return import.meta.env.VITE_API_URL as string
  if (typeof window !== 'undefined') {
    const m = window.location.pathname.match(/^\/(cards)(?=\/|$)/)
    if (m) return `/${m[1]}/api`
  }
  return '/api'
}

export type HydrateProgress = { phase: string; done: boolean; error?: string }

let hydratePromise: Promise<void> | null = null

export async function ensureCardsDemoHydrated(
  onProgress?: (p: HydrateProgress) => void
): Promise<void> {
  if (!isDemoHost()) return
  if (await isDemoHydrated()) {
    onProgress?.({ phase: 'ready', done: true })
    return
  }
  if (hydratePromise) return hydratePromise

  hydratePromise = (async () => {
    const api = axios.create({ baseURL: resolveApiBase() })
    try {
      onProgress?.({ phase: 'Loading games…', done: false })
      const games: Game[] = (await api.get('/games')).data || []
      const gameDetails: Record<string, Game> = {}
      const eloChanges: Record<string, unknown> = {}
      const sample = games.slice(0, 12)
      for (const g of sample) {
        try {
          gameDetails[g.id] = (await api.get(`/games/${g.id}`)).data
        } catch {
          gameDetails[g.id] = g
        }
        try {
          eloChanges[g.id] = (await api.get(`/games/${g.id}/elo-changes`)).data
        } catch {
          eloChanges[g.id] = []
        }
      }

      onProgress?.({ phase: 'Loading leaderboards…', done: false })
      let leaderboard: unknown = { players: [] }
      let elo: unknown = { entries: [] }
      let eloOverall: unknown = { entries: [] }
      let playerSearch: unknown[] = []
      try {
        leaderboard = (await api.get('/leaderboard')).data
      } catch {
        /* optional */
      }
      try {
        elo = (await api.get('/elo')).data
      } catch {
        /* optional */
      }
      try {
        eloOverall = (await api.get('/elo/overall')).data
      } catch {
        /* optional */
      }
      try {
        playerSearch = (await api.get('/players/search', { params: { limit: 24 } })).data || []
      } catch {
        /* optional */
      }

      const players: Record<string, unknown> = {}
      const names = (playerSearch as { name?: string }[])
        .map((p) => p.name)
        .filter(Boolean)
        .slice(0, 8) as string[]
      for (const name of names) {
        try {
          players[name] = (await api.get(`/players/${encodeURIComponent(name)}`)).data
        } catch {
          /* skip */
        }
      }

      const state: CardsDemoState = {
        games,
        gameDetails,
        leaderboard,
        elo,
        eloOverall,
        players,
        playerSearch,
        eloChanges
      }
      await saveCardsDemoState(state)
      await setDemoMeta({
        hydrated: true,
        hydratedAt: new Date().toISOString(),
        version: 1
      })
      onProgress?.({ phase: 'ready', done: true })
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : 'Hydrate failed'
      onProgress?.({ phase: msg, done: false, error: msg })
      throw e
    } finally {
      hydratePromise = null
    }
  })()

  return hydratePromise
}
