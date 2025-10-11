<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition ease-out duration-200"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition ease-in duration-150"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="show"
        class="fixed inset-0 z-[60] overflow-y-auto"
        @click.self="onClose"
      >
        <div class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm"></div>
        <div class="flex min-h-full items-center justify-center p-4">
          <Transition
            enter-active-class="transition ease-out duration-200"
            enter-from-class="opacity-0 scale-95"
            enter-to-class="opacity-100 scale-100"
            leave-active-class="transition ease-in duration-150"
            leave-from-class="opacity-100 scale-100"
            leave-to-class="opacity-0 scale-95"
          >
            <div
              v-if="show"
              class="relative bg-white rounded-2xl shadow-2xl max-w-2xl w-full overflow-hidden"
              @click.stop
            >
              <!-- Header -->
              <div class="px-6 py-4 bg-gradient-to-r from-indigo-50 to-white border-b border-gray-200">
                <div class="flex items-center justify-between">
                  <h3 class="text-lg font-bold text-gray-900">Детали записи</h3>
                  <button
                    @click="onClose"
                    class="text-gray-400 hover:text-gray-600 transition-colors"
                  >
                    <Icon name="ri:close-line" class="text-2xl" />
                  </button>
                </div>
              </div>

              <!-- Content -->
              <div class="px-6 py-4 max-h-[70vh] overflow-y-auto">
                <div v-if="log" class="space-y-4">
                  <!-- Basic Info -->
                  <div class="grid grid-cols-2 gap-4">
                    <div>
                      <label class="text-xs font-medium text-gray-500 uppercase">ID</label>
                      <p class="text-sm text-gray-900 mt-1">#{{ log.id }}</p>
                    </div>
                    <div>
                      <label class="text-xs font-medium text-gray-500 uppercase">Дата и время</label>
                      <p class="text-sm text-gray-900 mt-1">{{ formatDateTime(log.created_at) }}</p>
                    </div>
                    <div>
                      <label class="text-xs font-medium text-gray-500 uppercase">Пользователь</label>
                      <p class="text-sm text-gray-900 mt-1">{{ log.user_email || 'Система' }}</p>
                    </div>
                    <div>
                      <label class="text-xs font-medium text-gray-500 uppercase">Действие</label>
                      <p class="text-sm text-gray-900 mt-1">{{ actionLabel(log.action) }}</p>
                    </div>
                    <div>
                      <label class="text-xs font-medium text-gray-500 uppercase">Тип сущности</label>
                      <p class="text-sm text-gray-900 mt-1">{{ entityTypeLabel(log.entity_type) }}</p>
                    </div>
                    <div>
                      <label class="text-xs font-medium text-gray-500 uppercase">ID сущности</label>
                      <p class="text-sm text-gray-900 mt-1">#{{ log.entity_id }}</p>
                    </div>
                    <div v-if="log.ip_address">
                      <label class="text-xs font-medium text-gray-500 uppercase">IP адрес</label>
                      <p class="text-sm text-gray-900 mt-1">{{ log.ip_address }}</p>
                    </div>
                  </div>

                  <!-- Details JSON -->
                  <div v-if="log.details" class="mt-6">
                    <label class="text-xs font-medium text-gray-500 uppercase mb-2 block">Детали</label>
                    <pre class="bg-gray-50 rounded-lg p-4 text-xs overflow-x-auto border border-gray-200">{{ JSON.stringify(log.details, null, 2) }}</pre>
                  </div>

                  <!-- User Agent -->
                  <div v-if="log.user_agent" class="mt-4">
                    <label class="text-xs font-medium text-gray-500 uppercase mb-2 block">User Agent</label>
                    <p class="text-xs text-gray-600 bg-gray-50 rounded-lg p-3 border border-gray-200">{{ log.user_agent }}</p>
                  </div>
                </div>
              </div>

              <!-- Footer -->
              <div class="px-6 py-4 bg-gray-50 border-t border-gray-200 flex justify-end">
                <button
                  @click="onClose"
                  class="px-4 py-2 bg-gray-200 hover:bg-gray-300 text-gray-700 rounded-lg transition-colors"
                >
                  Закрыть
                </button>
              </div>
            </div>
          </Transition>
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
    created: 'Создание',
    updated: 'Обновление',
    deleted: 'Удаление',
    login: 'Вход',
    logout: 'Выход'
  }
  return labels[action] || action
}

const entityTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    employee: 'Сотрудник',
    computer: 'Компьютер',
    equipment: 'Оборудование',
    request: 'Заявка',
    user: 'Пользователь',
    department: 'Отдел',
    position: 'Должность'
  }
  return labels[type] || type
}
</script>
