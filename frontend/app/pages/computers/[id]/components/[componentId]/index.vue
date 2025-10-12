<template>
  <div class="max-w-4xl mx-auto space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4 mb-6">
        <NuxtLink :to="`/computers/${route.params.id}/components`" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
          <Icon name="ri:arrow-left-line" class="text-xl" />
        </NuxtLink>
        <div class="flex-1 flex items-center justify-between">
          <div class="flex items-center gap-4">
            <div :class="['h-16 w-16 rounded-2xl flex items-center justify-center shadow-lg', getComponentGradient(component.type)]">
              <Icon :name="getComponentIcon(component.type)" class="text-3xl text-white" />
            </div>
            <div>
              <h1 class="text-3xl font-bold text-gray-900">{{ component.name }}</h1>
              <p class="mt-1 text-gray-600">{{ getComponentTypeLabel(component.type) }}</p>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <NuxtLink :to="`/computers/${route.params.id}/components/${route.params.componentId}/edit`" class="btn btn-secondary">
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
      <div class="lg:col-span-2 space-y-6">
        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h2 class="text-lg font-semibold text-gray-900">Характеристики</h2>
          </div>
          <div class="p-6">
            <div class="grid grid-cols-2 gap-6">
              <div v-for="(value, key) in component.specs" :key="key">
                <p class="text-xs text-gray-600 mb-1">{{ key }}</p>
                <p class="text-sm font-medium text-gray-900">{{ value }}</p>
              </div>
            </div>
          </div>
        </div>

        <div v-if="component.notes" class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h2 class="text-lg font-semibold text-gray-900">Примечания</h2>
          </div>
          <div class="p-6">
            <p class="text-sm text-gray-700">{{ component.notes }}</p>
          </div>
        </div>

        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h2 class="text-lg font-semibold text-gray-900">История</h2>
          </div>
          <div class="p-6">
            <div class="space-y-4">
              <div v-for="event in history" :key="event.id" class="flex gap-4">
                <div class="flex flex-col items-center">
                  <div class="h-8 w-8 rounded-full bg-blue-500 flex items-center justify-center">
                    <Icon :name="event.icon" class="text-sm text-white" />
                  </div>
                </div>
                <div class="flex-1">
                  <p class="text-sm text-gray-900">{{ event.action }}</p>
                  <p class="text-xs text-gray-500 mt-1">{{ event.created_at }}</p>
                </div>
              </div>
            </div>
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
              <NuxtLink :to="`/computers/${route.params.id}`" class="text-sm font-medium text-primary-600 hover:text-primary-700">
                {{ computerName }}
              </NuxtLink>
            </div>
            <div>
              <p class="text-xs text-gray-600 mb-1">Добавлен</p>
              <p class="text-sm font-medium text-gray-900">{{ component.created_at }}</p>
            </div>
            <div>
              <p class="text-xs text-gray-600 mb-1">Обновлен</p>
              <p class="text-sm font-medium text-gray-900">{{ component.updated_at }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

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
                <h3 class="text-lg font-semibold text-gray-900 mb-2">Удалить компонент?</h3>
                <p class="text-sm text-gray-600 mb-6">Это действие нельзя отменить.</p>
                <div class="flex gap-3">
                  <button @click="showDeleteModal = false" class="flex-1 btn btn-secondary">Отмена</button>
                  <button @click="deleteComponent" class="flex-1 btn bg-red-600 text-white hover:bg-red-700">Удалить</button>
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
  middleware: 'auth'
})

const route = useRoute()
const router = useRouter()
const toast = useToast()

const showDeleteModal = ref(false)
const computerName = ref('PC-ADMIN-001')

const component = ref({
  id: route.params.componentId,
  type: 'processor',
  name: 'Intel Core i7-10700',
  specs: {
    'Ядра': '8',
    'Потоки': '16',
    'Частота': '2.90 GHz',
    'Кэш': '16 МБ',
    'Сокет': 'LGA1200'
  },
  notes: null,
  created_at: '15.01.2024 10:00',
  updated_at: '10.12.2025 14:30'
})

const history = ref([
  { id: 1, action: 'Компонент добавлен', icon: 'ri:add-line', created_at: '15.01.2024 10:00' },
  { id: 2, action: 'Обновлена информация', icon: 'ri:edit-line', created_at: '10.12.2025 14:30' }
])

useHead({
  title: `${component.value.name} - Компоненты`
})

const deleteComponent = () => {
  toast.success('Компонент удален')
  router.push(`/computers/${route.params.id}/components`)
}

const getComponentIcon = (type: string) => {
  const icons: Record<string, string> = {
    processor: 'ri:cpu-line',
    ram: 'ri:database-2-line',
    storage: 'ri:hard-drive-line',
    gpu: 'ri:gamepad-line',
    motherboard: 'ri:circuit-board-line',
    network: 'ri:wifi-line',
    monitor: 'ri:monitor-line',
    peripheral: 'ri:keyboard-line'
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
    peripheral: 'bg-gradient-to-br from-teal-500 to-teal-700'
  }
  return gradients[type] || 'bg-gradient-to-br from-gray-500 to-gray-700'
}

const getComponentTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    processor: 'Процессор',
    ram: 'Оперативная память',
    storage: 'Накопитель',
    gpu: 'Видеокарта',
    motherboard: 'Материнская плата',
    network: 'Сетевой адаптер',
    monitor: 'Монитор',
    peripheral: 'Периферия'
  }
  return labels[type] || type
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
