<template>
  <div>
    <!-- Header -->
    <div class="relative mb-10 pb-5">
      <div class="relative z-10 pt-4 pb-5">
        <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between">
          <div class="flex-1 min-w-0">
            <div class="flex items-center">
              <div class="h-16 w-16 flex-shrink-0 rounded-xl bg-gradient-to-br from-indigo-400 to-indigo-600 flex items-center justify-center mr-5 shadow-lg">
                <Icon name="ri:history-line" class="text-white text-3xl" />
              </div>
              <div>
                <h1 class="text-2xl font-bold text-gray-900 sm:text-3xl">Журнал активности</h1>
                <p class="mt-2 text-sm text-gray-600 max-w-4xl">
                  История всех действий пользователей в системе
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Filters -->
    <div class="mb-6 bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
      <div class="px-6 py-4 bg-gradient-to-r from-gray-50 to-white border-b border-gray-200">
        <div class="flex items-center">
          <Icon name="ri:filter-3-line" class="text-indigo-500 text-xl mr-2" />
          <h3 class="text-base font-semibold text-gray-900">Фильтры</h3>
        </div>
      </div>
      <div class="p-6">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Действие</label>
            <select
              v-model="filters.action"
              @change="loadLogs"
              class="block w-full rounded-lg border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm transition-colors"
            >
              <option value="">Все действия</option>
              <option value="created">Создание</option>
              <option value="updated">Обновление</option>
              <option value="deleted">Удаление</option>
              <option value="login">Вход</option>
              <option value="logout">Выход</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Тип сущности</label>
            <select
              v-model="filters.entity_type"
              @change="loadLogs"
              class="block w-full rounded-lg border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm transition-colors"
            >
              <option value="">Все типы</option>
              <option value="employee">Сотрудники</option>
              <option value="computer">Компьютеры</option>
              <option value="equipment">Оборудование</option>
              <option value="request">Заявки</option>
              <option value="user">Пользователи</option>
            </select>
          </div>
          <div class="flex items-end">
            <button
              v-if="filters.action || filters.entity_type"
              @click="resetFilters"
              class="w-full inline-flex items-center justify-center px-4 py-2 border border-gray-300 rounded-lg shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 transition-all"
            >
              <Icon name="ri:refresh-line" class="mr-2" />
              Сбросить фильтры
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="pending" class="flex justify-center py-12">
      <Icon name="ri:loader-4-line" class="text-4xl text-indigo-600 animate-spin" />
    </div>

    <!-- Error -->
    <div v-else-if="error" class="rounded-md bg-red-50 p-4">
      <div class="flex">
        <Icon name="ri:error-warning-line" class="h-5 w-5 text-red-400" />
        <div class="ml-3">
          <p class="text-sm font-medium text-red-800">{{ error }}</p>
        </div>
      </div>
    </div>

    <!-- Table -->
    <div v-else class="bg-white shadow-sm rounded-xl border border-gray-200 overflow-hidden">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Дата и время
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Пользователь
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Действие
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Тип
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                ID
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                IP адрес
              </th>
              <th class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                Действия
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-if="!logs.length">
              <td colspan="7" class="px-6 py-16 text-center">
                <div class="flex flex-col items-center justify-center">
                  <div class="h-24 w-24 rounded-full bg-gradient-to-br from-gray-100 to-gray-200 flex items-center justify-center mb-4">
                    <Icon name="ri:history-line" class="text-5xl text-gray-400" />
                  </div>
                  <h3 class="text-lg font-semibold text-gray-900 mb-2">Записи не найдены</h3>
                  <p class="text-sm text-gray-500 mb-6 max-w-sm">
                    {{ (filters.action || filters.entity_type) 
                      ? 'Попробуйте изменить параметры фильтров' 
                      : 'Журнал активности пуст' }}
                  </p>
                </div>
              </td>
            </tr>
            <tr v-for="log in logs" :key="log.id" class="hover:bg-gray-50">
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-900">{{ formatDate(log.created_at) }}</div>
                <div class="text-xs text-gray-500">{{ formatTime(log.created_at) }}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div v-if="log.user_email" class="flex items-center">
                  <div class="h-8 w-8 rounded-full bg-gradient-to-br from-indigo-400 to-indigo-600 flex items-center justify-center text-white text-xs font-bold mr-2">
                    {{ log.user_email.charAt(0).toUpperCase() }}
                  </div>
                  <div class="text-sm text-gray-900">{{ log.user_email }}</div>
                </div>
                <span v-else class="text-sm text-gray-400">Система</span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span
                  :class="{
                    'bg-green-100 text-green-800': log.action === 'created',
                    'bg-blue-100 text-blue-800': log.action === 'updated',
                    'bg-red-100 text-red-800': log.action === 'deleted',
                    'bg-purple-100 text-purple-800': log.action === 'login',
                    'bg-gray-100 text-gray-800': log.action === 'logout'
                  }"
                  class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full"
                >
                  {{ actionLabel(log.action) }}
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-900">{{ entityTypeLabel(log.entity_type) }}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-500">#{{ log.entity_id }}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <div class="text-sm text-gray-500">{{ log.ip_address || '-' }}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-right">
                <button
                  @click="openDetailModal(log)"
                  class="inline-flex items-center p-2 text-indigo-600 hover:text-indigo-900 hover:bg-indigo-50 rounded-lg transition-all"
                  title="Подробнее"
                >
                  <Icon name="ri:eye-line" class="text-lg" />
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      <div v-if="data && data.total > 0" class="bg-white px-6 py-4 flex items-center justify-between border-t border-gray-200">
        <div class="flex-1 flex justify-between sm:hidden">
          <button
            @click="prevPage"
            :disabled="currentPage === 1"
            class="relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-lg text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
          >
            <Icon name="ri:arrow-left-s-line" class="mr-1" />
            Назад
          </button>
          <button
            @click="nextPage"
            :disabled="currentPage >= totalPages"
            class="ml-3 relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-lg text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
          >
            Вперёд
            <Icon name="ri:arrow-right-s-line" class="ml-1" />
          </button>
        </div>
        <div class="hidden sm:flex-1 sm:flex sm:items-center sm:justify-between">
          <div>
            <p class="text-sm text-gray-700">
              Показано
              <span class="font-semibold text-gray-900">{{ Math.min((currentPage - 1) * perPage + 1, data.total) }}</span>
              -
              <span class="font-semibold text-gray-900">{{ Math.min(currentPage * perPage, data.total) }}</span>
              из
              <span class="font-semibold text-gray-900">{{ data.total }}</span>
              {{ data.total === 1 ? 'записи' : 'записей' }}
            </p>
          </div>
          <div>
            <nav class="relative z-0 inline-flex rounded-lg shadow-sm -space-x-px">
              <button
                @click="prevPage"
                :disabled="currentPage === 1"
                class="relative inline-flex items-center px-3 py-2 rounded-l-lg border border-gray-300 bg-white text-sm font-medium text-gray-700 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
              >
                <Icon name="ri:arrow-left-s-line" class="text-lg" />
              </button>
              <span class="relative inline-flex items-center px-4 py-2 border border-gray-300 bg-white text-sm font-medium text-gray-700">
                Страница {{ currentPage }} из {{ totalPages }}
              </span>
              <button
                @click="nextPage"
                :disabled="currentPage >= totalPages"
                class="relative inline-flex items-center px-3 py-2 rounded-r-lg border border-gray-300 bg-white text-sm font-medium text-gray-700 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
              >
                <Icon name="ri:arrow-right-s-line" class="text-lg" />
              </button>
            </nav>
          </div>
        </div>
      </div>
    </div>

    <!-- Detail Modal -->
    <ActivityLogDetailModal
      :show="showDetailModal"
      :log="selectedLog"
      @close="closeDetailModal"
    />
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

