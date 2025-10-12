<template>
  <div class="space-y-6">
    <div class="flex items-center gap-4 mb-8">
      <NuxtLink to="/employees" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
        <Icon name="ri:arrow-left-line" class="text-xl" />
      </NuxtLink>
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Профиль сотрудника</h1>
        <p class="text-sm text-gray-600">Детальная информация</p>
      </div>
    </div>

    <div v-if="pending" class="flex justify-center py-12">
      <Icon name="ri:loader-4-line" class="text-4xl text-primary-600 animate-spin" />
    </div>

    <div v-else-if="error" class="card">
      <div class="p-8 text-center">
        <Icon name="ri:error-warning-line" class="text-5xl text-red-500 mx-auto mb-4" />
        <h3 class="text-lg font-semibold text-gray-900 mb-2">Ошибка загрузки</h3>
        <p class="text-sm text-gray-600">Не удалось загрузить данные сотрудника</p>
      </div>
    </div>

    <div v-else-if="employee" class="space-y-6">
      <div class="card">
        <div class="p-6">
          <div class="flex items-start justify-between">
            <div class="flex items-start gap-4">
              <div class="h-24 w-24 rounded-2xl gradient-primary flex items-center justify-center flex-shrink-0 shadow-lg">
                <span class="text-3xl font-bold text-white">
                  {{ employee.first_name.charAt(0) }}{{ employee.last_name.charAt(0) }}
                </span>
              </div>
              <div>
                <h2 class="text-2xl font-bold text-gray-900 mb-2">{{ employee.full_name }}</h2>
                <div class="flex flex-wrap items-center gap-2">
                  <span class="badge bg-primary-100 text-primary-800 flex items-center gap-1">
                    <Icon name="ri:briefcase-line" />
                    {{ employee.position_name || 'Должность не указана' }}
                  </span>
                  <span v-if="employee.department_name" class="badge bg-purple-100 text-purple-800 flex items-center gap-1">
                    <Icon name="ri:building-line" />
                    {{ employee.department_name }}
                  </span>
                  <span
                    :class="{
                      'bg-green-100 text-green-800': employee.status === 'active',
                      'bg-gray-100 text-gray-800': employee.status === 'inactive',
                      'bg-red-100 text-red-800': employee.status === 'terminated'
                    }"
                    class="badge flex items-center gap-1"
                  >
                    <span
                      :class="{
                        'bg-green-500': employee.status === 'active',
                        'bg-gray-500': employee.status === 'inactive',
                        'bg-red-500': employee.status === 'terminated'
                      }"
                      class="h-2 w-2 rounded-full animate-pulse"
                    />
                    {{ statusLabel(employee.status) }}
                  </span>
                </div>
              </div>
            </div>

            <div class="flex gap-2">
              <NuxtLink :to="`/employees/${employee.id}/edit`" class="btn btn-secondary">
                <Icon name="ri:edit-line" class="mr-2" />
                Редактировать
              </NuxtLink>
              <button @click="openDeleteModal" class="btn bg-red-50 text-red-700 hover:bg-red-100 border border-red-200">
                <Icon name="ri:delete-bin-line" class="mr-2" />
                Удалить
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div class="card">
          <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center">
                <Icon name="ri:user-line" class="text-blue-600 text-xl" />
              </div>
              <h3 class="text-lg font-semibold text-gray-900">Личная информация</h3>
            </div>
          </div>
          <div class="p-6 space-y-4">
            <div>
              <p class="text-xs font-medium text-gray-500 mb-1">Фамилия</p>
              <p class="text-sm text-gray-900 font-medium">{{ employee.last_name }}</p>
            </div>
            <div>
              <p class="text-xs font-medium text-gray-500 mb-1">Имя</p>
              <p class="text-sm text-gray-900 font-medium">{{ employee.first_name }}</p>
            </div>
            <div v-if="employee.middle_name">
              <p class="text-xs font-medium text-gray-500 mb-1">Отчество</p>
              <p class="text-sm text-gray-900 font-medium">{{ employee.middle_name }}</p>
            </div>
            <div v-if="employee.ad_username" class="pt-4 border-t border-gray-100">
              <p class="text-xs font-medium text-gray-500 mb-1">AD Username</p>
              <p class="text-sm text-gray-900 font-medium flex items-center gap-2">
                <Icon name="ri:windows-fill" class="text-blue-600" />
                {{ employee.ad_username }}
              </p>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-green-100 flex items-center justify-center">
                <Icon name="ri:contacts-line" class="text-green-600 text-xl" />
              </div>
              <h3 class="text-lg font-semibold text-gray-900">Контакты</h3>
            </div>
          </div>
          <div class="p-6 space-y-4">
            <div v-if="employee.email">
              <p class="text-xs font-medium text-gray-500 mb-1">Email</p>
              <a :href="`mailto:${employee.email}`" class="text-sm text-primary-600 hover:text-primary-700 font-medium flex items-center gap-2">
                <Icon name="ri:mail-line" />
                {{ employee.email }}
              </a>
            </div>
            <div v-else class="text-sm text-gray-400">Email не указан</div>

            <div v-if="employee.phone" class="pt-4 border-t border-gray-100">
              <p class="text-xs font-medium text-gray-500 mb-1">Телефон</p>
              <a :href="`tel:${employee.phone}`" class="text-sm text-gray-900 font-medium flex items-center gap-2">
                <Icon name="ri:phone-line" class="text-green-600" />
                {{ employee.phone }}
              </a>
            </div>
            <div v-else class="pt-4 border-t border-gray-100 text-sm text-gray-400">Телефон не указан</div>
          </div>
        </div>

        <div class="card">
          <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center">
                <Icon name="ri:briefcase-line" class="text-purple-600 text-xl" />
              </div>
              <h3 class="text-lg font-semibold text-gray-900">Работа</h3>
            </div>
          </div>
          <div class="p-6 space-y-4">
            <div v-if="employee.hire_date">
              <p class="text-xs font-medium text-gray-500 mb-1">Дата приема</p>
              <p class="text-sm text-gray-900 font-medium flex items-center gap-2">
                <Icon name="ri:calendar-line" class="text-purple-600" />
                {{ formatDate(employee.hire_date) }}
              </p>
            </div>
            <div v-if="employee.termination_date" class="pt-4 border-t border-gray-100">
              <p class="text-xs font-medium text-gray-500 mb-1">Дата увольнения</p>
              <p class="text-sm text-gray-900 font-medium flex items-center gap-2">
                <Icon name="ri:calendar-close-line" class="text-red-600" />
                {{ formatDate(employee.termination_date) }}
              </p>
            </div>
            <div v-if="!employee.hire_date && !employee.termination_date" class="text-sm text-gray-400">
              Даты не указаны
            </div>
          </div>
        </div>
      </div>

      <div v-if="employee.notes" class="card">
        <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center gap-3">
            <div class="h-10 w-10 rounded-lg bg-amber-100 flex items-center justify-center">
              <Icon name="ri:file-text-line" class="text-amber-600 text-xl" />
            </div>
            <h3 class="text-lg font-semibold text-gray-900">Примечания</h3>
          </div>
        </div>
        <div class="p-6">
          <p class="text-sm text-gray-700 whitespace-pre-wrap">{{ employee.notes }}</p>
        </div>
      </div>

      <div class="card">
        <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-indigo-100 flex items-center justify-center">
                <Icon name="ri:history-line" class="text-indigo-600 text-xl" />
              </div>
              <h3 class="text-lg font-semibold text-gray-900">История изменений</h3>
            </div>
            <span class="badge bg-gray-100 text-gray-800">{{ activityLogs.length }}</span>
          </div>
        </div>
        <div v-if="loadingLogs" class="p-8 text-center">
          <Icon name="ri:loader-4-line" class="text-3xl text-primary-600 animate-spin mx-auto" />
        </div>
        <div v-else-if="activityLogs.length === 0" class="p-8 text-center">
          <Icon name="ri:history-line" class="text-4xl text-gray-300 mx-auto mb-2" />
          <p class="text-sm text-gray-500">История изменений пуста</p>
        </div>
        <div v-else class="divide-y divide-gray-200">
          <div
            v-for="log in activityLogs"
            :key="log.id"
            class="p-4 hover:bg-gray-50 transition-colors cursor-pointer"
            @click="openLogDetail(log)"
          >
            <div class="flex items-start gap-3">
              <div
                :class="[
                  'flex h-10 w-10 items-center justify-center rounded-lg flex-shrink-0',
                  getActionColor(log.action)
                ]"
              >
                <Icon :name="getActionIcon(log.action)" class="text-lg text-white" />
              </div>
              <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-gray-900">{{ getActionDescription(log) }}</p>
                <div class="mt-1 flex items-center gap-3 text-xs text-gray-500">
                  <span class="flex items-center gap-1">
                    <Icon name="ri:user-line" />
                    {{ log.user_email || 'Система' }}
                  </span>
                  <span class="flex items-center gap-1">
                    <Icon name="ri:time-line" />
                    {{ formatDateTime(log.created_at) }}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <ConfirmModal
      :show="showDeleteModal"
      :loading="deleting"
      title="Удалить сотрудника?"
      :message="employee ? `Вы уверены, что хотите удалить сотрудника ${employee.full_name}? Это действие нельзя будет отменить.` : ''"
      confirm-text="Да, удалить"
      cancel-text="Отмена"
      loading-text="Удаление..."
      variant="danger"
      icon="ri:delete-bin-line"
      @confirm="confirmDelete"
      @cancel="showDeleteModal = false"
    />

    <ActivityLogDetailModal
      :show="showLogModal"
      :log="selectedLog"
      @close="showLogModal = false"
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

