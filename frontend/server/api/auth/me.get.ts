/** Возвращает профиль текущего пользователя по httpOnly-cookie с JWT. */
export default defineEventHandler(async (event) => {
  const token = getCookie(event, 'auth_token')

  if (!token) {
    throw createError({ statusCode: 401, message: 'Требуется авторизация' })
  }

  try {
    return await $fetch(`${backendBase()}/auth/me`, {
      headers: { Authorization: `Bearer ${token}` },
    })
  } catch (error) {
    // Истёкший или отозванный токен больше не нужен — удаляем cookie.
    const status = (error as { response?: { status?: number } }).response?.status
    if (status === 401) {
      deleteCookie(event, 'auth_token', { path: '/' })
    }

    toHttpError(error)
  }
})
