<template>
  <div>
    <div class="mb-6">
      <NuxtLink to="/employees" class="text-sm text-gray-500 hover:text-gray-700 flex items-center mb-2">
        <Icon name="ri:arrow-left-line" class="mr-1" />
        Назад к списку
      </NuxtLink>
    </div>

    <div v-if="pending" class="flex justify-center py-12">
      <Icon name="ri:loader-4-line" class="text-4xl text-primary-600 animate-spin" />
    </div>

    <div v-else-if="error" class="rounded-md bg-red-50 p-4">
      <div class="flex">
        <Icon name="ri:error-warning-line" class="h-5 w-5 text-red-400" />
        <div class="ml-3">
          <p class="text-sm font-medium text-red-800">Ошибка загрузки данных сотрудника</p>
        </div>
      </div>
    </div>

    <div v-else-if="employee" class="space-y-6">
      <!-- Clean Header Card -->
      <div class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
        <div class="p-6">
          <div class="flex items-start justify-between">
            <!-- Avatar and Info -->
            <div class="flex items-start space-x-4">
              <div class="h-20 w-20 rounded-xl bg-gradient-to-br from-primary-500 to-indigo-600 flex items-center justify-center flex-shrink-0 shadow-lg">
                <span class="text-2xl font-bold text-white">
                  {{ employee.first_name.charAt(0) }}{{ employee.last_name.charAt(0) }}
                </span>
              </div>
              <div>
                <h1 class="text-2xl font-bold text-gray-900">{{ employee.full_name }}</h1>
                <div class="mt-2 flex flex-wrap items-center gap-2">
                  <span class="inline-flex items-center px-2.5 py-1 rounded-lg text-xs font-medium bg-primary-50 text-primary-700 border border-primary-200">
                    <Icon name="ri:briefcase-line" class="mr-1.5" />
                    {{ employee.position_name || 'Должность не указана' }}
                  </span>
                  <span v-if="employee.department_name" class="inline-flex items-center px-2.5 py-1 rounded-lg text-xs font-medium bg-gray-50 text-gray-700 border border-gray-200">
                    <Icon name="ri:building-line" class="mr-1.5" />
                    {{ employee.department_name }}
                  </span>
                  <span :class="{
                    'bg-green-50 text-green-700 border-green-200': employee.status === 'active',
                    'bg-gray-50 text-gray-700 border-gray-200': employee.status === 'inactive',
                    'bg-red-50 text-red-700 border-red-200': employee.status === 'terminated'
                  }" class="inline-flex items-center px-2.5 py-1 rounded-lg text-xs font-medium border">
                    <span :class="{
                      'bg-green-500': employee.status === 'active',
                      'bg-gray-500': employee.status === 'inactive',
                      'bg-red-500': employee.status === 'terminated'
                    }" class="h-1.5 w-1.5 rounded-full mr-1.5"></span>
                    {{ statusLabel(employee.status) }}
                  </span>
                </div>
              </div>
            </div>

            <!-- Actions -->
            <div class="flex space-x-2">
              <NuxtLink
                :to="`/employees/${employee.id}/edit`"
                class="inline-flex items-center px-4 py-2 border border-gray-300 rounded-lg text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 transition-all"
              >
                <Icon name="ri:edit-line" class="mr-2" />
                Редактировать
              </NuxtLink>
              <button
                @click="openDeleteModal"
                class="inline-flex items-center px-4 py-2 border border-transparent rounded-lg text-sm font-medium text-white bg-red-600 hover:bg-red-700 transition-all"
              >
                <Icon name="ri:delete-bin-line" class="mr-2" />
                Удалить
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Info Grid -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Personal Info -->
        <div class="bg-white shadow-sm rounded-xl border border-gray-200 overflow-hidden hover:shadow-md transition-shadow">
          <div class="px-6 py-4 bg-gradient-to-r from-blue-50 to-white border-b border-gray-200">
            <h3 class="text-base font-semibold text-gray-900 flex items-center">
              <div class="h-8 w-8 rounded-lg bg-blue-100 flex items-center justify-center mr-3">
                <Icon name="ri:user-line" class="text-blue-600" />
              </div>
              Личная информация
            </h3>
          </div>
          <div class="px-6 py-5 space-y-5">
            <div class="flex items-start">
              <div class="flex-shrink-0">
                <div class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center">
                  <Icon name="ri:user-3-line" class="text-gray-600" />
                </div>
              </div>
              <div class="ml-4 flex-1">
                <dt class="text-xs font-medium text-gray-500 uppercase tracking-wide">ФИО</dt>
                <dd class="mt-1 text-sm font-medium text-gray-900">{{ employee.full_name }}</dd>
              </div>
            </div>
            <div class="flex items-start">
              <div class="flex-shrink-0">
                <div class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center">
                  <Icon name="ri:mail-line" class="text-gray-600" />
                </div>
              </div>
              <div class="ml-4 flex-1">
                <dt class="text-xs font-medium text-gray-500 uppercase tracking-wide">Email</dt>
                <dd class="mt-1 text-sm text-gray-900">
                  <a v-if="employee.email" :href="`mailto:${employee.email}`" class="text-primary-600 hover:text-primary-900 transition-colors">
                    {{ employee.email }}
                  </a>
                  <span v-else class="text-gray-400">Не указан</span>
                </dd>
              </div>
            </div>
            <div class="flex items-start">
              <div class="flex-shrink-0">
                <div class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center">
                  <Icon name="ri:phone-line" class="text-gray-600" />
                </div>
              </div>
              <div class="ml-4 flex-1">
                <dt class="text-xs font-medium text-gray-500 uppercase tracking-wide">Телефон</dt>
                <dd class="mt-1 text-sm text-gray-900">{{ employee.phone || 'Не указан' }}</dd>
              </div>
            </div>
          </div>
        </div>

        <!-- Work Info -->
        <div class="bg-white shadow-sm rounded-xl border border-gray-200 overflow-hidden hover:shadow-md transition-shadow">
          <div class="px-6 py-4 bg-gradient-to-r from-purple-50 to-white border-b border-gray-200">
            <h3 class="text-base font-semibold text-gray-900 flex items-center">
              <div class="h-8 w-8 rounded-lg bg-purple-100 flex items-center justify-center mr-3">
                <Icon name="ri:briefcase-line" class="text-purple-600" />
              </div>
              Рабочая информация
            </h3>
          </div>
          <div class="px-6 py-5 space-y-5">
            <div class="flex items-start">
              <div class="flex-shrink-0">
                <div class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center">
                  <Icon name="ri:building-line" class="text-gray-600" />
                </div>
              </div>
              <div class="ml-4 flex-1">
                <dt class="text-xs font-medium text-gray-500 uppercase tracking-wide">Отдел</dt>
                <dd class="mt-1 text-sm font-medium text-gray-900">{{ employee.department_name || 'Не указан' }}</dd>
              </div>
            </div>
            <div class="flex items-start">
              <div class="flex-shrink-0">
                <div class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center">
                  <Icon name="ri:award-line" class="text-gray-600" />
                </div>
              </div>
              <div class="ml-4 flex-1">
                <dt class="text-xs font-medium text-gray-500 uppercase tracking-wide">Должность</dt>
                <dd class="mt-1 text-sm font-medium text-gray-900">{{ employee.position_name || 'Не указана' }}</dd>
              </div>
            </div>
            <div class="flex items-start">
              <div class="flex-shrink-0">
                <div class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center">
                  <Icon name="ri:checkbox-circle-line" class="text-gray-600" />
                </div>
              </div>
              <div class="ml-4 flex-1">
                <dt class="text-xs font-medium text-gray-500 uppercase tracking-wide">Статус</dt>
                <dd class="mt-1">
                  <span :class="{
                    'bg-green-100 text-green-800 ring-1 ring-green-600/20': employee.status === 'active',
                    'bg-gray-100 text-gray-800 ring-1 ring-gray-600/20': employee.status === 'inactive',
                    'bg-red-100 text-red-800 ring-1 ring-red-600/20': employee.status === 'terminated'
                  }" class="inline-flex items-center px-3 py-1 text-xs font-semibold rounded-full">
                    <span :class="{
                      'bg-green-500': employee.status === 'active',
                      'bg-gray-500': employee.status === 'inactive',
                      'bg-red-500': employee.status === 'terminated'
                    }" class="h-1.5 w-1.5 rounded-full mr-2"></span>
                    {{ statusLabel(employee.status) }}
                  </span>
                </dd>
              </div>
            </div>
          </div>
        </div>

        <!-- Additional Info -->
        <div class="bg-white shadow-sm rounded-xl border border-gray-200 overflow-hidden hover:shadow-md transition-shadow">
          <div class="px-6 py-4 bg-gradient-to-r from-amber-50 to-white border-b border-gray-200">
            <h3 class="text-base font-semibold text-gray-900 flex items-center">
              <div class="h-8 w-8 rounded-lg bg-amber-100 flex items-center justify-center mr-3">
                <Icon name="ri:information-line" class="text-amber-600" />
              </div>
              Дополнительно
            </h3>
          </div>
          <div class="px-6 py-5 space-y-5">
            <!-- AD Username -->
            <div v-if="employee.ad_username" class="flex items-start">
              <div class="flex-shrink-0">
                <div class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center">
                  <Icon name="ri:windows-fill" class="text-gray-600" />
                </div>
              </div>
              <div class="ml-4 flex-1">
                <dt class="text-xs font-medium text-gray-500 uppercase tracking-wide">Active Directory</dt>
                <dd class="mt-1 text-sm font-medium text-gray-900">{{ employee.ad_username }}</dd>
              </div>
            </div>

            <!-- Work Duration -->
            <div v-if="employee.hire_date" class="flex items-start">
              <div class="flex-shrink-0">
                <div class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center">
                  <Icon name="ri:time-line" class="text-gray-600" />
                </div>
              </div>
              <div class="ml-4 flex-1">
                <dt class="text-xs font-medium text-gray-500 uppercase tracking-wide">Стаж работы</dt>
                <dd class="mt-1 text-sm font-medium text-gray-900">{{ calculateWorkDuration(employee.hire_date) }}</dd>
              </div>
            </div>

            <!-- Notes -->
            <div v-if="employee.notes" class="flex items-start">
              <div class="flex-shrink-0">
                <div class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center">
                  <Icon name="ri:sticky-note-line" class="text-gray-600" />
                </div>
              </div>
              <div class="ml-4 flex-1">
                <dt class="text-xs font-medium text-gray-500 uppercase tracking-wide">Заметки</dt>
                <dd class="mt-1 text-sm text-gray-700 whitespace-pre-wrap">{{ employee.notes }}</dd>
              </div>
            </div>

            <!-- Created/Updated -->
            <div class="flex items-start">
              <div class="flex-shrink-0">
                <div class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center">
                  <Icon name="ri:calendar-line" class="text-gray-600" />
                </div>
              </div>
              <div class="ml-4 flex-1">
                <dt class="text-xs font-medium text-gray-500 uppercase tracking-wide">Запись создана</dt>
                <dd class="mt-1 text-sm text-gray-700">{{ formatDate(employee.created_at) }}</dd>
                <dd v-if="employee.updated_at !== employee.created_at" class="mt-1 text-xs text-gray-500">
                  Обновлена: {{ formatDate(employee.updated_at) }}
                </dd>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Computers Section (Placeholder) -->
      <div class="bg-white shadow-sm rounded-xl border border-gray-200 overflow-hidden">
        <div class="px-6 py-4 bg-gradient-to-r from-purple-50 to-white border-b border-gray-200">
          <div class="flex items-center justify-between">
            <div class="flex items-center">
              <Icon name="ri:computer-line" class="text-purple-500 text-xl mr-2" />
              <h3 class="text-base font-semibold text-gray-900">Компьютеры</h3>
            </div>
            <span class="text-xs text-gray-500">Скоро</span>
          </div>
        </div>
        <div class="p-6 text-center">
          <div class="h-20 w-20 rounded-full bg-purple-100 flex items-center justify-center mx-auto mb-3">
            <Icon name="ri:computer-line" class="text-3xl text-purple-500" />
          </div>
          <p class="text-sm text-gray-600">Здесь будут отображаться компьютеры, закрепленные за сотрудником</p>
        </div>
      </div>

      <!-- History Section -->
      <div class="bg-white shadow-sm rounded-xl border border-gray-200 overflow-hidden">
        <div class="px-6 py-4 bg-gradient-to-r from-indigo-50 to-white border-b border-gray-200">
          <div class="flex items-center">
            <Icon name="ri:history-line" class="text-indigo-500 text-xl mr-2" />
            <h3 class="text-base font-semibold text-gray-900">История изменений</h3>
          </div>
        </div>
        <div class="p-6">
          <div v-if="historyPending" class="flex justify-center py-8">
            <Icon name="ri:loader-4-line" class="text-2xl text-indigo-600 animate-spin" />
          </div>
          <div v-else-if="history && history.length > 0" class="space-y-4">
            <div
              v-for="log in history"
              :key="log.id"
              class="flex gap-4 pb-4 border-b border-gray-100 last:border-0"
            >
              <div class="flex-shrink-0">
                <div :class="{
                  'bg-green-100 text-green-600': log.action === 'created',
                  'bg-blue-100 text-blue-600': log.action === 'updated',
                  'bg-red-100 text-red-600': log.action === 'deleted'
                }" class="h-10 w-10 rounded-full flex items-center justify-center">
                  <Icon :name="{
                    'created': 'ri:add-line',
                    'updated': 'ri:edit-line',
                    'deleted': 'ri:delete-bin-line'
                  }[log.action] || 'ri:information-line'" class="text-lg" />
                </div>
              </div>
              <div class="flex-1 min-w-0">
                <div class="flex items-center justify-between mb-1">
                  <p class="text-sm font-medium text-gray-900">
                    {{ actionLabel(log.action) }}
                  </p>
                  <time class="text-xs text-gray-500">{{ formatDateTime(log.created_at) }}</time>
                </div>
                <p class="text-xs text-gray-600 mb-2">
                  {{ log.user_email || 'Система' }}
                </p>
                <div v-if="log.details?.changes" class="space-y-1">
                  <div
                    v-for="(change, field) in log.details.changes"
                    :key="String(field)"
                    class="text-xs bg-gray-50 rounded px-2 py-1"
                  >
                    <span class="font-medium text-gray-700">{{ fieldLabel(String(field)) }}:</span>
                    <span class="text-red-600 line-through mx-1">{{ formatValue(change.old) }}</span>
                    →
                    <span class="text-green-600 font-medium mx-1">{{ formatValue(change.new) }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div v-else class="text-center py-8 text-gray-500">
            <Icon name="ri:history-line" class="text-4xl text-gray-300 mb-2 mx-auto" />
            <p class="text-sm">История изменений пуста</p>
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
      @cancel="closeDeleteModal"
    />
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

const route = useRoute()
const router = useRouter()
const employeeId = parseInt(route.params.id as string)

const { fetchEmployee, deleteEmployee } = useEmployees()
const { fetchEntityHistory } = useActivityLog()

const { data: employee, pending, error } = await useAsyncData(
  `employee-${employeeId}`,
  () => fetchEmployee(employeeId)
)

const { data: historyData, pending: historyPending } = await useAsyncData(
  `employee-history-${employeeId}`,
  () => fetchEntityHistory('employee', employeeId)
)

const history = computed(() => historyData.value?.logs || [])

useHead({
  title: computed(() => employee.value ? employee.value.full_name : 'Сотрудник')
})

const statusLabel = (status: string) => {
  const labels: Record<string, string> = {
    active: 'Активен',
    inactive: 'Неактивен',
    terminated: 'Уволен'
  }
  return labels[status] || status
}

const actionLabel = (action: string) => {
  const labels: Record<string, string> = {
    created: 'Создан',
    updated: 'Обновлен',
    deleted: 'Удален'
  }
  return labels[action] || action
}

const fieldLabel = (field: string) => {
  const labels: Record<string, string> = {
    first_name: 'Имя',
    last_name: 'Фамилия',
    middle_name: 'Отчество',
    email: 'Email',
    phone: 'Телефон',
    status: 'Статус',
    position_id: 'Должность',
    department_id: 'Отдел'
  }
  return labels[field] || field
}

const formatValue = (value: any) => {
  if (value === null || value === undefined) return '-'
  return String(value)
}

const formatDateTime = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleString('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric'
  })
}

