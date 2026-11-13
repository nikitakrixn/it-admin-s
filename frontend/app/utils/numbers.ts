/**
 * Приводит значение поля ввода к числу.
 * Пустая строка и нечисловые значения превращаются в `undefined`,
 * чтобы не отправлять их на бэкенд.
 */
export function toOptionalNumber(value: number | string | null | undefined): number | undefined {
  if (value === null || value === undefined || value === '') return undefined

  const parsed = typeof value === 'number' ? value : Number(value)
  return Number.isFinite(parsed) ? parsed : undefined
}
