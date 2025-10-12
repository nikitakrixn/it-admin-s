<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="show" class="fixed inset-0 z-[60] overflow-y-auto">
        <div class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm" @click="onClose" />
        
        <div class="relative min-h-screen flex items-center justify-center p-4">
          <div class="relative bg-white rounded-2xl shadow-2xl max-w-3xl w-full">
            <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                  <div
                    v-if="log"
                    :class="[
                      'h-10 w-10 rounded-lg flex items-center justify-center',
                      getActionColor(log.action)
                    ]"
                  >
                    <Icon :name="getActionIcon(log.action)" class="text-xl text-white" />
                  </div>
                  <h2 class="text-xl font-semibold text-gray-900">Детали записи</h2>
                </div>
                <button @click="onClose" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
                  <Icon name="ri:close-line" class="text-xl" />
                </button>
              </div>
            </div>

            <div v-if="log" class="p-6 max-h-[70vh] overflow-y-auto scrollbar-thin">
              <div class="space-y-6">
                <div class="grid grid-cols-2 gap-4">
                  <div class="p-4 rounded-lg bg-gray-50 border border-gray-200">
                    <p class="text-xs font-medium text-gray-500 mb-1">ID записи</p>
                    <p class="text-lg font-bold text-gray-900">#{{ log.id }}</p>
                  </div>
                  <div class="p-4 rounded-lg bg-gray-50 border border-gray-200">
                    <p class="text-xs font-medium text-gray-500 mb-1">Дата и время</p>
                    <p class="text-sm font-semibold text-gray-900">{{ formatDateTime(log.created_at) }}</p>
                  </div>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div class="p-4 rounded-lg bg-blue-50 border border-blue-100">
                    <div class="flex items-center gap-2 mb-2">
                      <Icon name="ri:user-line" class="text-blue-600" />
                      <p class="text-xs font-medium text-gray-600">Пользователь</p>
                    </div>
                    <p class="text-sm font-medium text-gray-900">{{ log.user_email || 'Система' }}</p>
                  </div>

                  <div class="p-4 rounded-lg bg-purple-50 border border-purple-100">
                    <div class="flex items-center gap-2 mb-2">
                      <Icon name="ri:file-line" class="text-purple-600" />
                      <p class="text-xs font-medium text-gray-600">Действие</p>
                    </div>
                    <p class="text-sm font-medium text-gray-900">{{ actionLabel(log.action) }}</p>
                  </div>

                  <div class="p-4 rounded-lg bg-green-50 border border-green-100">
                    <div class="flex items-center gap-2 mb-2">
                      <Icon name="ri:folder-line" class="text-green-600" />
                      <p class="text-xs font-medium text-gray-600">Тип объекта</p>
                    </div>
                    <p class="text-sm font-medium text-gray-900">{{ entityTypeLabel(log.entity_type) }}</p>
                  </div>

                  <div class="p-4 rounded-lg bg-amber-50 border border-amber-100">
                    <div class="flex items-center gap-2 mb-2">
                      <Icon name="ri:hashtag" class="text-amber-600" />
                      <p class="text-xs font-medium text-gray-600">ID объекта</p>
                    </div>
                    <p class="text-sm font-medium text-gray-900">#{{ log.entity_id }}</p>
                  </div>
                </div>

                <div v-if="log.details?.changes && typeof log.details.changes === 'object'" class="space-y-3">
                  <div class="flex items-center gap-2">
                    <Icon name="ri:git-commit-line" class="text-primary-600" />
                    <h3 class="text-sm font-semibold text-gray-900">Изменения</h3>
                  </div>
                  <div class="space-y-2">
                    <div
                      v-for="(change, field) in log.details.changes"
                      :key="String(field)"
                      class="p-4 rounded-lg bg-white border border-gray-200"
                    >
                      <p class="text-xs font-medium text-gray-600 mb-2">{{ fieldLabel(String(field)) }}</p>
                      <div class="flex items-center gap-3">
                        <div class="flex-1 p-2 rounded bg-red-50 border border-red-200">
                          <p class="text-xs text-gray-500 mb-1">Было</p>
                          <p class="text-sm text-red-700 font-medium">{{ formatValue(change.old) }}</p>
                        </div>
                        <Icon name="ri:arrow-right-line" class="text-gray-400 flex-shrink-0" />
                        <div class="flex-1 p-2 rounded bg-green-50 border border-green-200">
                          <p class="text-xs text-gray-500 mb-1">Стало</p>
                          <p class="text-sm text-green-700 font-medium">{{ formatValue(change.new) }}</p>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>

                <div v-if="(log.action === 'deleted' || log.action === 'created') && log.details && !log.details.changes">
                  <div class="flex items-center gap-2 mb-3">
                    <Icon :name="log.action === 'deleted' ? 'ri:delete-bin-line' : 'ri:add-circle-line'" :class="log.action === 'deleted' ? 'text-red-600' : 'text-green-600'" />
                    <h3 class="text-sm font-semibold text-gray-900">
                      {{ log.action === 'deleted' ? 'Удаленные данные' : 'Созданные данные' }}
                    </h3>
                  </div>
                  <div :class="['p-4 rounded-lg border', log.action === 'deleted' ? 'bg-red-50 border-red-200' : 'bg-green-50 border-green-200']">
                    <div class="space-y-2">
                      <div v-for="(value, key) in log.details" :key="String(key)" class="flex justify-between items-center py-2 border-b border-gray-200 last:border-0">
                        <span class="text-sm font-medium text-gray-700">{{ fieldLabel(String(key)) }}</span>
                        <span class="text-sm text-gray-900">{{ formatValue(value) }}</span>
                      </div>
                    </div>
                  </div>
                </div>

                <div v-if="log.ip_address || log.user_agent" class="space-y-3">
                  <div class="flex items-center gap-2">
                    <Icon name="ri:information-line" class="text-gray-600" />
                    <h3 class="text-sm font-semibold text-gray-900">Техническая информация</h3>
                  </div>
                  
                  <div v-if="log.ip_address" class="p-3 rounded-lg bg-gray-50 border border-gray-200">
                    <div class="flex items-center gap-2">
                      <Icon name="ri:global-line" class="text-gray-500" />
                      <span class="text-xs font-medium text-gray-600">IP адрес:</span>
                      <span class="text-sm text-gray-900 font-mono">{{ log.ip_address }}</span>
                    </div>
                  </div>

                  <div v-if="log.user_agent" class="p-3 rounded-lg bg-gray-50 border border-gray-200">
                    <div class="flex items-start gap-2">
                      <Icon name="ri:computer-line" class="text-gray-500 mt-0.5" />
                      <div class="flex-1">
                        <span class="text-xs font-medium text-gray-600 block mb-1">User Agent:</span>
                        <span class="text-xs text-gray-700 break-all">{{ log.user_agent }}</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div class="px-6 py-4 bg-gray-50 border-t border-gray-200 flex justify-end">
              <button @click="onClose" class="btn btn-secondary">
                Закрыть
              </button>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import type { ActivityLog } from '~/composables/useActivityLog'

