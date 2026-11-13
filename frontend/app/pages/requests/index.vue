<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-amber-500 to-amber-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:file-list-3-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Заявки</h1>
            <p class="mt-1 text-gray-600">Управление заявками на обслуживание</p>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <button class="btn btn-secondary" @click="toggleView">
            <Icon
              :name="viewMode === 'list' ? 'ri:layout-grid-line' : 'ri:list-check'"
              class="mr-2"
            />
            {{ viewMode === 'list' ? 'Kanban' : 'Список' }}
          </button>
          <NuxtLink to="/requests/create" class="btn btn-primary">
            <Icon name="ri:add-line" class="mr-2" />
            Создать заявку
          </NuxtLink>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <div class="card card-hover cursor-pointer" @click="filterByStatus('new')">
        <div class="p-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600">Новые</p>
              <p class="text-2xl font-bold text-gray-900 mt-1">
                {{ stats.new }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-lg bg-blue-100 flex items-center justify-center">
              <Icon name="ri:file-add-line" class="text-2xl text-blue-600" />
            </div>
          </div>
        </div>
      </div>

      <div class="card card-hover cursor-pointer" @click="filterByStatus('in_progress')">
        <div class="p-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600">В работе</p>
              <p class="text-2xl font-bold text-gray-900 mt-1">
                {{ stats.in_progress }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-lg bg-amber-100 flex items-center justify-center">
              <Icon name="ri:time-line" class="text-2xl text-amber-600" />
            </div>
          </div>
        </div>
      </div>

      <div class="card card-hover cursor-pointer" @click="filterByStatus('resolved')">
        <div class="p-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600">Решено</p>
              <p class="text-2xl font-bold text-gray-900 mt-1">
                {{ stats.resolved }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-lg bg-green-100 flex items-center justify-center">
              <Icon name="ri:checkbox-circle-line" class="text-2xl text-green-600" />
            </div>
          </div>
        </div>
      </div>

      <div class="card card-hover cursor-pointer" @click="filterByStatus('closed')">
        <div class="p-4">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600">Закрыто</p>
              <p class="text-2xl font-bold text-gray-900 mt-1">
                {{ stats.closed }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-lg bg-red-100 flex items-center justify-center">
              <Icon name="ri:close-circle-line" class="text-2xl text-red-600" />
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-amber-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка заявок...</p>
    </div>

    <div v-else-if="viewMode === 'list'" class="card">
      <div class="p-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
        <div class="flex items-center gap-4">
          <div class="flex-1 relative">
            <Icon
              name="ri:search-line"
              class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
            />
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Поиск заявок..."
              class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            />
          </div>
          <select v-model="selectedStatus" class="input-field w-48">
            <option value="">Все статусы</option>
            <option value="new">Новые</option>
            <option value="in_progress">В работе</option>
            <option value="waiting">Ожидание</option>
            <option value="resolved">Решено</option>
            <option value="closed">Закрыто</option>
            <option value="cancelled">Отменено</option>
          </select>
        </div>
      </div>

      <div class="divide-y divide-gray-200">
        <NuxtLink
          v-for="request in filteredRequests"
          :key="request.id"
          :to="`/requests/${request.id}`"
          class="block p-4 hover:bg-gray-50 transition-colors cursor-pointer"
        >
          <div class="flex items-start gap-4">
            <div
              :class="[
                'flex h-10 w-10 items-center justify-center rounded-lg flex-shrink-0',
                getStatusColor(request.status),
              ]"
            >
              <Icon :name="getStatusIcon(request.status)" class="text-lg text-white" />
            </div>
            <div class="flex-1 min-w-0">
              <div class="flex items-start justify-between gap-4">
                <div class="flex-1">
                  <p class="text-sm font-medium text-gray-900">
                    {{ request.title }}
                  </p>
                  <p class="text-sm text-gray-600 mt-1">
                    {{ request.description }}
                  </p>
                  <div class="mt-2 flex items-center gap-3 text-xs text-gray-500">
                    <span class="flex items-center gap-1">
                      <Icon name="ri:user-line" />
                      {{ requesterName(request) }}
                    </span>
                    <span class="flex items-center gap-1">
                      <Icon name="ri:time-line" />
                      {{ formatDateTime(request.created_at) }}
                    </span>
                    <span :class="['badge', getPriorityColor(request.priority)]">
                      {{ getPriorityLabel(request.priority) }}
                    </span>
                  </div>
                </div>
                <span :class="['badge', getStatusBadgeColor(request.status)]">
                  {{ getStatusLabel(request.status) }}
                </span>
              </div>
            </div>
          </div>
        </NuxtLink>

        <div v-if="!filteredRequests.length" class="p-16 text-center">
          <div
            class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mx-auto mb-4"
          >
            <Icon name="ri:file-list-line" class="text-4xl text-gray-400" />
          </div>
          <h3 class="text-lg font-semibold text-gray-900 mb-2">Заявки не найдены</h3>
          <p class="text-sm text-gray-500 mb-4">
            {{ requests.length === 0 ? 'Создайте первую заявку' : 'Измените параметры поиска' }}
          </p>
          <NuxtLink to="/requests/create" class="btn btn-primary">
            <Icon name="ri:add-line" class="mr-2" />
            Создать заявку
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Kanban View -->
    <div v-else class="grid grid-cols-1 md:grid-cols-4 gap-6">
      <KanbanColumn
        title="Новые"
        status="new"
        color="blue"
        :requests="getRequestsByStatus('new')"
        @drop="handleDrop"
      />
      <KanbanColumn
        title="В работе"
        status="in_progress"
        color="amber"
        :requests="getRequestsByStatus('in_progress')"
        @drop="handleDrop"
      />
      <KanbanColumn
        title="Решено"
        status="resolved"
        color="green"
        :requests="getRequestsByStatus('resolved')"
        @drop="handleDrop"
      />
      <KanbanColumn
        title="Закрыто"
        status="closed"
        color="red"
        :requests="getRequestsByStatus('closed')"
        @drop="handleDrop"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { RequestItem } from '~/composables/useRequests'
import type { Employee } from '~/composables/useEmployees'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

useHead({
  title: 'Заявки',
})

const toast = useToast()
const { fetchRequests, updateRequest } = useRequests()
const { fetchEmployees } = useEmployees()

const searchQuery = ref('')
const selectedStatus = ref('')
const viewMode = ref<'list' | 'kanban'>('list')
const requests = ref<RequestItem[]>([])
const employees = ref<Employee[]>([])
const loading = ref(false)

const fetchData = async () => {
  loading.value = true
  try {
    const [requestList, employeeList] = await Promise.all([
      fetchRequests({ per_page: 100 }),
      fetchEmployees({ per_page: 100 }),
    ])
    requests.value = requestList.requests
    employees.value = employeeList.employees
  } catch (error) {
    toast.error('Ошибка загрузки заявок: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const requesterName = (request: RequestItem) => {
  const employee = employees.value.find((item) => item.id === request.requester_id)
  return employee?.full_name ?? `Сотрудник #${request.requester_id}`
}

const formatDateTime = (value: string | null) => {
  if (!value) return '—'
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString('ru-RU')
}

const stats = computed(() => ({
  new: requests.value.filter((r) => r.status === 'new').length,
  in_progress: requests.value.filter((r) => r.status === 'in_progress').length,
  resolved: requests.value.filter((r) => r.status === 'resolved').length,
  closed: requests.value.filter((r) => r.status === 'closed').length,
}))

const filteredRequests = computed(() => {
  let filtered = requests.value

  if (selectedStatus.value) {
    filtered = filtered.filter((request) => request.status === selectedStatus.value)
  }

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter((request) =>
      [request.title, request.description, requesterName(request)]
        .filter((value): value is string => Boolean(value))
        .some((value) => value.toLowerCase().includes(query)),
    )
  }

  return filtered
})

const filterByStatus = (status: string) => {
  selectedStatus.value = status
}

const getStatusIcon = (status: string) => {
  const icons: Record<string, string> = {
    new: 'ri:file-add-line',
    in_progress: 'ri:time-line',
    waiting: 'ri:pause-circle-line',
    resolved: 'ri:checkbox-circle-line',
    closed: 'ri:archive-line',
    cancelled: 'ri:close-circle-line',
  }
  return icons[status] || 'ri:file-line'
}

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    new: 'bg-blue-500',
    in_progress: 'bg-amber-500',
    waiting: 'bg-purple-500',
    resolved: 'bg-green-500',
    closed: 'bg-gray-500',
    cancelled: 'bg-red-500',
  }
  return colors[status] || 'bg-gray-500'
}

const getStatusBadgeColor = (status: string) => {
  const colors: Record<string, string> = {
    new: 'bg-blue-100 text-blue-800',
    in_progress: 'bg-amber-100 text-amber-800',
    waiting: 'bg-purple-100 text-purple-800',
    resolved: 'bg-green-100 text-green-800',
    closed: 'bg-gray-100 text-gray-800',
    cancelled: 'bg-red-100 text-red-800',
  }
  return colors[status] || 'bg-gray-100 text-gray-800'
}

const getStatusLabel = (status: string) => {
  const labels: Record<string, string> = {
    new: 'Новая',
    in_progress: 'В работе',
    waiting: 'Ожидание',
    resolved: 'Решена',
    closed: 'Закрыта',
    cancelled: 'Отменена',
  }
  return labels[status] || status
}

const getPriorityColor = (priority: string) => {
  const colors: Record<string, string> = {
    low: 'bg-gray-100 text-gray-800',
    medium: 'bg-blue-100 text-blue-800',
    high: 'bg-amber-100 text-amber-800',
    critical: 'bg-red-100 text-red-800',
  }
  return colors[priority] || 'bg-gray-100 text-gray-800'
}

const getPriorityLabel = (priority: string) => {
  const labels: Record<string, string> = {
    low: 'Низкий',
    medium: 'Средний',
    high: 'Высокий',
    critical: 'Критический',
  }
  return labels[priority] || priority
}

const toggleView = () => {
  viewMode.value = viewMode.value === 'list' ? 'kanban' : 'list'
}

const getRequestsByStatus = (status: string) => {
  return filteredRequests.value.filter((request) => request.status === status)
}

const handleDrop = async (requestId: number, newStatus: string) => {
  const request = requests.value.find((item) => item.id === requestId)
  if (!request || request.status === newStatus) return

  const previousStatus = request.status
  request.status = newStatus

  try {
    const updated = await updateRequest(request.id, { status: newStatus })
    Object.assign(request, updated)
    toast.success(`Заявка перемещена в «${getStatusLabel(newStatus)}»`)
  } catch (error) {
    request.status = previousStatus
    toast.error('Не удалось изменить статус: ' + getErrorMessage(error))
  }
}

onMounted(fetchData)
</script>
