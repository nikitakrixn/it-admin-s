<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-purple-500 to-purple-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:router-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Сетевое оборудование</h1>
            <p class="mt-1 text-gray-600">Коммутаторы, маршрутизаторы и точки доступа</p>
          </div>
        </div>
        <button class="btn btn-primary" @click="openCreateModal">
          <Icon name="ri:add-line" class="mr-2" />
          Добавить устройство
        </button>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">Всего устройств</p>
            <p class="text-2xl font-bold text-purple-600">{{ devices.length }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center">
            <Icon name="ri:hard-drive-2-line" class="text-2xl text-purple-600" />
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">Активные</p>
            <p class="text-2xl font-bold text-green-600">{{ activeCount }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center">
            <Icon name="ri:checkbox-circle-line" class="text-2xl text-green-600" />
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">Обслуживание</p>
            <p class="text-2xl font-bold text-amber-600">{{ maintenanceCount }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-amber-100 flex items-center justify-center">
            <Icon name="ri:tools-line" class="text-2xl text-amber-600" />
          </div>
        </div>
      </div>
    </div>

    <div class="card p-4">
      <div class="flex gap-4">
        <div class="flex-1 relative">
          <Icon
            name="ri:search-line"
            class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
          />
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Поиск по названию, IP или производителю..."
            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          />
        </div>
        <select v-model="filterType" class="input-field w-56">
          <option value="">Все типы</option>
          <option value="switch">Коммутаторы</option>
          <option value="router">Маршрутизаторы</option>
          <option value="firewall">Межсетевые экраны</option>
          <option value="access_point">Точки доступа</option>
          <option value="load_balancer">Балансировщики</option>
          <option value="vpn_gateway">VPN-шлюзы</option>
        </select>
      </div>
    </div>

    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-purple-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка устройств...</p>
    </div>

    <div v-else-if="filteredDevices.length === 0" class="card p-16 text-center">
      <Icon name="ri:inbox-line" class="text-4xl text-gray-300 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Устройства не найдены</h3>
      <p class="text-sm text-gray-500 mb-4">
        {{
          devices.length === 0 ? 'Добавьте первое сетевое устройство' : 'Измените параметры поиска'
        }}
      </p>
      <button v-if="devices.length === 0" class="btn btn-primary" @click="openCreateModal">
        <Icon name="ri:add-line" class="mr-2" />
        Добавить устройство
      </button>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <DeviceCard
        v-for="device in filteredDevices"
        :key="device.id"
        :device="device"
        @edit="openEditModal"
        @delete="deleteDevice"
      />
    </div>

    <Teleport to="body">
      <div
        v-if="showModal"
        class="fixed inset-0 z-50 flex items-center justify-center bg-gray-900/50 p-4"
        @click.self="closeModal"
      >
        <div class="w-full max-w-2xl rounded-xl bg-white shadow-2xl">
          <div class="flex items-center justify-between border-b border-gray-200 px-6 py-4">
            <h2 class="text-lg font-semibold text-gray-900">
              {{ isEditMode ? 'Редактировать устройство' : 'Новое устройство' }}
            </h2>
            <button class="p-2 rounded-lg hover:bg-gray-100" @click="closeModal">
              <Icon name="ri:close-line" class="text-xl" />
            </button>
          </div>

          <form class="p-6 space-y-4" @submit.prevent="handleSubmit">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Тип *</label>
                <select v-model="form.device_type" class="input-field" required>
                  <option value="switch">Коммутатор</option>
                  <option value="router">Маршрутизатор</option>
                  <option value="firewall">Межсетевой экран</option>
                  <option value="access_point">Точка доступа</option>
                  <option value="load_balancer">Балансировщик</option>
                  <option value="vpn_gateway">VPN-шлюз</option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Название *</label>
                <input v-model="form.name" type="text" class="input-field" required />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Производитель</label>
                <input v-model="form.manufacturer" type="text" class="input-field" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Модель</label>
                <input v-model="form.model" type="text" class="input-field" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">IP-адрес</label>
                <input v-model="form.ip_address" type="text" class="input-field font-mono" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">MAC-адрес</label>
                <input v-model="form.mac_address" type="text" class="input-field font-mono" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Серийный номер</label>
                <input v-model="form.serial_number" type="text" class="input-field" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Версия прошивки</label>
                <input v-model="form.firmware_version" type="text" class="input-field" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1"
                  >Количество портов</label
                >
                <input v-model.number="form.port_count" type="number" min="1" class="input-field" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Статус</label>
                <select v-model="form.status" class="input-field">
                  <option value="active">Активно</option>
                  <option value="inactive">Неактивно</option>
                  <option value="maintenance">Обслуживание</option>
                  <option value="failed">Сбой</option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Локация</label>
                <input v-model="form.location" type="text" class="input-field" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Адрес управления</label>
                <input
                  v-model="form.management_url"
                  type="url"
                  class="input-field"
                  placeholder="https://192.168.1.2"
                />
              </div>
              <div class="md:col-span-2">
                <label class="block text-sm font-medium text-gray-700 mb-1">Заметки</label>
                <textarea v-model="form.notes" rows="3" class="input-field"></textarea>
              </div>
            </div>

            <div class="flex justify-end gap-3 border-t border-gray-200 pt-4">
              <button type="button" class="btn btn-secondary" @click="closeModal">Отмена</button>
              <button type="submit" class="btn btn-primary" :disabled="saving">
                <Icon v-if="saving" name="ri:loader-4-line" class="mr-2 animate-spin" />
                {{ saving ? 'Сохранение...' : 'Сохранить' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useNetwork } from '~/composables/useNetwork'
import type { NetworkDevice, NewNetworkDeviceRequest } from '~/composables/useNetwork'
import DeviceCard from '~/components/network/DeviceCard.vue'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

useHead({
  title: 'Сетевое оборудование',
})

const toast = useToast()
const { fetchNetworkDevices, createNetworkDevice, updateNetworkDevice, deleteNetworkDevice } =
  useNetwork()

interface DeviceForm {
  device_type: string
  name: string
  manufacturer: string
  model: string
  serial_number: string
  ip_address: string
  mac_address: string
  management_url: string
  location: string
  firmware_version: string
  port_count: number | null
  status: string
  notes: string
}

const emptyForm = (): DeviceForm => ({
  device_type: 'switch',
  name: '',
  manufacturer: '',
  model: '',
  serial_number: '',
  ip_address: '',
  mac_address: '',
  management_url: '',
  location: '',
  firmware_version: '',
  port_count: null,
  status: 'active',
  notes: '',
})

const devices = ref<NetworkDevice[]>([])
const loading = ref(false)
const showModal = ref(false)
const isEditMode = ref(false)
const editingId = ref<number | null>(null)
const saving = ref(false)
const searchQuery = ref('')
const filterType = ref('')
const form = ref<DeviceForm>(emptyForm())

const activeCount = computed(
  () => devices.value.filter((device) => device.status === 'active').length,
)
const maintenanceCount = computed(
  () => devices.value.filter((device) => device.status === 'maintenance').length,
)

const filteredDevices = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()

  return devices.value.filter((device) => {
    if (filterType.value && device.device_type !== filterType.value) return false
    if (!query) return true

    return [device.name, device.ip_address, device.manufacturer, device.model, device.location]
      .filter((value): value is string => Boolean(value))
      .some((value) => value.toLowerCase().includes(query))
  })
})

const fetchDevices = async () => {
  loading.value = true
  try {
    const response = await fetchNetworkDevices({ per_page: 100 })
    devices.value = response.devices
  } catch (error) {
    toast.error('Ошибка загрузки устройств: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const openCreateModal = () => {
  isEditMode.value = false
  editingId.value = null
  form.value = emptyForm()
  showModal.value = true
}

const openEditModal = (device: NetworkDevice) => {
  isEditMode.value = true
  editingId.value = device.id
  form.value = {
    device_type: device.device_type,
    name: device.name,
    manufacturer: device.manufacturer ?? '',
    model: device.model ?? '',
    serial_number: device.serial_number ?? '',
    ip_address: device.ip_address ?? '',
    mac_address: device.mac_address ?? '',
    management_url: device.management_url ?? '',
    location: device.location ?? '',
    firmware_version: device.firmware_version ?? '',
    port_count: device.port_count,
    status: device.status ?? 'active',
    notes: device.notes ?? '',
  }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  editingId.value = null
  form.value = emptyForm()
}

const toRequest = (value: DeviceForm): NewNetworkDeviceRequest => ({
  device_type: value.device_type,
  name: value.name.trim(),
  manufacturer: value.manufacturer.trim() || undefined,
  model: value.model.trim() || undefined,
  serial_number: value.serial_number.trim() || undefined,
  ip_address: value.ip_address.trim() || undefined,
  mac_address: value.mac_address.trim() || undefined,
  management_url: value.management_url.trim() || undefined,
  location: value.location.trim() || undefined,
  firmware_version: value.firmware_version.trim() || undefined,
  port_count: toOptionalNumber(value.port_count),
  status: value.status || undefined,
  notes: value.notes.trim() || undefined,
})

const handleSubmit = async () => {
  if (!form.value.name.trim()) {
    toast.warning('Укажите название устройства')
    return
  }

  saving.value = true
  try {
    const payload = toRequest(form.value)
    if (isEditMode.value && editingId.value !== null) {
      await updateNetworkDevice(editingId.value, payload)
      toast.success('Устройство обновлено')
    } else {
      await createNetworkDevice(payload)
      toast.success('Устройство создано')
    }
    closeModal()
    await fetchDevices()
  } catch (error) {
    toast.error('Ошибка сохранения устройства: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

const deleteDevice = async (id: number) => {
  if (!confirm('Удалить устройство?')) return
  try {
    await deleteNetworkDevice(id)
    toast.success('Устройство удалено')
    await fetchDevices()
  } catch (error) {
    toast.error('Ошибка удаления устройства: ' + getErrorMessage(error))
  }
}

onMounted(fetchDevices)
</script>
