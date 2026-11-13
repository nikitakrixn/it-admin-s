<template>
  <div class="space-y-6">
    <!-- Заголовок и действия -->
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl gradient-primary flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:dashboard-3-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Панель управления</h1>
            <p class="mt-1 text-gray-600">Обзор основных показателей и статистики системы</p>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <button class="btn btn-secondary" :disabled="refreshing" @click="refreshData">
            <Icon name="ri:refresh-line" class="mr-2" :class="{ 'animate-spin': refreshing }" />
            Обновить
          </button>
          <button class="btn btn-primary" :disabled="!overview" @click="exportData">
            <Icon name="ri:download-line" class="mr-2" />
            Экспорт
          </button>
        </div>
      </div>
    </div>

    <!-- Карточки статистики -->
    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
      <StatCard
        title="Сотрудники"
        :value="overview?.total_employees ?? 0"
        :subtitle="`${overview?.active_employees ?? 0} активных`"
        subtitle-icon="ri:user-follow-line"
        icon="ri:team-line"
        color="blue"
        link="/employees"
      >
        <template #detail>
          <div class="flex items-center justify-between text-sm">
            <span class="text-gray-600">В штате</span>
            <span class="font-semibold text-gray-900">
              {{ overview?.active_employees ?? 0 }}
            </span>
          </div>
        </template>
      </StatCard>

      <StatCard
        title="Компьютеры"
        :value="overview?.total_computers ?? 0"
        :subtitle="`${overview?.active_computers ?? 0} активных`"
        subtitle-icon="ri:checkbox-circle-line"
        icon="ri:computer-line"
        color="purple"
        link="/computers"
      >
        <template #detail>
          <div class="flex items-center justify-between text-sm">
            <span class="text-gray-600">Требуют внимания</span>
            <span class="font-semibold text-amber-600">
              {{ (overview?.total_computers ?? 0) - (overview?.active_computers ?? 0) }}
            </span>
          </div>
        </template>
      </StatCard>

      <StatCard
        title="Оргтехника"
        :value="overview?.total_equipment ?? 0"
        :subtitle="`${overview?.equipment_needs_attention ?? 0} в ремонте`"
        subtitle-icon="ri:tools-line"
        icon="ri:printer-line"
        color="green"
        link="/equipment"
      >
        <template #detail>
          <div class="flex items-center justify-between text-sm">
            <span class="text-gray-600">Низкий остаток расходников</span>
            <span class="font-semibold text-gray-900">
              {{ overview?.low_stock_consumables ?? 0 }}
            </span>
          </div>
        </template>
      </StatCard>

      <StatCard
        title="Открытые заявки"
        :value="overview?.open_requests ?? 0"
        :subtitle="`${overview?.requests_created_in_period ?? 0} за 30 дней`"
        subtitle-icon="ri:time-line"
        icon="ri:file-list-3-line"
        color="amber"
        link="/requests"
      >
        <template #detail>
          <div class="flex items-center justify-between text-sm">
            <span class="text-gray-600">Решено за 30 дней</span>
            <span class="font-semibold text-gray-900">
              {{ overview?.requests_resolved_in_period ?? 0 }}
            </span>
          </div>
        </template>
      </StatCard>
    </div>

    <!-- Быстрые действия -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
      <QuickActionCard
        icon="ri:add-circle-line"
        title="Создать заявку"
        description="Новая заявка в систему"
        color="primary"
        link="/requests/create"
      />
      <QuickActionCard
        icon="ri:user-add-line"
        title="Добавить сотрудника"
        description="Новый профиль сотрудника"
        color="blue"
        link="/employees/create"
      />
      <QuickActionCard
        icon="ri:radar-line"
        title="Сканировать сеть"
        description="Поиск устройств в сети"
        color="indigo"
        link="/network/scan"
      />
      <QuickActionCard
        icon="ri:line-chart-line"
        title="Аналитика"
        description="Отчеты и статистика"
        color="purple"
        link="/analytics"
      />
    </div>

    <!-- Последние действия и статус системы -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="lg:col-span-2 card">
        <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-primary-100 flex items-center justify-center">
                <Icon name="ri:history-line" class="text-primary-600 text-xl" />
              </div>
              <h2 class="text-lg font-semibold text-gray-900">Последние действия</h2>
            </div>
            <NuxtLink
              to="/logs"
              class="text-sm text-primary-600 hover:text-primary-700 font-medium"
            >
              Все действия →
            </NuxtLink>
          </div>
        </div>
        <div class="p-6">
          <div v-if="recentActivities.length === 0" class="py-8 text-center">
            <Icon name="ri:history-line" class="text-4xl text-gray-300 mb-3" />
            <p class="text-sm text-gray-500">Журнал пока пуст</p>
          </div>
          <ActivityTimeline v-else :activities="recentActivities" />
        </div>
      </div>

      <div class="card">
        <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center gap-3">
            <div class="h-10 w-10 rounded-lg bg-green-100 flex items-center justify-center">
              <Icon name="ri:server-line" class="text-green-600 text-xl" />
            </div>
            <h2 class="text-lg font-semibold text-gray-900">Статус системы</h2>
          </div>
        </div>
        <div class="p-6 space-y-4">
          <SystemStatus label="API Сервер" status="online" message="Отвечает на запросы" />
          <SystemStatus
            label="База данных"
            status="online"
            :message="`${overview?.total_computers ?? 0} устройств в инвентаре`"
          />
          <SystemStatus
            label="Заявки"
            :status="(overview?.open_requests ?? 0) > 0 ? 'warning' : 'online'"
            :message="`${overview?.open_requests ?? 0} открыто`"
          />
          <SystemStatus
            label="Лицензии"
            :status="(overview?.expiring_licenses ?? 0) > 0 ? 'warning' : 'online'"
            :message="`Истекает: ${overview?.expiring_licenses ?? 0}`"
          />
        </div>
      </div>
    </div>

    <!-- Предстоящие события и требующие внимания -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div class="card">
        <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center gap-3">
            <div class="h-10 w-10 rounded-lg bg-amber-100 flex items-center justify-center">
              <Icon name="ri:calendar-check-line" class="text-amber-600 text-xl" />
            </div>
            <h2 class="text-lg font-semibold text-gray-900">Сроки лицензий</h2>
          </div>
        </div>
        <div class="p-6 space-y-3">
          <div v-if="upcomingLicenses.length === 0" class="py-8 text-center">
            <Icon name="ri:key-line" class="text-4xl text-gray-300 mb-3" />
            <p class="text-sm text-gray-500">Ближайших истечений нет</p>
          </div>
          <EventItem
            v-for="license in upcomingLicenses"
            :key="license.id"
            :date-month="license.month"
            :date-day="license.day"
            :title="license.title"
            :description="license.description"
            :badge="license.badge"
            :badge-color="license.badgeColor"
            icon="ri:key-line"
            icon-color="amber"
          />
        </div>
      </div>

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
          <div v-if="attentionItems.length === 0" class="py-8 text-center">
            <Icon name="ri:checkbox-circle-line" class="text-4xl text-green-300 mb-3" />
            <p class="text-sm text-gray-500">Всё в порядке</p>
          </div>
          <AlertItem
            v-for="item in attentionItems"
            :key="item.title"
            :title="item.title"
            :description="item.description"
            :type="item.type"
            :icon="item.icon"
          />
        </div>
      </div>
    </div>

    <!-- Информация о системе -->
    <div class="card">
      <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
        <div class="flex items-center gap-3">
          <div class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center">
            <Icon name="ri:information-line" class="text-blue-600 text-xl" />
          </div>
          <h2 class="text-lg font-semibold text-gray-900">Информация о системе</h2>
        </div>
      </div>

      <div class="p-6 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-5">
        <SystemInfoItem
          title="Сотрудники"
          :value="`${overview?.active_employees ?? 0} активных`"
          :subtitle="`Всего: ${overview?.total_employees ?? 0}`"
          icon="ri:team-line"
          color="blue"
        />
        <SystemInfoItem
          title="Устройства"
          :value="`${overview?.active_computers ?? 0} активных`"
          :subtitle="`Всего: ${overview?.total_computers ?? 0}`"
          icon="ri:computer-line"
          color="purple"
        />
        <SystemInfoItem
          title="Лицензии"
          :value="`${overview?.used_licenses ?? 0} занято`"
          :subtitle="`Всего: ${overview?.total_licenses ?? 0}`"
          icon="ri:key-line"
          color="green"
        />
        <SystemInfoItem
          title="Среднее решение"
          :value="`${formatHours(overview?.avg_resolution_hours ?? null)} ч`"
          :subtitle="`${overview?.requests_resolved_in_period ?? 0} заявок за 30 дней`"
          icon="ri:time-line"
          color="amber"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import StatCard from '~/components/dashboard/StatCard.vue'
