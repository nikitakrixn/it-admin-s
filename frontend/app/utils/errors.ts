/**
 * Извлекает человекочитаемое сообщение из ошибки любого типа.
 *
 * В async-обработчиках ошибка приходит как `unknown`, а API может вернуть
 * как `Error`, так и строку, или вовсе ничего — отсюда и такая сигнатура.
 */
export function getErrorMessage(error: unknown, fallback = 'Неизвестная ошибка'): string {
  if (typeof error === 'string' && error) {
    return error
  }

  if (error && typeof error === 'object') {
    const candidate = error as { message?: unknown; data?: { message?: unknown } }

    // Ошибки ofetch держат разобранный ответ сервера в `data` — это
    // единственный источник дружелюбного сообщения для пользователя.
    if (typeof candidate.data?.message === 'string' && candidate.data.message) {
      return candidate.data.message
    }
  }

  if (error instanceof Error && error.message) {
    return error.message
  }

  if (error && typeof error === 'object') {
    const message = (error as { message?: unknown }).message
    if (typeof message === 'string' && message) {
      return message
    }
  }

  return fallback
}

/**
 * Возвращает HTTP-статус из ошибки запроса или `undefined`, если определить
 * его невозможно (например, это была не ошибка сети, а ошибка приложения).
 */
export function getErrorStatus(error: unknown): number | undefined {
  if (!error || typeof error !== 'object') {
    return undefined
  }

  const candidate = error as {
    statusCode?: unknown
    status?: unknown
    response?: { status?: unknown }
  }

  for (const value of [candidate.statusCode, candidate.status, candidate.response?.status]) {
    if (typeof value === 'number') {
      return value
    }
  }

  return undefined
}
