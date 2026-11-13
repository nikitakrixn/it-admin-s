<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4 mb-6">
        <NuxtLink
          :to="`/computers/${route.params.id}`"
          class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
        >
          <Icon name="ri:arrow-left-line" class="text-xl" />
        </NuxtLink>
        <div class="flex-1 flex items-center justify-between">
          <div class="flex items-center gap-4">
            <div
              class="h-16 w-16 rounded-2xl bg-gradient-to-br from-purple-500 to-purple-700 flex items-center justify-center shadow-lg"
            >
              <Icon name="ri:cpu-line" class="text-3xl text-white" />
            </div>
            <div>
              <h1 class="text-3xl font-bold text-gray-900">Компоненты</h1>
              <p class="mt-1 text-gray-600">{{ computerName }}</p>
            </div>
          </div>

          <NuxtLink :to="`/computers/${route.params.id}/components/add`" class="btn btn-primary">
            <Icon name="ri:add-line" class="mr-2" />
            Добавить компонент
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Фильтры -->
    <div class="card">
      <div class="p-4">
        <div class="flex items-center gap-4">
          <div class="flex-1 relative">
            <Icon
              name="ri:search-line"
              class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
            />
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Поиск компонентов..."
              class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            />
          </div>

          <select v-model="selectedType" class="input-field w-48">
            <option value="">Все типы</option>
            <option v-for="type in componentTypes" :key="type.value" :value="type.value">
              {{ type.label }}
            </option>
          </select>
        </div>
      </div>
    </div>

    <div v-if="pending" class="flex justify-center py-16">
      <Icon name="ri:loader-4-line" class="text-3xl text-primary-500 animate-spin" />
    </div>

    <!-- Список компонентов -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div
        v-for="component in filteredComponents"
        :key="`${component.component_type}-${component.id}`"
        class="card card-hover group cursor-pointer"
        @click="viewComponent(component)"
      >
        <div class="p-5">
          <div class="flex items-start justify-between mb-4">
            <div class="flex items-center gap-3">
              <div
                :class="[
                  'h-12 w-12 rounded-xl flex items-center justify-center',
                  getComponentColor(component.component_type),
                ]"
              >
                <Icon
                  :name="getComponentIcon(component.component_type)"
                  class="text-2xl text-white"
                />
              </div>
              <div>
                <p
                  class="text-sm font-semibold text-gray-900 group-hover:text-primary-600 transition-colors"
                >
                  {{ getComponentTypeLabel(component.component_type) }}
                </p>
                <p class="text-xs text-gray-500">
                  {{ component.name }}
                </p>
              </div>
            </div>
            <div class="flex items-center gap-2">
              <button
                class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                title="Редактировать"
                @click.stop="editComponent(component)"
              >
                <Icon name="ri:edit-line" class="text-gray-600" />
              </button>
              <button
                class="p-2 rounded-lg hover:bg-red-50 transition-colors"
                title="Удалить"
                @click.stop="deleteComponent(component)"
              >
                <Icon name="ri:delete-bin-line" class="text-red-600" />
              </button>
            </div>
          </div>

          <div class="space-y-2">
            <div
              v-for="item in component.details"
              :key="item.label"
              class="flex items-center justify-between text-sm"
            >
              <span class="text-gray-600">{{ item.label }}</span>
              <span class="font-medium text-gray-900">{{ item.value }}</span>
            </div>
          </div>

          <div v-if="component.notes" class="mt-3 pt-3 border-t border-gray-200">
            <p class="text-xs text-gray-600">
              {{ component.notes }}
            </p>
          </div>
        </div>
      </div>

      <div v-if="filteredComponents.length === 0" class="col-span-full p-16 text-center">
        <div
          class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mx-auto mb-4"
        >
          <Icon name="ri:cpu-line" class="text-4xl text-gray-400" />
        </div>
        <h3 class="text-lg font-semibold text-gray-900 mb-2">Компоненты не найдены</h3>
        <p class="text-sm text-gray-500 mb-4">
          {{ components.length ? 'Измените фильтры поиска' : 'Добавьте первый компонент' }}
        </p>
        <NuxtLink :to="`/computers/${route.params.id}/components/add`" class="btn btn-primary">
          <Icon name="ri:add-line" class="mr-2" />
          Добавить компонент
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ComputerComponent } from '~/composables/useComputerComponents'

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
const { fetchComputer } = useComputers()
const { fetchComponents, deleteComponent: removeComponent } = useComputerComponents()

const computerId = computed(() => Number(route.params.id))
const computerName = ref('')
const searchQuery = ref('')
const selectedType = ref('')
const components = ref<ComputerComponent[]>([])
const pending = ref(true)

const componentTypes = [
  { value: 'processor', label: 'Процессор' },
  { value: 'ram', label: 'Оперативная память' },
  { value: 'storage', label: 'Накопитель' },
  { value: 'gpu', label: 'Видеокарта' },
  { value: 'motherboard', label: 'Материнская плата' },
  { value: 'network', label: 'Сетевой адаптер' },
  { value: 'monitor', label: 'Монитор' },
  { value: 'peripheral', label: 'Периферия' },
]

const filteredComponents = computed(() => {
  let filtered = components.value

  if (selectedType.value) {
    filtered = filtered.filter((component) => component.component_type === selectedType.value)
  }

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(
      (component) =>
        component.name.toLowerCase().includes(query) ||
        (component.manufacturer || '').toLowerCase().includes(query) ||
        (component.model || '').toLowerCase().includes(query) ||
        getComponentTypeLabel(component.component_type).toLowerCase().includes(query),
    )
  }

  return filtered
})

const loadData = async () => {
  pending.value = true
  try {
    const [computer, componentsResult] = await Promise.all([
      fetchComputer(computerId.value),
      fetchComponents(computerId.value),
    ])
    computerName.value = computer.hostname || `Компьютер #${computer.id}`
    components.value = componentsResult.components
  } catch (error) {
    toast.error('Не удалось загрузить компоненты: ' + getErrorMessage(error))
  } finally {
    pending.value = false
  }
}

const viewComponent = (component: ComputerComponent) => {
  router.push(
    `/computers/${computerId.value}/components/${component.component_type}/${component.id}`,
  )
}

const editComponent = (component: ComputerComponent) => {
  router.push(
    `/computers/${computerId.value}/components/${component.component_type}/${component.id}/edit`,
  )
}

const deleteComponent = async (component: ComputerComponent) => {
  if (!confirm(`Удалить компонент "${component.name}"?`)) return

  try {
    await removeComponent(computerId.value, component.component_type, component.id)
    components.value = components.value.filter(
      (item) => !(item.id === component.id && item.component_type === component.component_type),
    )
    toast.success('Компонент удалён')
  } catch (error) {
    toast.error('Не удалось удалить компонент: ' + getErrorMessage(error))
  }
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

const getComponentColor = (type: string) => {
  const colors: Record<string, string> = {
    processor: 'bg-blue-500',
    ram: 'bg-purple-500',
    storage: 'bg-green-500',
    gpu: 'bg-amber-500',
    motherboard: 'bg-indigo-500',
    network: 'bg-cyan-500',
    monitor: 'bg-pink-500',
    peripheral: 'bg-teal-500',
  }
  return colors[type] || 'bg-gray-500'
}

const getComponentTypeLabel = (type: string) => {
  return componentTypes.find((item) => item.value === type)?.label || type
}

useHead({
  title: computed(() => `Компоненты - ${computerName.value}`),
})

onMounted(loadData)
</script>