useHead({
  title: 'Журнал активности'
})

const { fetchActivityLogs } = useActivityLog()

const currentPage = ref(1)
const perPage = ref(50)
const filters = ref({
  action: '',
  entity_type: ''
})

const { data, pending, error, refresh } = await useAsyncData(
  'activity-logs',
  () => fetchActivityLogs({
    page: currentPage.value,
    per_page: perPage.value,
    action: filters.value.action || undefined,
    entity_type: filters.value.entity_type || undefined
  }),
  {
    watch: [currentPage]
  }
)

const logs = computed(() => data.value?.logs || [])
const totalPages = computed(() => data.value ? Math.ceil(data.value.total / data.value.per_page) : 0)

const loadLogs = () => {
  currentPage.value = 1
  refresh()
}

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++
  }
}

const prevPage = () => {
  if (currentPage.value > 1) {
    currentPage.value--
  }
}

const resetFilters = () => {
  filters.value.action = ''
  filters.value.entity_type = ''
  loadLogs()
}

const actionLabel = (action: string) => {
  const labels: Record<string, string> = {
    created: 'Создание',
    updated: 'Обновление',
    deleted: 'Удаление',
    login: 'Вход',
    logout: 'Выход'
  }
  return labels[action] || action
}

const entityTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    employee: 'Сотрудник',
    computer: 'Компьютер',
    equipment: 'Оборудование',
    request: 'Заявка',
    user: 'Пользователь',
    department: 'Отдел',
    position: 'Должность'
  }
  return labels[type] || type
}

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric'
  })
}

const formatTime = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleTimeString('ru-RU', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

const showDetailModal = ref(false)
const selectedLog = ref<any>(null)

const openDetailModal = (log: any) => {
  selectedLog.value = log
  showDetailModal.value = true
}

const closeDetailModal = () => {
  showDetailModal.value = false
  selectedLog.value = null
}
</script>