import QuickActionCard from '~/components/dashboard/QuickActionCard.vue'
import EventItem from '~/components/dashboard/EventItem.vue'
import AlertItem from '~/components/dashboard/AlertItem.vue'
import SystemInfoItem from '~/components/dashboard/SystemInfoItem.vue'
import type { ActivityLog } from '~/composables/useActivityLog'
import type { LicensePool } from '~/composables/useLicenses'
import type { Software } from '~/composables/useSoftware'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

useHead({
  title: 'Панель управления',
})

const toast = useToast()
const { fetchActivityLogs } = useActivityLog()
const { fetchAnalyticsOverview } = useAnalytics()
const { fetchLicenses } = useLicenses()
const { fetchSoftware } = useSoftware()

const refreshing = ref(false)
const overview = ref<Awaited<ReturnType<typeof fetchAnalyticsOverview>> | null>(null)
const logs = ref<ActivityLog[]>([])
const licenses = ref<LicensePool[]>([])
const software = ref<Software[]>([])

const entityLabels: Record<string, string> = {
  computer: 'компьютер',
  employee: 'сотрудник',
  department: 'подразделение',
  position: 'должность',
  request: 'заявка',
  software: 'программа',
  consumable: 'расходник',
  equipment: 'оборудование',
  license: 'лицензия',
  network: 'сеть',
  ad: 'Active Directory',
  user: 'пользователь',
}

