<template>
  <div class="space-y-6">
    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-green-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка оборудования...</p>
    </div>

    <div v-else-if="!equipment" class="card p-16 text-center">
      <Icon name="ri:error-warning-line" class="text-4xl text-gray-300 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Оборудование не найдено</h3>
      <p class="text-sm text-gray-500 mb-6">Возможно, оно было удалено или ссылка устарела</p>
      <NuxtLink to="/equipment" class="btn btn-primary">
        <Icon name="ri:arrow-left-line" class="mr-2" />
        К списку оборудования
      </NuxtLink>
    </div>

    <template v-else>
      <div class="mb-8">
        <div class="flex items-center gap-4 mb-6">
          <NuxtLink to="/equipment" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
            <Icon name="ri:arrow-left-line" class="text-xl" />
          </NuxtLink>
          <div class="flex-1 flex items-center justify-between">
            <div class="flex items-center gap-4">
              <div
                :class="[
                  'h-16 w-16 rounded-2xl flex items-center justify-center shadow-lg',
                  getTypeGradient(equipmentType),
                ]"
              >
                <Icon :name="getTypeIcon(equipmentType)" class="text-3xl text-white" />
              </div>
              <div>
                <div class="flex items-center gap-3">
                  <h1 class="text-3xl font-bold text-gray-900">
                    {{ equipment.name }}
                  </h1>
                  <span :class="['badge', getStatusColor(equipment.status)]">
                    {{ getStatusLabel(equipment.status) }}
                  </span>
                </div>
                <p class="mt-1 text-gray-600">
                  {{ equipment.inventory_number || 'Без инвентарного номера' }}
                </p>
              </div>
            </div>

            <div class="flex items-center gap-2">
              <NuxtLink :to="`/equipment/${equipment.id}/edit`" class="btn btn-secondary">
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
              <h2 class="text-lg font-semibold text-gray-900">Общая информация</h2>
            </div>
            <div class="p-6">
              <div class="grid grid-cols-2 gap-6">
                <div>
                  <p class="text-xs text-gray-600 mb-1">Производитель</p>
                  <p class="text-sm font-medium text-gray-900">
                    {{ equipment.manufacturer || '—' }}
                  </p>
                </div>
                <div>
                  <p class="text-xs text-gray-600 mb-1">Модель</p>
                  <p class="text-sm font-medium text-gray-900">
                    {{ equipment.model || '—' }}
                  </p>
                </div>
                <div>
                  <p class="text-xs text-gray-600 mb-1">Серийный номер</p>
                  <p class="text-sm font-medium text-gray-900 font-mono">
                    {{ equipment.serial_number || '—' }}
                  </p>
                </div>
                <div>
                  <p class="text-xs text-gray-600 mb-1">Тип</p>
                  <span class="badge bg-blue-100 text-blue-800">
                    {{ getTypeLabel(equipmentType) }}
                  </span>
                </div>
                <div v-if="equipment.location">
                  <p class="text-xs text-gray-600 mb-1">Местоположение</p>
                  <div class="flex items-center gap-2">
                    <Icon name="ri:map-pin-line" class="text-gray-400" />
                    <p class="text-sm font-medium text-gray-900">
                      {{ equipment.location }}
                    </p>
                  </div>
                </div>
                <div v-if="equipment.ip_address">
                  <p class="text-xs text-gray-600 mb-1">IP-адрес</p>
                  <p class="text-sm font-medium text-gray-900 font-mono">
                    {{ equipment.ip_address }}
                  </p>
                </div>
                <div v-if="equipment.mac_address">
                  <p class="text-xs text-gray-600 mb-1">MAC-адрес</p>
                  <p class="text-sm font-medium text-gray-900 font-mono">
                    {{ equipment.mac_address }}
                  </p>
                </div>
              </div>
              <div v-if="equipment.notes" class="mt-6 pt-6 border-t border-gray-200">
                <p class="text-xs text-gray-600 mb-2">Примечания</p>
                <p class="text-sm text-gray-700">
                  {{ equipment.notes }}
                </p>
              </div>
            </div>
          </div>

          <div v-if="employee" class="card">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <h2 class="text-lg font-semibold text-gray-900">Назначение</h2>
            </div>
            <div class="p-6">
              <NuxtLink
                :to="`/employees/${employee.id}`"
                class="flex items-center gap-4 p-4 rounded-lg bg-blue-50 border border-blue-100 hover:bg-blue-100 transition-colors"
              >
                <div class="h-12 w-12 rounded-full bg-blue-500 flex items-center justify-center">
                  <Icon name="ri:user-line" class="text-xl text-white" />
                </div>
                <div>
                  <p class="text-sm font-medium text-gray-900">
                    {{ employee.full_name }}
                  </p>
                  <p class="text-xs text-gray-600">
                    {{ employee.position_name || 'Должность не указана' }}
                  </p>
                </div>
              </NuxtLink>
            </div>
          </div>

          <div class="card">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <h2 class="text-lg font-semibold text-gray-900">История обслуживания</h2>
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
                <option value="active">Активное</option>
                <option value="inactive">Неактивное</option>
                <option value="repair">В ремонте</option>
                <option value="storage">На складе</option>
                <option value="decommissioned">Списано</option>
              </select>
            </div>
          </div>

          <div class="card">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <h3 class="text-sm font-semibold text-gray-900">Гарантия</h3>
            </div>
            <div class="p-6 space-y-4">
              <div>
                <p class="text-xs text-gray-600 mb-1">Дата покупки</p>
                <p class="text-sm font-medium text-gray-900">
                  {{ formatDate(equipment.purchase_date) }}
                </p>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Гарантия до</p>
                <div class="flex items-center gap-2">
                  <p class="text-sm font-medium text-gray-900">
                    {{ formatDate(equipment.warranty_end_date) }}
                  </p>
                  <span v-if="warrantyActive" class="badge bg-green-100 text-green-800 text-xs">
                    Активна
                  </span>
                </div>
              </div>
              <div v-if="equipment.next_maintenance_date">
                <p class="text-xs text-gray-600 mb-1">Следующее обслуживание</p>
                <p class="text-sm font-medium text-gray-900">
                  {{ formatDate(equipment.next_maintenance_date) }}
                </p>
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
                  <p class="text-xs text-gray-600">Добавлено</p>
                  <p class="text-gray-900">
                    {{ formatDateTime(equipment.created_at) }}
                  </p>
                </div>
              </div>
              <div class="flex items-center gap-2 text-sm">
                <Icon name="ri:refresh-line" class="text-gray-600" />
                <div>
                  <p class="text-xs text-gray-600">Обновлено</p>
                  <p class="text-gray-900">
                    {{ formatDateTime(equipment.updated_at) }}
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
                <h3 class="text-lg font-semibold text-gray-900 mb-2">Удалить оборудование?</h3>
                <p class="text-sm text-gray-600 mb-6">Это действие нельзя отменить.</p>
                <div class="flex gap-3">
                  <button class="flex-1 btn btn-secondary" @click="showDeleteModal = false">
                    Отмена
                  </button>
                  <button
                    class="flex-1 btn bg-red-600 text-white hover:bg-red-700 disabled:opacity-50"
                    :disabled="deleting"
                    @click="deleteEquipment"
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
import type { Employee } from '~/composables/useEmployees'
import type { Equipment } from '~/composables/useEquipment'

