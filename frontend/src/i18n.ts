import { createI18n } from 'vue-i18n';

// Base translations
import en from './locales/en.json';
import nl from './locales/nl.json';

// Common/shared translations
import commonEn from './locales/common.en.json';
import commonNl from './locales/common.nl.json';

// Page translations (root level)
import gameSelectionEn from './locales/gameSelection.en.json';
import gameSelectionNl from './locales/gameSelection.nl.json';
import leaderboardEn from './locales/leaderboard.en.json';
import leaderboardNl from './locales/leaderboard.nl.json';
import playerEn from './locales/player.en.json';
import playerNl from './locales/player.nl.json';

// Game translations
import heartsGameEn from './locales/games/heartsGame.en.json';
import heartsGameNl from './locales/games/heartsGame.nl.json';
import kingGameEn from './locales/games/kingGame.en.json';
import kingGameNl from './locales/games/kingGame.nl.json';
import doubleKingGameEn from './locales/games/doubleKingGame.en.json';
import doubleKingGameNl from './locales/games/doubleKingGame.nl.json';
import colorWhistGameEn from './locales/games/colorWhistGame.en.json';
import colorWhistGameNl from './locales/games/colorWhistGame.nl.json';
import whistGameEn from './locales/games/whistGame.en.json';
import whistGameNl from './locales/games/whistGame.nl.json';
import manilleGameEn from './locales/games/manilleGame.en.json';
import manilleGameNl from './locales/games/manilleGame.nl.json';
import pressGameEn from './locales/games/pressGame.en.json';
import pressGameNl from './locales/games/pressGame.nl.json';

// Setup translations
import heartsSetupEn from './locales/setup/heartsSetup.en.json';
import heartsSetupNl from './locales/setup/heartsSetup.nl.json';
import kingSetupEn from './locales/setup/kingSetup.en.json';
import kingSetupNl from './locales/setup/kingSetup.nl.json';
import doubleKingSetupEn from './locales/setup/doubleKingSetup.en.json';
import doubleKingSetupNl from './locales/setup/doubleKingSetup.nl.json';
import colorWhistSetupEn from './locales/setup/colorWhistSetup.en.json';
import colorWhistSetupNl from './locales/setup/colorWhistSetup.nl.json';
import whistSetupEn from './locales/setup/whistSetup.en.json';
import whistSetupNl from './locales/setup/whistSetup.nl.json';
import manilleSetupEn from './locales/setup/manilleSetup.en.json';
import manilleSetupNl from './locales/setup/manilleSetup.nl.json';
import pressSetupEn from './locales/setup/pressSetup.en.json';
import pressSetupNl from './locales/setup/pressSetup.nl.json';

// Rules translations
import heartsRulesEn from './locales/rules/heartsRules.en.json';
import heartsRulesNl from './locales/rules/heartsRules.nl.json';
import kingRulesEn from './locales/rules/kingRules.en.json';
import kingRulesNl from './locales/rules/kingRules.nl.json';
import doubleKingRulesEn from './locales/rules/doubleKingRules.en.json';
import doubleKingRulesNl from './locales/rules/doubleKingRules.nl.json';
import colorWhistRulesEn from './locales/rules/colorWhistRules.en.json';
import colorWhistRulesNl from './locales/rules/colorWhistRules.nl.json';
import whistRulesEn from './locales/rules/whistRules.en.json';
import whistRulesNl from './locales/rules/whistRules.nl.json';
import manilleRulesEn from './locales/rules/manilleRules.en.json';
import manilleRulesNl from './locales/rules/manilleRules.nl.json';
import pressRulesEn from './locales/rules/pressRules.en.json';
import pressRulesNl from './locales/rules/pressRules.nl.json';

const messages = {
  en: {
    ...en,
    ...commonEn,
    // Pages
    gameSelection: gameSelectionEn,
    leaderboard: leaderboardEn,
    player: playerEn,
    // Games
    heartsGame: heartsGameEn,
    kingGame: kingGameEn,
    doubleKingGame: doubleKingGameEn,
    colorWhistGame: colorWhistGameEn,
    whistGame: whistGameEn,
    manilleGame: manilleGameEn,
    pressGame: pressGameEn,
    // Setup
    heartsSetup: heartsSetupEn,
    kingSetup: kingSetupEn,
    doubleKingSetup: doubleKingSetupEn,
    colorWhistSetup: colorWhistSetupEn,
    whistSetup: whistSetupEn,
    manilleSetup: manilleSetupEn,
    pressSetup: pressSetupEn,
    // Rules
    heartsRules: heartsRulesEn,
    kingRules: kingRulesEn,
    doubleKingRules: doubleKingRulesEn,
    colorWhistRules: colorWhistRulesEn,
    whistRules: whistRulesEn,
    manilleRules: manilleRulesEn,
    pressRules: pressRulesEn,
  },
  nl: {
    ...nl,
    ...commonNl,
    // Pages
    gameSelection: gameSelectionNl,
    leaderboard: leaderboardNl,
    player: playerNl,
    // Games
    heartsGame: heartsGameNl,
    kingGame: kingGameNl,
    doubleKingGame: doubleKingGameNl,
    colorWhistGame: colorWhistGameNl,
    whistGame: whistGameNl,
    manilleGame: manilleGameNl,
    pressGame: pressGameNl,
    // Setup
    heartsSetup: heartsSetupNl,
    kingSetup: kingSetupNl,
    doubleKingSetup: doubleKingSetupNl,
    colorWhistSetup: colorWhistSetupNl,
    whistSetup: whistSetupNl,
    manilleSetup: manilleSetupNl,
    pressSetup: pressSetupNl,
    // Rules
    heartsRules: heartsRulesNl,
    kingRules: kingRulesNl,
    doubleKingRules: doubleKingRulesNl,
    colorWhistRules: colorWhistRulesNl,
    whistRules: whistRulesNl,
    manilleRules: manilleRulesNl,
    pressRules: pressRulesNl,
  },
};

const i18n = createI18n({
  legacy: false,
  globalInjection: true,
  locale: 'nl',
  fallbackLocale: 'nl',
  messages,
});

export default i18n;
