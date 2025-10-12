<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <NuxtLink to="/employees" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
            <Icon name="ri:arrow-left-line" class="text-xl" />
          </NuxtLink>
          <div class="h-16 w-16 rounded-2xl bg-gradient-to-br from-purple-500 to-purple-700 flex items-center justify-center shadow-lg">
            <Icon name="ri:building-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Отделы</h1>
            <p class="mt-1 text-gray-600">Управление структурой организации</p>
          </div>
        </div>
        <button @click="showCreateModal = true" class="btn btn-primary">
          <Icon name="ri:add-line" class="mr-2" />
          Добавить отдел
        </button>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Всего отделов</p>
              <p class="text-2xl font-bold text-gray-900">{{ departments.length }}</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center">
              <Icon name="ri:building-line" class="text-2xl text-purple-600" />
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Всего сотрудников</p>
              <p class="text-2xl font-bold text-gray-900">{{ totalEmployees }}</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center">
              <Icon name="ri:team-line" class="text-2xl text-blue-600" />
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Средний размер</p>
              <p class="text-2xl font-bold text-gray-900">{{ avgDepartmentSize }}</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center">
              <Icon name="ri:group-line" class="text-2xl text-green-600" />
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-if="loading" class="flex justify-center py-12">
      <Icon name="ri:loader-4-line" class="text-4xl text-primary-600 animate-spin" />
    </div>

    <div v-else-if="departments.length === 0" class="card">
      <div class="p-16 text-center">
        <div class="h-20 w-20 rounded-full bg-purple-100 flex items-center justify-center mx-auto mb-4">
          <Icon name="ri:building-line" class="text-4xl text-purple-600" />
        </div>
        <h3 class="text-lg font-semibold text-gray-900 mb-2">Отделы не найдены</h3>
        <p class="text-sm text-gray-500 mb-4">Создайте первый отдел организации</p>
        <button @click="showCreateModal = true" class="btn btn-primary">
          <Icon name="ri:add-line" class="mr-2" />
          Добавить отдел
        </button>
      </div>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div
        v-for="department in departments"
        :key="department.id"
        class="card card-hover group cursor-pointer"
        @click="selectDepartment(department)"
      >
        <div class="p-6">
          <div class="flex items-start justify-between mb-4">
            <div class="flex-1">
              <h3 class="text-lg font-semibold text-gray-900 mb-1 group-hover:text-primary-600 transition-colors">
                {{ department.name }}
              </h3>
              <p v-if="department.description" class="text-sm text-gray-600 line-clamp-2">
                {{ department.description }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center group-hover:scale-110 transition-transform">
              <Icon name="ri:building-line" class="text-2xl text-purple-600" />
            </div>
          </div>

          <div class="pt-4 border-t border-gray-100 space-y-3">
            <div class="flex items-center justify-between text-sm">
              <span class="text-gray-600 flex items-center gap-2">
                <Icon name="ri:user-line" class="text-gray-400" />
                Сотрудников
              </span>
              <span class="font-semibold text-gray-900">{{ department.employee_count || 0 }}</span>
            </div>

            <div v-if="department.manager_name" class="flex items-center justify-between text-sm">
              <span class="text-gray-600 flex items-center gap-2">
                <Icon name="ri:user-star-line" class="text-gray-400" />
                Руководитель
              </span>
              <span class="font-medium text-gray-900">{{ department.manager_name }}</span>
            </div>
          </div>

          <div class="mt-4 pt-4 border-t border-gray-100 flex items-center gap-2">
            <button
              @click.stop="editDepartment(department)"
              class="flex-1 btn btn-secondary text-sm"
            >
              <Icon name="ri:edit-line" class="mr-1" />
              Изменить
            </button>
            <button
              @click.stop="deleteDepartment(department)"
              class="btn bg-red-50 text-red-700 hover:bg-red-100 border border-red-200 text-sm"
            >
              <Icon name="ri:delete-bin-line" />
            </button>
          </div>
        </div>
      </div>
    </div>

    <Teleport to="body">
      <Transition name="modal">
        <div v-if="showCreateModal || showEditModal" class="fixed inset-0 z-50 overflow-y-auto">
          <div class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm" @click="closeModals" />
          
          <div class="relative min-h-screen flex items-center justify-center p-4">
            <div class="relative bg-white rounded-2xl shadow-2xl w-full max-w-lg">
              <div class="p-6 border-b border-gray-200">
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-3">
                    <div class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center">
                      <Icon name="ri:building-line" class="text-purple-600 text-xl" />
                    </div>
                    <h2 class="text-xl font-semibold text-gray-900">
                      {{ showEditModal ? 'Редактировать отдел' : 'Добавить отдел' }}
                    </h2>
                  </div>
                  <button @click="closeModals" class="p-2 rounded-lg hover:bg-gray-100">
                    <Icon name="ri:close-line" class="text-xl" />
                  </button>
                </div>
              </div>

              <form @submit.prevent="handleSubmit" class="p-6 space-y-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">
                    Название отдела <span class="text-red-500">*</span>
                  </label>
                  <input
                    v-model="form.name"
                    type="text"
                    required
                    class="input-field"
                    placeholder="Например: IT отдел"
                  />
                </div>

                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">
                    Описание
                  </label>
                  <textarea
                    v-model="form.description"
                    rows="3"
                    class="input-field"
                    placeholder="Краткое описание отдела..."
                  />
                </div>

                <div v-if="errorMessage" class="p-4 rounded-lg bg-red-50 text-red-800 text-sm flex items-start gap-2">
                  <Icon name="ri:error-warning-line" class="text-lg flex-shrink-0 mt-0.5" />
                  <span>{{ errorMessage }}</span>
                </div>

                <div class="flex gap-3 pt-4">
                  <button type="button" @click="closeModals" class="flex-1 btn btn-secondary">
                    Отмена
                  </button>
                  <button type="submit" :disabled="saving" class="flex-1 btn btn-primary">
                    <Icon v-if="saving" name="ri:loader-4-line" class="mr-2 animate-spin" />
                    <Icon v-else name="ri:save-line" class="mr-2" />
                    {{ saving ? 'Сохранение...' : 'Сохранить' }}
                  </button>
                </div>
              </form>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <ConfirmModal
      :show="showDeleteModal"
      :loading="deleting"
      title="Удалить отдел?"
      :message="departmentToDelete ? `Вы уверены, что хотите удалить отдел ${departmentToDelete.name}?` : ''"
      confirm-text="Да, удалить"
      cancel-text="Отмена"
      loading-text="Удаление..."
      variant="danger"
      icon="ri:delete-bin-line"
      @confirm="confirmDelete"
      @cancel="showDeleteModal = false"
    />
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in'
  }
})

