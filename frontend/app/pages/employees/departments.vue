<template>
  <div>
    <!-- Header -->
    <div class="mb-6">
      <NuxtLink to="/employees" class="text-sm text-gray-500 hover:text-gray-700 flex items-center mb-2">
        <Icon name="ri:arrow-left-line" class="mr-1" />
        Назад к сотрудникам
      </NuxtLink>
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">Отделы и должности</h1>
          <p class="mt-1 text-sm text-gray-500">Управление структурой организации</p>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Departments -->
      <div class="bg-white shadow-sm rounded-lg border border-gray-200">
        <div class="px-6 py-4 border-b border-gray-200 bg-gray-50 flex items-center justify-between">
          <h3 class="text-lg font-medium text-gray-900 flex items-center">
            <Icon name="ri:building-line" class="mr-2 text-primary-600" />
            Отделы
          </h3>
          <button
            @click="showDepartmentForm = !showDepartmentForm"
            class="inline-flex items-center px-3 py-1.5 border border-transparent text-xs font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          >
            <Icon name="ri:add-line" class="mr-1" />
            Добавить
          </button>
        </div>

        <!-- Add Department Form -->
        <div v-if="showDepartmentForm" class="px-6 py-4 bg-blue-50 border-b border-blue-100">
          <form @submit.prevent="handleCreateDepartment" class="space-y-3">
            <div>
              <label for="dept_name" class="block text-sm font-medium text-gray-700">
                Название отдела <span class="text-red-500">*</span>
              </label>
              <input
                id="dept_name"
                v-model="departmentForm.name"
                type="text"
                required
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
                placeholder="Например: IT отдел"
              />
            </div>
            <div>
              <label for="dept_description" class="block text-sm font-medium text-gray-700">
                Описание
              </label>
              <textarea
                id="dept_description"
                v-model="departmentForm.description"
                rows="2"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
                placeholder="Краткое описание отдела"
              ></textarea>
            </div>
            <div class="flex justify-end space-x-2">
              <button
                type="button"
                @click="showDepartmentForm = false"
                class="px-3 py-1.5 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
              >
                Отмена
              </button>
              <button
                type="submit"
                :disabled="loadingDept"
                class="inline-flex items-center px-3 py-1.5 border border-transparent rounded-md text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 disabled:opacity-50"
              >
                <Icon v-if="loadingDept" name="ri:loader-4-line" class="mr-1 animate-spin" />
                {{ loadingDept ? 'Сохранение...' : 'Сохранить' }}
              </button>
            </div>
          </form>
        </div>

        <!-- Departments List -->
        <div class="divide-y divide-gray-200">
          <div v-if="!departments.length" class="px-6 py-8 text-center text-sm text-gray-500">
            <Icon name="ri:building-line" class="text-gray-300 text-3xl mb-2 mx-auto" />
            <p>Отделы не найдены</p>
          </div>
          <div
            v-for="dept in departments"
            :key="dept.id"
            class="px-6 py-4 hover:bg-gray-50 transition-colors"
          >
            <div class="flex items-start justify-between">
              <div class="flex-1">
                <h4 class="text-sm font-medium text-gray-900">{{ dept.name }}</h4>
                <p v-if="dept.description" class="mt-1 text-sm text-gray-500">{{ dept.description }}</p>
                <p class="mt-1 text-xs text-gray-400">
                  Сотрудников: {{ dept.employee_count }}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Positions -->
      <div class="bg-white shadow-sm rounded-lg border border-gray-200">
        <div class="px-6 py-4 border-b border-gray-200 bg-gray-50 flex items-center justify-between">
          <h3 class="text-lg font-medium text-gray-900 flex items-center">
            <Icon name="ri:briefcase-line" class="mr-2 text-primary-600" />
            Должности
          </h3>
          <button
            @click="showPositionForm = !showPositionForm"
            class="inline-flex items-center px-3 py-1.5 border border-transparent text-xs font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
          >
            <Icon name="ri:add-line" class="mr-1" />
            Добавить
          </button>
        </div>

        <!-- Add Position Form -->
        <div v-if="showPositionForm" class="px-6 py-4 bg-blue-50 border-b border-blue-100">
          <form @submit.prevent="handleCreatePosition" class="space-y-3">
            <div>
              <label for="pos_name" class="block text-sm font-medium text-gray-700">
                Название должности <span class="text-red-500">*</span>
              </label>
              <input
                id="pos_name"
                v-model="positionForm.name"
                type="text"
                required
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
                placeholder="Например: Системный администратор"
              />
            </div>
            <div>
              <label for="pos_department" class="block text-sm font-medium text-gray-700">
                Отдел
              </label>
              <select
                id="pos_department"
                v-model="positionForm.department_id"
                class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-primary-500 focus:ring-primary-500 sm:text-sm"
              >
                <option :value="undefined">Не привязан к отделу</option>
                <option v-for="dept in departments" :key="dept.id" :value="dept.id">
                  {{ dept.name }}
                </option>
              </select>
            </div>
            <div class="flex justify-end space-x-2">
              <button
                type="button"
                @click="showPositionForm = false"
                class="px-3 py-1.5 border border-gray-300 rounded-md text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
              >
                Отмена
              </button>
              <button
                type="submit"
                :disabled="loadingPos"
                class="inline-flex items-center px-3 py-1.5 border border-transparent rounded-md text-sm font-medium text-white bg-primary-600 hover:bg-primary-700 disabled:opacity-50"
              >
                <Icon v-if="loadingPos" name="ri:loader-4-line" class="mr-1 animate-spin" />
                {{ loadingPos ? 'Сохранение...' : 'Сохранить' }}
              </button>
            </div>
          </form>
        </div>

        <!-- Positions List -->
        <div class="divide-y divide-gray-200">
          <div v-if="!positions.length" class="px-6 py-8 text-center text-sm text-gray-500">
            <Icon name="ri:briefcase-line" class="text-gray-300 text-3xl mb-2 mx-auto" />
            <p>Должности не найдены</p>
          </div>
          <div
            v-for="pos in positions"
            :key="pos.id"
            class="px-6 py-4 hover:bg-gray-50 transition-colors"
          >
            <div class="flex items-start justify-between">
              <div class="flex-1">
                <h4 class="text-sm font-medium text-gray-900">{{ pos.name }}</h4>
                <p v-if="pos.department_name" class="mt-1 text-xs text-gray-500">
                  <Icon name="ri:building-line" class="inline mr-1" />
                  {{ pos.department_name }}
                </p>
              </div>
            </div>
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
  title: 'Отделы и должности'
})

