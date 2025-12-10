<template>
  <div class="lang-selector">
    <select
      v-model="selectedLocale"
      @change="handleChange"
      class="lang-dropdown"
      aria-label="Select language"
    >
      <option value="nl">Nederlands</option>
      <option value="en">English</option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'

const router = useRouter()
const route = useRoute()
const { locale } = useI18n()

const selectedLocale = ref<string>(locale.value)

const handleChange = () => {
  if (selectedLocale.value === locale.value) {
    return
  }

  locale.value = selectedLocale.value
  const name = route.name as string | undefined

  if (name) {
    router.push({
      name,
      params: {
        ...route.params,
        locale: selectedLocale.value,
      },
      query: route.query,
    })
  } else {
    router.push({ path: `/${selectedLocale.value}` })
  }
}

watch(locale, (newLocale) => {
  if (newLocale !== selectedLocale.value) {
    selectedLocale.value = newLocale
  }
})
</script>

<style scoped>
.lang-selector {
  display: inline-flex;
  align-items: center;
  justify-content: flex-end;
  min-width: 120px;
  gap: 6px;
}

.lang-dropdown {
  width: 100%;
  padding: 6px 12px;
  border-radius: 6px;
  border: 2px solid #2366a8;
  background: white;
  color: #2366a8;
  font-size: clamp(0.8rem, 1.5vw, 0.95rem);
  font-family: 'Segoe UI', 'Arial', sans-serif;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.lang-dropdown:hover {
  background: #f5f5f5;
  border-color: #1a4d7a;
}

.lang-dropdown:focus {
  outline: none;
  border-color: #1a4d7a;
  box-shadow: 0 0 0 3px rgba(35, 102, 168, 0.1);
}

@media (max-width: 480px) {
  .lang-selector {
    min-width: 110px;
  }
}
</style>
