<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-blue-500 to-blue-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:computer-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Компьютеры</h1>
            <p class="mt-1 text-gray-600">Управление компьютерным парком</p>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <button class="btn btn-secondary" @click="toggleView">
            <Icon :name="viewMode === 'grid' ? 'ri:list-check' : 'ri:grid-line'" class="mr-2" />
            {{ viewMode === 'grid' ? 'Список' : 'Сетка' }}
          </button>
          <NuxtLink to="/computers/create" class="btn btn-primary">
            <Icon name="ri:add-line" class="mr-2" />
            Добавить компьютер
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Статистика -->
    <div class="card">
      <div class="p-4">
        <div class="grid grid-cols-5 gap-4">
          <div
            class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors"
            @click="filterByStatus('active')"
          >
            <div class="flex items-center gap-3">
              <div
                class="h-10 w-10 rounded-lg bg-green-100 flex items-center justify-center flex-shrink-0"
              >
                <Icon name="ri:checkbox-circle-line" class="text-xl text-green-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">Активные</p>
                <p class="text-xl font-bold text-green-600">
                  {{ stats.active }}
                </p>
              </div>
            </div>
          </div>

          <div
            class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors"
            @click="filterByStatus('repair')"
          >
            <div class="flex items-center gap-3">
              <div
                class="h-10 w-10 rounded-lg bg-amber-100 flex items-center justify-center flex-shrink-0"
              >
                <Icon name="ri:tools-line" class="text-xl text-amber-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">В ремонте</p>
                <p class="text-xl font-bold text-amber-600">
                  {{ stats.repair }}
                </p>
              </div>
            </div>
          </div>

          <div
            class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors"
            @click="filterByStatus('storage')"
          >
            <div class="flex items-center gap-3">
              <div
                class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center flex-shrink-0"
              >
                <Icon name="ri:archive-line" class="text-xl text-blue-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">На складе</p>
                <p class="text-xl font-bold text-blue-600">
                  {{ stats.storage }}
                </p>
              </div>
            </div>
          </div>

          <div
            class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors"
            @click="filterByStatus('inactive')"
          >
            <div class="flex items-center gap-3">
              <div
                class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center flex-shrink-0"
              >
                <Icon name="ri:pause-circle-line" class="text-xl text-gray-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">Неактивные</p>
                <p class="text-xl font-bold text-gray-600">
                  {{ stats.inactive }}
                </p>
              </div>
            </div>
          </div>

          <div class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors">
            <div class="flex items-center gap-3">
              <div
                class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center flex-shrink-0"
              >
                <Icon name="ri:database-line" class="text-xl text-purple-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">Всего</p>
                <p class="text-xl font-bold text-purple-600">
                  {{ stats.total }}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Фильтры и поиск -->
    <div class="card">
      <div class="p-4 bg-gradient-to-r from-gray-50 to-white">
        <div class="flex flex-wrap items-center gap-4">
          <div class="flex-1 min-w-[300px] relative">
            <Icon
              name="ri:search-line"
              class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
            />
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Поиск по имени, инвентарному номеру, сотруднику..."
              class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            />
          </div>

          <select v-model="selectedStatus" class="input-field w-40">
            <option value="">Все статусы</option>
            <option value="active">Активные</option>
            <option value="inactive">Неактивные</option>
            <option value="repair">В ремонте</option>
            <option value="storage">На складе</option>
            <option value="decommissioned">Списаны</option>
          </select>

          <select v-model="selectedType" class="input-field w-40">
            <option value="">Все типы</option>
            <option value="desktop">Десктоп</option>
            <option value="laptop">Ноутбук</option>
            <option value="server">Сервер</option>
            <option value="workstation">Рабочая станция</option>
            <option value="thin_client">Тонкий клиент</option>
          </select>

          <button class="btn btn-secondary" @click="resetFilters">
            <Icon name="ri:refresh-line" class="mr-2" />
            Сбросить
          </button>
        </div>
      </div>
    </div>

    <!-- Список компьютеров (таблица) -->
    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-blue-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка компьютеров...</p>
    </div>

    <div v-else-if="viewMode === 'list'" class="card">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Компьютер
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Тип</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Сотрудник
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">ОС</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Характеристики
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Статус
              </th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">
                Действия
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr
              v-for="computer in filteredComputers"
              :key="computer.id"
              class="hover:bg-gray-50 transition-colors"
            >
              <td class="px-4 py-4">
                <div class="flex items-center gap-3">
                  <div
                    :class="[
                      'h-10 w-10 rounded-lg flex items-center justify-center',
                      getTypeColor(computer.computer_type),
                    ]"
                  >
                    <Icon :name="getTypeIcon(computer.computer_type)" class="text-xl text-white" />
                  </div>
                  <div>
                    <p class="text-sm font-medium text-gray-900">
                      {{ computer.hostname }}
                    </p>
                    <p class="text-xs text-gray-500">
                      {{ computer.inventory_number }}
                    </p>
                  </div>
                </div>
              </td>
              <td class="px-4 py-4">
                <span class="badge bg-blue-100 text-blue-800">
                  {{ getTypeLabel(computer.computer_type) }}
                </span>
              </td>
              <td class="px-4 py-4">
                <div v-if="employeeName(computer)" class="flex items-center gap-2">
                  <Icon name="ri:user-line" class="text-gray-400" />
                  <span class="text-sm text-gray-900">{{ employeeName(computer) }}</span>
                </div>
                <span v-else class="text-sm text-gray-400">Не назначен</span>
              </td>
              <td class="px-4 py-4">
                <div class="flex items-center gap-2">
                  <Icon :name="getOSIcon(computer.os)" class="text-gray-600" />
                  <div>
                    <p class="text-sm text-gray-900">
                      {{ computer.os || 'Не указана' }}
                    </p>
                    <p class="text-xs text-gray-500">
                      {{ computer.os_version || '—' }}
                    </p>
                  </div>
                </div>
              </td>
              <td class="px-4 py-4">
                <div class="text-xs text-gray-600 space-y-1">
                  <div class="flex items-center gap-1">
                    <Icon name="ri:computer-line" class="text-gray-400" />
                    <span>{{ modelLabel(computer) }}</span>
                  </div>
                  <div class="flex items-center gap-1">
                    <Icon name="ri:map-pin-line" class="text-gray-400" />
                    <span>{{ computer.location || 'Локация не указана' }}</span>
                  </div>
                  <div class="flex items-center gap-1">
                    <Icon name="ri:time-line" class="text-gray-400" />
                    <span>{{ formatLastSeen(computer.last_seen_online) }}</span>
                  </div>
                </div>
              </td>
              <td class="px-4 py-4">
                <span :class="['badge', getStatusColor(computer.status)]">
                  {{ getStatusLabel(computer.status) }}
                </span>
              </td>
              <td class="px-4 py-4 text-right">
                <div class="flex items-center justify-end gap-2">
                  <NuxtLink
                    :to="`/computers/${computer.id}`"
                    class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                    title="Просмотр"
                  >
                    <Icon name="ri:eye-line" class="text-gray-600" />
                  </NuxtLink>
                  <NuxtLink
                    :to="`/computers/${computer.id}/edit`"
                    class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                    title="Редактировать"
                  >
                    <Icon name="ri:edit-line" class="text-gray-600" />
                  </NuxtLink>
                </div>
              </td>
            </tr>
          </tbody>
        </table>

        <div v-if="filteredComputers.length === 0" class="p-16 text-center">
          <div
            class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mx-auto mb-4"
          >
            <Icon name="ri:computer-line" class="text-4xl text-gray-400" />
          </div>
          <h3 class="text-lg font-semibold text-gray-900 mb-2">Компьютеры не найдены</h3>
          <p class="text-sm text-gray-500 mb-4">Попробуйте изменить параметры поиска</p>
        </div>
      </div>
    </div>

    <!-- Сетка компьютеров -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
      <NuxtLink
        v-for="computer in filteredComputers"
        :key="computer.id"
        :to="`/computers/${computer.id}`"
        class="card card-hover group"
      >
        <div class="p-5">
          <div class="flex items-start justify-between mb-4">
            <div
              :class="[
                'h-12 w-12 rounded-xl flex items-center justify-center',
                getTypeColor(computer.computer_type),
              ]"
            >
              <Icon :name="getTypeIcon(computer.computer_type)" class="text-2xl text-white" />
            </div>
            <span :class="['badge', getStatusColor(computer.status)]">
              {{ getStatusLabel(computer.status) }}
            </span>
          </div>

          <h3
            class="text-lg font-semibold text-gray-900 mb-1 group-hover:text-primary-600 transition-colors"
          >
            {{ computer.hostname }}
          </h3>
          <p class="text-sm text-gray-500 mb-3">
            {{ computer.inventory_number }}
          </p>

          <div class="space-y-2 mb-4">
            <div class="flex items-center gap-2 text-sm text-gray-600">
              <Icon :name="getOSIcon(computer.os)" class="text-gray-400" />
              <span>{{ computer.os }} {{ computer.os_version }}</span>
            </div>
            <div
              v-if="employeeName(computer)"
              class="flex items-center gap-2 text-sm text-gray-600"
            >
              <Icon name="ri:user-line" class="text-gray-400" />
              <span>{{ employeeName(computer) }}</span>
            </div>
          </div>

          <div class="pt-3 border-t border-gray-200 space-y-1">
            <div class="flex items-center justify-between text-xs text-gray-600">
              <span class="flex items-center gap-1">
                <Icon name="ri:computer-line" />
                Модель
              </span>
              <span class="font-medium">{{ modelLabel(computer) }}</span>
            </div>
            <div class="flex items-center justify-between text-xs text-gray-600">
              <span class="flex items-center gap-1">
                <Icon name="ri:map-pin-line" />
                Локация
              </span>
              <span class="font-medium">{{ computer.location || '—' }}</span>
            </div>
            <div class="flex items-center justify-between text-xs text-gray-600">
              <span class="flex items-center gap-1">
                <Icon name="ri:time-line" />
                В сети
              </span>
              <span class="font-medium">{{ formatLastSeen(computer.last_seen_online) }}</span>
            </div>
          </div>
        </div>
      </NuxtLink>

      <div v-if="filteredComputers.length === 0" class="col-span-full p-16 text-center">
        <div
          class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mx-auto mb-4"
        >
          <Icon name="ri:computer-line" class="text-4xl text-gray-400" />
        </div>
        <h3 class="text-lg font-semibold text-gray-900 mb-2">Компьютеры не найдены</h3>
        <p class="text-sm text-gray-500 mb-4">Попробуйте изменить параметры поиска</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Computer } from '~/composables/useComputers'
