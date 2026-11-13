<template>
  <div class="max-w-4xl mx-auto space-y-6">
    <div v-if="pending" class="flex justify-center py-16">
      <Icon name="ri:loader-4-line" class="text-3xl text-primary-500 animate-spin" />
    </div>

    <div v-else-if="!component" class="card p-16 text-center">
      <Icon name="ri:error-warning-line" class="text-5xl text-red-400 mb-4" />
      <h2 class="text-xl font-semibold text-gray-900 mb-2">Компонент не найден</h2>
      <p class="text-sm text-gray-500 mb-6">{{ errorMessage }}</p>
      <NuxtLink :to="`/computers/${route.params.id}/components`" class="btn btn-primary">
        <Icon name="ri:arrow-left-line" class="mr-2" />
        Вернуться к компонентам
      </NuxtLink>
    </div>

    <template v-else>
      <div class="mb-8">
        <div class="flex items-center gap-4 mb-6">
          <NuxtLink
            :to="`/computers/${route.params.id}/components`"
            class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
          >
            <Icon name="ri:arrow-left-line" class="text-xl" />
          </NuxtLink>
          <div class="flex-1 flex items-center justify-between">
            <div class="flex items-center gap-4">
              <div
                :class="[
                  'h-16 w-16 rounded-2xl flex items-center justify-center shadow-lg',
                  getComponentGradient(component.component_type),
                ]"
              >
                <Icon
                  :name="getComponentIcon(component.component_type)"
                  class="text-3xl text-white"
                />
              </div>
              <div>
                <h1 class="text-3xl font-bold text-gray-900">
                  {{ component.name }}
                </h1>
                <p class="mt-1 text-gray-600">
                  {{ getComponentTypeLabel(component.component_type) }}
                </p>
              </div>
            </div>

            <div class="flex items-center gap-2">
              <NuxtLink
                :to="`/computers/${route.params.id}/components/${component.component_type}/${component.id}/edit`"
                class="btn btn-secondary"
              >
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
              <h2 class="text-lg font-semibold text-gray-900">Характеристики</h2>
            </div>
            <div class="p-6">
              <div v-if="component.details.length" class="grid grid-cols-2 gap-6">
                <div v-for="item in component.details" :key="item.label">
                  <p class="text-xs text-gray-600 mb-1">
                    {{ item.label }}
                  </p>
                  <p class="text-sm font-medium text-gray-900">
                    {{ item.value }}
                  </p>
                </div>
              </div>
              <p v-else class="text-sm text-gray-500">Характеристики не заполнены</p>
            </div>
          </div>

          <div v-if="component.notes" class="card">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <h2 class="text-lg font-semibold text-gray-900">Примечания</h2>
            </div>
            <div class="p-6">
              <p class="text-sm text-gray-700">
                {{ component.notes }}
              </p>
            </div>
          </div>

          <div class="card">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <h2 class="text-lg font-semibold text-gray-900">История</h2>
            </div>
            <div class="p-6">
              <div v-if="history.length" class="space-y-4">
                <div v-for="event in history" :key="event.id" class="flex gap-4">
                  <div class="flex flex-col items-center">
                    <div
                      :class="[
                        'h-8 w-8 rounded-full flex items-center justify-center',
                        getActivityColor(event.action),
                      ]"
                    >
                      <Icon :name="getActivityIcon(event.action)" class="text-sm text-white" />
                    </div>
                  </div>
                  <div class="flex-1">
                    <p class="text-sm text-gray-900">
                      {{ getActivityDescription(event) }}
                    </p>
                    <p v-if="getChangesSummary(event)" class="text-xs text-gray-600 mt-1">
                      {{ getChangesSummary(event) }}
                    </p>
                    <p class="text-xs text-gray-500 mt-1">
                      {{ formatDateTime(event.created_at) }} · {{ event.user_email || 'Система' }}
                    </p>
                  </div>
                </div>
              </div>
              <p v-else class="text-sm text-gray-500">Событий по компоненту пока нет</p>
            </div>
          </div>
        </div>

        <div class="space-y-6">
          <div class="card">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
              <h3 class="text-sm font-semibold text-gray-900">Информация</h3>
            </div>
            <div class="p-6 space-y-4">
              <div>
                <p class="text-xs text-gray-600 mb-1">Компьютер</p>
                <NuxtLink
                  :to="`/computers/${route.params.id}`"
                  class="text-sm font-medium text-primary-600 hover:text-primary-700"
                >
                  {{ computerName }}
                </NuxtLink>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Производитель</p>
                <p class="text-sm font-medium text-gray-900">
                  {{ component.manufacturer || '—' }}
                </p>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Модель</p>
                <p class="text-sm font-medium text-gray-900">{{ component.model || '—' }}</p>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Добавлен</p>
                <p class="text-sm font-medium text-gray-900">
                  {{ formatDateTime(component.created_at) }}
                </p>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Обновлен</p>
                <p class="text-sm font-medium text-gray-900">
                  {{ formatDateTime(component.updated_at) }}
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

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
                  <h3 class="text-lg font-semibold text-gray-900 mb-2">Удалить компонент?</h3>
                  <p class="text-sm text-gray-600 mb-6">Это действие нельзя отменить.</p>
                  <div class="flex gap-3">
                    <button class="flex-1 btn btn-secondary" @click="showDeleteModal = false">
                      Отмена
                    </button>
                    <button
                      class="flex-1 btn bg-red-600 text-white hover:bg-red-700 disabled:opacity-50"
                      :disabled="deleting"
                      @click="deleteComponent"
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
    </template>
  </div>