const calculateWorkDuration = (hireDateStr: string) => {
  const hireDate = new Date(hireDateStr)
  const now = new Date()
  const diffTime = Math.abs(now.getTime() - hireDate.getTime())
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24))
  
  const years = Math.floor(diffDays / 365)
  const months = Math.floor((diffDays % 365) / 30)
  
  if (years > 0) {
    return `${years} ${years === 1 ? 'год' : years < 5 ? 'года' : 'лет'} ${months > 0 ? `${months} мес.` : ''}`
  } else if (months > 0) {
    return `${months} ${months === 1 ? 'месяц' : months < 5 ? 'месяца' : 'месяцев'}`
  } else {
    return `${diffDays} ${diffDays === 1 ? 'день' : diffDays < 5 ? 'дня' : 'дней'}`
  }
}

const showDeleteModal = ref(false)
const deleting = ref(false)

const openDeleteModal = () => {
  showDeleteModal.value = true
}

const closeDeleteModal = () => {
  if (!deleting.value) {
    showDeleteModal.value = false
  }
}

const confirmDelete = async () => {
  if (!employee.value) return
  
  deleting.value = true
  try {
    await deleteEmployee(employee.value.id)
    
    await refreshNuxtData('employees')
    
    await navigateTo('/employees')
  } catch (err) {
    alert('Ошибка при удалении сотрудника')
    deleting.value = false
    showDeleteModal.value = false
  }
}
</script>