const actionLabels: Record<string, string> = {
  created: 'Создан',
  updated: 'Обновлён',
  deleted: 'Удалён',
  login: 'Вход в систему',
  logout: 'Выход из системы',
}

const actionForms: Record<string, [string, string, string]> = {
  created: ['Создан', 'Создана', 'Создано'],
  updated: ['Обновлён', 'Обновлена', 'Обновлено'],
  deleted: ['Удалён', 'Удалена', 'Удалено'],
}

const entityGenders: Record<string, 0 | 1 | 2> = {
  computer: 0,
  employee: 0,
  department: 2,
  position: 1,
  request: 1,
  software: 1,
  consumable: 0,
  equipment: 2,
  license: 1,
  network: 1,
  ad: 2,
  user: 0,
}

const messageOverrides: Record<string, string> = {
  component_added: 'Добавлен компонент компьютера',
  component_removed: 'Удалён компонент компьютера',
  software_installed: 'Установлено ПО',
  software_removed: 'Удалено ПО',
  login: 'Вход в систему',
  logout: 'Выход из системы',
}

const actionColors: Record<string, string> = {
  created: 'green',
  updated: 'blue',
  deleted: 'red',
  login: 'purple',
}

const actionIcons: Record<string, string> = {
  created: 'ri:add-line',
  updated: 'ri:edit-line',
  deleted: 'ri:delete-bin-line',
  login: 'ri:login-box-line',
}

const recentActivities = computed(() =>
  logs.value.slice(0, 5).map((log) => {
    const entity = entityLabels[log.entity_type] ?? log.entity_type
    const gender = entityGenders[log.entity_type] ?? 0
    const form = actionForms[log.action]
    const action = form ? form[gender] : (actionLabels[log.action] ?? log.action)
    const name = typeof log.details?.name === 'string' ? ` «${log.details.name}»` : ''
    const message = messageOverrides[log.action] ?? `${action} ${entity}${name}`

    return {
      id: log.id,
      type: log.entity_type,
      message,
      time: formatRelativeTime(log.created_at),
      icon: actionIcons[log.action] ?? 'ri:history-line',
      color: actionColors[log.action] ?? 'amber',
    }
  }),
)

const upcomingLicenses = computed(() => {
  const today = new Date()
  const limit = new Date(today.getTime() + 90 * 24 * 60 * 60 * 1000)
  const names = new Map(software.value.map((item) => [item.id, item.name]))

  return licenses.value
    .filter((pool) => {
      if (!pool.expiration_date) return false
      const date = new Date(pool.expiration_date)
      return !Number.isNaN(date.getTime()) && date >= today && date <= limit
    })
    .sort((a, b) => new Date(a.expiration_date!).getTime() - new Date(b.expiration_date!).getTime())
    .slice(0, 3)
    .map((pool) => {
      const date = new Date(pool.expiration_date!)
      const daysLeft = Math.ceil((date.getTime() - today.getTime()) / (24 * 60 * 60 * 1000))

      return {
        id: pool.id,
        month: date.toLocaleDateString('ru-RU', { month: 'short' }).replace('.', '').toUpperCase(),
        day: date.toLocaleDateString('ru-RU', { day: '2-digit' }),
        title: names.get(pool.software_catalog_id) ?? `Лицензия #${pool.id}`,
        description: `${pool.used_licenses ?? 0} из ${pool.total_licenses} занято`,
        badge: daysLeft <= 14 ? 'Срочно' : 'Скоро',
        badgeColor: daysLeft <= 14 ? 'red' : 'amber',
      }
    })
})

