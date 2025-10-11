<template>
  <div>
    <!-- Header -->
    <div class="mb-6">
      <NuxtLink to="/employees" class="text-sm text-gray-500 hover:text-gray-700 flex items-center mb-2">
        <Icon name="ri:arrow-left-line" class="mr-1" />
        Назад к списку
      </NuxtLink>
      <h1 class="text-2xl font-bold text-gray-900">Добавить сотрудника</h1>
      <p class="mt-1 text-sm text-gray-500">Заполните информацию о новом сотруднике</p>
    </div>

    <!-- Form -->
    <form @submit.prevent="handleSubmit" class="bg-white shadow-sm rounded-lg border border-gray-200">
      <div class="p-6 space-y-6">
        <!-- Personal Info -->
        <div>
          <h3 class="text-lg font-medium text-gray-900 mb-4">Личная информация</h3>
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
        <div>
          <h3 class="text-lg font-medium text-gray-900 mb-4">Рабочая информация</h3>
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
                <option value="terminated">Уволен</option>
              </select>
            </div>
          </div>
        </div>

        <!-- Contact Info -->
        <div>
          <h3 class="text-lg font-medium text-gray-900 mb-4">Контактная информация</h3>
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
        <div>
          <label for="notes" class="block text-sm font-medium text-gray-700">
            Примечания
          </label>
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
      <div class="bg-gray-50 px-6 py-4 flex items-center justify-end space-x-3 rounded-b-lg">
        <NuxtLink
          to="/employees"
          class="px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
        >
          Отмена
        </NuxtLink>
        <button
          type="submit"
          :disabled="loading"
          class="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50"
        >
          <Icon v-if="loading" name="ri:loader-4-line" class="mr-2 animate-spin" />
          {{ loading ? 'Сохранение...' : 'Сохранить' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

useHead({
  title: 'Добавить сотрудника'
})

const { createEmployee, fetchDepartments, fetchPositions } = useEmployees()

const { data: departmentsData } = await useAsyncData('departments', () => fetchDepartments())
const { data: positionsData } = await useAsyncData('positions', () => fetchPositions())

const departments = computed(() => departmentsData.value || [])
const positions = computed(() => positionsData.value || [])

const form = ref({
  first_name: '',
  last_name: '',
  middle_name: '',
  position_id: undefined as number | undefined,
  department_id: undefined as number | undefined,
  email: '',
  phone: '',
  ad_username: '',
  hire_date: '',
  status: 'active',
  notes: ''
})

const loading = ref(false)
const errorMessage = ref('')

const handleSubmit = async () => {
  loading.value = true
  errorMessage.value = ''

  try {
    const data: any = {
      first_name: form.value.first_name,
      last_name: form.value.last_name,
      status: form.value.status
    }

    if (form.value.middle_name) data.middle_name = form.value.middle_name
    if (form.value.position_id) data.position_id = form.value.position_id
    if (form.value.department_id) data.department_id = form.value.department_id
    if (form.value.email) data.email = form.value.email
    if (form.value.phone) data.phone = form.value.phone
    if (form.value.ad_username) data.ad_username = form.value.ad_username
    if (form.value.hire_date) data.hire_date = form.value.hire_date
    if (form.value.notes) data.notes = form.value.notes

    await createEmployee(data)
    await navigateTo('/employees')
  } catch (err: any) {
    errorMessage.value = err.message || 'Ошибка при создании сотрудника'
  } finally {
    loading.value = false
  }
}
</script>