definePageMeta({ middleware: 'auth' })

const route = useRoute()
const router = useRouter()
const toast = useToast()

const { fetchEquipment, updateEquipment, deleteEquipment: removeEquipment } = useEquipment()
const { fetchEmployee } = useEmployees()
const { fetchEntityHistory } = useActivityLog()

const equipmentId = Number(route.params.id)
const equipment = ref<Equipment | null>(null)
const employee = ref<Employee | null>(null)
const history = ref<ActivityLog[]>([])
const statusDraft = ref('')
const loading = ref(true)
const deleting = ref(false)
const showDeleteModal = ref(false)

const equipmentType = computed(() => {
  if (!equipment.value) return 'printer'
  const value = `${equipment.value.name} ${equipment.value.model ?? ''}`.toLowerCase()
  if (value.includes('сканер') || value.includes('scanner')) return 'scanner'
  if (value.includes('мфу') || value.includes('mfu') || value.includes('imagerunner')) return 'mfu'
  if (value.includes('ups') || value.includes('ибп')) return 'ups'
  if (value.includes('switch') || value.includes('router') || value.includes('tp-link')) {
    return 'network'
  }
  return 'printer'
})

const warrantyActive = computed(() => {
  const end = equipment.value?.warranty_end_date
  if (!end) return false
  return end >= new Date().toISOString().slice(0, 10)
})

const loadHistory = async () => {
  try {
    const response = await fetchEntityHistory('equipment', equipmentId)
    history.value = response.logs
  } catch {
    history.value = []
  }
}

