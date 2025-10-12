<template>
  <div class="max-w-5xl mx-auto space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4 mb-6">
        <NuxtLink to="/requests" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
          <Icon name="ri:arrow-left-line" class="text-xl" />
        </NuxtLink>
        <div class="flex-1 flex items-center justify-between">
          <div class="flex items-center gap-4">
            <div
              :class="[
                'h-16 w-16 rounded-2xl flex items-center justify-center shadow-lg',
                getStatusGradient(request.status)
              ]"
            >
              <Icon :name="getStatusIcon(request.status)" class="text-3xl text-white" />
            </div>
            <div>
              <div class="flex items-center gap-3">
                <h1 class="text-3xl font-bold text-gray-900">Заявка #{{ request.id }}</h1>
                <span
                  :class="[
                    'badge',
                    getStatusBadgeColor(request.status)
                  ]"
                >
                  {{ getStatusLabel(request.status) }}
                </span>
              </div>
              <p class="mt-1 text-gray-600">Создана {{ request.created_at }}</p>
            </div>
          </div>
          
          <div class="flex items-center gap-2">
            <NuxtLink :to="`/requests/${request.id}/edit`" class="btn btn-secondary">
              <Icon name="ri:edit-line" class="mr-2" />
              Редактировать
            </NuxtLink>
            <button @click="showDeleteModal = true" class="btn bg-red-50 text-red-600 hover:bg-red-100">
              <Icon name="ri:delete-bin-line" class="mr-2" />
              Удалить
            </button>
          </div>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Основная информация -->
      <div class="lg:col-span-2 space-y-6">
        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h2 class="text-lg font-semibold text-gray-900">Описание заявки</h2>
          </div>
          
          <div class="p-6 space-y-4">
            <div>
              <h3 class="text-xl font-semibold text-gray-900 mb-2">{{ request.title }}</h3>
              <p class="text-gray-700 leading-relaxed">{{ request.description }}</p>
            </div>

            <div v-if="request.equipment" class="p-4 rounded-lg bg-blue-50 border border-blue-100">
              <div class="flex items-center gap-2 mb-2">
                <Icon name="ri:computer-line" class="text-blue-600" />
                <p class="text-sm font-medium text-gray-900">Оборудование</p>
              </div>
              <p class="text-sm text-gray-700">{{ request.equipment }}</p>
            </div>

            <div v-if="request.files && request.files.length > 0" class="space-y-2">
              <p class="text-sm font-medium text-gray-900">Прикрепленные файлы</p>
              <div class="space-y-2">
                <a
                  v-for="file in request.files"
                  :key="file.id"
                  :href="file.url"
                  target="_blank"
                  class="flex items-center gap-3 p-3 rounded-lg bg-gray-50 hover:bg-gray-100 transition-colors"
                >
                  <Icon name="ri:file-line" class="text-gray-600 text-xl" />
                  <div class="flex-1">
                    <p class="text-sm font-medium text-gray-900">{{ file.name }}</p>
                    <p class="text-xs text-gray-500">{{ file.size }}</p>
                  </div>
                  <Icon name="ri:download-line" class="text-gray-400" />
                </a>
              </div>
            </div>
          </div>
        </div>

        <!-- Комментарии -->
        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <Icon name="ri:chat-3-line" class="text-gray-600" />
                <h2 class="text-lg font-semibold text-gray-900">Комментарии</h2>
                <span class="badge bg-gray-100 text-gray-800">{{ comments.length }}</span>
              </div>
            </div>
          </div>
          
          <div class="p-6 space-y-4">
            <div
              v-for="comment in comments"
              :key="comment.id"
              class="flex gap-4"
            >
              <div class="h-10 w-10 rounded-full bg-gradient-to-br from-primary-500 to-primary-700 flex items-center justify-center flex-shrink-0">
                <span class="text-white font-semibold text-sm">{{ getInitials(comment.author) }}</span>
              </div>
              <div class="flex-1">
                <div class="flex items-center gap-2 mb-1">
                  <span class="text-sm font-semibold text-gray-900">{{ comment.author }}</span>
                  <span class="text-xs text-gray-500">{{ comment.created_at }}</span>
                  <span v-if="comment.is_internal" class="badge bg-amber-100 text-amber-800 text-xs">
                    Внутренний
                  </span>
                </div>
                <p class="text-sm text-gray-700">{{ comment.text }}</p>
                <div v-if="comment.attachments" class="mt-2 flex gap-2">
                  <a
                    v-for="attachment in comment.attachments"
                    :key="attachment.id"
                    :href="attachment.url"
                    class="text-xs text-primary-600 hover:text-primary-700 flex items-center gap-1"
                  >
                    <Icon name="ri:attachment-line" />
                    {{ attachment.name }}
                  </a>
                </div>
              </div>
            </div>

            <div v-if="comments.length === 0" class="text-center py-8 text-gray-500">
              <Icon name="ri:chat-off-line" class="text-4xl mx-auto mb-2 text-gray-400" />
              <p class="text-sm">Комментариев пока нет</p>
            </div>

            <!-- Форма добавления комментария -->
            <div class="pt-4 border-t border-gray-200">
              <div class="flex gap-3">
                <div class="h-10 w-10 rounded-full bg-gradient-to-br from-gray-400 to-gray-600 flex items-center justify-center flex-shrink-0">
                  <span class="text-white font-semibold text-sm">Вы</span>
                </div>
                <div class="flex-1 space-y-2">
                  <textarea
                    v-model="newComment"
                    rows="3"
                    placeholder="Добавить комментарий..."
                    class="input-field resize-none"
                  />
                  <div class="flex items-center justify-between">
                    <label class="flex items-center gap-2 text-sm text-gray-600 cursor-pointer">
                      <input v-model="isInternalComment" type="checkbox" class="rounded" />
                      Внутренний комментарий
                    </label>
                    <button @click="addComment" class="btn btn-primary btn-sm">
                      <Icon name="ri:send-plane-line" class="mr-2" />
                      Отправить
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- История изменений -->
        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center gap-2">
              <Icon name="ri:history-line" class="text-gray-600" />
              <h2 class="text-lg font-semibold text-gray-900">История изменений</h2>
            </div>
          </div>
          
          <div class="p-6">
            <div class="space-y-4">
              <div
                v-for="(event, index) in history"
                :key="event.id"
                class="flex gap-4"
              >
                <div class="flex flex-col items-center">
                  <div
                    :class="[
                      'h-8 w-8 rounded-full flex items-center justify-center',
                      getEventColor(event.type)
                    ]"
                  >
                    <Icon :name="getEventIcon(event.type)" class="text-sm text-white" />
                  </div>
                  <div v-if="index < history.length - 1" class="w-0.5 flex-1 bg-gray-200 mt-2" />
                </div>
                <div class="flex-1 pb-4">
                  <p class="text-sm text-gray-900">
                    <span class="font-semibold">{{ event.user }}</span>
                    {{ event.action }}
                  </p>
                  <p class="text-xs text-gray-500 mt-1">{{ event.created_at }}</p>
                  <p v-if="event.details" class="text-sm text-gray-600 mt-1">{{ event.details }}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Боковая панель -->
      <div class="space-y-6">
        <!-- Статус -->
        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h3 class="text-sm font-semibold text-gray-900">Статус</h3>
          </div>
          <div class="p-6">
            <select v-model="request.status" @change="updateStatus" class="input-field">
              <option value="new">Новая</option>
              <option value="in_progress">В работе</option>
              <option value="completed">Завершена</option>
              <option value="cancelled">Отменена</option>
            </select>
          </div>
        </div>

        <!-- Детали -->
        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h3 class="text-sm font-semibold text-gray-900">Детали</h3>
          </div>
          <div class="p-6 space-y-4">
            <div>
              <p class="text-xs text-gray-600 mb-1">Заявитель</p>
              <div class="flex items-center gap-2">
                <Icon name="ri:user-line" class="text-gray-600" />
                <p class="text-sm font-medium text-gray-900">{{ request.requester }}</p>
              </div>
            </div>

            <div>
              <p class="text-xs text-gray-600 mb-1">Приоритет</p>
              <span
                :class="[
                  'badge',
                  getPriorityColor(request.priority)
                ]"
              >
                {{ getPriorityLabel(request.priority) }}
              </span>
            </div>

            <div>
              <p class="text-xs text-gray-600 mb-1">Тип</p>
              <span class="badge bg-blue-100 text-blue-800">
                {{ getTypeLabel(request.type) }}
              </span>
            </div>

            <div v-if="request.category">
              <p class="text-xs text-gray-600 mb-1">Категория</p>
              <span class="badge bg-purple-100 text-purple-800">
                {{ getCategoryLabel(request.category) }}
              </span>
            </div>

            <div v-if="request.location">
              <p class="text-xs text-gray-600 mb-1">Местоположение</p>
              <div class="flex items-center gap-2">
                <Icon name="ri:map-pin-line" class="text-gray-600" />
                <p class="text-sm text-gray-900">{{ request.location }}</p>
              </div>
            </div>

            <div v-if="request.phone">
              <p class="text-xs text-gray-600 mb-1">Телефон</p>
              <div class="flex items-center gap-2">
                <Icon name="ri:phone-line" class="text-gray-600" />
                <p class="text-sm text-gray-900">{{ request.phone }}</p>
              </div>
            </div>

            <div v-if="request.assigned_to">
              <p class="text-xs text-gray-600 mb-1">Исполнитель</p>
              <div class="flex items-center gap-2">
                <Icon name="ri:user-settings-line" class="text-gray-600" />
                <p class="text-sm font-medium text-gray-900">{{ request.assigned_to }}</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Временные метки -->
        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h3 class="text-sm font-semibold text-gray-900">Временные метки</h3>
          </div>
          <div class="p-6 space-y-3">
            <div class="flex items-center gap-2 text-sm">
              <Icon name="ri:time-line" class="text-gray-600" />
              <div>
                <p class="text-xs text-gray-600">Создана</p>
                <p class="text-gray-900">{{ request.created_at }}</p>
              </div>
            </div>
            <div v-if="request.updated_at" class="flex items-center gap-2 text-sm">
              <Icon name="ri:refresh-line" class="text-gray-600" />
              <div>
                <p class="text-xs text-gray-600">Обновлена</p>
                <p class="text-gray-900">{{ request.updated_at }}</p>
              </div>
            </div>
            <div v-if="request.completed_at" class="flex items-center gap-2 text-sm">
              <Icon name="ri:checkbox-circle-line" class="text-green-600" />
              <div>
                <p class="text-xs text-gray-600">Завершена</p>
                <p class="text-gray-900">{{ request.completed_at }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Модальное окно удаления -->
    <Teleport to="body">
      <Transition name="modal">
        <div v-if="showDeleteModal" class="fixed inset-0 z-[60] overflow-y-auto">
          <div class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm" @click="showDeleteModal = false" />
          
          <div class="relative min-h-screen flex items-center justify-center p-4">
            <div class="relative bg-white rounded-2xl shadow-2xl max-w-md w-full p-6">
              <div class="text-center">
                <div class="h-16 w-16 rounded-full bg-red-100 flex items-center justify-center mx-auto mb-4">
                  <Icon name="ri:delete-bin-line" class="text-3xl text-red-600" />
                </div>
                <h3 class="text-lg font-semibold text-gray-900 mb-2">Удалить заявку?</h3>
                <p class="text-sm text-gray-600 mb-6">
                  Это действие нельзя отменить. Заявка будет удалена навсегда.
                </p>
                <div class="flex gap-3">
                  <button @click="showDeleteModal = false" class="flex-1 btn btn-secondary">
                    Отмена
                  </button>
                  <button @click="deleteRequest" class="flex-1 btn bg-red-600 text-white hover:bg-red-700">
                    Удалить
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
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

const showDeleteModal = ref(false)
const newComment = ref('')
const isInternalComment = ref(false)

// Мок данные заявки
const request = ref({
  id: route.params.id,
  title: 'Не работает принтер в офисе 301',
  description: 'Принтер HP LaserJet не печатает документы. При попытке печати появляется ошибка "Printer offline". Проверил подключение - кабель подключен, индикаторы горят. Требуется срочная помощь, так как нужно распечатать важные документы для встречи.',
  requester: 'Иванов Иван Иванович',
  status: 'in_progress',
  priority: 'high',
  type: 'hardware',
  category: 'printer',
  location: 'Офис 301, 3 этаж',
  phone: '+7 (999) 123-45-67',
  equipment: 'HP LaserJet Pro M404dn, инв. №12345',
  assigned_to: 'Петров Петр Петрович',
  created_at: '10.12.2025 14:30',
  updated_at: '10.12.2025 15:45',
  completed_at: null,
  files: [
    { id: 1, name: 'error_screenshot.png', size: '245 KB', url: '#' },
    { id: 2, name: 'printer_manual.pdf', size: '1.2 MB', url: '#' }
  ]
})

const comments = ref([
  {
    id: 1,
    author: 'Техподдержка',
    text: 'Заявка принята в работу. Проверяем принтер.',
    created_at: '10.12.2025 15:00',
    is_internal: false
  },
  {
    id: 2,
    author: 'Петров П.П.',
    text: 'Проблема в драйвере. Переустанавливаю.',
    created_at: '10.12.2025 15:30',
    is_internal: true
  }
])

const history = ref([
  {
    id: 1,
    type: 'created',
    user: 'Иванов И.И.',
    action: 'создал заявку',
    created_at: '10.12.2025 14:30'
  },
  {
    id: 2,
    type: 'status_change',
    user: 'Система',
    action: 'изменил статус',
    details: 'Новая → В работе',
    created_at: '10.12.2025 15:00'
  },
  {
    id: 3,
    type: 'assigned',
    user: 'Администратор',
    action: 'назначил исполнителя',
    details: 'Петров П.П.',
    created_at: '10.12.2025 15:05'
  }
])

useHead({
  title: `Заявка #${request.value.id}`
})

const updateStatus = () => {
  toast.success(`Статус изменен на "${getStatusLabel(request.value.status)}"`)
  history.value.unshift({
    id: history.value.length + 1,
    type: 'status_change',
    user: 'Вы',
    action: 'изменил статус',
    details: getStatusLabel(request.value.status),
    created_at: new Date().toLocaleString('ru-RU')
  })
}

const addComment = () => {
  if (!newComment.value.trim()) return
  
  comments.value.push({
    id: comments.value.length + 1,
    author: 'Вы',
    text: newComment.value,
    created_at: new Date().toLocaleString('ru-RU'),
    is_internal: isInternalComment.value
  })
  
  newComment.value = ''
  isInternalComment.value = false
  toast.success('Комментарий добавлен')
}

const deleteRequest = () => {
  toast.success('Заявка удалена')
  router.push('/requests')
}

const getInitials = (name: string) => {
  return name.split(' ').map(n => n[0]).join('').toUpperCase().slice(0, 2)
}

const getStatusIcon = (status: string) => {
  const icons: Record<string, string> = {
    new: 'ri:file-add-line',
    in_progress: 'ri:time-line',
    completed: 'ri:checkbox-circle-line',
    cancelled: 'ri:close-circle-line'
  }
  return icons[status] || 'ri:file-line'
}

const getStatusGradient = (status: string) => {
  const gradients: Record<string, string> = {
    new: 'bg-gradient-to-br from-blue-500 to-blue-700',
    in_progress: 'bg-gradient-to-br from-amber-500 to-amber-700',
    completed: 'bg-gradient-to-br from-green-500 to-green-700',
    cancelled: 'bg-gradient-to-br from-red-500 to-red-700'
  }
  return gradients[status] || 'bg-gradient-to-br from-gray-500 to-gray-700'
}

const getStatusBadgeColor = (status: string) => {
  const colors: Record<string, string> = {
    new: 'bg-blue-100 text-blue-800',
    in_progress: 'bg-amber-100 text-amber-800',
    completed: 'bg-green-100 text-green-800',
    cancelled: 'bg-red-100 text-red-800'
  }
  return colors[status] || 'bg-gray-100 text-gray-800'
}

const getStatusLabel = (status: string) => {
  const labels: Record<string, string> = {
    new: 'Новая',
    in_progress: 'В работе',
    completed: 'Завершена',
    cancelled: 'Отменена'
  }
  return labels[status] || status
}

const getPriorityColor = (priority: string) => {
  const colors: Record<string, string> = {
    low: 'bg-gray-100 text-gray-800',
    medium: 'bg-blue-100 text-blue-800',
    high: 'bg-red-100 text-red-800'
  }
  return colors[priority] || 'bg-gray-100 text-gray-800'
}

const getPriorityLabel = (priority: string) => {
  const labels: Record<string, string> = {
    low: 'Низкий',
    medium: 'Средний',
    high: 'Высокий'
  }
  return labels[priority] || priority
}

const getTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    hardware: 'Оборудование',
    software: 'ПО',
    network: 'Сеть',
    access: 'Доступ',
    other: 'Другое'
  }
  return labels[type] || type
}

const getCategoryLabel = (category: string) => {
  const labels: Record<string, string> = {
    printer: 'Принтеры',
    computer: 'Компьютеры',
    phone: 'Телефония',
    email: 'Email',
    internet: 'Интернет',
    software_install: 'Установка ПО',
    account: 'Учетные записи'
  }
  return labels[category] || category
}

const getEventIcon = (type: string) => {
  const icons: Record<string, string> = {
    created: 'ri:add-line',
    status_change: 'ri:refresh-line',
    assigned: 'ri:user-add-line',
    comment: 'ri:chat-3-line',
    file: 'ri:attachment-line'
  }
  return icons[type] || 'ri:record-circle-line'
}

const getEventColor = (type: string) => {
  const colors: Record<string, string> = {
    created: 'bg-blue-500',
    status_change: 'bg-amber-500',
    assigned: 'bg-purple-500',
    comment: 'bg-green-500',
    file: 'bg-gray-500'
  }
  return colors[type] || 'bg-gray-500'
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
</style>
