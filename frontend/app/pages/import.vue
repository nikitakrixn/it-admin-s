<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-indigo-500 to-indigo-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:upload-cloud-2-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Импорт из CSV</h1>
            <p class="mt-1 text-gray-600">
              Загрузка сотрудников и компьютеров из выгрузок Excel и 1С
            </p>
          </div>
        </div>
        <button class="btn btn-secondary" @click="downloadTemplate">
          <Icon name="ri:download-2-line" class="mr-2" />
          Скачать шаблон
        </button>
      </div>
    </div>

    <!-- Tabs -->
    <div class="border-b border-gray-200">
      <nav class="-mb-px flex space-x-8">
        <button
          v-for="tab in tabs"
          :key="tab.value"
          :class="[
            'flex items-center gap-2 py-4 px-1 border-b-2 font-medium text-sm transition-colors',
            activeTab === tab.value
              ? 'border-indigo-500 text-indigo-600'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300',
          ]"
          @click="activeTab = tab.value"
        >
          <Icon :name="tab.icon" class="text-lg" />
          {{ tab.label }}
        </button>
      </nav>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Загрузка файла -->
      <div class="lg:col-span-2 space-y-6">
        <div class="card">
          <div class="p-6">
            <div class="flex items-start justify-between gap-4 mb-4">
              <div>
                <h2 class="text-lg font-semibold text-gray-900">
                  Файл: {{ entityLabel.toLowerCase() }}
                </h2>
                <p class="mt-1 text-sm text-gray-600">
                  CSV с разделителем «;», «,» или табуляцией. Кодировка UTF-8 или Windows-1251.
                </p>
              </div>
              <span class="badge bg-indigo-100 text-indigo-700 whitespace-nowrap"> до 10 МБ </span>
            </div>

            <div
              :class="[
                'relative rounded-xl border-2 border-dashed p-8 text-center transition-colors cursor-pointer',
                isDragging
                  ? 'border-indigo-500 bg-indigo-50'
                  : 'border-gray-300 hover:border-indigo-400 hover:bg-gray-50',
              ]"
              @click="fileInput?.click()"
              @dragover.prevent="isDragging = true"
              @dragleave.prevent="isDragging = false"
              @drop.prevent="handleDrop"
            >
              <input
                ref="fileInput"
                type="file"
                accept=".csv,.txt,text/csv,text/plain"
                class="hidden"
                @change="handleFileInput"
              />
              <Icon name="ri:file-upload-line" class="text-4xl text-indigo-500 mb-3" />
              <p class="text-sm font-medium text-gray-900">
                {{
                  reading ? 'Чтение файла...' : 'Перетащите файл сюда или нажмите, чтобы выбрать'
                }}
              </p>
              <p class="mt-1 text-xs text-gray-500">.csv или .txt</p>
            </div>

            <div
              v-if="fileName"
              class="mt-4 flex items-center justify-between gap-3 p-3 rounded-lg bg-gray-50 border border-gray-200"
            >
              <div class="flex items-center gap-3 min-w-0">
                <div class="h-10 w-10 rounded-lg bg-indigo-100 flex items-center justify-center">
                  <Icon name="ri:file-text-line" class="text-indigo-600" />
                </div>
                <div class="min-w-0">
                  <p class="text-sm font-medium text-gray-900 truncate">{{ fileName }}</p>
                  <p class="text-xs text-gray-500">
                    {{ fileSizeLabel }}
                    <span v-if="encodingNotice"> · {{ encodingNotice }}</span>
                  </p>
                </div>
              </div>
              <button
                type="button"
                class="p-2 rounded-lg text-gray-400 hover:text-red-600 hover:bg-red-50"
                title="Убрать файл"
                @click.stop="resetFile"
              >
                <Icon name="ri:close-line" class="text-lg" />
              </button>
            </div>

            <div
              v-if="errorMessage"
              class="mt-4 p-4 rounded-lg bg-red-50 text-red-800 text-sm flex items-start gap-2"
            >
              <Icon name="ri:error-warning-line" class="text-lg flex-shrink-0 mt-0.5" />
              <span>{{ errorMessage }}</span>
            </div>

            <div class="mt-6 flex flex-col sm:flex-row gap-3">
              <button
                type="button"
                class="btn btn-secondary flex-1"
                :disabled="!csvContent || checking || importing || reading"
                @click="runImport(true)"
              >
                <Icon
                  :name="checking ? 'ri:loader-4-line' : 'ri:search-eye-line'"
                  :class="['mr-2', checking && 'animate-spin']"
                />
                {{ checking ? 'Проверка...' : 'Проверить файл' }}
              </button>
              <button
                type="button"
                class="btn btn-primary flex-1"
                :disabled="!canImport || importing || reading"
                @click="runImport(false)"
              >
                <Icon
                  :name="importing ? 'ri:loader-4-line' : 'ri:upload-2-line'"
                  :class="['mr-2', importing && 'animate-spin']"
                />
                {{ importing ? 'Импорт...' : 'Импортировать' }}
              </button>
            </div>

            <p v-if="csvContent && !report" class="mt-3 text-xs text-gray-500">
              Сначала проверьте файл: отчёт покажет, что будет создано и обновлено, ничего не меняя
              в базе.
            </p>
          </div>
        </div>

        <!-- Отчёт -->
        <div v-if="report" class="card">
          <div class="p-6 border-b border-gray-200">
            <div class="flex items-center justify-between gap-3">
              <div class="flex items-center gap-3">
                <div
                  :class="[
                    'h-10 w-10 rounded-lg flex items-center justify-center',
                    report.dry_run ? 'bg-amber-100' : 'bg-green-100',
                  ]"
                >
                  <Icon
                    :name="report.dry_run ? 'ri:search-eye-line' : 'ri:checkbox-circle-line'"
                    :class="['text-xl', report.dry_run ? 'text-amber-600' : 'text-green-600']"
                  />
                </div>
                <div>
                  <h2 class="text-lg font-semibold text-gray-900">
                    {{ report.dry_run ? 'Предварительный отчёт' : 'Импорт выполнен' }}
                  </h2>
                  <p class="text-sm text-gray-500">
                    {{ entityLabel }} · строк в файле: {{ report.total_rows }}
                  </p>
                </div>
              </div>
              <span
                :class="[
                  'badge',
                  report.dry_run ? 'bg-amber-100 text-amber-800' : 'bg-green-100 text-green-800',
                ]"
              >
                {{ report.dry_run ? 'Проверка' : 'Готово' }}
              </span>
            </div>
          </div>

          <div class="p-6">
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div class="p-4 rounded-lg bg-gray-50">
                <p class="text-xs text-gray-500">Строк</p>
                <p class="mt-1 text-2xl font-bold text-gray-900">{{ report.total_rows }}</p>
              </div>
              <div class="p-4 rounded-lg bg-green-50">
                <p class="text-xs text-green-700">
                  {{ report.dry_run ? 'Будет создано' : 'Создано' }}
                </p>
                <p class="mt-1 text-2xl font-bold text-green-700">{{ report.created }}</p>
              </div>
              <div class="p-4 rounded-lg bg-blue-50">
                <p class="text-xs text-blue-700">
                  {{ report.dry_run ? 'Будет обновлено' : 'Обновлено' }}
                </p>
                <p class="mt-1 text-2xl font-bold text-blue-700">{{ report.updated }}</p>
              </div>
              <div class="p-4 rounded-lg bg-red-50">
                <p class="text-xs text-red-700">Пропущено</p>
                <p class="mt-1 text-2xl font-bold text-red-700">{{ report.skipped }}</p>
              </div>
            </div>

            <div
              v-if="report.dry_run && report.created + report.updated > 0"
              class="mt-4 p-4 rounded-lg bg-amber-50 border border-amber-200 text-sm text-amber-800 flex items-start gap-2"
            >
              <Icon name="ri:information-line" class="text-lg flex-shrink-0 mt-0.5" />
              <span>
                Это проверка: в базе ничего не изменилось. Нажмите «Импортировать», чтобы применить
                результат.
              </span>
            </div>

            <div
              v-else-if="!report.dry_run"
              class="mt-4 p-4 rounded-lg bg-green-50 border border-green-200 text-sm text-green-800 flex items-start gap-2"
            >
              <Icon name="ri:checkbox-circle-line" class="text-lg flex-shrink-0 mt-0.5" />
              <span>
                Данные записаны. Событие отмечено в журнале действий, при необходимости файл можно
                импортировать повторно — дубликатов не будет.
              </span>
            </div>

            <div v-if="report.errors.length > 0" class="mt-6">
              <div class="flex items-center justify-between mb-2">
                <h3 class="text-sm font-semibold text-gray-900">
                  Проблемные строки ({{ report.errors.length }})
                </h3>
                <span class="text-xs text-gray-500">Остальные строки обработаны</span>
              </div>
              <div class="rounded-lg border border-gray-200 overflow-hidden">
                <table class="w-full text-sm">
                  <thead class="bg-gray-50">
                    <tr>
                      <th class="px-4 py-2 text-left text-xs font-medium text-gray-500 w-24">
                        Строка
                      </th>
                      <th class="px-4 py-2 text-left text-xs font-medium text-gray-500">Ошибка</th>
                    </tr>
                  </thead>
                  <tbody class="divide-y divide-gray-100">
                    <tr v-for="error in visibleErrors" :key="error.row + error.message">
                      <td class="px-4 py-2 text-gray-500">{{ error.row }}</td>
                      <td class="px-4 py-2 text-gray-800">{{ error.message }}</td>
                    </tr>
                  </tbody>
                </table>
              </div>
              <p
                v-if="report.errors.length > visibleErrors.length"
                class="mt-2 text-xs text-gray-500"
              >
                Показаны первые {{ visibleErrors.length }} из {{ report.errors.length }} ошибок.
              </p>
            </div>

            <button
              v-if="!report.dry_run"
              type="button"
              class="btn btn-secondary mt-6"
              @click="resetFile"
            >
              <Icon name="ri:refresh-line" class="mr-2" />
              Импортировать ещё файл
            </button>
          </div>
        </div>
      </div>

      <!-- Справка -->
      <div class="space-y-6">
        <div class="card">
          <div class="p-6">
            <h2 class="text-base font-semibold text-gray-900 mb-3">Как это работает</h2>
            <ol class="space-y-3 text-sm text-gray-600">
              <li class="flex gap-3">
                <span
                  class="h-6 w-6 rounded-full bg-indigo-100 text-indigo-700 text-xs font-bold flex items-center justify-center flex-shrink-0"
                  >1</span
                >
                <span>Скачайте шаблон и заполните его в Excel или 1С.</span>
              </li>
              <li class="flex gap-3">
                <span
                  class="h-6 w-6 rounded-full bg-indigo-100 text-indigo-700 text-xs font-bold flex items-center justify-center flex-shrink-0"
                  >2</span
                >
                <span>Загрузите файл и нажмите «Проверить файл» — база не изменится.</span>
              </li>
              <li class="flex gap-3">
                <span
                  class="h-6 w-6 rounded-full bg-indigo-100 text-indigo-700 text-xs font-bold flex items-center justify-center flex-shrink-0"
                  >3</span
                >
                <span>Если отчёт устраивает, нажмите «Импортировать».</span>
              </li>
            </ol>
          </div>
        </div>

        <div class="card">
          <div class="p-6">
            <h2 class="text-base font-semibold text-gray-900 mb-2">Правила сопоставления</h2>
            <p class="text-sm text-gray-600">
              {{ activeTab === 'employees' ? employeeRules : computerRules }}
            </p>
            <div class="mt-4">
              <p class="text-xs font-medium text-gray-500 uppercase tracking-wide mb-1">
                Пример строки
              </p>
              <pre
                class="p-3 rounded-lg bg-gray-900 text-gray-100 text-xs overflow-x-auto whitespace-pre"
                >{{ IMPORT_EXAMPLES[activeTab] }}</pre>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="p-6">
            <h2 class="text-base font-semibold text-gray-900 mb-2">Обязательные колонки</h2>
            <ul class="space-y-1 text-sm text-gray-600">
              <template v-if="activeTab === 'employees'">
                <li class="flex items-center gap-2">
                  <Icon name="ri:check-line" class="text-green-600" /> Фамилия
                </li>
                <li class="flex items-center gap-2">
                  <Icon name="ri:check-line" class="text-green-600" /> Имя
                </li>
                <li class="flex items-center gap-2">
                  <Icon name="ri:check-line" class="text-green-600" /> Email или логин AD
                </li>
              </template>
              <template v-else>
                <li class="flex items-center gap-2">
                  <Icon name="ri:check-line" class="text-green-600" /> Инвентарный номер или имя
                  компьютера
                </li>
              </template>
            </ul>
            <p class="mt-3 text-xs text-gray-500">
              Остальные колонки необязательны. Даты — в формате ГГГГ-ММ-ДД или ДД.ММ.ГГГГ.
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ImportEntity, ImportReport } from '~/composables/useImport'
import { IMPORT_EXAMPLES, IMPORT_TEMPLATES } from '~/composables/useImport'

