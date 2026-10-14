<template>
  <div class="not-found">
    <p class="code">404</p>
    <h1>{{ t('notFound.title') }}</h1>
    <p class="body">{{ t('notFound.body') }}</p>
    <router-link class="home-link" :to="`/${locale}`">
      {{ t('goHome') }}
    </router-link>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

const { t, locale: i18nLocale } = useI18n()
const locale = computed(() => (i18nLocale.value === 'en' ? 'en' : 'nl'))
</script>

<style scoped>
.not-found {
  flex: 1;
  display: grid;
  place-content: center;
  text-align: center;
  padding: 2rem 1.25rem 4rem;
  gap: 0.75rem;
}

.code {
  margin: 0;
  font-family: 'Playfair Display', serif;
  font-size: clamp(3.5rem, 10vw, 5.5rem);
  line-height: 1;
  color: rgba(125, 73, 58, 0.35);
}

h1 {
  margin: 0;
  font-family: 'Playfair Display', serif;
  font-size: clamp(1.6rem, 4vw, 2.2rem);
  color: var(--ink, #2c241c);
}

.body {
  margin: 0 auto 0.5rem;
  max-width: 28rem;
  color: var(--ink-muted, #7a6a5c);
  line-height: 1.55;
}

.home-link {
  justify-self: center;
  display: inline-flex;
  align-items: center;
  padding: 0.7rem 1.3rem;
  border-radius: 999px;
  background: #7d493a;
  color: #fffdf8;
  text-decoration: none;
  font-weight: 600;
}

.home-link:hover {
  background: #63392e;
}
</style>
