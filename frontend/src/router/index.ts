import { createRouter, createWebHistory } from 'vue-router'
import GameSelection from '../views/GameSelection.vue'
import HeartsGame from '../views/HeartsGame.vue'
import HeartsSetup from '../views/HeartsSetup.vue'
import i18n from '../i18n';

function routerBase(): string {
  if (typeof window === 'undefined') return '/'
  const m = window.location.pathname.match(/^\/(cards)(?=\/|$)/)
  return m ? `/${m[1]}/` : '/'
}

const router = createRouter({
  history: createWebHistory(routerBase()),
  scrollBehavior: () => ({ top: 0 }),
  routes: [
    {
      path: '/:locale(en|nl)',
      name: 'home',
      component: GameSelection
    },
    {
      path: '/:locale(en|nl)/hearts/setup',
      name: 'hearts-setup',
      component: HeartsSetup
    },
    {
      path: '/:locale(en|nl)/hearts/game/:id',
      name: 'hearts-game',
      component: HeartsGame
    },
    {
      path: '/:locale(en|nl)/king/setup',
      name: 'KingSetup',
      component: () => import('../views/KingSetup.vue')
    },
    {
      path: '/:locale(en|nl)/king/game/:id',
      name: 'KingGame',
      component: () => import('../views/KingGame.vue')
    },
    {
      path: '/:locale(en|nl)/double-king/setup',
      name: 'DoubleKingSetup',
      component: () => import('../views/DoubleKingSetup.vue')
    },
    {
      path: '/:locale(en|nl)/double-king/game/:id',
      name: 'DoubleKingGame',
      component: () => import('../views/DoubleKingGame.vue')
    },
    {
      path: '/:locale(en|nl)/color-whist/setup',
      name: 'ColorWhistSetup',
      component: () => import('../views/ColorWhistSetup.vue')
    },
    {
      path: '/:locale(en|nl)/color-whist/game/:id',
      name: 'ColorWhistGame',
      component: () => import('../views/ColorWhistGame.vue')
    },
    {
      path: '/:locale(en|nl)/whist/setup',
      name: 'WhistSetup',
      component: () => import('../views/WhistSetup.vue')
    },
    {
      path: '/:locale(en|nl)/whist/game/:id',
      name: 'WhistGame',
      component: () => import('../views/WhistGame.vue')
    },
    {
      path: '/:locale(en|nl)/manille',
      name: 'ManilleSetup',
      component: () => import('../views/ManilleSetup.vue')
    },
    {
      path: '/:locale(en|nl)/manille/game/:id',
      name: 'ManilleGame',
      component: () => import('../views/ManilleGame.vue')
    },
    {
      path: '/:locale(en|nl)/press',
      name: 'PressSetup',
      component: () => import('../views/PressSetup.vue')
    },
    {
      path: '/:locale(en|nl)/press/game/:id',
      name: 'PressGame',
      component: () => import('../views/PressGame.vue')
    },
    {
      path: '/:locale(en|nl)/leaderboard',
      name: 'Leaderboard',
      component: () => import('../views/Leaderboard.vue')
    },
    {
      path: '/:locale(en|nl)/leaderboard/records/:mode/:scope',
      name: 'LeaderboardRecordsBoard',
      component: () => import('../views/LeaderboardRecordBoard.vue')
    },
    {
      path: '/:locale(en|nl)/leaderboard/:board',
      name: 'LeaderboardBoard',
      component: () => import('../views/LeaderboardBoard.vue')
    },
    {
      path: '/:locale(en|nl)/players',
      name: 'PlayerSearch',
      component: () => import('../views/PlayerSearch.vue')
    },
    {
      path: '/:locale(en|nl)/players/:playerName',
      name: 'PlayerDetails',
      component: () => import('../views/PlayerDetails.vue')
    },
    {
      path: '/:locale(en|nl)/settings',
      name: 'Settings',
      component: () => import('../views/Settings.vue')
    },
    {
      path: '/',
      redirect: '/nl'
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: () => import('../views/NotFound.vue')
    }
  ]
});

router.beforeEach((to: any) => {
  if (to.name === 'not-found') {
    document.title = i18n.global.t('notFound.title') as string
    return true
  }

  const localeParam = to.params.locale as string | undefined;
  if (localeParam && ['en', 'nl'].includes(localeParam)) {
    // assign to the i18n global locale's value to satisfy types
    (i18n.global.locale as any).value = localeParam as 'en' | 'nl';
  }

  // set a localized document title based on route
  let titleKey = 'gameSelection.title';
  if ((to.name && String(to.name).startsWith('hearts')) || (to.path && String(to.path).includes('/hearts'))) {
    titleKey = 'heartsGame.title';
  } else if ((to.name && String(to.name).toLowerCase().includes('leaderboard')) || (to.path && String(to.path).includes('/leaderboard'))) {
    titleKey = 'leaderboard.title';
  } else if ((to.name && String(to.name).toLowerCase().includes('player')) || (to.path && String(to.path).includes('/players'))) {
    titleKey = to.name === 'PlayerSearch' ? 'player.searchTitle' : 'player.title';
  } else if ((to.name && String(to.name).toLowerCase().includes('colorwhist')) || (to.path && String(to.path).includes('/color-whist'))) {
    titleKey = 'colorWhistGame.title';
  } else if ((to.name && String(to.name).toLowerCase().includes('whist') && !String(to.name).toLowerCase().includes('colorwhist')) || (to.path && String(to.path).includes('/whist') && !String(to.path).includes('/color-whist'))) {
    titleKey = 'whistGame.title';
  } else if ((to.name && String(to.name).toLowerCase().includes('manille')) || (to.path && String(to.path).includes('/manille'))) {
    titleKey = 'manilleGame.title';
  } else if ((to.name && String(to.name).toLowerCase().includes('press')) || (to.path && String(to.path).includes('/press'))) {
    titleKey = 'pressGame.title';
  } else if ((to.name && String(to.name).toLowerCase().includes('king')) || (to.path && String(to.path).includes('/king'))) {
    // For both King setup and game pages use the kingGame.title localization
    titleKey = 'kingGame.title';
  } else if ((to.name && String(to.name).toLowerCase().includes('doubleking')) || (to.path && String(to.path).includes('/double-king'))) {
    // For both Double King setup and game pages use the doubleKingGame.title localization
    titleKey = 'doubleKingGame.title';
  }
  const localizedTitle = i18n.global.t(titleKey) as string;
  if (localizedTitle) document.title = localizedTitle;

  return true;
});

export default router
