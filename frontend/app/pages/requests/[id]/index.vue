<template>
  <div class="max-w-5xl mx-auto space-y-6">
    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-amber-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка заявки...</p>
    </div>

    <div v-else-if="!request" class="card p-16 text-center">
      <Icon name="ri:error-warning-line" class="text-4xl text-gray-300 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Заявка не найдена</h3>
      <p class="text-sm text-gray-500 mb-6">Возможно, она была удалена или ссылка устарела</p>
      <NuxtLink to="/requests" class="btn btn-primary">
        <Icon name="ri:arrow-left-line" class="mr-2" />
        К списку заявок
      </NuxtLink>
    </div>

    <template v-else>
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
                  getStatusGradient(request.status),
                ]"
              >
                <Icon :name="getStatusIcon(request.status)" class="text-3xl text-white" />
              </div>
              <div>
                <div class="flex items-center gap-3">
                  <h1 class="text-3xl font-bold text-gray-900">Заявка #{{ request.id }}</h1>
                  <span :class="['badge', getStatusBadgeColor(request.status)]">
                    {{ getStatusLabel(request.status) }}
                  </span>
                </div>
                <p class="mt-1 text-gray-600">Создана {{ formatDateTime(request.created_at) }}</p>
              </div>
            </div>

            <div class="flex items-center gap-2">
              <NuxtLink :to="`/requests/${request.id}/edit`" class="btn btn-secondary">
                <Icon name="ri:edit-line" class="mr-2" />
                Редактировать
              </NuxtLink>
              <button
                class="btn bg-red-50 text-red-600 hover:bg-red-100"
                @click="showDeleteModal = true"
              >
                <Icon name="ri:delete-bin-line" class="mr-2" />
                Удалить
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div class="lg:col-span-2 space-y-6">
          <div class="card">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <h2 class="text-lg font-semibold text-gray-900">Описание заявки</h2>
            </div>

            <div class="p-6 space-y-4">
              <div>
                <h3 class="text-xl font-semibold text-gray-900 mb-2">
                  {{ request.title }}
                </h3>
                <p class="text-gray-700 leading-relaxed whitespace-pre-line">
                  {{ request.description }}
                </p>
              </div>

              <div v-if="computer" class="p-4 rounded-lg bg-blue-50 border border-blue-100">
                <div class="flex items-center gap-2 mb-2">
                  <Icon name="ri:computer-line" class="text-blue-600" />
                  <p class="text-sm font-medium text-gray-900">Компьютер</p>
                </div>
                <NuxtLink
                  :to="`/computers/${computer.id}`"
                  class="text-sm text-blue-700 hover:text-blue-800"
                >
                  {{ computer.hostname || 'Без имени' }}
                  <span v-if="computer.inventory_number" class="text-blue-500">
                    ({{ computer.inventory_number }})
                  </span>
                </NuxtLink>
              </div>

              <div v-if="equipment" class="p-4 rounded-lg bg-green-50 border border-green-100">
                <div class="flex items-center gap-2 mb-2">
                  <Icon name="ri:printer-line" class="text-green-600" />
                  <p class="text-sm font-medium text-gray-900">Оборудование</p>
                </div>
                <NuxtLink
                  :to="`/equipment/${equipment.id}`"
                  class="text-sm text-green-700 hover:text-green-800"
                >
                  {{ equipment.name }}
                  <span v-if="equipment.inventory_number" class="text-green-500">
                    ({{ equipment.inventory_number }})
                  </span>
                </NuxtLink>
              </div>

              <div v-if="request.sla_deadline" class="flex items-center gap-2 text-sm">
                <Icon name="ri:timer-line" class="text-gray-500" />
                <span class="text-gray-600">SLA до:</span>
                <span class="font-medium text-gray-900">
                  {{ formatDateTime(request.sla_deadline) }}
                </span>
                <span v-if="request.sla_breached" class="badge bg-red-100 text-red-800 text-xs">
                  Просрочено
                </span>
              </div>
            </div>
          </div>

          <div class="card">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <div class="flex items-center gap-2">
                <Icon name="ri:chat-3-line" class="text-gray-600" />
                <h2 class="text-lg font-semibold text-gray-900">Комментарии</h2>
                <span class="badge bg-gray-100 text-gray-800">{{ comments.length }}</span>
              </div>
            </div>

            <div class="p-6 space-y-4">
              <div v-for="comment in comments" :key="comment.id" class="flex gap-4">
                <div
                  class="h-10 w-10 rounded-full bg-gradient-to-br from-primary-500 to-primary-700 flex items-center justify-center flex-shrink-0"
                >
                  <span class="text-white font-semibold text-sm">
                    {{ getInitials(comment.user_email) }}
                  </span>
                </div>
                <div class="flex-1">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="text-sm font-semibold text-gray-900">
                      {{ comment.user_email || 'Пользователь' }}
                    </span>
                    <span class="text-xs text-gray-500">
                      {{ formatDateTime(comment.created_at) }}
                    </span>
                    <span
                      v-if="comment.is_internal"
                      class="badge bg-amber-100 text-amber-800 text-xs"
                    >
                      Внутренний
                    </span>
                    <button
                      class="ml-auto p-1 rounded text-gray-400 hover:text-red-600 hover:bg-red-50 transition-colors"
                      title="Удалить комментарий"
                      @click="removeComment(comment.id)"
                    >
                      <Icon name="ri:delete-bin-line" />
                    </button>
                  </div>
                  <p class="text-sm text-gray-700 whitespace-pre-line">
                    {{ comment.comment }}
                  </p>
                </div>
              </div>

              <div v-if="comments.length === 0" class="text-center py-8 text-gray-500">
                <Icon name="ri:chat-off-line" class="text-4xl mx-auto mb-2 text-gray-400" />
                <p class="text-sm">Комментариев пока нет</p>
              </div>

              <div class="pt-4 border-t border-gray-200">
                <div class="flex gap-3">
                  <div
                    class="h-10 w-10 rounded-full bg-gradient-to-br from-gray-400 to-gray-600 flex items-center justify-center flex-shrink-0"
                  >
                    <span class="text-white font-semibold text-sm">Вы</span>
                  </div>
                  <div class="flex-1 space-y-2">
                    <textarea
                      v-model="newComment"
                      rows="3"
                      placeholder="Добавить комментарий..."
                      class="input-field resize-none"
                    ></textarea>
                    <div class="flex items-center justify-between">
                      <label class="flex items-center gap-2 text-sm text-gray-600 cursor-pointer">
                        <input v-model="isInternalComment" type="checkbox" class="rounded" />
                        Внутренний комментарий
                      </label>
                      <button
                        class="btn btn-primary btn-sm disabled:opacity-50"
                        :disabled="sendingComment || !newComment.trim()"
                        @click="addComment"
                      >
                        <Icon name="ri:send-plane-line" class="mr-2" />
                        {{ sendingComment ? 'Отправка...' : 'Отправить' }}
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="card">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <div class="flex items-center gap-2">
                <Icon name="ri:history-line" class="text-gray-600" />
                <h2 class="text-lg font-semibold text-gray-900">История изменений</h2>
              </div>
            </div>

            <div class="p-6">
              <div v-if="history.length === 0" class="text-center py-8">
                <Icon name="ri:history-line" class="text-3xl text-gray-300 mb-2" />
                <p class="text-sm text-gray-500">Изменений пока не было</p>
              </div>
              <div v-else class="space-y-4">
                <div v-for="(event, index) in history" :key="event.id" class="flex gap-4">
                  <div class="flex flex-col items-center">
                    <div
                      :class="[
                        'h-8 w-8 rounded-full flex items-center justify-center',
                        getEventColor(event.action),
                      ]"
                    >
                      <Icon :name="getEventIcon(event.action)" class="text-sm text-white" />
                    </div>
                    <div
                      v-if="index < history.length - 1"
                      class="w-0.5 flex-1 bg-gray-200 mt-2"
                    ></div>
                  </div>
                  <div class="flex-1 pb-4">
                    <p class="text-sm text-gray-900">
                      <span class="font-semibold">{{ event.user_email || 'Система' }}</span>
                      {{ getActionLabel(event.action) }}
                    </p>
                    <p v-if="getEventSummary(event)" class="text-sm text-gray-600 mt-1">
                      {{ getEventSummary(event) }}
                    </p>
                    <p class="text-xs text-gray-500 mt-1">
                      {{ formatDateTime(event.created_at) }}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="space-y-6">
          <div class="card">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <h3 class="text-sm font-semibold text-gray-900">Статус</h3>
            </div>
            <div class="p-6">
              <select v-model="statusDraft" class="input-field" @change="updateStatus">
                <option value="new">Новая</option>
                <option value="in_progress">В работе</option>
                <option value="waiting">Ожидание</option>
                <option value="resolved">Решена</option>
                <option value="closed">Закрыта</option>
                <option value="cancelled">Отменена</option>
              </select>
            </div>
          </div>

          <div class="card">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <h3 class="text-sm font-semibold text-gray-900">Детали</h3>
            </div>
            <div class="p-6 space-y-4">
              <div>
                <p class="text-xs text-gray-600 mb-1">Заявитель</p>
                <div class="flex items-center gap-2">
                  <Icon name="ri:user-line" class="text-gray-600" />
                  <p class="text-sm font-medium text-gray-900">
                    {{ requesterName }}
                  </p>
                </div>
              </div>

              <div>
                <p class="text-xs text-gray-600 mb-1">Приоритет</p>
                <span :class="['badge', getPriorityColor(request.priority)]">
                  {{ getPriorityLabel(request.priority) }}
                </span>
              </div>

              <div v-if="request.category">
                <p class="text-xs text-gray-600 mb-1">Категория</p>
                <span class="badge bg-purple-100 text-purple-800">
                  {{ getCategoryLabel(request.category) }}
                </span>
              </div>

              <div v-if="assigneeName">
                <p class="text-xs text-gray-600 mb-1">Исполнитель</p>
                <div class="flex items-center gap-2">
                  <Icon name="ri:user-settings-line" class="text-gray-600" />
                  <p class="text-sm font-medium text-gray-900">
                    {{ assigneeName }}
                  </p>
                </div>
              </div>

              <div v-if="request.estimated_time">
                <p class="text-xs text-gray-600 mb-1">Оценка</p>
                <p class="text-sm text-gray-900">{{ request.estimated_time }} мин</p>
              </div>

              <div v-if="request.actual_time">
                <p class="text-xs text-gray-600 mb-1">Фактически</p>
                <p class="text-sm text-gray-900">{{ request.actual_time }} мин</p>
              </div>
            </div>
          </div>

          <div class="card">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <h3 class="text-sm font-semibold text-gray-900">Временные метки</h3>
            </div>
            <div class="p-6 space-y-3">
              <div class="flex items-center gap-2 text-sm">
                <Icon name="ri:time-line" class="text-gray-600" />
                <div>
                  <p class="text-xs text-gray-600">Создана</p>
                  <p class="text-gray-900">
                    {{ formatDateTime(request.created_at) }}
                  </p>
                </div>
              </div>
              <div class="flex items-center gap-2 text-sm">
                <Icon name="ri:refresh-line" class="text-gray-600" />
                <div>
                  <p class="text-xs text-gray-600">Обновлена</p>
                  <p class="text-gray-900">
                    {{ formatDateTime(request.updated_at) }}
                  </p>
                </div>
              </div>
              <div v-if="request.resolved_at" class="flex items-center gap-2 text-sm">
                <Icon name="ri:checkbox-circle-line" class="text-green-600" />
                <div>
                  <p class="text-xs text-gray-600">Решена</p>
                  <p class="text-gray-900">
                    {{ formatDateTime(request.resolved_at) }}
                  </p>
                </div>
              </div>
              <div v-if="request.closed_at" class="flex items-center gap-2 text-sm">
                <Icon name="ri:archive-line" class="text-gray-600" />
                <div>
                  <p class="text-xs text-gray-600">Закрыта</p>
                  <p class="text-gray-900">
                    {{ formatDateTime(request.closed_at) }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </template>

    <Teleport to="body">
      <Transition name="modal">
        <div v-if="showDeleteModal" class="fixed inset-0 z-[60] overflow-y-auto">
          <div
            class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm"
            @click="showDeleteModal = false"
          ></div>

          <div class="relative min-h-screen flex items-center justify-center p-4">
            <div class="relative bg-white rounded-2xl shadow-2xl max-w-md w-full p-6">
              <div class="text-center">
                <div
                  class="h-16 w-16 rounded-full bg-red-100 flex items-center justify-center mx-auto mb-4"
                >
                  <Icon name="ri:delete-bin-line" class="text-3xl text-red-600" />
                </div>
                <h3 class="text-lg font-semibold text-gray-900 mb-2">Удалить заявку?</h3>
                <p class="text-sm text-gray-600 mb-6">
                  Это действие нельзя отменить. Заявка будет удалена навсегда.
                </p>
                <div class="flex gap-3">
                  <button class="flex-1 btn btn-secondary" @click="showDeleteModal = false">
                    Отмена
                  </button>
                  <button
                    class="flex-1 btn bg-red-600 text-white hover:bg-red-700 disabled:opacity-50"
                    :disabled="deleting"
                    @click="deleteRequest"
                  >
                    {{ deleting ? 'Удаление...' : 'Удалить' }}
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
import type { ActivityLog } from '~/composables/useActivityLog'
import type { NewRequestRequest, RequestComment, RequestItem } from '~/composables/useRequests'
import type { Computer } from '~/composables/useComputers'
import type { Equipment } from '~/composables/useEquipment'
import type { Employee } from '~/composables/useEmployees'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

const route = useRoute()
const router = useRouter()
const toast = useToast()

const {
  fetchRequest,
  updateRequest,
  deleteRequest: removeRequest,
  fetchRequestComments,
  createRequestComment,
  deleteRequestComment,
} = useRequests()
const { fetchEmployees } = useEmployees()
const { fetchComputer } = useComputers()
const { fetchEquipment } = useEquipment()
const { fetchEntityHistory } = useActivityLog()

const requestId = Number(route.params.id)
const request = ref<RequestItem | null>(null)
const employees = ref<Employee[]>([])
const computer = ref<Computer | null>(null)
const equipment = ref<Equipment | null>(null)
const comments = ref<RequestComment[]>([])
const history = ref<ActivityLog[]>([])
const statusDraft = ref('')
const newComment = ref('')
const isInternalComment = ref(false)
const loading = ref(true)
const deleting = ref(false)
const sendingComment = ref(false)
const showDeleteModal = ref(false)

const requesterName = computed(() => {
  if (!request.value) return '—'
  const employee = employees.value.find((item) => item.id === request.value?.requester_id)
  return employee?.full_name ?? `Сотрудник #${request.value.requester_id}`
})

const assigneeName = computed(() => {
  const id = request.value?.assignee_id
  if (id === null || id === undefined) return ''
  const employee = employees.value.find((item) => item.id === id)
  return employee?.full_name ?? `Сотрудник #${id}`
})

const loadComments = async () => {
  try {
    const response = await fetchRequestComments(requestId)
    comments.value = response.comments
  } catch {
    comments.value = []
  }
}

const loadHistory = async () => {
  try {
    const response = await fetchEntityHistory('request', requestId)
    history.value = response.logs
  } catch {
    history.value = []
  }
}

const fetchData = async () => {
  if (!Number.isFinite(requestId)) {
    loading.value = false
    return
  }

  loading.value = true
  try {
    const [item, employeeList] = await Promise.all([
      fetchRequest(requestId),
      fetchEmployees({ per_page: 100 }),
    ])
    request.value = item
    statusDraft.value = item.status
    employees.value = employeeList.employees

    const tasks: Promise<unknown>[] = [loadComments(), loadHistory()]
    if (item.computer_id !== null) {
      tasks.push(
        fetchComputer(item.computer_id)
          .then((value) => {
            computer.value = value
          })
          .catch(() => {
            computer.value = null
          }),
      )
    }
    if (item.equipment_id !== null) {
      tasks.push(
        fetchEquipment(item.equipment_id)
          .then((value) => {
            equipment.value = value
          })
          .catch(() => {
            equipment.value = null
          }),
      )
    }
    await Promise.all(tasks)
  } catch (error) {
    request.value = null
    toast.error('Не удалось загрузить заявку: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const updateStatus = async () => {
  if (!request.value || statusDraft.value === request.value.status) return

  const payload: Partial<NewRequestRequest> = { status: statusDraft.value }
  if (statusDraft.value === 'resolved') payload.resolved_at = new Date().toISOString()
  if (statusDraft.value === 'closed') payload.closed_at = new Date().toISOString()

  try {
    const updated = await updateRequest(request.value.id, payload)
    request.value = updated
    statusDraft.value = updated.status
    toast.success(`Статус изменён на «${getStatusLabel(updated.status)}»`)
    await loadHistory()
  } catch (error) {
    statusDraft.value = request.value.status
    toast.error('Не удалось изменить статус: ' + getErrorMessage(error))
  }
}

const addComment = async () => {
  const text = newComment.value.trim()
  if (!text) return

  sendingComment.value = true
  try {
    const comment = await createRequestComment(requestId, {
      comment: text,
      is_internal: isInternalComment.value,
    })
    comments.value.push(comment)
    newComment.value = ''
    isInternalComment.value = false
    toast.success('Комментарий добавлен')
  } catch (error) {
    toast.error('Не удалось добавить комментарий: ' + getErrorMessage(error))
  } finally {
    sendingComment.value = false
  }
}

const removeComment = async (commentId: number) => {
  try {
    await deleteRequestComment(commentId)
    comments.value = comments.value.filter((comment) => comment.id !== commentId)
    toast.success('Комментарий удалён')
  } catch (error) {
    toast.error('Не удалось удалить комментарий: ' + getErrorMessage(error))
  }
}

const deleteRequest = async () => {
  if (!request.value) return

  deleting.value = true
  try {
    await removeRequest(request.value.id)
    toast.success('Заявка удалена')
    router.push('/requests')
  } catch (error) {
    toast.error('Не удалось удалить заявку: ' + getErrorMessage(error))
  } finally {
    deleting.value = false
    showDeleteModal.value = false
  }
}

const formatDateTime = (value: string | null) => {
  if (!value) return '—'
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString('ru-RU')
}

const getInitials = (email: string | null) => {
  if (!email) return '?'
  return email.split('@')[0].slice(0, 2).toUpperCase()
}

const getEventSummary = (event: ActivityLog) => {
  const details = event.details
  if (!details) return ''

  if (details.changes) {
    return Object.entries(details.changes)
      .map(([field, change]) => {
        const value = change as { old: unknown; new: unknown }
        return `${field}: ${value.old ?? '—'} → ${value.new ?? '—'}`
      })
      .join(', ')
  }

  return Object.values(details)
    .map((value) => String(value))
    .join(', ')
}

const getActionLabel = (action: string) => {
  const labels: Record<string, string> = {
    created: 'создал(а) заявку',
    updated: 'изменил(а) заявку',
    deleted: 'удалил(а) заявку',
  }
  return labels[action] || action
}

const getStatusIcon = (status: string) => {
  const icons: Record<string, string> = {
    new: 'ri:file-add-line',
    in_progress: 'ri:time-line',
    waiting: 'ri:pause-circle-line',
    resolved: 'ri:checkbox-circle-line',
    closed: 'ri:archive-line',
    cancelled: 'ri:close-circle-line',
  }
  return icons[status] || 'ri:file-line'
}

const getStatusGradient = (status: string) => {
  const gradients: Record<string, string> = {
    new: 'bg-gradient-to-br from-blue-500 to-blue-700',
    in_progress: 'bg-gradient-to-br from-amber-500 to-amber-700',
    waiting: 'bg-gradient-to-br from-purple-500 to-purple-700',
    resolved: 'bg-gradient-to-br from-green-500 to-green-700',
    closed: 'bg-gradient-to-br from-gray-500 to-gray-700',
    cancelled: 'bg-gradient-to-br from-red-500 to-red-700',
  }
  return gradients[status] || 'bg-gradient-to-br from-gray-500 to-gray-700'
}

const getStatusBadgeColor = (status: string) => {
  const colors: Record<string, string> = {
    new: 'bg-blue-100 text-blue-800',
    in_progress: 'bg-amber-100 text-amber-800',
    waiting: 'bg-purple-100 text-purple-800',
    resolved: 'bg-green-100 text-green-800',
    closed: 'bg-gray-100 text-gray-800',
    cancelled: 'bg-red-100 text-red-800',
  }
  return colors[status] || 'bg-gray-100 text-gray-800'
}

const getStatusLabel = (status: string) => {
  const labels: Record<string, string> = {
    new: 'Новая',
    in_progress: 'В работе',
    waiting: 'Ожидание',
    resolved: 'Решена',
    closed: 'Закрыта',
    cancelled: 'Отменена',
  }
  return labels[status] || status
}

const getPriorityColor = (priority: string) => {
  const colors: Record<string, string> = {
    low: 'bg-gray-100 text-gray-800',
    medium: 'bg-blue-100 text-blue-800',
    high: 'bg-amber-100 text-amber-800',
    critical: 'bg-red-100 text-red-800',
  }
  return colors[priority] || 'bg-gray-100 text-gray-800'
}

const getPriorityLabel = (priority: string) => {
  const labels: Record<string, string> = {
    low: 'Низкий',
    medium: 'Средний',
    high: 'Высокий',
    critical: 'Критический',
  }
  return labels[priority] || priority
}

const getCategoryLabel = (category: string) => {
  const labels: Record<string, string> = {
    hardware: 'Оборудование',
    software: 'ПО',
    network: 'Сеть',
    access: 'Доступ',
    consultation: 'Консультация',
    other: 'Другое',
  }
  return labels[category] || category
}

const getEventIcon = (action: string) => {
  const icons: Record<string, string> = {
    created: 'ri:add-line',
    updated: 'ri:edit-line',
    deleted: 'ri:delete-bin-line',
  }
  return icons[action] || 'ri:record-circle-line'
}

const getEventColor = (action: string) => {
  const colors: Record<string, string> = {
    created: 'bg-blue-500',
    updated: 'bg-amber-500',
    deleted: 'bg-red-500',
  }
  return colors[action] || 'bg-gray-500'
}

useHead({
  title: () => (request.value ? `Заявка #${request.value.id}` : 'Заявка'),
})

onMounted(fetchData)
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
