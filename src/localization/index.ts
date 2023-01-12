import { createI18n } from 'vue-i18n'
import messages from './languages'

export const i18n = createI18n({
  legacy: false,
  globalInjection: true,
  locale: 'ru',
  fallbackLocale: 'ru',
  messages,
})