<template>
  <div class="max-w-3xl mx-auto space-y-6">
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
            :to="`/computers/${route.params.id}/components/${componentType}/${componentId}`"
            class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
          >
            <Icon name="ri:arrow-left-line" class="text-xl" />
          </NuxtLink>
          <div class="flex items-center gap-4">
            <div
              class="h-16 w-16 rounded-2xl bg-gradient-to-br from-amber-500 to-amber-700 flex items-center justify-center shadow-lg"
            >
              <Icon name="ri:edit-line" class="text-3xl text-white" />
            </div>
            <div>
              <h1 class="text-3xl font-bold text-gray-900">Редактирование компонента</h1>
              <p class="mt-1 text-gray-600">{{ getComponentTypeLabel(componentType) }}</p>
            </div>
          </div>
        </div>
      </div>

      <form class="space-y-6" @submit.prevent="handleSubmit">
        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h2 class="text-lg font-semibold text-gray-900">Основное</h2>
          </div>
          <div class="p-6 space-y-4">
            <input
              v-model="form.name"
              type="text"
              placeholder="Название компонента *"
              class="input-field"
              required
            />
            <div class="grid grid-cols-2 gap-4">
              <input
                v-model="form.manufacturer"
                type="text"
                placeholder="Производитель"
                class="input-field"
              />
              <input v-model="form.model" type="text" placeholder="Модель" class="input-field" />
            </div>
          </div>
        </div>

        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h2 class="text-lg font-semibold text-gray-900">Характеристики</h2>
          </div>
          <div class="p-6 space-y-4">
            <!-- Процессор -->
            <template v-if="componentType === 'processor'">
              <div class="grid grid-cols-2 gap-4">
                <input
                  v-model="form.specs.cores"
                  type="number"
                  placeholder="Ядра"
                  class="input-field"
                />
                <input
                  v-model="form.specs.threads"
                  type="number"
                  placeholder="Потоки"
                  class="input-field"
                />
                <input
                  v-model="form.specs.frequency"
                  type="text"
                  placeholder="Базовая частота"
                  class="input-field"
                />
                <input
                  v-model="form.specs.max_frequency"
                  type="text"
                  placeholder="Макс. частота"
                  class="input-field"
                />
                <input
                  v-model="form.specs.cache"
                  type="text"
                  placeholder="Кэш"
                  class="input-field"
                />
                <input
                  v-model="form.specs.socket"
                  type="text"
                  placeholder="Сокет"
                  class="input-field"
                />
                <input v-model="form.specs.tdp" type="text" placeholder="TDP" class="input-field" />
              </div>
            </template>

            <!-- RAM -->
            <template v-if="componentType === 'ram'">
              <div class="grid grid-cols-2 gap-4">
                <input
                  v-model="form.specs.capacity"
                  type="number"
                  placeholder="Объем (ГБ) *"
                  class="input-field"
                  required
                />
                <select v-model="form.specs.type" class="input-field">
                  <option value="">Тип памяти</option>
                  <option value="DDR3">DDR3</option>
                  <option value="DDR4">DDR4</option>
                  <option value="DDR5">DDR5</option>
                </select>
                <input
                  v-model="form.specs.frequency"
                  type="text"
                  placeholder="Частота (МГц)"
                  class="input-field"
                />
                <input
                  v-model="form.specs.slot"
                  type="number"
                  placeholder="Номер слота"
                  class="input-field"
                />
                <input
                  v-model="form.specs.serial"
                  type="text"
                  placeholder="Серийный номер"
                  class="input-field"
                />
              </div>
            </template>

            <!-- Storage -->
            <template v-if="componentType === 'storage'">
              <div class="grid grid-cols-2 gap-4">
                <select v-model="form.specs.type" class="input-field" required>
                  <option value="">Тип накопителя *</option>
                  <option value="HDD">HDD</option>
                  <option value="SSD">SSD</option>
                  <option value="NVMe">NVMe</option>
                  <option value="M.2">M.2</option>
                  <option value="eMMC">eMMC</option>
                </select>
                <input
                  v-model="form.specs.capacity"
                  type="number"
                  placeholder="Объем (ГБ) *"
                  class="input-field"
                  required
                />
                <input
                  v-model="form.specs.interface"
                  type="text"
                  placeholder="Интерфейс"
                  class="input-field"
                />
                <input
                  v-model="form.specs.serial"
                  type="text"
                  placeholder="Серийный номер"
                  class="input-field"
                />
                <input
                  v-model="form.specs.mount_point"
                  type="text"
                  placeholder="Точка монтирования"
                  class="input-field"
                />
              </div>
              <div class="flex items-center gap-2">
                <input
                  id="is-system"
                  v-model="form.specs.is_system"
                  type="checkbox"
                  class="h-4 w-4 text-primary-600 rounded"
                />
                <label for="is-system" class="text-sm text-gray-700">Системный диск</label>
              </div>
            </template>

            <!-- GPU -->
            <template v-if="componentType === 'gpu'">
              <div class="grid grid-cols-2 gap-4">
                <input
                  v-model="form.specs.memory"
                  type="number"
                  placeholder="Память (ГБ)"
                  class="input-field"
                />
                <input
                  v-model="form.specs.memory_type"
                  type="text"
                  placeholder="Тип памяти"
                  class="input-field"
                />
                <input
                  v-model="form.specs.driver"
                  type="text"
                  placeholder="Версия драйвера"
                  class="input-field"
                />
              </div>
            </template>

            <!-- Motherboard -->
            <template v-if="componentType === 'motherboard'">
              <div class="grid grid-cols-2 gap-4">
                <input
                  v-model="form.specs.chipset"
                  type="text"
                  placeholder="Чипсет"
                  class="input-field"
                />
                <input
                  v-model="form.specs.bios"
                  type="text"
                  placeholder="Версия BIOS"
                  class="input-field"
                />
                <input
                  v-model="form.specs.form_factor"
                  type="text"
                  placeholder="Форм-фактор"
                  class="input-field"
                />
                <input
                  v-model="form.specs.serial"
                  type="text"
                  placeholder="Серийный номер"
                  class="input-field"
                />
              </div>
            </template>

            <!-- Network -->
            <template v-if="componentType === 'network'">
              <div class="grid grid-cols-2 gap-4">
                <select v-model="form.specs.adapter_type" class="input-field">
                  <option value="">Тип адаптера</option>
                  <option value="Ethernet">Ethernet</option>
                  <option value="WiFi">WiFi</option>
                  <option value="Bluetooth">Bluetooth</option>
                  <option value="Virtual">Virtual</option>
                </select>
                <input
                  v-model="form.specs.speed"
                  type="text"
                  placeholder="Скорость"
                  class="input-field"
                />
                <input
                  v-model="form.specs.mac"
                  type="text"
                  placeholder="MAC-адрес"
                  class="input-field"
                />
                <input
                  v-model="form.specs.ip"
                  type="text"
                  placeholder="IP-адрес"
                  class="input-field"
                />
              </div>
            </template>

            <!-- Monitor -->
            <template v-if="componentType === 'monitor'">
              <div class="grid grid-cols-2 gap-4">
                <input
                  v-model="form.specs.size"
                  type="number"
                  step="0.1"
                  placeholder="Размер (дюймы)"
                  class="input-field"
                />
                <input
                  v-model="form.specs.resolution"
                  type="text"
                  placeholder="Разрешение"
                  class="input-field"
                />
                <input
                  v-model="form.specs.panel_type"
                  type="text"
                  placeholder="Тип матрицы"
                  class="input-field"
                />
                <input
                  v-model="form.specs.refresh_rate"
                  type="number"
                  placeholder="Частота обновления (Гц)"
                  class="input-field"
                />
                <input
                  v-model="form.specs.serial"
                  type="text"
                  placeholder="Серийный номер"
                  class="input-field"
                />
              </div>
            </template>

            <!-- Peripheral -->
            <template v-if="componentType === 'peripheral'">
              <div class="grid grid-cols-2 gap-4">
                <select v-model="form.specs.device_type" class="input-field">
                  <option value="">Тип устройства</option>
                  <option value="keyboard">Клавиатура</option>
                  <option value="mouse">Мышь</option>
                  <option value="webcam">Веб-камера</option>
                  <option value="headset">Гарнитура</option>
                  <option value="microphone">Микрофон</option>
                  <option value="speaker">Колонки</option>
                  <option value="ups">ИБП</option>
                  <option value="docking_station">Док-станция</option>
                  <option value="other">Другое</option>
                </select>
                <select v-model="form.specs.connection" class="input-field">
                  <option value="">Подключение</option>
                  <option value="USB">USB</option>
                  <option value="Bluetooth">Bluetooth</option>
                  <option value="PS/2">PS/2</option>
                  <option value="Wireless">Wireless</option>
                  <option value="Wired">Wired</option>
                  <option value="Thunderbolt">Thunderbolt</option>
                  <option value="USB-C">USB-C</option>
                </select>
                <input
                  v-model="form.specs.serial"
                  type="text"
                  placeholder="Серийный номер"
                  class="input-field"
                />
              </div>
            </template>

            <textarea
              v-model="form.notes"
              rows="3"
              placeholder="Примечания"
              class="input-field resize-none"
            ></textarea>
          </div>
        </div>

        <div class="flex items-center justify-end gap-3">
          <NuxtLink
            :to="`/computers/${route.params.id}/components/${componentType}/${componentId}`"
            class="btn btn-secondary"
          >
            Отмена
          </NuxtLink>
          <button
            type="submit"
            class="btn btn-primary disabled:opacity-50"
            :disabled="saving || !form.name.trim()"
          >
            <Icon v-if="saving" name="ri:loader-4-line" class="mr-2 animate-spin" />
            <Icon v-else name="ri:save-line" class="mr-2" />
            {{ saving ? 'Сохранение...' : 'Сохранить' }}
          </button>
        </div>
      </form>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { ComponentRequest, ComputerComponent } from '~/composables/useComputerComponents'

