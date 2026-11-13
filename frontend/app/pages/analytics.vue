<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl gradient-secondary flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:bar-chart-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Аналитика</h1>
            <p class="mt-1 text-gray-600">Детальная статистика и отчеты системы</p>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <select v-model="selectedPeriod" class="input-field w-48">
            <option value="7">Последние 7 дней</option>
            <option value="30">Последние 30 дней</option>
            <option value="90">Последние 90 дней</option>
          </select>
          <button
            class="btn btn-primary whitespace-nowrap"
            :disabled="!overview"
            @click="downloadReport"
          >
            <Icon name="ri:file-download-line" class="mr-2" />
            Скачать отчет
          </button>
        </div>
      </div>
    </div>

    <div v-if="loading && !overview" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-indigo-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка аналитики...</p>
    </div>

    <div v-else-if="!overview" class="card p-16 text-center">
      <Icon name="ri:error-warning-line" class="text-4xl text-red-400 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Не удалось загрузить аналитику</h3>
      <p class="text-sm text-gray-500 mb-4">Проверьте подключение к API и попробуйте ещё раз</p>
      <button class="btn btn-primary" @click="loadOverview">Повторить</button>
    </div>

    <template v-else>
      <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
        <div class="card">
          <div class="p-6">
            <div class="flex justify-between items-start mb-4">
              <div class="flex-1">
                <p class="text-sm font-medium text-gray-600 mb-2">Активные сотрудники</p>
                <div class="flex items-baseline gap-2">
                  <p class="text-3xl font-bold text-gray-900">{{ overview.active_employees }}</p>
                  <span class="text-sm font-medium text-gray-500">
                    из {{ overview.total_employees }}
                  </span>
                </div>
              </div>
              <div
                class="flex h-14 w-14 items-center justify-center rounded-xl bg-blue-100 text-blue-600"
              >
                <Icon name="ri:user-line" class="text-2xl" />
              </div>
            </div>
            <div class="pt-4 border-t border-gray-100">
              <div class="flex items-center justify-between text-sm">
                <span class="text-gray-600">В штате</span>
                <span class="font-semibold text-gray-900">
                  {{ share(overview.active_employees, overview.total_employees) }}%
                </span>
              </div>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="p-6">
            <div class="flex justify-between items-start mb-4">
              <div class="flex-1">
                <p class="text-sm font-medium text-gray-600 mb-2">Активные устройства</p>
                <div class="flex items-baseline gap-2">
                  <p class="text-3xl font-bold text-gray-900">{{ overview.active_computers }}</p>
                  <span class="text-sm font-medium text-gray-500">
                    из {{ overview.total_computers }}
                  </span>
                </div>
              </div>
              <div
                class="flex h-14 w-14 items-center justify-center rounded-xl bg-purple-100 text-purple-600"
              >
                <Icon name="ri:computer-line" class="text-2xl" />
              </div>
            </div>
            <div class="pt-4 border-t border-gray-100">
              <div class="flex items-center justify-between text-sm">
                <span class="text-gray-600">Оргтехника</span>
                <span class="font-semibold text-gray-900">{{ overview.total_equipment }}</span>
              </div>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="p-6">
            <div class="flex justify-between items-start mb-4">
              <div class="flex-1">
                <p class="text-sm font-medium text-gray-600 mb-2">Открытые заявки</p>
                <div class="flex items-baseline gap-2">
                  <p class="text-3xl font-bold text-gray-900">{{ overview.open_requests }}</p>
                  <span class="text-sm font-medium text-amber-600">
                    из {{ overview.total_requests }}
                  </span>
                </div>
              </div>
              <div
                class="flex h-14 w-14 items-center justify-center rounded-xl bg-amber-100 text-amber-600"
              >
                <Icon name="ri:time-line" class="text-2xl" />
              </div>
            </div>
            <div class="pt-4 border-t border-gray-100">
              <div class="flex items-center justify-between text-sm">
                <span class="text-gray-600">Создано за период</span>
                <span class="font-semibold text-gray-900">
                  {{ overview.requests_created_in_period }}
                </span>
              </div>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="p-6">
            <div class="flex justify-between items-start mb-4">
              <div class="flex-1">
                <p class="text-sm font-medium text-gray-600 mb-2">Лицензии</p>
                <div class="flex items-baseline gap-2">
                  <p class="text-3xl font-bold text-gray-900">{{ overview.used_licenses }}</p>
                  <span class="text-sm font-medium text-gray-500">
                    из {{ overview.total_licenses }}
                  </span>
                </div>
              </div>
              <div
                class="flex h-14 w-14 items-center justify-center rounded-xl bg-green-100 text-green-600"
              >
                <Icon name="ri:key-line" class="text-2xl" />
              </div>
            </div>
            <div class="pt-4 border-t border-gray-100">
              <div class="flex items-center justify-between text-sm">
                <span class="text-gray-600">Истекает в 30 дней</span>
                <span class="font-semibold text-amber-600">{{ overview.expiring_licenses }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div class="card">
          <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center">
                <Icon name="ri:pie-chart-line" class="text-blue-600 text-xl" />
              </div>
              <h2 class="text-lg font-semibold text-gray-900">Заявки по категориям</h2>
            </div>
          </div>
          <div class="p-6">
            <div v-if="categoryRows.length === 0" class="py-8 text-center">
              <Icon name="ri:inbox-line" class="text-4xl text-gray-300 mb-3" />
              <p class="text-sm text-gray-500">Заявок пока нет</p>
            </div>
            <div v-else class="space-y-4">
              <div v-for="category in categoryRows" :key="category.key">
                <div class="flex items-center justify-between mb-2">
                  <div class="flex items-center gap-2">
                    <div :class="['w-3 h-3 rounded-full', category.color]"></div>
                    <span class="text-sm font-medium text-gray-700">{{ category.label }}</span>
                  </div>
                  <span class="text-sm text-gray-600">
                    {{ category.count }} · {{ category.percentage }}%
                  </span>
                </div>
                <div class="w-full bg-gray-200 rounded-full h-2">
                  <div
                    :class="category.color"
                    :style="{ width: `${category.percentage}%` }"
                    class="h-2 rounded-full transition-all duration-500"
                  ></div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center">
                <Icon name="ri:time-line" class="text-purple-600 text-xl" />
              </div>
              <h2 class="text-lg font-semibold text-gray-900">Среднее время решения</h2>
            </div>
          </div>
          <div class="p-6">
            <div class="text-center mb-6">
              <div
                class="inline-flex items-center justify-center h-32 w-32 rounded-full bg-gradient-to-br from-primary-500 to-primary-700 text-white mb-4 shadow-lg"
              >
                <div>
                  <p class="text-4xl font-bold">{{ formatHours(overview.avg_resolution_hours) }}</p>
                  <p class="text-sm">часов</p>
                </div>
              </div>
              <p class="text-sm text-gray-600">
                Решено за период: {{ overview.requests_resolved_in_period }}
              </p>
            </div>
            <div v-if="priorityRows.length === 0" class="py-4 text-center">
              <p class="text-sm text-gray-500">Нет решённых заявок за период</p>
            </div>
            <div v-else class="grid grid-cols-3 gap-4">
              <div
                v-for="priority in priorityRows"
                :key="priority.priority"
                class="text-center p-3 rounded-lg bg-gray-50"
              >
                <p class="text-2xl font-bold text-gray-900">
                  {{ formatHours(priority.avg_hours) }}ч
                </p>
                <p class="text-xs text-gray-600 mt-1">{{ priorityLabel(priority.priority) }}</p>
                <p class="text-xs text-gray-400 mt-1">{{ priority.count }} шт.</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center gap-3">
            <div class="h-10 w-10 rounded-lg bg-amber-100 flex items-center justify-center">
              <Icon name="ri:fire-line" class="text-amber-600 text-xl" />
            </div>
            <h2 class="text-lg font-semibold text-gray-900">Топ проблем за период</h2>
          </div>
        </div>
        <div v-if="overview.top_issues.length === 0" class="p-10 text-center">
          <Icon name="ri:inbox-line" class="text-4xl text-gray-300 mb-3" />
          <p class="text-sm text-gray-500">За выбранный период заявок не было</p>
        </div>
        <div v-else class="overflow-x-auto">
          <table class="min-w-full divide-y divide-gray-200">
            <thead class="bg-gray-50">
              <tr>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">#</th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Проблема
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Количество
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Среднее время
                </th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-200">
              <tr
                v-for="(issue, index) in overview.top_issues"
                :key="issue.title"
                class="hover:bg-gray-50 transition-colors"
              >
                <td class="px-6 py-4">
                  <div
                    class="flex items-center justify-center w-8 h-8 rounded-lg bg-gray-100 text-gray-700 font-semibold text-sm"
                  >
                    {{ index + 1 }}
                  </div>
                </td>
                <td class="px-6 py-4">
                  <div class="flex items-center gap-3">
                    <div class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center">
                      <Icon name="ri:tools-line" class="text-blue-600" />
                    </div>
                    <span class="text-sm font-medium text-gray-900">{{ issue.title }}</span>
                  </div>
                </td>
                <td class="px-6 py-4">
                  <span class="badge bg-gray-100 text-gray-800">{{ issue.count }}</span>
                </td>
                <td class="px-6 py-4 text-sm text-gray-600">
                  {{ issue.avg_hours === null ? '—' : `${formatHours(issue.avg_hours)}ч` }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div class="card">
          <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-red-100 flex items-center justify-center">
                <Icon name="ri:alert-line" class="text-red-600 text-xl" />
              </div>
              <h2 class="text-lg font-semibold text-gray-900">Требуют внимания</h2>
            </div>
          </div>
          <div class="p-6 space-y-3">
            <div class="flex items-center justify-between p-3 rounded-lg bg-amber-50">
              <div class="flex items-center gap-3">
                <Icon name="ri:archive-line" class="text-xl text-amber-600" />
                <span class="text-sm font-medium text-gray-900">Низкий остаток</span>
              </div>
              <span class="text-sm font-bold text-amber-600">
                {{ overview.low_stock_consumables }}
              </span>
            </div>
            <div class="flex items-center justify-between p-3 rounded-lg bg-orange-50">
              <div class="flex items-center gap-3">
                <Icon name="ri:key-line" class="text-xl text-orange-600" />
                <span class="text-sm font-medium text-gray-900">Истекает лицензий</span>
              </div>
              <span class="text-sm font-bold text-orange-600">
                {{ overview.expiring_licenses }}
              </span>
            </div>
            <div class="flex items-center justify-between p-3 rounded-lg bg-red-50">
              <div class="flex items-center gap-3">
                <Icon name="ri:error-warning-line" class="text-xl text-red-600" />
                <span class="text-sm font-medium text-gray-900">Просрочено лицензий</span>
              </div>
              <span class="text-sm font-bold text-red-600">{{ overview.expired_licenses }}</span>
            </div>
            <div class="flex items-center justify-between p-3 rounded-lg bg-blue-50">
              <div class="flex items-center gap-3">
                <Icon name="ri:tools-line" class="text-xl text-blue-600" />
                <span class="text-sm font-medium text-gray-900">Оргтехника в ремонте</span>
              </div>
              <span class="text-sm font-bold text-blue-600">
                {{ overview.equipment_needs_attention }}
              </span>
            </div>
          </div>
        </div>

        <div class="card lg:col-span-2">
          <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-indigo-100 flex items-center justify-center">
                <Icon name="ri:calendar-line" class="text-indigo-600 text-xl" />
              </div>
              <h2 class="text-lg font-semibold text-gray-900">Активность по дням недели</h2>
            </div>
          </div>
          <div class="p-6">
            <div class="grid grid-cols-7 gap-2">
              <div v-for="day in weekActivity" :key="day.label" class="text-center">
                <div class="mb-2">
                  <div class="h-24 bg-gray-100 rounded-lg flex items-end justify-center p-2">
                    <div
                      :style="{ height: `${day.height}%` }"
                      class="w-full gradient-primary rounded transition-all duration-500"
                    ></div>
                  </div>
                </div>
                <p class="text-xs font-medium text-gray-600">{{ day.label }}</p>
                <p class="text-xs text-gray-500">{{ day.count }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { AnalyticsOverview } from '~/composables/useAnalytics'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

useHead({
  title: 'Аналитика',
})

const toast = useToast()
const { fetchAnalyticsOverview } = useAnalytics()

const selectedPeriod = ref('30')
const overview = ref<AnalyticsOverview | null>(null)
const loading = ref(false)

const categoryLabels: Record<string, string> = {
  hardware: 'Оборудование',
  software: 'Программное обеспечение',
  network: 'Сеть',
  access: 'Доступы',
  consultation: 'Консультации',
  other: 'Другое',
}

const categoryColors = [
  'bg-blue-500',
  'bg-purple-500',
  'bg-amber-500',
  'bg-green-500',
  'bg-cyan-500',
  'bg-gray-400',
]

const priorityLabels: Record<string, string> = {
  low: 'Низкий',
  medium: 'Средний',
  high: 'Высокий',
  critical: 'Критический',
}

const weekdayLabels = ['Пн', 'Вт', 'Ср', 'Чт', 'Пт', 'Сб', 'Вс']

const categoryRows = computed(() => {
  const items = overview.value?.requests_by_category ?? []
  const total = items.reduce((sum, item) => sum + item.count, 0)

  return items.map((item, index) => ({
    ...item,
    label: categoryLabels[item.key] ?? item.key,
    color: categoryColors[index % categoryColors.length],
    percentage: total > 0 ? Math.round((item.count / total) * 100) : 0,
  }))
})

const priorityRows = computed(() => overview.value?.resolution_by_priority ?? [])

const weekActivity = computed(() => {
  const counts = new Map<number, number>()
  for (const item of overview.value?.activity_by_weekday ?? []) {
    counts.set(item.weekday, item.count)
  }

  const values = weekdayLabels.map((label, index) => ({
    label,
    count: counts.get(index + 1) ?? 0,
  }))
  const max = Math.max(...values.map((item) => item.count), 1)

  return values.map((item) => ({
    ...item,
    height: item.count === 0 ? 0 : Math.max(4, Math.round((item.count / max) * 100)),
  }))
})

const share = (part: number, total: number) => {
  if (total === 0) return 0
  return Math.round((part / total) * 100)
}

const formatHours = (value: number | null) => {
  if (value === null || Number.isNaN(value)) return '—'
  return value < 10 ? value.toFixed(1) : Math.round(value).toString()
}

const priorityLabel = (value: string) => priorityLabels[value] ?? value

const loadOverview = async () => {
  loading.value = true
  try {
    overview.value = await fetchAnalyticsOverview(Number(selectedPeriod.value))
  } catch (error) {
    overview.value = null
    toast.error('Не удалось загрузить аналитику: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

watch(selectedPeriod, loadOverview)

const downloadReport = () => {
  if (!overview.value) return

  const rows: string[][] = [
    ['Показатель', 'Значение'],
    ['Период, дней', String(overview.value.period_days)],
    ['Активные сотрудники', String(overview.value.active_employees)],
    ['Всего сотрудников', String(overview.value.total_employees)],
    ['Активные устройства', String(overview.value.active_computers)],
    ['Всего устройств', String(overview.value.total_computers)],
    ['Открытые заявки', String(overview.value.open_requests)],
    ['Создано заявок за период', String(overview.value.requests_created_in_period)],
    ['Решено заявок за период', String(overview.value.requests_resolved_in_period)],
    ['Среднее время решения, ч', formatHours(overview.value.avg_resolution_hours)],
    ['Лицензий всего', String(overview.value.total_licenses)],
    ['Лицензий занято', String(overview.value.used_licenses)],
    ['Истекает лицензий', String(overview.value.expiring_licenses)],
    ['Просрочено лицензий', String(overview.value.expired_licenses)],
    ['Оргтехника в ремонте', String(overview.value.equipment_needs_attention)],
    ['Низкий остаток расходников', String(overview.value.low_stock_consumables)],
    [],
    ['Категория заявок', 'Количество'],
    ...categoryRows.value.map((item) => [item.label, String(item.count)]),
  ]

  const csv = rows
    .map((row) => row.map((cell) => `"${cell.replaceAll('"', '""')}"`).join(';'))
    .join('\r\n')
  const blob = new Blob(['\uFEFF' + csv], { type: 'text/csv;charset=utf-8;' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = `analytics-${overview.value.period_days}d.csv`
  link.click()
  URL.revokeObjectURL(url)

  toast.success('Отчет сформирован')
}

onMounted(loadOverview)
</script>
