/**
 * CSV для Excel с русской локалью.
 *
 * Разделитель — точка с запятой, как в шаблонах импорта: Excel с русской
 * локалью открывает такой файл без «мастера импорта».
 */
const DELIMITER = ';'

/** Excel без BOM читает UTF-8 как ANSI и ломает кириллицу. */
const BOM = '\uFEFF'

const escapeCell = (value: string) =>
  /[";\n\r]/.test(value) ? `"${value.replace(/"/g, '""')}"` : value

export const toCsv = (rows: string[][]): string =>
  `${rows.map((row) => row.map(escapeCell).join(DELIMITER)).join('\r\n')}\r\n`

/** Отдаёт текст браузеру как файл. */
export const downloadCsv = (filename: string, content: string) => {
  const url = URL.createObjectURL(new Blob([BOM + content], { type: 'text/csv;charset=utf-8' }))
  const link = document.createElement('a')
  link.href = url
  link.download = filename
  // Ссылка должна быть в документе: Safari игнорирует click() у detached-элемента.
  document.body.appendChild(link)
  link.click()
  link.remove()
  // Отзываем ссылку следующим тиком: Safari может отменить загрузку,
  // если освободить URL сразу после click().
  window.setTimeout(() => URL.revokeObjectURL(url), 0)
}
