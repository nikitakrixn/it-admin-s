/** Дата в формате ДД.ММ.ГГГГ; пустое значение — прочерк. */
export const formatDate = (value: string | null | undefined) => {
  if (!value) return '—'

  const [date] = value.split('T')
  const [year, month, day] = date.split('-')
  return year && month && day ? `${day}.${month}.${year}` : date
}

/** Дата и время в часовом поясе пользователя. */
export const formatDateTime = (value: string | null | undefined) => {
  if (!value) return '—'

  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return formatDate(value)

  return date.toLocaleString('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}