const { fetchDepartments, fetchPositions, createDepartment, createPosition } = useEmployees()

const { data: departmentsData, refresh: refreshDepartments } = await useAsyncData('departments', () => fetchDepartments())
const { data: positionsData, refresh: refreshPositions } = await useAsyncData('positions', () => fetchPositions())

const departments = computed(() => departmentsData.value || [])
const positions = computed(() => positionsData.value || [])

const showDepartmentForm = ref(false)
const showPositionForm = ref(false)

const departmentForm = ref({
  name: '',
  description: ''
})

const positionForm = ref({
  name: '',
  department_id: undefined as number | undefined
})

const loadingDept = ref(false)
const loadingPos = ref(false)

const handleCreateDepartment = async () => {
  loadingDept.value = true
  try {
    await createDepartment({
      name: departmentForm.value.name,
      description: departmentForm.value.description || undefined
    })
    departmentForm.value = { name: '', description: '' }
    showDepartmentForm.value = false
    
    // Обновляем данные
    await refreshDepartments()
    await refreshNuxtData('departments')
  } catch (err) {
    alert('Ошибка при создании отдела')
  } finally {
    loadingDept.value = false
  }
}

const handleCreatePosition = async () => {
  loadingPos.value = true
  try {
    await createPosition({
      name: positionForm.value.name,
      department_id: positionForm.value.department_id
    })
    positionForm.value = { name: '', department_id: undefined }
    showPositionForm.value = false
    
    // Обновляем данные
    await refreshPositions()
    await refreshNuxtData('positions')
  } catch (err) {
    alert('Ошибка при создании должности')
  } finally {
    loadingPos.value = false
  }
}
</script>
