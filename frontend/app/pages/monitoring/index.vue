<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-cyan-500 to-blue-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:pulse-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Мониторинг</h1>
            <p class="mt-1 text-gray-600">Нагрузка на компьютеры и состояние оборудования</p>
          </div>
        </div>
        <button class="btn btn-secondary" :disabled="loading" @click="loadDashboard">
          <Icon name="ri:refresh-line" class="mr-2" :class="{ 'animate-spin': loading }" />
          Обновить
        </button>
      </div>
    </div>

    <div v-if="loading && !summary" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-cyan-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка данных мониторинга...</p>
    </div>

    <template v-else>
      <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
        <div class="card">
          <div class="p-5 flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Компьютеров под наблюдением</p>
              <p class="text-2xl font-bold text-cyan-600">
                {{ summary?.total_computers_monitored ?? 0 }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-cyan-100 flex items-center justify-center">
              <Icon name="ri:computer-line" class="text-2xl text-cyan-600" />
            </div>
          </div>
        </div>

        <div class="card">
          <div class="p-5 flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Записей метрик</p>
              <p class="text-2xl font-bold text-blue-600">
                {{ summary?.total_monitoring_records ?? 0 }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center">
              <Icon name="ri:database-2-line" class="text-2xl text-blue-600" />
            </div>
          </div>
        </div>

        <div class="card">
          <div class="p-5 flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Средняя загрузка CPU</p>
              <p class="text-2xl font-bold text-indigo-600">
                {{ formatPercent(summary?.avg_cpu_usage) }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-indigo-100 flex items-center justify-center">
              <Icon name="ri:cpu-line" class="text-2xl text-indigo-600" />
            </div>
          </div>
        </div>

        <div class="card">
          <div class="p-5 flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Средняя загрузка RAM</p>
              <p class="text-2xl font-bold text-purple-600">
                {{ formatPercent(summary?.avg_ram_usage_percent) }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center">
              <Icon name="ri:ram-line" class="text-2xl text-purple-600" />
            </div>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div class="card">
          <div class="p-5 flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Высокая нагрузка CPU</p>
              <p class="text-2xl font-bold text-red-600">
                {{ summary?.computers_with_high_cpu ?? 0 }}
              </p>
              <p class="text-xs text-gray-500 mt-1">больше 80%</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-red-100 flex items-center justify-center">
              <Icon name="ri:alarm-warning-line" class="text-2xl text-red-600" />
            </div>
          </div>
        </div>

        <div class="card">
          <div class="p-5 flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Высокая нагрузка RAM</p>
              <p class="text-2xl font-bold text-amber-600">
                {{ summary?.computers_with_high_ram ?? 0 }}
              </p>
              <p class="text-xs text-gray-500 mt-1">больше 80%</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-amber-100 flex items-center justify-center">
              <Icon name="ri:error-warning-line" class="text-2xl text-amber-600" />
            </div>
          </div>
        </div>

        <div class="card">
          <div class="p-5">
            <p class="text-sm text-gray-600 mb-1">Последний сбор данных</p>
            <p class="text-lg font-semibold text-gray-900">
              {{ formatDate(summary?.last_collected_at) }}
            </p>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center gap-2">
            <Icon name="ri:line-chart-line" class="text-gray-600" />
            <h2 class="text-lg font-semibold text-gray-900">Последние измерения</h2>
          </div>
        </div>
        <div class="overflow-x-auto">
          <table class="min-w-full divide-y divide-gray-200">
            <thead class="bg-gray-50">
              <tr>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Компьютер
                </th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">CPU</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">RAM</th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Память
                </th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Температура
                </th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Uptime
                </th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Собрано
                </th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-200">
              <tr v-if="recentMonitoring.length === 0">
                <td colspan="7" class="px-6 py-16 text-center">
                  <div class="flex flex-col items-center">
                    <div
                      class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mb-4"
                    >
                      <Icon name="ri:pulse-line" class="text-4xl text-gray-400" />
                    </div>
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">Метрик пока нет</h3>
                    <p class="text-sm text-gray-500">
                      Данные появятся после первого отчёта агента мониторинга
                    </p>
                  </div>
                </td>
              </tr>
              <tr v-for="item in recentMonitoring" :key="item.id" class="hover:bg-gray-50">
                <td class="px-4 py-4 text-sm font-medium text-gray-900">#{{ item.computer_id }}</td>
                <td class="px-4 py-4">
                  <span :class="['badge', loadClass(item.cpu_usage)]">
                    {{ formatPercent(item.cpu_usage) }}
                  </span>
                </td>
                <td class="px-4 py-4">
                  <span :class="['badge', loadClass(item.ram_usage_percent)]">
                    {{ formatPercent(item.ram_usage_percent) }}
                  </span>
                </td>
                <td class="px-4 py-4 text-sm text-gray-900">
                  {{ formatRam(item.ram_used_gb, item.ram_total_gb) }}
                </td>
                <td class="px-4 py-4 text-sm text-gray-900">
                  {{ formatTemperature(item.cpu_temperature) }}
                </td>
                <td class="px-4 py-4 text-sm text-gray-900">
                  {{ formatUptime(item.uptime_seconds) }}
                </td>
                <td class="px-4 py-4 text-sm text-gray-500">
                  {{ formatDate(item.collected_at) }}
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <div class="card">
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center gap-2">
            <Icon name="ri:tv-line" class="text-gray-600" />
            <h2 class="text-lg font-semibold text-gray-900">Физические мониторы</h2>
          </div>
        </div>
        <div class="overflow-x-auto">
          <table class="min-w-full divide-y divide-gray-200">
            <thead class="bg-gray-50">
              <tr>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Монитор
                </th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Компьютер
                </th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Разрешение
                </th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Диагональ
                </th>
                <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Основной
                </th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-200">
              <tr v-if="monitors.length === 0">
                <td colspan="5" class="px-6 py-12 text-center text-sm text-gray-500">
                  Мониторы не добавлены
                </td>
              </tr>
              <tr v-for="monitor in monitors" :key="monitor.id" class="hover:bg-gray-50">
                <td class="px-4 py-4">
                  <p class="text-sm font-medium text-gray-900">{{ monitor.name }}</p>
                  <p class="text-xs text-gray-500">
                    {{ [monitor.manufacturer, monitor.model].filter(Boolean).join(' ') || '—' }}
                  </p>
                </td>
                <td class="px-4 py-4 text-sm text-gray-900">#{{ monitor.computer_id }}</td>
                <td class="px-4 py-4 text-sm text-gray-900">{{ monitor.resolution || '—' }}</td>
                <td class="px-4 py-4 text-sm text-gray-900">{{ monitor.size_inches || '—' }}</td>
                <td class="px-4 py-4">
                  <span
                    :class="[
                      'badge',
                      monitor.is_primary
                        ? 'bg-green-100 text-green-800'
                        : 'bg-gray-100 text-gray-700',
                    ]"
                  >
                    {{ monitor.is_primary ? 'Да' : 'Нет' }}
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useMonitoring } from '~/composables/useMonitoring'
import type {
  ComputerMonitor,
  ComputerMonitoringResponse,
  MonitoringDashboardSummary,
} from '~/composables/useMonitoring'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

useHead({
  title: 'Мониторинг',
})

const toast = useToast()
const { fetchDashboardSummary, fetchComputerMonitors } = useMonitoring()

const loading = ref(false)
const summary = ref<MonitoringDashboardSummary | null>(null)
const recentMonitoring = ref<ComputerMonitoringResponse[]>([])
const monitors = ref<ComputerMonitor[]>([])

const loadDashboard = async () => {
  loading.value = true
  try {
    const [dashboard, monitorList] = await Promise.all([
      fetchDashboardSummary(),
      fetchComputerMonitors({ per_page: 50 }),
    ])
    summary.value = dashboard.summary
    recentMonitoring.value = dashboard.recent_monitoring
    monitors.value = monitorList.monitors
  } catch (error) {
    toast.error('Ошибка загрузки мониторинга: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const toNumber = (value: string | number | null | undefined): number | null => {
  if (value === null || value === undefined || value === '') return null
  const parsed = typeof value === 'number' ? value : Number.parseFloat(value)
  return Number.isFinite(parsed) ? parsed : null
}

const formatPercent = (value: string | number | null | undefined): string => {
  const parsed = toNumber(value)
  return parsed === null ? '—' : `${parsed.toFixed(1)}%`
}

const formatRam = (used: string | null, total: number | null): string => {
  const usedValue = toNumber(used)
  if (usedValue === null && total === null) return '—'
  if (total === null) return `${usedValue?.toFixed(1)} ГБ`
  if (usedValue === null) return `${total} ГБ`
  return `${usedValue.toFixed(1)} / ${total} ГБ`
}

const formatTemperature = (value: string | null): string => {
  const parsed = toNumber(value)
  return parsed === null ? '—' : `${parsed.toFixed(1)} °C`
}

const formatUptime = (seconds: number | null): string => {
  if (seconds === null) return '—'
  const days = Math.floor(seconds / 86400)
  const hours = Math.floor((seconds % 86400) / 3600)
  if (days > 0) return `${days} д ${hours} ч`
  const minutes = Math.floor((seconds % 3600) / 60)
  return `${hours} ч ${minutes} мин`
}

const formatDate = (value: string | null | undefined): string => {
  if (!value) return '—'
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString('ru-RU')
}

const loadClass = (value: string | null): string => {
  const parsed = toNumber(value)
  if (parsed === null) return 'bg-gray-100 text-gray-700'
  if (parsed >= 80) return 'bg-red-100 text-red-800'
  if (parsed >= 60) return 'bg-amber-100 text-amber-800'
  return 'bg-green-100 text-green-800'
}

onMounted(loadDashboard)
</script>
