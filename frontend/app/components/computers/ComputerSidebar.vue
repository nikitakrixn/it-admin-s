<template>
  <div class="space-y-6">
    <!-- Статус и назначение -->
    <div class="card">
      <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
        <h3 class="text-sm font-semibold text-gray-900">Статус</h3>
      </div>
      <div class="p-6 space-y-4">
        <div>
          <p class="text-xs text-gray-600 mb-2">Текущий статус</p>
          <select v-model="status" class="input-field">
            <option value="active">Активный</option>
            <option value="inactive">Неактивный</option>
            <option value="repair">В ремонте</option>
            <option value="storage">На складе</option>
            <option value="decommissioned">Списан</option>
          </select>
        </div>

        <div>
          <p class="text-xs text-gray-600 mb-2">Назначен сотруднику</p>
          <div
            v-if="employee"
            class="flex items-center gap-2 p-3 rounded-lg bg-blue-50 border border-blue-100"
          >
            <div class="h-8 w-8 rounded-full bg-blue-500 flex items-center justify-center">
              <Icon name="ri:user-line" class="text-white" />
            </div>
            <div class="flex-1 min-w-0">
              <p class="text-sm font-medium text-gray-900 truncate">{{ employee.full_name }}</p>
              <p class="text-xs text-gray-600 truncate">
                {{ employee.position_name || 'Должность не указана' }}
              </p>
            </div>
          </div>
          <NuxtLink
            v-else
            :to="`/computers/${computer.id}/edit`"
            class="w-full btn btn-secondary btn-sm"
          >
            <Icon name="ri:user-add-line" class="mr-2" />
            Назначить
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Гарантия и покупка -->
    <div class="card">
      <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
        <h3 class="text-sm font-semibold text-gray-900">Гарантия и покупка</h3>
      </div>
      <div class="p-6 space-y-4">
        <div>
          <p class="text-xs text-gray-600 mb-1">Дата покупки</p>
          <p class="text-sm font-medium text-gray-900">{{ formatDate(computer.purchase_date) }}</p>
        </div>
        <div>
          <p class="text-xs text-gray-600 mb-1">Гарантия до</p>
          <div class="flex items-center gap-2">
            <p class="text-sm font-medium text-gray-900">
              {{ formatDate(computer.warranty_end_date) }}
            </p>
            <span v-if="warrantyActive" class="badge bg-green-100 text-green-800 text-xs">
              Активна
            </span>
            <span
              v-else-if="computer.warranty_end_date"
              class="badge bg-red-100 text-red-800 text-xs"
            >
              Истекла
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Мониторинг -->
    <div class="card">
      <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
        <div class="flex items-center gap-2">
          <Icon name="ri:pulse-line" class="text-gray-600" />
          <h3 class="text-sm font-semibold text-gray-900">Мониторинг</h3>
        </div>
      </div>
      <div class="p-6 space-y-4">
        <template v-if="monitoring">
          <div>
            <div class="flex items-center justify-between mb-2">
              <span class="text-xs text-gray-600">CPU</span>
              <span class="text-xs font-medium text-gray-900"
                >{{ percent(monitoring.cpu_usage) }}%</span
              >
            </div>
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div
                class="bg-blue-500 h-2 rounded-full"
                :style="{ width: `${percent(monitoring.cpu_usage)}%` }"
              ></div>
            </div>
          </div>

          <div>
            <div class="flex items-center justify-between mb-2">
              <span class="text-xs text-gray-600">RAM</span>
              <span class="text-xs font-medium text-gray-900">
                {{ percent(monitoring.ram_usage_percent) }}%
              </span>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div
                class="bg-purple-500 h-2 rounded-full"
                :style="{ width: `${percent(monitoring.ram_usage_percent)}%` }"
              ></div>
            </div>
          </div>

          <div>
            <div class="flex items-center justify-between mb-2">
              <span class="text-xs text-gray-600">Диск C:</span>
              <span class="text-xs font-medium text-gray-900">{{ diskUsage }}%</span>
            </div>
            <div class="w-full bg-gray-200 rounded-full h-2">
              <div class="bg-green-500 h-2 rounded-full" :style="{ width: `${diskUsage}%` }"></div>
            </div>
          </div>

          <div class="pt-3 border-t border-gray-200">
            <p class="text-xs text-gray-600 mb-1">Время работы</p>
            <p class="text-sm font-medium text-gray-900">
              {{ formatUptime(monitoring.uptime_seconds) }}
            </p>
          </div>
          <div class="text-xs text-gray-500">
            Замер: {{ formatDateTime(monitoring.collected_at) }}
          </div>
        </template>
        <div v-else class="text-center py-4">
          <Icon name="ri:pulse-line" class="text-3xl text-gray-300 mb-2" />
          <p class="text-sm text-gray-500">Данных мониторинга пока нет</p>
          <NuxtLink to="/monitoring" class="text-xs text-primary-600 hover:text-primary-700">
            Перейти в мониторинг
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Временные метки -->
    <div class="card">
      <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
        <h3 class="text-sm font-semibold text-gray-900">Временные метки</h3>
      </div>
      <div class="p-6 space-y-3">
        <div class="flex items-center gap-2 text-sm">
          <Icon name="ri:time-line" class="text-gray-600" />
          <div>
            <p class="text-xs text-gray-600">Создан</p>
            <p class="text-gray-900">{{ formatDateTime(computer.created_at) }}</p>
          </div>
        </div>
        <div class="flex items-center gap-2 text-sm">
          <Icon name="ri:refresh-line" class="text-gray-600" />
          <div>
            <p class="text-xs text-gray-600">Обновлен</p>
            <p class="text-gray-900">{{ formatDateTime(computer.updated_at) }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Computer } from '~/composables/useComputers'
import type { Employee } from '~/composables/useEmployees'
import type { ComputerMonitoring } from '~/composables/useMonitoring'

interface Props {
  computer: Computer
  employee: Employee | null
  monitoring: ComputerMonitoring | null
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:status': [value: string]
}>()

const status = computed({
  get: () => props.computer.status,
  set: (value: string) => emit('update:status', value),
})

const warrantyActive = computed(() => {
  if (!props.computer.warranty_end_date) return false
  const end = new Date(props.computer.warranty_end_date)
  if (Number.isNaN(end.getTime())) return false
  return end.getTime() >= Date.now()
})

const diskUsage = computed(() => {
  const disk = props.monitoring?.disk_usage
  if (!disk || typeof disk !== 'object') return 0
  const record = disk as Record<string, unknown>
  const value = record.c ?? record.C ?? record.used_percent ?? record.usage_percent
  const numeric = Number(value)
  return Number.isFinite(numeric) ? Math.round(numeric) : 0
})

const percent = (value: string | null) => {
  const numeric = Number(value)
  return Number.isFinite(numeric) ? Math.round(numeric) : 0
}

const formatDate = (value: string | null) => {
  if (!value) return '—'
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleDateString('ru-RU')
}

const formatDateTime = (value: string | null) => {
  if (!value) return '—'
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString('ru-RU')
}

const formatUptime = (seconds: number | null) => {
  if (!seconds || seconds < 0) return '—'
  const days = Math.floor(seconds / 86400)
  const hours = Math.floor((seconds % 86400) / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  const parts: string[] = []
  if (days) parts.push(`${days} дн.`)
  if (hours) parts.push(`${hours} ч.`)
  parts.push(`${minutes} мин.`)
  return parts.join(' ')
}
</script>
