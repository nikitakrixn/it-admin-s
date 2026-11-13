<template>
  <div class="space-y-6">
    <div v-if="pending" class="flex flex-col items-center justify-center py-24">
      <Icon name="ri:loader-4-line" class="text-4xl text-primary-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка компьютера...</p>
    </div>

    <div v-else-if="!computer" class="card p-16 text-center">
      <Icon name="ri:error-warning-line" class="text-5xl text-red-400 mb-4" />
      <h2 class="text-xl font-semibold text-gray-900 mb-2">Компьютер не найден</h2>
      <p class="text-sm text-gray-500 mb-6">{{ errorMessage }}</p>
      <NuxtLink to="/computers" class="btn btn-primary">
        <Icon name="ri:arrow-left-line" class="mr-2" />
        Вернуться к списку
      </NuxtLink>
    </div>

    <template v-else>
      <!-- Заголовок и действия -->
      <div class="mb-8">
        <div class="flex items-center gap-4 mb-6">
          <NuxtLink to="/computers" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
            <Icon name="ri:arrow-left-line" class="text-xl" />
          </NuxtLink>
          <div class="flex-1 flex items-center justify-between">
            <div class="flex items-center gap-4">
              <div
                :class="[
                  'h-16 w-16 rounded-2xl flex items-center justify-center shadow-lg',
                  getTypeGradient(computer.computer_type),
                ]"
              >
                <Icon :name="getTypeIcon(computer.computer_type)" class="text-3xl text-white" />
              </div>
              <div>
                <div class="flex items-center gap-3">
                  <h1 class="text-3xl font-bold text-gray-900">
                    {{ computer.hostname || `Компьютер #${computer.id}` }}
                  </h1>
                  <span class="badge" :class="getStatusColor(computer.status)">
                    {{ getStatusLabel(computer.status) }}
                  </span>
                  <span v-if="computer.is_online" class="badge bg-green-100 text-green-800">
                    <Icon name="ri:checkbox-circle-line" class="mr-1" />
                    Онлайн
                  </span>
                </div>
                <p class="mt-1 text-gray-600">
                  {{ computer.inventory_number || 'Инвентарный номер не указан' }}
                </p>
              </div>
            </div>

            <div class="flex items-center gap-2">
              <NuxtLink :to="`/computers/${computer.id}/edit`" class="btn btn-secondary">
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
        <!-- Основная информация -->
        <div class="lg:col-span-2 space-y-6">
          <ComputerInfoCard :computer="computer" />
          <ComputerOsCard :computer="computer" />
          <ComputerComponentsCard
            :components="components"
            :computer-id="computer.id"
            @edit="editComponent"
          />
          <ComputerSoftwareCard :software="software" :computer-id="computer.id" />
          <ComputerActivityCard
            :activity-log="activityLog"
            :format-event-date="formatEventDate"
            :get-activity-description="getActivityDescription"
            :get-event-summary="getEventSummary"
            :get-activity-icon="getActivityIcon"
            :get-activity-color="getActivityColor"
            @show-details="showActivityDetails"
          />
        </div>

        <!-- Боковая панель -->
        <ComputerSidebar
          :computer="computer"
          :employee="employee"
          :monitoring="monitoring"
          @update:status="updateStatus"
        />
      </div>

      <!-- Модальное окно удаления -->
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
                  <h3 class="text-lg font-semibold text-gray-900 mb-2">Удалить компьютер?</h3>
                  <p class="text-sm text-gray-600 mb-6">
                    Это действие нельзя отменить. Компьютер будет удален навсегда.
                  </p>
                  <div class="flex gap-3">
                    <button class="flex-1 btn btn-secondary" @click="showDeleteModal = false">
                      Отмена
                    </button>
                    <button
                      class="flex-1 btn bg-red-600 text-white hover:bg-red-700 disabled:opacity-50"
                      :disabled="deleting"
                      @click="deleteComputer"
                    >
                      <Icon v-if="deleting" name="ri:loader-4-line" class="mr-2 animate-spin" />
                      Удалить
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </Transition>
      </Teleport>

      <!-- Модальное окно деталей активности -->
      <ActivityLogDetailModal
        :show="showActivityModal"
        :log="selectedActivity"
        @close="showActivityModal = false"
      />
    </template>
  </div>