definePageMeta({
  middleware: 'auth',
})

const route = useRoute()
const router = useRouter()
const toast = useToast()
const { fetchComponent, updateComponent } = useComputerComponents()

const computerId = computed(() => Number(route.params.id))
const componentType = computed(() => String(route.params.componentType))
const componentId = computed(() => Number(route.params.componentId))

const component = ref<ComputerComponent | null>(null)
const pending = ref(true)
const saving = ref(false)
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

const form = ref({
  name: '',
  manufacturer: '',
  model: '',
  notes: '',
  specs: {} as Record<string, string | number | boolean>,
})

// Ответ API отдаёт характеристики плоским списком «подпись — значение».
// Чтобы заполнить форму, раскладываем знакомые подписи обратно по полям.
const detailValue = (item: ComputerComponent, label: string) =>
  item.details.find((detail) => detail.label === label)?.value

const numericValue = (value: string | undefined) => {
  const parsed = Number(value?.replace(/[^\d.,-]/g, '').replace(',', '.'))
  return Number.isFinite(parsed) ? parsed : ''
}

const fillSpecs = (item: ComputerComponent) => {
  const specs: Record<string, string | number | boolean> = {}

  switch (item.component_type) {
    case 'processor':
      specs.cores = numericValue(detailValue(item, 'Ядра'))
      specs.threads = numericValue(detailValue(item, 'Потоки'))
      specs.frequency = detailValue(item, 'Базовая частота') || ''
      specs.max_frequency = detailValue(item, 'Макс. частота') || ''
      specs.cache = detailValue(item, 'Кэш') || ''
      specs.socket = detailValue(item, 'Сокет') || ''
      specs.tdp = detailValue(item, 'TDP') || ''
      break
    case 'ram':
      specs.capacity = numericValue(detailValue(item, 'Объем'))
      specs.type = detailValue(item, 'Тип') || ''
      specs.frequency = detailValue(item, 'Частота') || ''
      specs.slot = numericValue(detailValue(item, 'Слот'))
      specs.serial = detailValue(item, 'Серийный номер') || ''
      break
    case 'storage':
      specs.type = detailValue(item, 'Тип') || ''
      specs.capacity = numericValue(detailValue(item, 'Объем'))
      specs.interface = detailValue(item, 'Интерфейс') || ''
      specs.serial = detailValue(item, 'Серийный номер') || ''
      specs.mount_point = detailValue(item, 'Точка монтирования') || ''
      specs.is_system = item.is_primary
      break
    case 'gpu':
      specs.memory = numericValue(detailValue(item, 'Память'))
      specs.memory_type = detailValue(item, 'Тип памяти') || ''
      specs.driver = detailValue(item, 'Драйвер') || ''
      break
    case 'motherboard':
      specs.chipset = detailValue(item, 'Чипсет') || ''
      specs.bios = detailValue(item, 'BIOS') || ''
      specs.form_factor = detailValue(item, 'Форм-фактор') || ''
      specs.serial = detailValue(item, 'Серийный номер') || ''
      break
    case 'network':
      specs.adapter_type = detailValue(item, 'Тип') || ''
      specs.speed = detailValue(item, 'Скорость') || ''
      specs.mac = detailValue(item, 'MAC-адрес') || ''
      specs.ip = detailValue(item, 'IP-адрес') || ''
      break
    case 'monitor':
      specs.size = numericValue(detailValue(item, 'Размер'))
      specs.resolution = detailValue(item, 'Разрешение') || ''
      specs.panel_type = detailValue(item, 'Тип матрицы') || ''
      specs.refresh_rate = numericValue(detailValue(item, 'Частота обновления'))
      specs.serial = detailValue(item, 'Серийный номер') || ''
      break
    case 'peripheral':
      specs.device_type = detailValue(item, 'Тип устройства') || ''
      specs.connection = detailValue(item, 'Подключение') || ''
      specs.serial = detailValue(item, 'Серийный номер') || ''
      break
  }

  return specs
}

