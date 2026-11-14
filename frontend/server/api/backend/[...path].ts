/**
 * Прокси всех запросов к API через Nuxt-сервер.
 *
 * Браузер обращается к `/api/backend/...` без токена: Nuxt-сервер берёт JWT
 * из httpOnly-cookie и подставляет заголовок Authorization сам.
 */
export default defineEventHandler(async (event) => {
  assertSameOrigin(event)

  const token = getCookie(event, 'auth_token')

  if (!token) {
    throw createError({ statusCode: 401, message: 'Требуется авторизация' })
  }

  const path = getRouterParam(event, 'path') ?? ''
  const target = `${backendBase()}/${path}${getRequestURL(event).search}`

  await proxyRequest(event, target, {
    headers: { Authorization: `Bearer ${token}` },
    onResponse(_event, response) {
      // Токен истёк или отозван — удаляем cookie, чтобы клиент ушёл на вход.
      if (response.status === 401) {
        deleteCookie(event, 'auth_token', { path: '/' })
      }
    },
  })
})
