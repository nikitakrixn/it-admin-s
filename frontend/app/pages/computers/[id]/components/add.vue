<template>
  <div class="max-w-3xl mx-auto space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4 mb-6">
        <NuxtLink
          :to="`/computers/${route.params.id}/components`"
          class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
        >
          <Icon name="ri:arrow-left-line" class="text-xl" />
        </NuxtLink>
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-green-500 to-green-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:add-circle-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Добавление компонента</h1>
            <p class="mt-1 text-gray-600">{{ computerName }}</p>
          </div>
        </div>
      </div>
    </div>

    <form class="space-y-6" @submit.prevent="handleSubmit">
      <div class="card">
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <h2 class="text-lg font-semibold text-gray-900">Тип компонента</h2>
        </div>
        <div class="p-6">
          <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
            <button
              v-for="type in componentTypes"
              :key="type.value"
              type="button"
              :class="[
                'p-4 rounded-xl border-2 transition-all',
                form.type === type.value
                  ? 'border-primary-500 bg-primary-50'
                  : 'border-gray-200 hover:border-gray-300',
              ]"
              @click="form.type = type.value"
            >
              <Icon
                :name="type.icon"
                :class="[
                  'text-3xl mb-2',
                  form.type === type.value ? 'text-primary-600' : 'text-gray-400',
                ]"
              />
              <p
                :class="[
                  'text-sm font-medium',
                  form.type === type.value ? 'text-primary-900' : 'text-gray-700',
                ]"
              >
                {{ type.label }}
              </p>
            </button>
          </div>
        </div>
      </div>

      <div v-if="form.type" class="card">
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <h2 class="text-lg font-semibold text-gray-900">Характеристики</h2>
        </div>
        <div class="p-6 space-y-4">
          <!-- Процессор -->
          <template v-if="form.type === 'processor'">
            <input
              v-model="form.name"
              type="text"
              placeholder="Название процессора *"
              class="input-field"
              required
            />
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
                placeholder="Частота"
                class="input-field"
              />
              <input v-model="form.specs.cache" type="text" placeholder="Кэш" class="input-field" />
            </div>
          </template>

          <!-- RAM -->
          <template v-if="form.type === 'ram'">
            <input
              v-model="form.name"
              type="text"
              placeholder="Производитель и модель *"
              class="input-field"
              required
            />
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
            </div>
          </template>

          <!-- Storage -->
          <template v-if="form.type === 'storage'">
            <input
              v-model="form.name"
              type="text"
              placeholder="Производитель и модель *"
              class="input-field"
              required
            />
            <div class="grid grid-cols-2 gap-4">
              <select v-model="form.specs.type" class="input-field" required>
                <option value="">Тип накопителя *</option>
                <option value="HDD">HDD</option>
                <option value="SSD">SSD</option>
                <option value="NVMe">NVMe</option>
                <option value="M.2">M.2</option>
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
          <template v-if="form.type === 'gpu'">
            <input
              v-model="form.name"
              type="text"
              placeholder="Название видеокарты *"
              class="input-field"
              required
            />
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
          <template v-if="form.type === 'motherboard'">
            <input
              v-model="form.name"
              type="text"
              placeholder="Производитель и модель *"
              class="input-field"
              required
            />
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
          <template v-if="form.type === 'network'">
            <input
              v-model="form.name"
              type="text"
              placeholder="Название адаптера *"
              class="input-field"
              required
            />
            <div class="grid grid-cols-2 gap-4">
              <select v-model="form.specs.adapter_type" class="input-field">
                <option value="">Тип адаптера</option>
                <option value="Ethernet">Ethernet</option>
                <option value="WiFi">WiFi</option>
                <option value="Bluetooth">Bluetooth</option>
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
            </div>
          </template>

          <!-- Monitor -->
          <template v-if="form.type === 'monitor'">
            <input
              v-model="form.name"
              type="text"
              placeholder="Производитель и модель *"
              class="input-field"
              required
            />
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
          <template v-if="form.type === 'peripheral'">
            <input
              v-model="form.name"
              type="text"
              placeholder="Название устройства *"
              class="input-field"
              required
            />
            <div class="grid grid-cols-2 gap-4">
              <select v-model="form.specs.device_type" class="input-field">
                <option value="">Тип устройства</option>
                <option value="keyboard">Клавиатура</option>
                <option value="mouse">Мышь</option>
                <option value="webcam">Веб-камера</option>
                <option value="headset">Гарнитура</option>
                <option value="ups">ИБП</option>
              </select>
              <select v-model="form.specs.connection" class="input-field">
                <option value="">Тип подключения</option>
                <option value="USB">USB</option>
                <option value="Bluetooth">Bluetooth</option>
                <option value="Wireless">Беспроводное</option>
              </select>
            </div>
          </template>

          <textarea
            v-model="form.notes"
            rows="3"
            placeholder="Примечания..."
            class="input-field resize-none"
          ></textarea>
        </div>
      </div>

      <div class="flex items-center justify-between gap-4">
        <NuxtLink :to="`/computers/${route.params.id}/components`" class="btn btn-secondary">
          <Icon name="ri:close-line" class="mr-2" />
          Отмена
        </NuxtLink>

        <button
          type="submit"
          class="btn btn-primary disabled:opacity-50"
          :disabled="saving || !form.type || !form.name"
        >
          <Icon v-if="saving" name="ri:loader-4-line" class="mr-2 animate-spin" />
          <Icon v-else name="ri:save-line" class="mr-2" />
          {{ saving ? 'Добавление...' : 'Добавить компонент' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import type { ComponentRequest } from '~/composables/useComputerComponents'

definePageMeta({
  middleware: 'auth',
})

const route = useRoute()
const router = useRouter()
const toast = useToast()
const { fetchComputer } = useComputers()
const { createComponent } = useComputerComponents()

const computerId = computed(() => Number(route.params.id))
const computerName = ref('')
const saving = ref(false)

const componentTypes = [
  { value: 'processor', label: 'Процессор', icon: 'ri:cpu-line' },
  { value: 'ram', label: 'RAM', icon: 'ri:database-2-line' },
  { value: 'storage', label: 'Накопитель', icon: 'ri:hard-drive-line' },
  { value: 'gpu', label: 'Видеокарта', icon: 'ri:gamepad-line' },
  { value: 'motherboard', label: 'Мат. плата', icon: 'ri:cpu-line' },
  { value: 'network', label: 'Сеть', icon: 'ri:wifi-line' },
  { value: 'monitor', label: 'Монитор', icon: 'ri:tv-line' },
  { value: 'peripheral', label: 'Периферия', icon: 'ri:keyboard-line' },
]

const form = ref({
  type: '',
  name: '',
  // Характеристики зависят от типа компонента: текст, числа и чекбокс
  specs: {} as Record<string, string | number | boolean>,
  notes: '',
})

const stringValue = (key: string) => {
  const value = form.value.specs[key]
  return typeof value === 'string' && value.trim() ? value.trim() : undefined
}

const buildRequest = (): ComponentRequest => {
  const request: ComponentRequest = {
    component_type: form.value.type,
    name: form.value.name.trim(),
    notes: form.value.notes.trim() || undefined,
  }

  switch (form.value.type) {
    case 'processor':
      request.cores = toOptionalNumber(form.value.specs.cores)
      request.threads = toOptionalNumber(form.value.specs.threads)
      request.base_frequency = stringValue('frequency')
      request.cache_size = stringValue('cache')
      break
    case 'ram':
      request.capacity_gb = toOptionalNumber(form.value.specs.capacity)
      request.memory_type = stringValue('type')
      request.frequency = stringValue('frequency')
      request.slot_number = toOptionalNumber(form.value.specs.slot)
      break
    case 'storage':
      request.storage_type = stringValue('type')
      request.capacity_gb = toOptionalNumber(form.value.specs.capacity)
      request.interface = stringValue('interface')
      request.serial_number = stringValue('serial')
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
      break
    case 'monitor':
      request.size_inches = toOptionalNumber(form.value.specs.size)
      request.resolution = stringValue('resolution')
      request.refresh_rate = toOptionalNumber(form.value.specs.refresh_rate)
      request.serial_number = stringValue('serial')
      break
    case 'peripheral':
      request.peripheral_type = stringValue('device_type')
      request.connection_type = stringValue('connection')
      break
  }

  return request
}

const handleSubmit = async () => {
  if (!form.value.type || !form.value.name.trim()) {
    toast.warning('Выберите тип и укажите название компонента')
    return
  }

  if ((form.value.type === 'ram' || form.value.type === 'storage') && !form.value.specs.capacity) {
    toast.warning('Укажите объем компонента')
    return
  }

  saving.value = true
  try {
    await createComponent(computerId.value, buildRequest())
    toast.success('Компонент добавлен')
    router.push(`/computers/${computerId.value}/components`)
  } catch (error) {
    toast.error('Не удалось добавить компонент: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

const loadComputer = async () => {
  try {
    const computer = await fetchComputer(computerId.value)
    computerName.value = computer.hostname || `Компьютер #${computer.id}`
  } catch (error) {
    toast.error('Не удалось загрузить компьютер: ' + getErrorMessage(error))
  }
}

useHead({
  title: computed(() => `Добавление компонента - ${computerName.value}`),
})

onMounted(loadComputer)
</script>
