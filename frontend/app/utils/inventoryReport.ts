import type { InventoryAudit, InventoryItem } from '~/composables/useInventory'
import {
  AUDIT_STATUS_LABELS,
  AUDIT_TYPE_LABELS,
  CONDITION_LABELS,
  ITEM_STATUS_LABELS,
  summarizeInventory,
} from '~/composables/useInventory'
import { toCsv } from '~/utils/csv'
import { formatDate, formatDateTime } from '~/utils/dates'

const COLUMNS = [
  '№',
  'Инвентарный номер',
  'Компьютер',
  'Место по учёту',
  'Фактическое место',
  'Статус',
  'Состояние',
  'Замечания',
  'Проверено',
  'Проверил',
]

/**
 * Собирает отчёт по сессии: шапка с итогами и таблица позиций.
 *
 * Пустые значения оставляем пустыми, а не прочерками: так в Excel работают
 * фильтры и сортировка. Сводку считаем по позициям, потому что audit.counts
 * заполняются только при завершении сессии.
 */
export const buildInventoryReportCsv = (audit: InventoryAudit, items: InventoryItem[]) => {
  const counts = summarizeInventory(items)

  const summary: string[][] = [
    ['Отчёт по инвентаризации'],
    ['Название', audit.audit_name],
    ['Дата', formatDate(audit.audit_date)],
    ['Тип', AUDIT_TYPE_LABELS[audit.audit_type]],
    ['Статус', AUDIT_STATUS_LABELS[audit.status]],
    ['Расположение', audit.location ?? 'все расположения'],
    ['Автор', audit.audited_by_email ?? ''],
    ['Всего позиций', String(counts.total)],
    ['Проверено', String(counts.checked)],
    ['Не проверено', String(counts.total - counts.checked)],
    ['Найдено', String(counts.found)],
    ['Отсутствует', String(counts.missing)],
    ['Повреждено', String(counts.damaged)],
    ['Завершено', audit.completed_at ? formatDateTime(audit.completed_at) : ''],
    [],
  ]

  const rows = items.map((item, index) => [
    String(index + 1),
    item.entity_inventory_number ?? '',
    item.entity_name ?? '',
    item.expected_location ?? '',
    item.actual_location ?? '',
    ITEM_STATUS_LABELS[item.status],
    item.actual_condition ? CONDITION_LABELS[item.actual_condition] : '',
    item.discrepancy_notes ?? '',
    item.checked_at ? formatDateTime(item.checked_at) : '',
    item.checked_by_email ?? '',
  ])

  return toCsv([...summary, COLUMNS, ...rows])
}