</template>

<script setup lang="ts">
import type { ActivityLog } from '~/composables/useActivityLog'
import type { ComputerComponent, InstalledSoftware } from '~/composables/useComputerComponents'
import type { Computer } from '~/composables/useComputers'
import type { Employee } from '~/composables/useEmployees'
import type { ComputerMonitoring } from '~/composables/useMonitoring'
import ComputerInfoCard from '~/components/computers/ComputerInfoCard.vue'
import ComputerOsCard from '~/components/computers/ComputerOsCard.vue'
import ComputerComponentsCard from '~/components/computers/ComputerComponentsCard.vue'
import ComputerSoftwareCard from '~/components/computers/ComputerSoftwareCard.vue'
import ComputerActivityCard from '~/components/computers/ComputerActivityCard.vue'
import ComputerSidebar from '~/components/computers/ComputerSidebar.vue'

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
const { fetchComputer, updateComputer, deleteComputer: removeComputer } = useComputers()
const { fetchComponents, fetchInstalledSoftware } = useComputerComponents()
const { fetchEmployee } = useEmployees()
const { fetchLatestMonitoring } = useMonitoring()
const { fetchEntityHistory } = useActivityLog()

const computer = ref<Computer | null>(null)
const components = ref<ComputerComponent[]>([])
const software = ref<InstalledSoftware[]>([])
const employee = ref<Employee | null>(null)
const monitoring = ref<ComputerMonitoring | null>(null)
const activityLog = ref<ActivityLog[]>([])

const pending = ref(true)
const deleting = ref(false)
const errorMessage = ref('')
const showDeleteModal = ref(false)
const showActivityModal = ref(false)
const selectedActivity = ref<ActivityLog | null>(null)

const computerId = computed(() => Number(route.params.id))

