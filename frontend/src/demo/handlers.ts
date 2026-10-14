import type { Game, Player } from '../types'
import { demoGet, demoSet, newId } from './db'
import { parseBody, type DemoHandleResult, type DemoRequestHandler } from './adapter'

export interface CardsDemoState {
  games: Game[]
  gameDetails: Record<string, Game>
  leaderboard: unknown
  elo: unknown
  eloOverall: unknown
  players: Record<string, unknown>
  playerSearch: unknown[]
  eloChanges: Record<string, unknown>
}

const STATE_KEY = 'state'

async function state(): Promise<CardsDemoState> {
  const s = await demoGet<CardsDemoState>(STATE_KEY)
  if (!s) throw new Error('Cards demo state missing')
  return s
}

async function save(s: CardsDemoState): Promise<void> {
  await demoSet(STATE_KEY, s)
}

function ok(data: unknown, status = 200): DemoHandleResult {
  return { status, data }
}

function nowIso(): string {
  return new Date().toISOString()
}

function recomputeLeaderboard(games: Game[]): unknown {
  const totals = new Map<string, { name: string; games: number; total_score: number; wins: number }>()
  for (const g of games) {
    if (!g.players?.length) continue
    const sorted = [...g.players].sort((a, b) => (b.total_score ?? 0) - (a.total_score ?? 0))
    const winner = sorted[0]?.name
    for (const p of g.players) {
      const cur = totals.get(p.name) || { name: p.name, games: 0, total_score: 0, wins: 0 }
      cur.games += 1
      cur.total_score += p.total_score ?? 0
      if (p.name === winner) cur.wins += 1
      totals.set(p.name, cur)
    }
  }
  const entries = [...totals.values()].sort((a, b) => b.wins - a.wins || b.total_score - a.total_score)
  return { players: entries, demo_local: true }
}

export async function saveCardsDemoState(s: CardsDemoState): Promise<void> {
  await demoSet(STATE_KEY, s)
}

export const cardsDemoHandler: DemoRequestHandler = async ({ method, path, data }) => {
  const body = parseBody(data) as Record<string, unknown>

  if (path === '/health' && method === 'get') {
    return ok({ status: 'ok', service: 'cards-demo-local' })
  }

  if (path === '/games' && method === 'get') {
    return ok((await state()).games)
  }

  if (path === '/games' && method === 'post') {
    const s = await state()
    const names = (body.players as string[]) || []
    const players: Player[] = names.map((name) => ({
      id: newId(),
      name,
      scores: [],
      game_indices: [],
      total_score: 0
    }))
    const game: Game = {
      id: newId(),
      game_type: (body.game_type as Game['game_type']) || 'hearts',
      players,
      created_at: nowIso(),
      current_round: 0
    }
    s.games = [game, ...s.games]
    s.gameDetails[game.id] = game
    s.leaderboard = recomputeLeaderboard(s.games)
    await save(s)
    return ok(game, 201)
  }

  const gameMatch = path.match(/^\/games\/([^/]+)(.*)$/)
  if (gameMatch) {
    const id = decodeURIComponent(gameMatch[1])
    const rest = gameMatch[2]
    const s = await state()
    let game = s.gameDetails[id] || s.games.find((g) => g.id === id)
    if (!game && method === 'get' && rest === '') {
      return { status: 404, data: { error: 'Game not found' } }
    }

    if (rest === '' && method === 'get') return ok(game)
    if (rest === '' && method === 'delete') {
      s.games = s.games.filter((g) => g.id !== id)
      delete s.gameDetails[id]
      s.leaderboard = recomputeLeaderboard(s.games)
      await save(s)
      return ok({ ok: true })
    }
    if (rest === '/elo-changes' && method === 'get') {
      return ok(s.eloChanges[id] ?? [])
    }
    if (rest === '/round' && method === 'post' && game) {
      const scores = (body.scores as number[]) || []
      const next: Game = {
        ...game,
        current_round: (game.current_round || 0) + 1,
        players: game.players.map((p, i) => {
          const add = scores[i] ?? 0
          return {
            ...p,
            scores: [...(p.scores || []), add],
            total_score: (p.total_score || 0) + add
          }
        })
      }
      s.gameDetails[id] = next
      s.games = s.games.map((g) => (g.id === id ? next : g))
      s.leaderboard = recomputeLeaderboard(s.games)
      await save(s)
      return ok(next)
    }
    if (rest.match(/^\/round\/\d+$/) && method === 'patch' && game) {
      const roundIndex = Number(rest.split('/')[2])
      const scores = (body.scores as number[]) || []
      const next: Game = {
        ...game,
        players: game.players.map((p, i) => {
          const scoresArr = [...(p.scores || [])]
          const old = scoresArr[roundIndex] ?? 0
          const neu = scores[i] ?? old
          scoresArr[roundIndex] = neu
          return {
            ...p,
            scores: scoresArr,
            total_score: (p.total_score || 0) - old + neu
          }
        })
      }
      s.gameDetails[id] = next
      s.games = s.games.map((g) => (g.id === id ? next : g))
      s.leaderboard = recomputeLeaderboard(s.games)
      await save(s)
      return ok(next)
    }
    if (rest.match(/^\/players\//) && method === 'patch' && game) {
      const playerId = decodeURIComponent(rest.split('/')[2])
      const next: Game = {
        ...game,
        players: game.players.map((p) =>
          p.id === playerId ? { ...p, name: String(body.name || p.name) } : p
        )
      }
      s.gameDetails[id] = next
      s.games = s.games.map((g) => (g.id === id ? next : g))
      await save(s)
      return ok(next)
    }
  }

  if (path === '/leaderboard' && method === 'get') {
    const s = await state()
    return ok(s.leaderboard ?? recomputeLeaderboard(s.games))
  }
  if (path === '/elo' && method === 'get') {
    return ok((await state()).elo ?? { entries: [] })
  }
  if (path === '/elo/overall' && method === 'get') {
    return ok((await state()).eloOverall ?? { entries: [] })
  }
  if (path.startsWith('/players/search') && method === 'get') {
    return ok((await state()).playerSearch ?? [])
  }

  const playerMatch = path.match(/^\/players\/([^/]+)(.*)$/)
  if (playerMatch && method === 'get') {
    const name = decodeURIComponent(playerMatch[1])
    const rest = playerMatch[2]
    const s = await state()
    if (rest === '' || rest === '/') {
      return ok(
        s.players[name] ?? {
          name,
          games_played: s.games.filter((g) => g.players.some((p) => p.name === name)).length,
          demo_local: true
        }
      )
    }
    if (rest === '/games') {
      return ok(s.games.filter((g) => g.players.some((p) => p.name === name)))
    }
    if (rest.startsWith('/elo')) {
      return ok(s.players[`${name}:elo${rest}`] ?? [])
    }
    if (rest === '/settings') {
      return ok(s.players[`${name}:settings`] ?? {})
    }
  }

  if (path.startsWith('/players/') && path.endsWith('/settings') && (method === 'put' || method === 'patch')) {
    const s = await state()
    const name = decodeURIComponent(path.split('/')[2])
    s.players[`${name}:settings`] = body
    await save(s)
    return ok(body)
  }

  if (path.startsWith('/admin')) {
    return { status: 403, data: { error: 'demo_readonly', message: 'Admin is disabled on the demo host.' } }
  }

  return null
}
