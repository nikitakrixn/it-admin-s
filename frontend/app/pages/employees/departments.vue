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
      <div class="bg-white shadow-sm rounded-xl border border-gray-200 overflow-hidden hover:shadow-md transition-shadow">
        <div class="px-6 py-4 bg-gradient-to-r from-blue-50 to-white border-b border-gray-200 flex items-center justify-between">
          <h3 class="text-lg font-semibold text-gray-900 flex items-center">
            <div class="h-8 w-8 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
              <Icon name="ri:building-line" class="text-blue-600" />
            </div>
            Отделы
          </h3>
          <button
            @click="showDepartmentForm = !showDepartmentForm"
            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-lg text-white bg-gradient-to-r from-primary-600 to-primary-700 hover:from-primary-700 hover:to-primary-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 shadow-sm transition-all"
          >
            <Icon :name="showDepartmentForm ? 'ri:close-line' : 'ri:add-line'" class="mr-2" />
            {{ showDepartmentForm ? 'Отмена' : 'Добавить' }}
          </button>
        </div>

        <!-- Add Department Form -->
        <div v-if="showDepartmentForm" class="px-6 py-5 bg-gradient-to-r from-blue-50 to-indigo-50 border-b border-blue-100">
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
            class="px-6 py-4 hover:bg-gradient-to-r hover:from-blue-50 hover:to-transparent transition-all group"
          >
            <div class="flex items-start justify-between">
              <div class="flex items-start flex-1">
                <div class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center mr-3 group-hover:bg-blue-200 transition-colors">
                  <Icon name="ri:building-line" class="text-blue-600" />
                </div>
                <div class="flex-1">
                  <h4 class="text-sm font-semibold text-gray-900">{{ dept.name }}</h4>
                  <p v-if="dept.description" class="mt-1 text-sm text-gray-600">{{ dept.description }}</p>
                  <div class="mt-2 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                    <Icon name="ri:user-line" class="mr-1" />
                    {{ dept.employee_count }} сотрудников
                  </div>
                </div>
              </div>
              <button
                @click="openDeleteDepartmentModal(dept)"
                class="opacity-0 group-hover:opacity-100 inline-flex items-center p-2 text-red-600 hover:text-red-900 hover:bg-red-50 rounded-lg transition-all"
                title="Удалить отдел"
              >
                <Icon name="ri:delete-bin-line" class="text-lg" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Positions -->
      <div class="bg-white shadow-sm rounded-xl border border-gray-200 overflow-hidden hover:shadow-md transition-shadow">
        <div class="px-6 py-4 bg-gradient-to-r from-purple-50 to-white border-b border-gray-200 flex items-center justify-between">
          <h3 class="text-lg font-semibold text-gray-900 flex items-center">
            <div class="h-8 w-8 rounded-lg bg-purple-100 flex items-center justify-center mr-3">
              <Icon name="ri:briefcase-line" class="text-purple-600" />
            </div>
            Должности
          </h3>
          <button
            @click="showPositionForm = !showPositionForm"
            class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-lg text-white bg-gradient-to-r from-purple-600 to-purple-700 hover:from-purple-700 hover:to-purple-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-purple-500 shadow-sm transition-all"
          >
            <Icon :name="showPositionForm ? 'ri:close-line' : 'ri:add-line'" class="mr-2" />
            {{ showPositionForm ? 'Отмена' : 'Добавить' }}
          </button>
        </div>

        <!-- Add Position Form -->
        <div v-if="showPositionForm" class="px-6 py-5 bg-gradient-to-r from-purple-50 to-pink-50 border-b border-purple-100">
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
            class="px-6 py-4 hover:bg-gradient-to-r hover:from-purple-50 hover:to-transparent transition-all group"
          >
            <div class="flex items-start justify-between">
              <div class="flex items-start flex-1">
                <div class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center mr-3 group-hover:bg-purple-200 transition-colors">
                  <Icon name="ri:briefcase-line" class="text-purple-600" />
                </div>
                <div class="flex-1">
                  <h4 class="text-sm font-semibold text-gray-900">{{ pos.name }}</h4>
                  <p v-if="pos.department_name" class="mt-1 inline-flex items-center text-xs text-gray-600">
                    <Icon name="ri:building-line" class="mr-1" />
                    {{ pos.department_name }}
                  </p>
                </div>
              </div>
              <button
                @click="openDeletePositionModal(pos)"
                class="opacity-0 group-hover:opacity-100 inline-flex items-center p-2 text-red-600 hover:text-red-900 hover:bg-red-50 rounded-lg transition-all"
                title="Удалить должность"
              >
                <Icon name="ri:delete-bin-line" class="text-lg" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Delete Department Modal -->
    <ConfirmModal
      :show="showDeleteDeptModal"
      :loading="deletingDept"
      title="Удалить отдел?"
      :message="deptToDelete ? `Вы уверены, что хотите удалить отдел ${deptToDelete.name}? Это действие нельзя будет отменить.` : ''"
      confirm-text="Да, удалить"
      cancel-text="Отмена"
      loading-text="Удаление..."
      variant="danger"
      icon="ri:delete-bin-line"
      @confirm="confirmDeleteDept"
      @cancel="closeDeleteDeptModal"
    />

    <!-- Delete Position Modal -->
    <ConfirmModal
      :show="showDeletePosModal"
      :loading="deletingPos"
      title="Удалить должность?"
      :message="posToDelete ? `Вы уверены, что хотите удалить должность ${posToDelete.name}? Это действие нельзя будет отменить.` : ''"
      confirm-text="Да, удалить"
      cancel-text="Отмена"
      loading-text="Удаление..."
      variant="danger"
      icon="ri:delete-bin-line"
      @confirm="confirmDeletePos"
      @cancel="closeDeletePosModal"
    />
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

useHead({
  title: 'Отделы и должности'
})

const { fetchDepartments, fetchPositions, createDepartment, createPosition, deleteDepartment, deletePosition } = useEmployees()

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

// Delete Department
const showDeleteDeptModal = ref(false)
const deptToDelete = ref<any>(null)
const deletingDept = ref(false)

const openDeleteDepartmentModal = (dept: any) => {
  deptToDelete.value = dept
  showDeleteDeptModal.value = true
}

const closeDeleteDeptModal = () => {
  if (!deletingDept.value) {
    showDeleteDeptModal.value = false
    deptToDelete.value = null
  }
}

const confirmDeleteDept = async () => {
  if (!deptToDelete.value) return
  
  deletingDept.value = true
  try {
    await deleteDepartment(deptToDelete.value.id)
    await refreshDepartments()
    await refreshNuxtData('departments')
    showDeleteDeptModal.value = false
    deptToDelete.value = null
  } catch (err) {
    alert('Ошибка при удалении отдела')
  } finally {
    deletingDept.value = false
  }
}

// Delete Position
const showDeletePosModal = ref(false)
const posToDelete = ref<any>(null)
const deletingPos = ref(false)

const openDeletePositionModal = (pos: any) => {
  posToDelete.value = pos
  showDeletePosModal.value = true
}

const closeDeletePosModal = () => {
  if (!deletingPos.value) {
    showDeletePosModal.value = false
    posToDelete.value = null
  }
}

const confirmDeletePos = async () => {
  if (!posToDelete.value) return
  
  deletingPos.value = true
  try {
    await deletePosition(posToDelete.value.id)
    await refreshPositions()
    await refreshNuxtData('positions')
    showDeletePosModal.value = false
    posToDelete.value = null
  } catch (err) {
    alert('Ошибка при удалении должности')
  } finally {
    deletingPos.value = false
  }
}
</script>
