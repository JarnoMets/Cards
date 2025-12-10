<template>
  <footer class="site-footer">
    <p>
      <span>{{ t('footer.bugPrompt') }}</span>
      <a :href="bugMailto">{{ bugEmail }}</a>
    </p>
  </footer>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const bugEmail = 'bugs@jarnomets.com'
const bugMailto = `mailto:${bugEmail}`
</script>

<style scoped>
.site-footer {
  margin-top: auto;
  background: linear-gradient(140deg, rgba(255, 255, 255, 0.92), rgba(249, 236, 214, 0.92));
  color: var(--ink);
  padding: 18px 24px;
  text-align: center;
  font-size: 0.9rem;
  letter-spacing: 0.02em;
  border-top: 1px dashed rgba(45, 31, 26, 0.2);
  box-shadow: 0 -12px 28px rgba(33, 22, 12, 0.12);
}

.site-footer p {
  margin: 0;
  display: inline-flex;
  gap: 8px;
  align-items: center;
  flex-wrap: wrap;
  justify-content: center;
  font-family: var(--font-heading);
}

.site-footer a {
  color: var(--primary-color);
  font-weight: 600;
  text-decoration: none;
  border-bottom: 1px solid transparent;
}

.site-footer a:hover,
.site-footer a:focus-visible {
  border-bottom-color: currentColor;
}
</style>
