<template>
  <div>
    <!-- Header -->
    <div class="mb-8">
      <NuxtLink :to="`/employees/${employeeId}`" class="inline-flex items-center text-sm text-gray-500 hover:text-gray-700 mb-4 group">
        <Icon name="ri:arrow-left-line" class="mr-1 group-hover:-translate-x-1 transition-transform" />
        Назад к просмотру
      </NuxtLink>
      <div class="flex items-center">
        <div class="h-14 w-14 rounded-xl bg-gradient-to-br from-indigo-400 to-indigo-600 flex items-center justify-center mr-4 shadow-lg">
          <Icon name="ri:edit-line" class="text-white text-2xl" />
        </div>
        <div>
          <h1 class="text-2xl font-bold text-gray-900">Редактировать сотрудника</h1>
          <p class="mt-1 text-sm text-gray-600">Обновите информацию о сотруднике</p>
        </div>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="pending" class="flex justify-center py-12">
      <Icon name="ri:loader-4-line" class="text-4xl text-primary-600 animate-spin" />
    </div>

    <!-- Form -->
    <form v-else-if="employee" @submit.prevent="handleSubmit" class="bg-white shadow-sm rounded-xl border border-gray-200 overflow-hidden">
      <div class="p-6 space-y-8">
        <!-- Personal Info -->
        <div>
          <div class="flex items-center mb-5">
            <div class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <Icon name="ri:user-line" class="text-blue-600 text-lg" />
            </div>
            <h3 class="text-lg font-semibold text-gray-900">Личная информация</h3>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label for="last_name" class="block text-sm font-medium text-gray-700">
                Фамилия <span class="text-red-500">*</span>
              </label>
              <input
                id="last_name"
                v-model="form.last_name"
                type="text"
                required
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              />
            </div>
            <div>
              <label for="first_name" class="block text-sm font-medium text-gray-700">
                Имя <span class="text-red-500">*</span>
              </label>
              <input
                id="first_name"
                v-model="form.first_name"
                type="text"
                required
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              />
            </div>
            <div>
              <label for="middle_name" class="block text-sm font-medium text-gray-700">
                Отчество
              </label>
              <input
                id="middle_name"
                v-model="form.middle_name"
                type="text"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              />
            </div>
          </div>
        </div>

        <!-- Work Info -->
        <div class="pt-6 border-t border-gray-200">
          <div class="flex items-center mb-5">
            <div class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center mr-3">
              <Icon name="ri:briefcase-line" class="text-purple-600 text-lg" />
            </div>
            <h3 class="text-lg font-semibold text-gray-900">Рабочая информация</h3>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label for="department_id" class="block text-sm font-medium text-gray-700">
                Отдел
              </label>
              <select
                id="department_id"
                v-model="form.department_id"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              >
                <option :value="undefined">Не выбрано</option>
                <option v-for="dept in departments" :key="dept.id" :value="dept.id">
                  {{ dept.name }}
                </option>
              </select>
            </div>
            <div>
              <label for="position_id" class="block text-sm font-medium text-gray-700">
                Должность
              </label>
              <select
                id="position_id"
                v-model="form.position_id"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              >
                <option :value="undefined">Не выбрано</option>
                <option v-for="pos in positions" :key="pos.id" :value="pos.id">
                  {{ pos.name }}
                </option>
              </select>
            </div>
            <div>
              <label for="hire_date" class="block text-sm font-medium text-gray-700">
                Дата приёма
              </label>
              <input
                id="hire_date"
                v-model="form.hire_date"
                type="date"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              />
            </div>
            <div>
              <label for="termination_date" class="block text-sm font-medium text-gray-700">
                Дата увольнения
              </label>
              <input
                id="termination_date"
                v-model="form.termination_date"
                type="date"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              />
            </div>
            <div>
              <label for="status" class="block text-sm font-medium text-gray-700">
                Статус <span class="text-red-500">*</span>
              </label>
              <select
                id="status"
                v-model="form.status"
                required
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              >
                <option value="active">Активен</option>
                <option value="inactive">Неактивен</option>
                <option value="vacation">В отпуске</option>
                <option value="sick_leave">На больничном</option>
                <option value="terminated">Уволен</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Contact Info -->
        <div class="pt-6 border-t border-gray-200">
          <div class="flex items-center mb-5">
            <div class="h-10 w-10 rounded-lg bg-green-100 flex items-center justify-center mr-3">
              <Icon name="ri:contacts-line" class="text-green-600 text-lg" />
            </div>
            <h3 class="text-lg font-semibold text-gray-900">Контактная информация</h3>
          </div>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label for="email" class="block text-sm font-medium text-gray-700">
                Email
              </label>
              <input
                id="email"
                v-model="form.email"
                type="email"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              />
            </div>
            <div>
              <label for="phone" class="block text-sm font-medium text-gray-700">
                Телефон
              </label>
              <input
                id="phone"
                v-model="form.phone"
                type="tel"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              />
            </div>
            <div>
              <label for="ad_username" class="block text-sm font-medium text-gray-700">
                AD Username
              </label>
              <input
                id="ad_username"
                v-model="form.ad_username"
                type="text"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              />
            </div>
          </div>
        </div>

        <!-- Notes -->
        <div class="pt-6 border-t border-gray-200">
          <div class="flex items-center mb-3">
            <Icon name="ri:file-text-line" class="text-gray-400 mr-2" />
            <label for="notes" class="block text-sm font-medium text-gray-700">
              Примечания
            </label>
          </div>
          <textarea
            id="notes"
            v-model="form.notes"
            rows="3"
            class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
          ></textarea>
        </div>

        <!-- Error Message -->
        <div v-if="errorMessage" class="rounded-md bg-red-50 p-4">
          <div class="flex">
            <Icon name="ri:error-warning-line" class="h-5 w-5 text-red-400" />
            <div class="ml-3">
              <p class="text-sm font-medium text-red-800">{{ errorMessage }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Actions -->
      <div class="bg-gradient-to-r from-gray-50 to-white px-6 py-5 flex items-center justify-between border-t border-gray-200">
        <p class="text-sm text-gray-500">
          <Icon name="ri:information-line" class="inline mr-1" />
          Поля отмеченные <span class="text-red-500">*</span> обязательны для заполнения
        </p>
        <div class="flex items-center space-x-3">
          <NuxtLink
            :to="`/employees/${employeeId}`"
            class="inline-flex items-center px-5 py-2.5 border border-gray-300 rounded-lg shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 transition-all"
          >
            <Icon name="ri:close-line" class="mr-2" />
            Отмена
          </NuxtLink>
          <button
            type="submit"
            :disabled="loading"
            class="inline-flex items-center px-5 py-2.5 border border-transparent rounded-lg shadow-sm text-sm font-medium text-white bg-gradient-to-r from-indigo-600 to-indigo-700 hover:from-indigo-700 hover:to-indigo-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
          >
            <Icon v-if="loading" name="ri:loader-4-line" class="mr-2 animate-spin" />
            <Icon v-else name="ri:save-line" class="mr-2" />
            {{ loading ? 'Сохранение...' : 'Сохранить изменения' }}
          </button>
        </div>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

const route = useRoute()
const employeeId = parseInt(route.params.id as string)

const { fetchEmployee, updateEmployee, fetchDepartments, fetchPositions } = useEmployees()

const { data: employee, pending } = await useAsyncData(
  `employee-${employeeId}`,
  () => fetchEmployee(employeeId)
)

const { data: departmentsData } = await useAsyncData('departments', () => fetchDepartments())
const { data: positionsData } = await useAsyncData('positions', () => fetchPositions())

const departments = computed(() => departmentsData.value || [])
const positions = computed(() => positionsData.value || [])

useHead({
  title: computed(() => employee.value ? `Редактировать ${employee.value.full_name}` : 'Редактировать сотрудника')
})

const form = ref({
  first_name: '',
  last_name: '',
  middle_name: '',
  position_id: undefined as number | undefined,
  department_id: undefined as number | undefined,
  email: '',
  phone: '',
  ad_username: '',
  hire_date: '' as string | undefined,
  termination_date: '' as string | undefined,
  status: 'active',
  notes: ''
})

// Initialize form with employee data
watch(employee, (emp) => {
  if (emp) {
    form.value.first_name = emp.first_name
    form.value.last_name = emp.last_name
    form.value.middle_name = emp.middle_name || ''
    form.value.position_id = emp.position_id || undefined
    form.value.department_id = emp.department_id || undefined
    form.value.email = emp.email || ''
    form.value.phone = emp.phone || ''
    form.value.ad_username = emp.ad_username || ''
    form.value.hire_date = emp.hire_date ? emp.hire_date.split('T')[0] : undefined
    form.value.termination_date = emp.termination_date ? emp.termination_date.split('T')[0] : undefined
    form.value.status = emp.status
    form.value.notes = emp.notes || ''
  }
}, { immediate: true })

const loading = ref(false)
const errorMessage = ref('')

const handleSubmit = async () => {
  loading.value = true
  errorMessage.value = ''

  try {
    const data: any = {}

    if (form.value.first_name !== employee.value?.first_name) data.first_name = form.value.first_name
    if (form.value.last_name !== employee.value?.last_name) data.last_name = form.value.last_name
    if (form.value.middle_name !== (employee.value?.middle_name || '')) data.middle_name = form.value.middle_name || undefined
    if (form.value.position_id !== employee.value?.position_id) data.position_id = form.value.position_id
    if (form.value.department_id !== employee.value?.department_id) data.department_id = form.value.department_id
    if (form.value.email !== (employee.value?.email || '')) data.email = form.value.email || undefined
    if (form.value.phone !== (employee.value?.phone || '')) data.phone = form.value.phone || undefined
    if (form.value.ad_username !== (employee.value?.ad_username || '')) data.ad_username = form.value.ad_username || undefined
    const empHireDate = employee.value?.hire_date?.split('T')[0]
    const empTermDate = employee.value?.termination_date?.split('T')[0]
    
    if (form.value.hire_date !== empHireDate) data.hire_date = form.value.hire_date || undefined
    if (form.value.termination_date !== empTermDate) data.termination_date = form.value.termination_date || undefined
    if (form.value.status !== employee.value?.status) data.status = form.value.status
    if (form.value.notes !== (employee.value?.notes || '')) data.notes = form.value.notes || undefined

    await updateEmployee(employeeId, data)
    
    // Инвалидируем кеш для обновления данных
    await refreshNuxtData(`employee-${employeeId}`)
    await refreshNuxtData('employees')
    
    await navigateTo(`/employees/${employeeId}`)
  } catch (err: any) {
    errorMessage.value = err.message || 'Ошибка при обновлении сотрудника'
  } finally {
    loading.value = false
  }
}
</script>