const loadComponent = async () => {
  pending.value = true
  errorMessage.value = ''

  try {
    const loaded = await fetchComponent(computerId.value, componentType.value, componentId.value)
    component.value = loaded
    form.value = {
      name: loaded.name,
      manufacturer: loaded.manufacturer || '',
      model: loaded.model || '',
      notes: loaded.notes || '',
      specs: fillSpecs(loaded),
    }
  } catch (error) {
    component.value = null
    errorMessage.value = getErrorMessage(error)
  } finally {
    pending.value = false
  }
}

const stringValue = (key: string) => {
  const value = form.value.specs[key]
  return typeof value === 'string' && value.trim() ? value.trim() : undefined
}

const buildRequest = (): ComponentRequest => {
  const request: ComponentRequest = {
    component_type: componentType.value,
    name: form.value.name.trim(),
    manufacturer: form.value.manufacturer.trim() || undefined,
    model: form.value.model.trim() || undefined,
    notes: form.value.notes.trim() || undefined,
  }

  switch (componentType.value) {
    case 'processor':
      request.cores = toOptionalNumber(form.value.specs.cores)
      request.threads = toOptionalNumber(form.value.specs.threads)
      request.base_frequency = stringValue('frequency')
      request.max_frequency = stringValue('max_frequency')
      request.cache_size = stringValue('cache')
      request.socket = stringValue('socket')
      request.tdp = stringValue('tdp')
      break
    case 'ram':
      request.capacity_gb = toOptionalNumber(form.value.specs.capacity)
      request.memory_type = stringValue('type')
      request.frequency = stringValue('frequency')
      request.slot_number = toOptionalNumber(form.value.specs.slot)
      request.serial_number = stringValue('serial')
      break
    case 'storage':
      request.storage_type = stringValue('type')
      request.capacity_gb = toOptionalNumber(form.value.specs.capacity)
      request.interface = stringValue('interface')
      request.serial_number = stringValue('serial')
      request.mount_point = stringValue('mount_point')
      request.is_system_drive = Boolean(form.value.specs.is_system)
      break
    case 'gpu':
      request.memory_gb = toOptionalNumber(form.value.specs.memory)
      request.gpu_memory_type = stringValue('memory_type')
      request.driver_version = stringValue('driver')
      break
    case 'motherboard':
      request.chipset = stringValue('chipset')
      request.bios_version = stringValue('bios')
      request.form_factor = stringValue('form_factor')
      request.serial_number = stringValue('serial')
      break
    case 'network':
      request.adapter_type = stringValue('adapter_type')
      request.speed = stringValue('speed')
      request.mac_address = stringValue('mac')
      request.ip_address = stringValue('ip')
      break
    case 'monitor':
      request.size_inches = toOptionalNumber(form.value.specs.size)
      request.resolution = stringValue('resolution')
      request.panel_type = stringValue('panel_type')
      request.refresh_rate = toOptionalNumber(form.value.specs.refresh_rate)
      request.serial_number = stringValue('serial')
      break
    case 'peripheral':
      request.peripheral_type = stringValue('device_type')
      request.connection_type = stringValue('connection')
      request.serial_number = stringValue('serial')
      break
  }

  return request
}

const handleSubmit = async () => {
  if (!form.value.name.trim()) {
    toast.warning('Укажите название компонента')
    return
  }

  if (
    (componentType.value === 'ram' || componentType.value === 'storage') &&
    !form.value.specs.capacity
  ) {
    toast.warning('Укажите объем компонента')
    return
  }

  saving.value = true
  try {
    await updateComponent(computerId.value, componentType.value, componentId.value, buildRequest())
    toast.success('Компонент обновлён')
    router.push(
      `/computers/${computerId.value}/components/${componentType.value}/${componentId.value}`,
    )
  } catch (error) {
    toast.error('Не удалось обновить компонент: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

useHead({
  title: computed(
    () => `Редактирование компонента - ${getComponentTypeLabel(componentType.value)}`,
  ),
})

onMounted(loadComponent)
</script>