interface Props {
  show: boolean
  log: ActivityLog | null
}

const props = defineProps<Props>()
const emit = defineEmits<{ close: [] }>()

const onClose = () => {
  emit('close')
}

const formatDateTime = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleString('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  })
}

const actionLabel = (action: string) => {
  const labels: Record<string, string> = {
    create: 'Создание',
    created: 'Создание',
    update: 'Обновление',
    updated: 'Обновление',
    delete: 'Удаление',
    deleted: 'Удаление',
    login: 'Вход',
    logout: 'Выход'
  }
  return labels[action.toLowerCase()] || action
}

const entityTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    employee: 'Сотрудник',
    employees: 'Сотрудник',
    computer: 'Компьютер',
    computers: 'Компьютер',
    equipment: 'Оборудование',
    request: 'Заявка',
    requests: 'Заявка',
    user: 'Пользователь',
    department: 'Отдел',
    departments: 'Отдел',
    position: 'Должность'
  }
  return labels[type.toLowerCase()] || type
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
    department_id: 'Отдел',
    hire_date: 'Дата приема',
    termination_date: 'Дата увольнения',
    employee_name: 'ФИО',
    position: 'Должность',
    department: 'Отдел',
    department_name: 'Название отдела',
    position_name: 'Название должности',
    description: 'Описание',
    name: 'Название',
    ad_username: 'AD Username',
    notes: 'Примечания'
  }
  return labels[field] || field
}

const formatValue = (value: any) => {
  if (value === null || value === undefined) return '-'
  if (typeof value === 'boolean') return value ? 'Да' : 'Нет'
  if (value === '') return '(пусто)'
  return String(value)
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