const fetchData = async () => {
  if (!Number.isFinite(equipmentId)) {
    loading.value = false
    return
  }

  loading.value = true
  try {
    const item = await fetchEquipment(equipmentId)
    equipment.value = item
    statusDraft.value = item.status

    const tasks: Promise<unknown>[] = [loadHistory()]
    if (item.employee_id !== null) {
      tasks.push(
        fetchEmployee(item.employee_id)
          .then((person) => {
            employee.value = person
          })
          .catch(() => {
            employee.value = null
          }),
      )
    }
    await Promise.all(tasks)
  } catch (error) {
    equipment.value = null
    toast.error('Не удалось загрузить оборудование: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const updateStatus = async () => {
  if (!equipment.value || statusDraft.value === equipment.value.status) return

  try {
    const updated = await updateEquipment(equipment.value.id, { status: statusDraft.value })
    equipment.value = updated
    statusDraft.value = updated.status
    toast.success(`Статус изменён на «${getStatusLabel(updated.status)}»`)
    await loadHistory()
  } catch (error) {
    statusDraft.value = equipment.value.status
    toast.error('Не удалось изменить статус: ' + getErrorMessage(error))
  }
}

const deleteEquipment = async () => {
  if (!equipment.value) return

  deleting.value = true
  try {
    await removeEquipment(equipment.value.id)
    toast.success('Оборудование удалено')
    router.push('/equipment')
  } catch (error) {
    toast.error('Не удалось удалить оборудование: ' + getErrorMessage(error))
  } finally {
    deleting.value = false
    showDeleteModal.value = false
  }
}

const formatDate = (value: string | null) => {
  if (!value) return '—'
  const [date] = value.split('T')
  const [year, month, day] = date.split('-')
  return day && month && year ? `${day}.${month}.${year}` : value
}

const formatDateTime = (value: string | null) => {
  if (!value) return '—'
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString('ru-RU')
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
    created: 'добавил(а) оборудование',
    updated: 'изменил(а) информацию',
    deleted: 'удалил(а) оборудование',
    maintenance: 'провёл(а) обслуживание',
    status_change: 'изменил(а) статус',
  }
  return labels[action] || action
}

const getTypeIcon = (type: string) => {
  const icons: Record<string, string> = {
    printer: 'ri:printer-line',
    scanner: 'ri:scan-line',
    mfu: 'ri:printer-line',
    network: 'ri:router-line',
    ups: 'ri:battery-charge-line',
  }
  return icons[type] || 'ri:device-line'
}

const getTypeGradient = (type: string) => {
  const gradients: Record<string, string> = {
    printer: 'bg-gradient-to-br from-blue-500 to-blue-700',
    scanner: 'bg-gradient-to-br from-purple-500 to-purple-700',
    mfu: 'bg-gradient-to-br from-indigo-500 to-indigo-700',
    network: 'bg-gradient-to-br from-cyan-500 to-cyan-700',
    ups: 'bg-gradient-to-br from-green-500 to-green-700',
  }
  return gradients[type] || 'bg-gradient-to-br from-gray-500 to-gray-700'
}

const getTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    printer: 'Принтер',
    scanner: 'Сканер',
    mfu: 'МФУ',
    network: 'Сетевое оборудование',
    ups: 'ИБП',
  }
  return labels[type] || type
}

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    active: 'bg-green-100 text-green-800',
    inactive: 'bg-gray-100 text-gray-800',
    repair: 'bg-amber-100 text-amber-800',
    storage: 'bg-blue-100 text-blue-800',
    decommissioned: 'bg-red-100 text-red-800',
  }
  return colors[status] || 'bg-gray-100 text-gray-800'
}

const getStatusLabel = (status: string) => {
  const labels: Record<string, string> = {
    active: 'Активное',
    inactive: 'Неактивное',
    repair: 'В ремонте',
    storage: 'На складе',
    decommissioned: 'Списано',
  }
  return labels[status] || status
}

const getEventIcon = (action: string) => {
  const icons: Record<string, string> = {
    created: 'ri:add-line',
    updated: 'ri:edit-line',
    deleted: 'ri:delete-bin-line',
    maintenance: 'ri:tools-line',
    status_change: 'ri:refresh-line',
  }
  return icons[action] || 'ri:record-circle-line'
}

const getEventColor = (action: string) => {
  const colors: Record<string, string> = {
    created: 'bg-blue-500',
    updated: 'bg-purple-500',
    deleted: 'bg-red-500',
    maintenance: 'bg-amber-500',
    status_change: 'bg-cyan-500',
  }
  return colors[action] || 'bg-gray-500'
}

useHead({
  title: () => equipment.value?.name || 'Оборудование',
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