definePageMeta({
  middleware: 'admin',
})

useHead({
  title: 'Импорт из CSV',
})

const toast = useToast()
const { importCsv } = useImport()

const MAX_FILE_BYTES = 10 * 1024 * 1024
const MAX_VISIBLE_ERRORS = 50

const tabs: { value: ImportEntity; label: string; icon: string }[] = [
  { value: 'employees', label: 'Сотрудники', icon: 'ri:team-line' },
  { value: 'computers', label: 'Компьютеры', icon: 'ri:computer-line' },
]

const activeTab = ref<ImportEntity>('employees')
const fileInput = ref<HTMLInputElement | null>(null)
const fileName = ref('')
const fileSize = ref(0)
const csvContent = ref('')
const encodingNotice = ref('')
const isDragging = ref(false)
const reading = ref(false)
const checking = ref(false)
const importing = ref(false)
const report = ref<ImportReport | null>(null)
const errorMessage = ref('')

const entityLabel = computed(() => (activeTab.value === 'employees' ? 'Сотрудники' : 'Компьютеры'))

const employeeRules =
  'Строка ищется по email (без учёта регистра), затем по логину AD. Если совпадений нет — создаётся сотрудник. Строки без email и логина отклоняются, чтобы повторный импорт не плодил дубликаты. Подразделения и должности создаются автоматически по названию.'

