<template>
  <div>
    <!-- Header -->
    <div class="relative mb-10 pb-5">
      <div class="relative z-10 pt-4 pb-5">
        <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between">
          <div class="flex-1 min-w-0">
            <div class="flex items-center">
              <div class="h-16 w-16 flex-shrink-0 rounded-xl bg-gradient-to-br from-primary-400 to-primary-600 flex items-center justify-center mr-5 shadow-lg">
                <Icon name="ri:team-line" class="text-white text-3xl" />
              </div>
              <div>
                <h1 class="text-2xl font-bold text-gray-900 sm:text-3xl">Сотрудники</h1>
                <p class="mt-2 text-sm text-gray-600 max-w-4xl">
                  Управление сотрудниками организации, их учетными записями и оборудованием
                </p>
              </div>
            </div>
          </div>
          <div class="mt-5 sm:mt-0 flex flex-shrink-0 space-x-2">
            <NuxtLink
              to="/employees/departments"
              class="inline-flex items-center rounded-lg border border-gray-300 bg-white px-4 py-2.5 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 transition-all"
            >
              <Icon name="ri:building-line" class="mr-2 h-5 w-5 text-gray-500" />
              Отделы
            </NuxtLink>
            <NuxtLink
              to="/employees/create"
              class="inline-flex items-center rounded-lg border border-transparent bg-primary-600 px-4 py-2.5 text-sm font-medium text-white shadow-sm hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2 transition-all"
            >
              <Icon name="ri:add-line" class="mr-2 h-5 w-5" />
              Добавить сотрудника
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>

    <!-- Filters -->
    <div class="mb-6 bg-white rounded-lg shadow-sm border border-gray-200 p-4">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Статус</label>
          <select
            v-model="filters.status"
            @change="loadEmployees"
            class="block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
          >
            <option value="">Все</option>
            <option value="active">Активные</option>
            <option value="inactive">Неактивные</option>
            <option value="terminated">Уволенные</option>
          </select>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Отдел</label>
          <select
            v-model="filters.department_id"
            @change="loadEmployees"
            class="block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
          >
            <option :value="undefined">Все отделы</option>
            <option v-for="dept in departments" :key="dept.id" :value="dept.id">
              {{ dept.name }}
            </option>
          </select>
        </div>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="pending" class="flex justify-center py-12">
      <Icon name="ri:loader-4-line" class="text-4xl text-primary-600 animate-spin" />
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
    <div v-else class="bg-white shadow-sm rounded-lg border border-gray-200 overflow-hidden">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              ФИО
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Должность
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Отдел
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Email
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Телефон
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Статус
            </th>
            <th class="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
              Действия
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="employee in employees" :key="employee.id" class="hover:bg-gray-50">
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm font-medium text-gray-900">{{ employee.full_name }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm text-gray-900">{{ employee.position_name || '-' }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm text-gray-900">{{ employee.department_name || '-' }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm text-gray-900">{{ employee.email || '-' }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm text-gray-900">{{ employee.phone || '-' }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span
                :class="{
                  'bg-green-100 text-green-800': employee.status === 'active',
                  'bg-gray-100 text-gray-800': employee.status === 'inactive',
                  'bg-red-100 text-red-800': employee.status === 'terminated'
                }"
                class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full"
              >
                {{ statusLabel(employee.status) }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
              <NuxtLink
                :to="`/employees/${employee.id}`"
                class="text-primary-600 hover:text-primary-900 mr-3"
              >
                Просмотр
              </NuxtLink>
              <NuxtLink
                :to="`/employees/${employee.id}/edit`"
                class="text-indigo-600 hover:text-indigo-900 mr-3"
              >
                Изменить
              </NuxtLink>
              <button
                @click="confirmDelete(employee)"
                class="text-red-600 hover:text-red-900"
              >
                Удалить
              </button>
            </td>
          </tr>
        </tbody>
      </table>

      <!-- Pagination -->
      <div v-if="data" class="bg-white px-4 py-3 flex items-center justify-between border-t border-gray-200 sm:px-6">
        <div class="flex-1 flex justify-between sm:hidden">
          <button
            @click="prevPage"
            :disabled="currentPage === 1"
            class="relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50"
          >
            Назад
          </button>
          <button
            @click="nextPage"
            :disabled="currentPage >= totalPages"
            class="ml-3 relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50"
          >
            Вперёд
          </button>
        </div>
        <div class="hidden sm:flex-1 sm:flex sm:items-center sm:justify-between">
          <div>
            <p class="text-sm text-gray-700">
              Показано
              <span class="font-medium">{{ (currentPage - 1) * perPage + 1 }}</span>
              -
              <span class="font-medium">{{ Math.min(currentPage * perPage, data.total) }}</span>
              из
              <span class="font-medium">{{ data.total }}</span>
              результатов
            </p>
          </div>
          <div>
            <nav class="relative z-0 inline-flex rounded-md shadow-sm -space-x-px">
              <button
                @click="prevPage"
                :disabled="currentPage === 1"
                class="relative inline-flex items-center px-2 py-2 rounded-l-md border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 disabled:opacity-50"
              >
                <Icon name="ri:arrow-left-s-line" />
              </button>
              <button
                @click="nextPage"
                :disabled="currentPage >= totalPages"
                class="relative inline-flex items-center px-2 py-2 rounded-r-md border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 disabled:opacity-50"
              >
                <Icon name="ri:arrow-right-s-line" />
              </button>
            </nav>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

useHead({
  title: 'Сотрудники'
})

const { fetchEmployees, fetchDepartments, deleteEmployee } = useEmployees()

const currentPage = ref(1)
const perPage = ref(20)
const filters = ref({
  status: '',
  department_id: undefined as number | undefined
})

const { data: departmentsData } = await useAsyncData('departments', () => fetchDepartments())
const departments = computed(() => departmentsData.value || [])

const { data, pending, error, refresh } = await useAsyncData(
  'employees',
  () => fetchEmployees({
    page: currentPage.value,
    per_page: perPage.value,
    status: filters.value.status || undefined,
    department_id: filters.value.department_id
  }),
  {
    watch: [currentPage]
  }
)

const employees = computed(() => data.value?.employees || [])
const totalPages = computed(() => data.value ? Math.ceil(data.value.total / data.value.per_page) : 0)

const loadEmployees = () => {
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

const statusLabel = (status: string) => {
  const labels: Record<string, string> = {
    active: 'Активен',
    inactive: 'Неактивен',
    terminated: 'Уволен'
  }
  return labels[status] || status
}

const confirmDelete = async (employee: any) => {
  if (confirm(`Вы уверены, что хотите удалить сотрудника ${employee.full_name}?`)) {
    try {
      await deleteEmployee(employee.id)
      refresh()
    } catch (err) {
      alert('Ошибка при удалении сотрудника')
    }
  }
}
</script>