const route = useRoute()
const router = useRouter()
const toast = useToast()
const { fetchEmployee, deleteEmployee } = useEmployees()
const { fetchEntityHistory } = useActivityLog()

const employeeId = computed(() => Number(route.params.id))

const { data: employee, pending, error } = await useAsyncData(
  `employee-${employeeId.value}`,
  () => fetchEmployee(employeeId.value)
)

useHead({
  title: employee.value ? `${employee.value.full_name} - Сотрудник` : 'Сотрудник'
})

const showDeleteModal = ref(false)
const deleting = ref(false)
const loadingLogs = ref(false)
const activityLogs = ref<any[]>([])
const showLogModal = ref(false)
const selectedLog = ref<any>(null)

const statusLabel = (status: string) => {
  const labels: Record<string, string> = {
    active: 'Активен',
    inactive: 'Неактивен',
    vacation: 'В отпуске',
    sick_leave: 'На больничном',
    terminated: 'Уволен'
  }
  return labels[status] || status
}

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric'
  })
}

const formatDateTime = (date: string) => {
  return new Date(date).toLocaleString('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const openDeleteModal = () => {
  showDeleteModal.value = true
}

const confirmDelete = async () => {
  if (!employee.value) return
  
  deleting.value = true
  try {
    await deleteEmployee(employee.value.id)
    toast.success('Сотрудник успешно удален')
    router.push('/employees')
  } catch (err) {
    toast.error('Ошибка при удалении сотрудника')
  } finally {
    deleting.value = false
  }
}

const loadActivityLogs = async () => {
  if (!employee.value) return
  
  loadingLogs.value = true
  try {
    const result = await fetchEntityHistory('employee', employee.value.id)
    activityLogs.value = result.logs
  } catch (err) {
    console.error('Failed to load activity logs:', err)
  } finally {
    loadingLogs.value = false
  }
}

const openLogDetail = (log: any) => {
  selectedLog.value = log
  showLogModal.value = true
}

const getActionIcon = (action: string) => {
  const icons: Record<string, string> = {
    create: 'ri:add-circle-line',
    created: 'ri:add-circle-line',
    update: 'ri:edit-line',
    updated: 'ri:edit-line',
    delete: 'ri:delete-bin-line',
    deleted: 'ri:delete-bin-line'
  }
  return icons[action.toLowerCase()] || 'ri:file-line'
}

const getActionColor = (action: string) => {
  const colors: Record<string, string> = {
    create: 'bg-green-500',
    created: 'bg-green-500',
    update: 'bg-blue-500',
    updated: 'bg-blue-500',
    delete: 'bg-red-500',
    deleted: 'bg-red-500'
  }
  return colors[action.toLowerCase()] || 'bg-gray-500'
}

const getActionDescription = (log: any) => {
  const actionLabels: Record<string, string> = {
    create: 'Создан',
    created: 'Создан',
    update: 'Изменен',
    updated: 'Изменен',
    delete: 'Удален',
    deleted: 'Удален'
  }
  
  const action = actionLabels[log.action.toLowerCase()] || log.action
  return `${action} профиль сотрудника`
}

onMounted(() => {
  loadActivityLogs()
})
</script>
