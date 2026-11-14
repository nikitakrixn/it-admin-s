/** Завершает сессию: удаляет httpOnly-cookie с токеном. */
export default defineEventHandler((event) => {
  assertSameOrigin(event)
  deleteCookie(event, 'auth_token', { path: '/' })
  return { message: 'Вы вышли из системы' }
})