const computerRules =
  'Строка ищется по инвентарному номеру, затем по имени компьютера. Ответственный ищется по email среди уже существующих сотрудников, поэтому сотрудников импортируйте первыми.'

const fileSizeLabel = computed(() => {
  if (fileSize.value < 1024) return `${fileSize.value} Б`
  if (fileSize.value < 1024 * 1024) return `${Math.round(fileSize.value / 1024)} КБ`
  return `${(fileSize.value / (1024 * 1024)).toFixed(1)} МБ`
})

const visibleErrors = computed(() => report.value?.errors.slice(0, MAX_VISIBLE_ERRORS) ?? [])

const canImport = computed(() => {
  const current = report.value
  if (!current || !current.dry_run) return false
  return current.created + current.updated > 0
})

/**
 * Читает файл как текст. Сначала пробуем UTF-8: если байты не складываются
 * в валидную последовательность, значит это выгрузка из 1С в Windows-1251 —
 * без запасного варианта кириллица превратилась бы в «кракозябры».
 */
const readFile = async (file: File) => {
  errorMessage.value = ''

  if (file.size === 0) {
    errorMessage.value = 'Файл пуст'
    return
  }
  if (file.size > MAX_FILE_BYTES) {
    errorMessage.value = 'Файл больше 10 МБ — разделите его на части'
    return
  }

  reading.value = true
  try {
    const buffer = await file.arrayBuffer()
    let text: string
    let notice = ''

    try {
      text = new TextDecoder('utf-8', { fatal: true }).decode(buffer)
    } catch {
      text = new TextDecoder('windows-1251').decode(buffer)
      notice = 'прочитан как Windows-1251'
    }

    if (!text.trim()) {
      errorMessage.value = 'Файл пуст'
      return
    }

    fileName.value = file.name
    fileSize.value = file.size
    csvContent.value = text
    encodingNotice.value = notice
    report.value = null
  } catch {
    errorMessage.value = 'Не удалось прочитать файл'
  } finally {
    reading.value = false
  }
}

