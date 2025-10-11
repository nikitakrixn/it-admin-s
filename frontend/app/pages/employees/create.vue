<template>
  <div>
    <!-- Header -->
    <div class="mb-8">
      <NuxtLink to="/employees" class="inline-flex items-center text-sm text-gray-500 hover:text-gray-700 mb-4 group">
        <Icon name="ri:arrow-left-line" class="mr-1 group-hover:-translate-x-1 transition-transform" />
        Назад к списку
      </NuxtLink>
      <div class="flex items-center">
        <div class="h-14 w-14 rounded-xl bg-gradient-to-br from-primary-400 to-primary-600 flex items-center justify-center mr-4 shadow-lg">
          <Icon name="ri:user-add-line" class="text-white text-2xl" />
        </div>
        <div>
          <h1 class="text-2xl font-bold text-gray-900">Добавить сотрудника</h1>
          <p class="mt-1 text-sm text-gray-600">Заполните информацию о новом сотруднике</p>
        </div>
      </div>
    </div>

    <!-- Form -->
    <form @submit.prevent="handleSubmit" class="bg-white shadow-sm rounded-xl border border-gray-200 overflow-hidden">
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
            to="/employees"
            class="inline-flex items-center px-5 py-2.5 border border-gray-300 rounded-lg shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 transition-all"
          >
            <Icon name="ri:close-line" class="mr-2" />
            Отмена
          </NuxtLink>
          <button
            type="submit"
            :disabled="loading"
            class="inline-flex items-center px-5 py-2.5 border border-transparent rounded-lg shadow-sm text-sm font-medium text-white bg-gradient-to-r from-primary-600 to-primary-700 hover:from-primary-700 hover:to-primary-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
          >
            <Icon v-if="loading" name="ri:loader-4-line" class="mr-2 animate-spin" />
            <Icon v-else name="ri:save-line" class="mr-2" />
            {{ loading ? 'Сохранение...' : 'Сохранить сотрудника' }}
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
