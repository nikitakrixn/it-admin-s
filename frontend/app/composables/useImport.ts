export type ImportEntity = 'employees' | 'computers'

export interface ImportRowError {
  /** Номер строки в файле, считая заголовок. */
  row: number
  message: string
}

export interface ImportReport {
  entity: ImportEntity
  dry_run: boolean
  total_rows: number
  created: number
  updated: number
  skipped: number
  errors: ImportRowError[]
}

/**
 * Шаблоны CSV с точным набором колонок, которые понимает бэкенд.
 * Разделитель — точка с запятой: так файл открывается в Excel с русской
 * локалью без «мастера импорта». Строк с данными нет специально: пример
 * в шаблоне рано или поздно попал бы в базу.
 */
export const IMPORT_TEMPLATES: Record<ImportEntity, string> = {
  employees:
    'Фамилия;Имя;Отчество;Email;Телефон;Логин AD;Должность;Подразделение;Дата приёма;Статус;Заметки\n',
  computers:
    'Инвентарный номер;Имя компьютера;Тип;Производитель;Модель;Серийный номер;ОС;Версия ОС;Email сотрудника;Расположение;Дата покупки;Гарантия до;Статус;Заметки\n',
}

export const IMPORT_EXAMPLES: Record<ImportEntity, string> = {
  employees:
    'Иванов;Иван;Иванович;ivanov@company.ru;+7 (495) 000-00-00;ivanov;Системный администратор;IT отдел;01.01.2024;active;Комментарий',
  computers:
    'PC-001;pc-001;Ноутбук;Dell;Latitude 5540;SN-001;Windows 11;23H2;ivanov@company.ru;Кабинет 101;01.01.2024;01.01.2027;active;Комментарий',
}

export const useImport = () => {
  const { $api } = useNuxtApp()

  /**
   * Отправляет содержимое файла на разбор. `dryRun` включает режим проверки:
   * бэкенд возвращает такой же отчёт, но ничего не записывает в базу.
   */
  const importCsv = (entity: ImportEntity, csv: string, dryRun: boolean) =>
    $api<ImportReport>(`/import/${entity}`, {
      method: 'POST',
      body: { csv, dry_run: dryRun },
    })

  return { importCsv }
}
