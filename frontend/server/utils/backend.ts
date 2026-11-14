import type { H3Event } from 'h3'

/**
 * Адрес бэкенда для серверных запросов.
 *
 * Браузер общается только с Nuxt-сервером, поэтому адрес API и JWT-токен
 * не попадают в клиентский код.
 */
export const backendBase = () => {
  const config = useRuntimeConfig()
  return String(config.apiBaseServer).replace(/\/+$/, '')
}

/**
 * Проверяет, что изменяющий запрос пришёл с того же источника.
 *
 * Cookie сессии выставлена как httpOnly + SameSite=Lax, что уже отсекает
 * большинство CSRF-сценариев. Проверка Origin добавляет второй барьер
 * для браузеров со старыми настройками и для запросов из iframe.
 */
export const assertSameOrigin = (event: H3Event) => {
  const method = getMethod(event).toUpperCase()
  if (method === 'GET' || method === 'HEAD' || method === 'OPTIONS') {
    return
  }

  const origin = getHeader(event, 'origin')
  if (!origin) {
    // Запросы без Origin (curl, серверные вызовы) не могут использовать cookie
    // браузера, поэтому пропускаем их дальше — их отсечёт проверка токена.
    return
  }

  let originHost: string
  try {
    originHost = new URL(origin).host
  } catch {
    throw createError({ statusCode: 403, message: 'Некорректный заголовок Origin' })
  }

  if (originHost !== getHeader(event, 'host')) {
    throw createError({
      statusCode: 403,
      message: 'Запрос со стороннего источника отклонён',
    })
  }
}

/** Сохраняет JWT в httpOnly-cookie и возвращает браузеру только данные пользователя. */
export const setSessionCookie = (event: H3Event, token: string) => {
  const config = useRuntimeConfig()

  setCookie(event, 'auth_token', token, {
    httpOnly: true,
    sameSite: 'lax',
    secure: config.public.appEnv === 'production',
    path: '/',
    maxAge: 60 * 60 * 24,
  })
}

/** Преобразует ошибку ofetch в HTTP-ошибку Nitro, сохраняя статус и сообщение бэкенда. */
export const toHttpError = (error: unknown): never => {
  const err = error as {
    response?: { status?: number }
    data?: { message?: string }
    statusCode?: number
    statusMessage?: string
  }

  const statusCode = err.response?.status ?? err.statusCode ?? 500
  const message = err.data?.message ?? err.statusMessage ?? 'Ошибка обращения к API'

  throw createError({ statusCode, message, data: err.data })
}