import type { Employee } from '~/composables/useEmployees'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

useHead({
  title: 'Компьютеры',
})

const toast = useToast()
const { fetchComputers } = useComputers()
const { fetchEmployees } = useEmployees()

const searchQuery = ref('')
const selectedStatus = ref('')
const selectedType = ref('')
const viewMode = ref<'list' | 'grid'>('list')
const computers = ref<Computer[]>([])
const employees = ref<Employee[]>([])
const loading = ref(false)

const fetchData = async () => {
  loading.value = true
  try {
    const [computerList, employeeList] = await Promise.all([
      fetchComputers({ per_page: 100 }),
      fetchEmployees({ per_page: 100 }),
    ])
    computers.value = computerList.computers
    employees.value = employeeList.employees
  } catch (error) {
    toast.error('Ошибка загрузки компьютеров: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const employeeName = (computer: Computer) => {
  if (computer.employee_id === null) return ''
  const employee = employees.value.find((item) => item.id === computer.employee_id)
  return employee?.full_name ?? `Сотрудник #${computer.employee_id}`
}

const modelLabel = (computer: Computer) =>
  [computer.manufacturer, computer.model].filter(Boolean).join(' ') || 'Не указана'

const formatLastSeen = (value: string | null) => {
  if (!value) return 'Нет данных'
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString('ru-RU')
}

const stats = computed(() => ({
  active: computers.value.filter((c) => c.status === 'active').length,
  repair: computers.value.filter((c) => c.status === 'repair').length,
  storage: computers.value.filter((c) => c.status === 'storage').length,
  inactive: computers.value.filter((c) => c.status === 'inactive').length,
  total: computers.value.length,
}))

const filteredComputers = computed(() => {
  let filtered = computers.value

  if (selectedStatus.value) {
    filtered = filtered.filter((c) => c.status === selectedStatus.value)
  }

  if (selectedType.value) {
    filtered = filtered.filter((c) => c.computer_type === selectedType.value)
  }

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter((c) =>
      [c.hostname, c.inventory_number, c.manufacturer, c.model, c.location, employeeName(c)]
        .filter((value): value is string => Boolean(value))
        .some((value) => value.toLowerCase().includes(query)),
    )
  }

  return filtered
})

const toggleView = () => {
  viewMode.value = viewMode.value === 'list' ? 'grid' : 'list'
}

const filterByStatus = (status: string) => {
  selectedStatus.value = selectedStatus.value === status ? '' : status
}

const resetFilters = () => {
  searchQuery.value = ''
  selectedStatus.value = ''
  selectedType.value = ''
}

const getTypeIcon = (type: string) => {
  const icons: Record<string, string> = {
    desktop: 'ri:computer-line',
    laptop: 'ri:macbook-line',
    server: 'ri:server-line',
    workstation: 'ri:dashboard-line',
    thin_client: 'ri:tv-line',
  }
  return icons[type] || 'ri:computer-line'
}

const getTypeColor = (type: string) => {
  const colors: Record<string, string> = {
    desktop: 'bg-blue-500',
    laptop: 'bg-purple-500',
    server: 'bg-red-500',
    workstation: 'bg-green-500',
    thin_client: 'bg-gray-500',
  }
  return colors[type] || 'bg-gray-500'
}

const getTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    desktop: 'Десктоп',
    laptop: 'Ноутбук',
    server: 'Сервер',
    workstation: 'Рабочая станция',
    thin_client: 'Тонкий клиент',
  }
  return labels[type] || type
}

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    active: 'bg-green-100 text-green-800',
    inactive: 'bg-gray-100 text-gray-800',
    repair: 'bg-amber-100 text-amber-800',
    storage: 'bg-blue-100 text-blue-800',
    decommissioned: 'bg-red-100 text-red-800',
  }
  return colors[status] || 'bg-gray-100 text-gray-800'
}

const getStatusLabel = (status: string) => {
  const labels: Record<string, string> = {
    active: 'Активный',
    inactive: 'Неактивный',
    repair: 'В ремонте',
    storage: 'На складе',
    decommissioned: 'Списан',
  }
  return labels[status] || status
}

const getOSIcon = (os: string | null) => {
  if (!os) return 'ri:computer-line'
  if (os.includes('Windows')) return 'ri:windows-line'
  if (os.includes('Linux')) return 'ri:ubuntu-line'
  if (os.includes('macOS')) return 'ri:apple-line'
  return 'ri:computer-line'
}

onMounted(fetchData)
</script>