</template>

<script setup lang="ts">
import type { ActivityLog } from '~/composables/useActivityLog'
import type { ComputerComponent } from '~/composables/useComputerComponents'

definePageMeta({
  middleware: 'auth',
})

const route = useRoute()
const router = useRouter()
const toast = useToast()
const { fetchComputer } = useComputers()
const { fetchComponent, deleteComponent: removeComponent } = useComputerComponents()
const { fetchEntityHistory } = useActivityLog()

const computerId = computed(() => Number(route.params.id))
const componentType = computed(() => String(route.params.componentType))
const componentId = computed(() => Number(route.params.componentId))

const component = ref<ComputerComponent | null>(null)
const computerName = ref('')
const history = ref<ActivityLog[]>([])
const pending = ref(true)
const deleting = ref(false)
const showDeleteModal = ref(false)
const errorMessage = ref('')

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

const getComponentTypeLabel = (type: string) => componentTypeLabels[type] || type

const loadData = async () => {
  pending.value = true
  errorMessage.value = ''

  try {
    const [loadedComponent, computer, activity] = await Promise.all([
      fetchComponent(computerId.value, componentType.value, componentId.value),
      fetchComputer(computerId.value),
      fetchEntityHistory('computer', computerId.value),
    ])

    component.value = loadedComponent
    computerName.value = computer.hostname || `Компьютер #${computer.id}`
    history.value = activity.logs.filter((event) => {
      const details = event.details as Record<string, unknown> | null
      return (
        details?.component_id === componentId.value &&
        details?.component_type === componentType.value
      )
    })
  } catch (error) {
    component.value = null
    errorMessage.value = getErrorMessage(error)
  } finally {
    pending.value = false
  }
}

const deleteComponent = async () => {
  deleting.value = true
  try {
    await removeComponent(computerId.value, componentType.value, componentId.value)
    toast.success('Компонент удалён')
    router.push(`/computers/${computerId.value}/components`)
  } catch (error) {
    toast.error('Не удалось удалить компонент: ' + getErrorMessage(error))
  } finally {
    deleting.value = false
    showDeleteModal.value = false
  }
}

const formatDateTime = (value: string) => {
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString('ru-RU')
}

const getActivityDescription = (event: ActivityLog) => {
  const descriptions: Record<string, string> = {
    component_added: 'Компонент добавлен',
    updated: 'Обновлена информация',
    component_removed: 'Компонент удалён',
  }
  return descriptions[event.action] || event.action
}

const getChangesSummary = (event: ActivityLog) => {
  const changes = event.details?.changes
  if (!changes || typeof changes !== 'object') return ''
  return Object.values(changes)
    .map((change) => {
      const c = change as { old: unknown; new: unknown }
      return `${c.old ?? '—'} → ${c.new ?? '—'}`
    })
    .join(', ')
}

const getActivityIcon = (action: string) => {
  const icons: Record<string, string> = {
    component_added: 'ri:add-line',
    updated: 'ri:edit-line',
    component_removed: 'ri:delete-bin-line',
  }
  return icons[action] || 'ri:record-circle-line'
}

const getActivityColor = (action: string) => {
  const colors: Record<string, string> = {
    component_added: 'bg-green-500',
    updated: 'bg-blue-500',
    component_removed: 'bg-red-500',
  }
  return colors[action] || 'bg-gray-500'
}

const getComponentIcon = (type: string) => {
  const icons: Record<string, string> = {
    processor: 'ri:cpu-line',
    ram: 'ri:database-2-line',
    storage: 'ri:hard-drive-line',
    gpu: 'ri:gamepad-line',
    motherboard: 'ri:cpu-line',
    network: 'ri:wifi-line',
    monitor: 'ri:tv-line',
    peripheral: 'ri:keyboard-line',
  }
  return icons[type] || 'ri:cpu-line'
}

const getComponentGradient = (type: string) => {
  const gradients: Record<string, string> = {
    processor: 'bg-gradient-to-br from-blue-500 to-blue-700',
    ram: 'bg-gradient-to-br from-purple-500 to-purple-700',
    storage: 'bg-gradient-to-br from-green-500 to-green-700',
    gpu: 'bg-gradient-to-br from-amber-500 to-amber-700',
    motherboard: 'bg-gradient-to-br from-indigo-500 to-indigo-700',
    network: 'bg-gradient-to-br from-cyan-500 to-cyan-700',
    monitor: 'bg-gradient-to-br from-pink-500 to-pink-700',
    peripheral: 'bg-gradient-to-br from-teal-500 to-teal-700',
  }
  return gradients[type] || 'bg-gradient-to-br from-gray-500 to-gray-700'
}

useHead({
  title: computed(() => `${component.value?.name || 'Компонент'} - Компоненты`),
})

onMounted(loadData)
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