const attentionItems = computed(() => {
  const items: Array<{
    title: string
    description: string
    type: 'error' | 'warning' | 'info'
    icon: string
  }> = []

  if ((overview.value?.expired_licenses ?? 0) > 0) {
    items.push({
      title: 'Просроченные лицензии',
      description: `${overview.value?.expired_licenses} требуют продления`,
      type: 'error',
      icon: 'ri:error-warning-line',
    })
  }

  if ((overview.value?.expiring_licenses ?? 0) > 0) {
    items.push({
      title: 'Истекают лицензии',
      description: `${overview.value?.expiring_licenses} в ближайшие 30 дней`,
      type: 'warning',
      icon: 'ri:key-line',
    })
  }

  if ((overview.value?.low_stock_consumables ?? 0) > 0) {
    items.push({
      title: 'Низкий остаток расходников',
      description: `${overview.value?.low_stock_consumables} позиций на минимуме`,
      type: 'warning',
      icon: 'ri:archive-line',
    })
  }

  if ((overview.value?.equipment_needs_attention ?? 0) > 0) {
    items.push({
      title: 'Оргтехника в ремонте',
      description: `${overview.value?.equipment_needs_attention} единиц вне работы`,
      type: 'warning',
      icon: 'ri:tools-line',
    })
  }

  if ((overview.value?.open_requests ?? 0) > 0) {
    items.push({
      title: 'Открытые заявки',
      description: `${overview.value?.open_requests} ожидают решения`,
      type: 'info',
      icon: 'ri:file-list-3-line',
    })
  }

  return items
})

const formatHours = (value: number | null) => {
  if (value === null || Number.isNaN(value)) return '—'
  return value < 10 ? value.toFixed(1) : Math.round(value).toString()
}

const formatRelativeTime = (value: string) => {
  const hasTimezone = value.endsWith('Z') || /[+-]\d{2}:\d{2}$/.test(value)
  const date = new Date(hasTimezone ? value : `${value}Z`)
  if (Number.isNaN(date.getTime())) return value

  const diffMinutes = Math.floor((Date.now() - date.getTime()) / 60000)
  if (diffMinutes < 1) return 'только что'
  if (diffMinutes < 60) return `${diffMinutes} мин назад`

  const diffHours = Math.floor(diffMinutes / 60)
  if (diffHours < 24) return `${diffHours} ч назад`

  const diffDays = Math.floor(diffHours / 24)
  if (diffDays === 1) return 'вчера'

  return date.toLocaleDateString('ru-RU')
}

const loadData = async () => {
  try {
    const [overviewResponse, logsResponse, licensesResponse, softwareResponse] = await Promise.all([
      fetchAnalyticsOverview(30),
      fetchActivityLogs({ per_page: 5 }),
      fetchLicenses({ per_page: 100 }),
      fetchSoftware({ per_page: 100 }),
    ])

    overview.value = overviewResponse
    logs.value = logsResponse.logs
    licenses.value = licensesResponse.licenses
    software.value = softwareResponse.software
  } catch (error) {
    toast.error('Не удалось загрузить данные: ' + getErrorMessage(error))
  }
}

const refreshData = async () => {
  refreshing.value = true
  try {
    await loadData()
    toast.success('Данные обновлены')
  } finally {
    refreshing.value = false
  }
}

const exportData = () => {
  if (!overview.value) return

  const rows: string[][] = [
    ['Показатель', 'Значение'],
    ['Сотрудники', String(overview.value.total_employees)],
    ['Активные сотрудники', String(overview.value.active_employees)],
    ['Компьютеры', String(overview.value.total_computers)],
    ['Активные компьютеры', String(overview.value.active_computers)],
    ['Оргтехника', String(overview.value.total_equipment)],
    ['Открытые заявки', String(overview.value.open_requests)],
    ['Заявки за 30 дней', String(overview.value.requests_created_in_period)],
    ['Лицензий всего', String(overview.value.total_licenses)],
    ['Лицензий занято', String(overview.value.used_licenses)],
  ]

  const csv = rows
    .map((row) => row.map((cell) => `"${cell.replaceAll('"', '""')}"`).join(';'))
    .join('\r\n')
  const blob = new Blob(['\uFEFF' + csv], { type: 'text/csv;charset=utf-8;' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = 'dashboard.csv'
  link.click()
  URL.revokeObjectURL(url)

  toast.success('Экспорт сформирован')
}

onMounted(loadData)
</script>

<style scoped></style>
