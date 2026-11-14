interface LoginBody {
  email?: string
  password?: string
}

interface BackendLoginResponse {
  token: string
  user: unknown
}

/**
 * Вход в систему через BFF.
 *
 * Nuxt-сервер получает JWT от бэкенда, кладёт его в httpOnly-cookie и отдаёт
 * браузеру только профиль пользователя. Токен недоступен JavaScript.
 */
export default defineEventHandler(async (event) => {
  assertSameOrigin(event)

  const body = await readBody<LoginBody>(event)
  const email = body?.email?.trim()
  const password = body?.password

  if (!email || !password) {
    throw createError({ statusCode: 400, message: 'Укажите email и пароль' })
  }

  try {
    const response = await $fetch<BackendLoginResponse>(`${backendBase()}/auth/login`, {
      method: 'POST',
      body: { email, password },
    })

    setSessionCookie(event, response.token)

    return { user: response.user }
  } catch (error) {
    toHttpError(error)
  }
})
