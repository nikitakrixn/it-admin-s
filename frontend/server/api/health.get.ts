/** Проверка живости Nuxt-сервера для Docker healthcheck. */
export default defineEventHandler(() => ({ status: 'ok' }))
