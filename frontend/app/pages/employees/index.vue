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
            <button
              v-if="selectedIds.length > 0"
              @click="openBulkDeleteModal"
              class="inline-flex items-center rounded-lg border border-red-300 bg-white px-4 py-2.5 text-sm font-medium text-red-700 shadow-sm hover:bg-red-50 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2 transition-all"
            >
              <Icon name="ri:delete-bin-line" class="mr-2 h-5 w-5" />
              Удалить ({{ selectedIds.length }})
            </button>
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
    <div class="mb-6 bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
      <div class="px-6 py-4 bg-gradient-to-r from-gray-50 to-white border-b border-gray-200">
        <div class="flex items-center">
          <Icon name="ri:filter-3-line" class="text-primary-500 text-xl mr-2" />
          <h3 class="text-base font-semibold text-gray-900">Фильтры</h3>
        </div>
      </div>
      <div class="p-6">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Статус</label>
            <select
              v-model="filters.status"
              @change="loadEmployees"
              class="block w-full rounded-lg border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm transition-colors"
            >
              <option value="">Все статусы</option>
              <option value="active">Активные</option>
              <option value="inactive">Неактивные</option>
              <option value="terminated">Уволенные</option>
            </select>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Отдел</label>
            <select
              v-model="filters.department_id"
              @change="loadEmployees"
              class="block w-full rounded-lg border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm transition-colors"
            >
              <option :value="undefined">Все отделы</option>
              <option v-for="dept in departments" :key="dept.id" :value="dept.id">
                {{ dept.name }}
              </option>
            </select>
          </div>
          <div class="flex items-end">
            <button
              v-if="filters.status || filters.department_id"
              @click="resetFilters"
              class="w-full inline-flex items-center justify-center px-4 py-2 border border-gray-300 rounded-lg shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 transition-all"
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
    <div v-else class="bg-white shadow-sm rounded-xl border border-gray-200 overflow-hidden">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left">
              <input
                type="checkbox"
                v-model="selectAll"
                @change="toggleSelectAll"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded cursor-pointer"
              />
            </th>
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
          <tr v-if="!employees.length">
            <td colspan="8" class="px-6 py-16 text-center">
              <div class="flex flex-col items-center justify-center">
                <div class="h-24 w-24 rounded-full bg-gradient-to-br from-gray-100 to-gray-200 flex items-center justify-center mb-4">
                  <Icon name="ri:user-search-line" class="text-5xl text-gray-400" />
                </div>
                <h3 class="text-lg font-semibold text-gray-900 mb-2">Сотрудники не найдены</h3>
                <p class="text-sm text-gray-500 mb-6 max-w-sm">
                  {{ (filters.status || filters.department_id) 
                    ? 'Попробуйте изменить параметры фильтров или сбросить их' 
                    : 'Начните с добавления первого сотрудника в систему' }}
                </p>
                <div class="flex items-center space-x-3">
                  <button
                    v-if="filters.status || filters.department_id"
                    @click="resetFilters"
                    class="inline-flex items-center px-4 py-2 border border-gray-300 rounded-lg shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 transition-all"
                  >
                    <Icon name="ri:refresh-line" class="mr-2" />
                    Сбросить фильтры
                  </button>
                  <NuxtLink
                    to="/employees/create"
                    class="inline-flex items-center px-4 py-2 border border-transparent rounded-lg shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 transition-all"
                  >
                    <Icon name="ri:add-line" class="mr-2" />
                    Добавить сотрудника
                  </NuxtLink>
                </div>
              </div>
            </td>
          </tr>
          <tr v-for="employee in employees" :key="employee.id" class="hover:bg-gray-50">
            <td class="px-6 py-4 whitespace-nowrap">
              <input
                type="checkbox"
                :checked="selectedIds.includes(employee.id)"
                @change="toggleSelect(employee.id)"
                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded cursor-pointer"
              />
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="flex items-center">
                <div class="h-10 w-10 flex-shrink-0 rounded-full bg-gradient-to-br from-primary-400 to-primary-600 flex items-center justify-center text-white font-bold shadow-sm">
                  {{ employee.first_name.charAt(0) }}{{ employee.last_name.charAt(0) }}
                </div>
                <div class="ml-3">
                  <div class="text-sm font-medium text-gray-900">{{ employee.full_name }}</div>
                  <div v-if="employee.ad_username" class="text-xs text-gray-500">
                    <Icon name="ri:windows-fill" class="inline mr-1" />
                    {{ employee.ad_username }}
                  </div>
                </div>
              </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm text-gray-900">{{ employee.position_name || '-' }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm text-gray-900">{{ employee.department_name || '-' }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div v-if="employee.email" class="flex items-center text-sm text-gray-900">
                <Icon name="ri:mail-line" class="text-gray-400 mr-2" />
                <a :href="`mailto:${employee.email}`" class="text-primary-600 hover:text-primary-900 transition-colors">
                  {{ employee.email }}
                </a>
              </div>
              <span v-else class="text-sm text-gray-400">-</span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div v-if="employee.phone" class="flex items-center text-sm text-gray-900">
                <Icon name="ri:phone-line" class="text-gray-400 mr-2" />
                {{ employee.phone }}
              </div>
              <span v-else class="text-sm text-gray-400">-</span>
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
              <div class="flex items-center justify-end space-x-2">
                <NuxtLink
                  :to="`/employees/${employee.id}`"
                  class="inline-flex items-center p-2 text-primary-600 hover:text-primary-900 hover:bg-primary-50 rounded-lg transition-all"
                  title="Просмотр"
                >
                  <Icon name="ri:eye-line" class="text-lg" />
                </NuxtLink>
                <NuxtLink
                  :to="`/employees/${employee.id}/edit`"
                  class="inline-flex items-center p-2 text-indigo-600 hover:text-indigo-900 hover:bg-indigo-50 rounded-lg transition-all"
                  title="Редактировать"
                >
                  <Icon name="ri:edit-line" class="text-lg" />
                </NuxtLink>
                <button
                  @click="openDeleteModal(employee)"
                  class="inline-flex items-center p-2 text-red-600 hover:text-red-900 hover:bg-red-50 rounded-lg transition-all"
                  title="Удалить"
                >
                  <Icon name="ri:delete-bin-line" class="text-lg" />
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>

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
              {{ data.total === 1 ? 'сотрудника' : 'сотрудников' }}
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

    <!-- Delete Confirmation Modal -->
    <ConfirmModal
      :show="showDeleteModal"
      :loading="deleting"
      title="Удалить сотрудника?"
      :message="employeeToDelete ? `Вы уверены, что хотите удалить сотрудника ${employeeToDelete.full_name}? Это действие нельзя будет отменить.` : ''"
      confirm-text="Да, удалить"
      cancel-text="Отмена"
      loading-text="Удаление..."
      variant="danger"
      icon="ri:delete-bin-line"
      @confirm="confirmDelete"
      @cancel="closeDeleteModal"
    />

    <!-- Bulk Delete Confirmation Modal -->
    <ConfirmModal
      :show="showBulkDeleteModal"
      :loading="bulkDeleting"
      title="Массовое удаление сотрудников"
      :message="`Вы уверены, что хотите удалить ${selectedIds.length} ${selectedIds.length === 1 ? 'сотрудника' : 'сотрудников'}? Это действие нельзя будет отменить.`"
      confirm-text="Да, удалить всех"
      cancel-text="Отмена"
      loading-text="Удаление..."
      variant="danger"
      icon="ri:delete-bin-line"
      @confirm="confirmBulkDelete"
      @cancel="closeBulkDeleteModal"
    />
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

useHead({
  title: 'Сотрудники'
})

const { fetchEmployees, fetchDepartments, deleteEmployee, bulkDeleteEmployees } = useEmployees()

const currentPage = ref(1)
const perPage = ref(20)
const filters = ref({
  status: '',
  department_id: undefined as number | undefined
})

const selectedIds = ref<number[]>([])
const selectAll = ref(false)

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

const resetFilters = () => {
  filters.value.status = ''
  filters.value.department_id = undefined
  loadEmployees()
}

const showDeleteModal = ref(false)
const employeeToDelete = ref<any>(null)
const deleting = ref(false)

const showBulkDeleteModal = ref(false)
const bulkDeleting = ref(false)

const toggleSelectAll = () => {
  if (selectAll.value) {
    selectedIds.value = employees.value.map(e => e.id)
  } else {
    selectedIds.value = []
  }
}

const toggleSelect = (id: number) => {
  const index = selectedIds.value.indexOf(id)
  if (index > -1) {
    selectedIds.value.splice(index, 1)
  } else {
    selectedIds.value.push(id)
  }
  selectAll.value = selectedIds.value.length === employees.value.length
}

watch(employees, () => {
  selectedIds.value = selectedIds.value.filter(id => 
    employees.value.some(e => e.id === id)
  )
  selectAll.value = selectedIds.value.length > 0 && selectedIds.value.length === employees.value.length
})

const openDeleteModal = (employee: any) => {
  employeeToDelete.value = employee
  showDeleteModal.value = true
}

const closeDeleteModal = () => {
  if (!deleting.value) {
    showDeleteModal.value = false
    employeeToDelete.value = null
  }
}

const confirmDelete = async () => {
  if (!employeeToDelete.value) return
  
  deleting.value = true
  try {
    await deleteEmployee(employeeToDelete.value.id)
    await refresh()
    showDeleteModal.value = false
    employeeToDelete.value = null
  } catch (err) {
    alert('Ошибка при удалении сотрудника')
  } finally {
    deleting.value = false
  }
}

const openBulkDeleteModal = () => {
  if (selectedIds.value.length === 0) return
  showBulkDeleteModal.value = true
}

const closeBulkDeleteModal = () => {
  if (!bulkDeleting.value) {
    showBulkDeleteModal.value = false
  }
}

const confirmBulkDelete = async () => {
  if (selectedIds.value.length === 0) return
  
  bulkDeleting.value = true
  try {
    const result = await bulkDeleteEmployees(selectedIds.value)
    if (result.failed_ids.length > 0) {
      alert(`Удалено: ${result.deleted_count}. Не удалось удалить: ${result.failed_ids.length}`)
    }
    selectedIds.value = []
    selectAll.value = false
    await refresh()
    showBulkDeleteModal.value = false
  } catch (err) {
    alert('Ошибка при массовом удалении сотрудников')
  } finally {
    bulkDeleting.value = false
  }
}
</script>