const handleFileInput = async (event: Event) => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  // Сбрасываем input, чтобы повторный выбор того же файла снова вызывал change.
  target.value = ''
  if (file) await readFile(file)
}

const handleDrop = async (event: DragEvent) => {
  isDragging.value = false
  const file = event.dataTransfer?.files?.[0]
  if (file) await readFile(file)
}

const resetFile = () => {
  fileName.value = ''
  fileSize.value = 0
  csvContent.value = ''
  encodingNotice.value = ''
  report.value = null
  errorMessage.value = ''
}

const runImport = async (dryRun: boolean) => {
  if (!csvContent.value) return

  if (dryRun) {
    checking.value = true
  } else {
    importing.value = true
  }
  errorMessage.value = ''

  try {
    const result = await importCsv(activeTab.value, csvContent.value, dryRun)
    report.value = result

    if (dryRun) {
      toast.success(
        `Проверка завершена: создано ${result.created}, обновлено ${result.updated}, пропущено ${result.skipped}`,
      )
    } else {
      toast.success(
        `Импорт завершён: создано ${result.created}, обновлено ${result.updated}, пропущено ${result.skipped}`,
      )
    }
  } catch (error) {
    errorMessage.value = getErrorMessage(error, 'Не удалось обработать файл')
    toast.error(errorMessage.value)
  } finally {
    checking.value = false
    importing.value = false
  }
}

const downloadTemplate = () => {
  // BOM нужен Excel: без него русские заголовки откроются в неправильной кодировке.
  const content = '\uFEFF' + IMPORT_TEMPLATES[activeTab.value].replace(/\n/g, '\r\n')
  const blob = new Blob([content], { type: 'text/csv;charset=utf-8' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download =
    activeTab.value === 'employees' ? 'employees-template.csv' : 'computers-template.csv'
  link.click()
  URL.revokeObjectURL(url)
  toast.success('Шаблон скачан')
}

watch(activeTab, () => {
  resetFile()
})
</script>