const formatEventDate = (dateStr: string) => {
  const date = new Date(dateStr)
  if (Number.isNaN(date.getTime())) return dateStr
  return date.toLocaleString('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

const componentTypeLabels: Record<string, string> = {
  processor: 'Процессор',
  ram: 'Оперативная память',
  storage: 'Накопитель',
  gpu: 'Видеокарта',
  motherboard: 'Материнская плата',
  network: 'Сетевой адаптер',
  monitor: 'Монитор',
  peripheral: 'Периферия',
}

// Человекочитаемое описание действия в журнале
const getActivityDescription = (event: ActivityLog) => {
  const descriptions: Record<string, string> = {
    created: 'создал(а) компьютер',
    updated: 'обновил(а) информацию',
    assigned: 'назначил(а) компьютер сотруднику',
    status_change: 'изменил(а) статус',
    component_added: 'добавил(а) компонент',
    component_removed: 'удалил(а) компонент',
    software_installed: 'установил(а) ПО',
    software_removed: 'удалил(а) ПО',
    maintenance: 'провёл(а) обслуживание',
  }
  return descriptions[event.action] || event.action
}

// Краткая сводка изменений для списка (полный текст — в модальном окне)
const getEventSummary = (event: ActivityLog) => {
  if (!event.details) return ''

  if (event.details.changes && typeof event.details.changes === 'object') {
    return Object.values(event.details.changes)
      .map((change) => {
        const c = change as { old: unknown; new: unknown }
        return `${c.old ?? '—'} → ${c.new ?? '—'}`
      })
      .join(', ')
  }

  const parts: string[] = []
  const componentType = event.details.component_type
  if (typeof componentType === 'string') {
    parts.push(componentTypeLabels[componentType] || componentType)
  }
  if (typeof event.details.name === 'string') {
    parts.push(event.details.name)
  }
  if (typeof event.details.version === 'string' && event.details.version) {
    parts.push(`версия ${event.details.version}`)
  }
  return parts.join(' · ')
}

const getActivityIcon = (type: string) => {
  const icons: Record<string, string> = {
    created: 'ri:add-line',
    updated: 'ri:edit-line',
    assigned: 'ri:user-add-line',
    status_change: 'ri:refresh-line',
    component_added: 'ri:cpu-line',
    component_removed: 'ri:delete-bin-line',
    software_installed: 'ri:download-line',
    software_removed: 'ri:uninstall-line',
    maintenance: 'ri:tools-line',
  }
  return icons[type] || 'ri:record-circle-line'
}

const getActivityColor = (type: string) => {
  const colors: Record<string, string> = {
    created: 'bg-blue-500',
    updated: 'bg-purple-500',
    assigned: 'bg-green-500',
    status_change: 'bg-amber-500',
    component_added: 'bg-cyan-500',
    component_removed: 'bg-red-500',
    software_installed: 'bg-indigo-500',
    software_removed: 'bg-pink-500',
    maintenance: 'bg-orange-500',
  }
  return colors[type] || 'bg-gray-500'
}

const getTypeIcon = (type: string) => {
  const icons: Record<string, string> = {
    desktop: 'ri:computer-line',
    laptop: 'ri:macbook-line',
    server: 'ri:server-line',
    workstation: 'ri:dashboard-line',
    thin_client: 'ri:tv-line',
  }
  return icons[type] || 'ri:computer-line'
}

const getTypeGradient = (type: string) => {
  const gradients: Record<string, string> = {
    desktop: 'bg-gradient-to-br from-blue-500 to-blue-700',
    laptop: 'bg-gradient-to-br from-purple-500 to-purple-700',
    server: 'bg-gradient-to-br from-red-500 to-red-700',
    workstation: 'bg-gradient-to-br from-green-500 to-green-700',
    thin_client: 'bg-gradient-to-br from-gray-500 to-gray-700',
  }
  return gradients[type] || 'bg-gradient-to-br from-gray-500 to-gray-700'
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
    active: 'Активный',
    inactive: 'Неактивный',
    repair: 'В ремонте',
    storage: 'На складе',
    decommissioned: 'Списан',
  }
  return labels[status] || status
}

const loadComputer = async () => {
  pending.value = true
  errorMessage.value = ''

  try {
    const loaded = await fetchComputer(computerId.value)
    computer.value = loaded

    const [componentsResult, softwareResult, activityResult, employeeResult, monitoringResult] =
      await Promise.allSettled([
        fetchComponents(loaded.id),
        fetchInstalledSoftware(loaded.id),
        fetchEntityHistory('computer', loaded.id),
        loaded.employee_id ? fetchEmployee(loaded.employee_id) : Promise.resolve(null),
        fetchLatestMonitoring(loaded.id),
      ])

    components.value =
      componentsResult.status === 'fulfilled' ? componentsResult.value.components : []
    software.value = softwareResult.status === 'fulfilled' ? softwareResult.value.software : []
    activityLog.value = activityResult.status === 'fulfilled' ? activityResult.value.logs : []
    employee.value = employeeResult.status === 'fulfilled' ? employeeResult.value : null
    monitoring.value = monitoringResult.status === 'fulfilled' ? monitoringResult.value : null
  } catch (error) {
    computer.value = null
    errorMessage.value = getErrorMessage(error)
  } finally {
    pending.value = false
  }
}

const updateStatus = async (value: string) => {
  if (!computer.value || computer.value.status === value) return

  try {
    const updated = await updateComputer(computer.value.id, { status: value })
    computer.value = updated
    toast.success('Статус компьютера обновлён')
    const history = await fetchEntityHistory('computer', updated.id)
    activityLog.value = history.logs
  } catch (error) {
    toast.error('Не удалось изменить статус: ' + getErrorMessage(error))
  }
}

const deleteComputer = async () => {
  if (!computer.value) return

  deleting.value = true
  try {
    await removeComputer(computer.value.id)
    toast.success('Компьютер удалён')
    router.push('/computers')
  } catch (error) {
    toast.error('Не удалось удалить компьютер: ' + getErrorMessage(error))
  } finally {
    deleting.value = false
    showDeleteModal.value = false
  }
}

const editComponent = (componentType: string, componentId: number) => {
  router.push(`/computers/${computerId.value}/components/${componentType}/${componentId}/edit`)
}

const showActivityDetails = (event: ActivityLog) => {
  selectedActivity.value = event
  showActivityModal.value = true
}

useHead({
  title: computed(() =>
    computer.value
      ? `${computer.value.hostname || `Компьютер #${computer.value.id}`} - Компьютеры`
      : 'Компьютер',
  ),
})

onMounted(loadComputer)
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