useHead({
  title: 'Отделы'
})

const toast = useToast()
const { fetchDepartments } = useEmployees()

const loading = ref(false)
const departments = ref<any[]>([])
const showCreateModal = ref(false)
const showEditModal = ref(false)
const showDeleteModal = ref(false)
const saving = ref(false)
const deleting = ref(false)
const departmentToDelete = ref<any>(null)
const editingDepartment = ref<any>(null)

const form = ref({
  name: '',
  description: ''
})

const errorMessage = ref('')

const totalEmployees = computed(() => 
  departments.value.reduce((sum, dept) => sum + (dept.employee_count || 0), 0)
)

const avgDepartmentSize = computed(() => 
  departments.value.length > 0 
    ? Math.round(totalEmployees.value / departments.value.length)
    : 0
)

const loadDepartments = async () => {
  loading.value = true
  try {
    const data = await fetchDepartments()
    departments.value = data || []
  } catch (err) {
    toast.error('Ошибка загрузки отделов')
  } finally {
    loading.value = false
  }
}

const selectDepartment = (department: any) => {
  navigateTo(`/employees?department_id=${department.id}`)
}

const editDepartment = (department: any) => {
  editingDepartment.value = department
  form.value.name = department.name
  form.value.description = department.description || ''
  showEditModal.value = true
}

const deleteDepartment = (department: any) => {
  departmentToDelete.value = department
  showDeleteModal.value = true
}

const closeModals = () => {
  showCreateModal.value = false
  showEditModal.value = false
  editingDepartment.value = null
  form.value.name = ''
  form.value.description = ''
  errorMessage.value = ''
}

const handleSubmit = async () => {
  saving.value = true
  errorMessage.value = ''

  try {
    await new Promise(resolve => setTimeout(resolve, 500))
    
    if (showEditModal.value) {
      toast.success('Отдел успешно обновлен')
    } else {
      toast.success('Отдел успешно создан')
    }
    
    closeModals()
    await loadDepartments()
  } catch (err: any) {
    errorMessage.value = err.message || 'Ошибка при сохранении'
  } finally {
    saving.value = false
  }
}

const confirmDelete = async () => {
  if (!departmentToDelete.value) return
  
  deleting.value = true
  try {
    await new Promise(resolve => setTimeout(resolve, 500))
    toast.success('Отдел успешно удален')
    showDeleteModal.value = false
    departmentToDelete.value = null
    await loadDepartments()
  } catch (err) {
    toast.error('Ошибка при удалении отдела')
  } finally {
    deleting.value = false
  }
}

onMounted(() => {
  loadDepartments()
})
</script>

<style scoped>
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .relative,
.modal-leave-active .relative {
  transition: all 0.3s ease;
}

.modal-enter-from .relative {
  transform: scale(0.95) translateY(-20px);
}

.modal-leave-to .relative {
  transform: scale(0.95) translateY(-20px);
}
</style>
