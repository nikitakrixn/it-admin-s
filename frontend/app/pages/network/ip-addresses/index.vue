<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-emerald-500 to-emerald-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:global-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">IP-адреса</h1>
            <p class="mt-1 text-gray-600">Учёт и резервирование адресного пространства</p>
          </div>
        </div>
        <button class="btn btn-primary" @click="openCreateModal">
          <Icon name="ri:add-line" class="mr-2" />
          Добавить IP
        </button>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">Всего адресов</p>
            <p class="text-2xl font-bold text-emerald-600">{{ addresses.length }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center">
            <Icon name="ri:global-line" class="text-2xl text-emerald-600" />
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
            <p class="text-sm text-gray-600 mb-1">Зарезервировано</p>
            <p class="text-2xl font-bold text-amber-600">{{ reservedCount }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-amber-100 flex items-center justify-center">
            <Icon name="ri:lock-line" class="text-2xl text-amber-600" />
          </div>
        </div>
      </div>
    </div>

    <div class="card p-4">
      <div class="flex flex-col gap-4 md:flex-row">
        <div class="flex-1 relative">
          <Icon
            name="ri:search-line"
            class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
          />
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Поиск по IP, хосту или MAC..."
            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          />
        </div>
        <select v-model="filterSegment" class="input-field md:w-72">
          <option :value="null">Все сегменты</option>
          <option v-for="segment in segments" :key="segment.id" :value="segment.id">
            {{ segment.name }}
          </option>
        </select>
      </div>
    </div>

    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-emerald-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка IP-адресов...</p>
    </div>

    <div v-else-if="filteredAddresses.length === 0" class="card p-16 text-center">
      <Icon name="ri:inbox-line" class="text-4xl text-gray-300 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">IP-адреса не найдены</h3>
      <p class="text-sm text-gray-500 mb-4">
        {{ addresses.length === 0 ? 'Добавьте первый IP-адрес' : 'Измените параметры поиска' }}
      </p>
      <button v-if="addresses.length === 0" class="btn btn-primary" @click="openCreateModal">
        <Icon name="ri:add-line" class="mr-2" />
        Добавить IP
      </button>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <IpAddressCard
        v-for="address in filteredAddresses"
        :key="address.id"
        :address="address"
        @edit="openEditModal"
        @delete="deleteAddress"
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
              {{ isEditMode ? 'Редактировать IP-адрес' : 'Новый IP-адрес' }}
            </h2>
            <button class="p-2 rounded-lg hover:bg-gray-100" @click="closeModal">
              <Icon name="ri:close-line" class="text-xl" />
            </button>
          </div>

          <form class="p-6 space-y-4" @submit.prevent="handleSubmit">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">IP-адрес *</label>
                <input
                  v-model="form.ip_address"
                  type="text"
                  class="input-field font-mono"
                  placeholder="192.168.1.10"
                  required
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Сеть</label>
                <select v-model="form.network_segment_id" class="input-field">
                  <option :value="null">Не указана</option>
                  <option v-for="segment in segments" :key="segment.id" :value="segment.id">
                    {{ segment.name }}
                  </option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Статус</label>
                <select v-model="form.status" class="input-field">
                  <option value="available">Свободен</option>
                  <option value="assigned">Назначен</option>
                  <option value="reserved">Резерв</option>
                  <option value="offline">Не в сети</option>
                  <option value="conflict">Конфликт</option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Имя хоста</label>
                <input v-model="form.hostname" type="text" class="input-field" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">MAC-адрес</label>
                <input v-model="form.mac_address" type="text" class="input-field font-mono" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Тип назначения</label>
                <select v-model="form.assigned_to_type" class="input-field">
                  <option value="">Не назначен</option>
                  <option value="employee">Сотрудник</option>
                  <option value="computer">Компьютер</option>
                  <option value="device">Сетевое устройство</option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">ID назначения</label>
                <input
                  v-model.number="form.assigned_to_id"
                  type="number"
                  min="1"
                  class="input-field"
                  :disabled="!form.assigned_to_type"
                />
              </div>
              <div class="flex items-end">
                <label class="flex items-center gap-2 text-sm font-medium text-gray-700 pb-2">
                  <input v-model="form.is_reserved" type="checkbox" class="rounded" />
                  Зарезервирован
                </label>
              </div>
              <div v-if="form.is_reserved" class="md:col-span-2">
                <label class="block text-sm font-medium text-gray-700 mb-1">Причина резерва</label>
                <input v-model="form.reservation_reason" type="text" class="input-field" />
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
import type { IpAddress, NetworkSegment, NewIpAddressRequest } from '~/composables/useNetwork'
import IpAddressCard from '~/components/network/IpAddressCard.vue'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

useHead({
  title: 'IP-адреса',
})

const toast = useToast()
const {
  fetchIpAddresses,
  fetchNetworkSegments,
  createIpAddress,
  updateIpAddress,
  deleteIpAddress,
} = useNetwork()

interface AddressForm {
  network_segment_id: number | null
  ip_address: string
  status: string
  assigned_to_type: string
  assigned_to_id: number | null
  hostname: string
  mac_address: string
  is_reserved: boolean
  reservation_reason: string
  notes: string
}

const emptyForm = (): AddressForm => ({
  network_segment_id: null,
  ip_address: '',
  status: 'available',
  assigned_to_type: '',
  assigned_to_id: null,
  hostname: '',
  mac_address: '',
  is_reserved: false,
  reservation_reason: '',
  notes: '',
})

const addresses = ref<IpAddress[]>([])
const segments = ref<NetworkSegment[]>([])
const loading = ref(false)
const showModal = ref(false)
const isEditMode = ref(false)
const editingId = ref<number | null>(null)
const saving = ref(false)
const searchQuery = ref('')
const filterSegment = ref<number | null>(null)
const form = ref<AddressForm>(emptyForm())

const activeCount = computed(
  () => addresses.value.filter((item) => item.status === 'assigned').length,
)
const reservedCount = computed(
  () => addresses.value.filter((item) => item.is_reserved || item.status === 'reserved').length,
)

const filteredAddresses = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()

  return addresses.value.filter((address) => {
    if (filterSegment.value !== null && address.network_segment_id !== filterSegment.value) {
      return false
    }
    if (!query) return true

    return [address.ip_address, address.hostname, address.mac_address]
      .filter((value): value is string => Boolean(value))
      .some((value) => value.toLowerCase().includes(query))
  })
})

const fetchAddresses = async () => {
  loading.value = true
  try {
    const response = await fetchIpAddresses({ per_page: 100 })
    addresses.value = response.addresses
  } catch (error) {
    toast.error('Ошибка загрузки IP-адресов: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const fetchSegments = async () => {
  try {
    const response = await fetchNetworkSegments({ per_page: 100 })
    segments.value = response.segments
  } catch (error) {
    toast.error('Ошибка загрузки сетей: ' + getErrorMessage(error))
  }
}

const openCreateModal = () => {
  isEditMode.value = false
  editingId.value = null
  form.value = emptyForm()
  showModal.value = true
}

const openEditModal = (address: IpAddress) => {
  isEditMode.value = true
  editingId.value = address.id
  form.value = {
    network_segment_id: address.network_segment_id,
    ip_address: address.ip_address,
    status: address.status ?? 'available',
    assigned_to_type: address.assigned_to_type ?? '',
    assigned_to_id: address.assigned_to_id,
    hostname: address.hostname ?? '',
    mac_address: address.mac_address ?? '',
    is_reserved: address.is_reserved ?? false,
    reservation_reason: address.reservation_reason ?? '',
    notes: address.notes ?? '',
  }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  editingId.value = null
  form.value = emptyForm()
}

const toRequest = (value: AddressForm): NewIpAddressRequest => ({
  network_segment_id: value.network_segment_id ?? undefined,
  ip_address: value.ip_address.trim(),
  status: value.status || undefined,
  assigned_to_type: value.assigned_to_type || undefined,
  assigned_to_id: value.assigned_to_type ? toOptionalNumber(value.assigned_to_id) : undefined,
  hostname: value.hostname.trim() || undefined,
  mac_address: value.mac_address.trim() || undefined,
  is_reserved: value.is_reserved,
  reservation_reason: value.is_reserved ? value.reservation_reason.trim() || undefined : undefined,
  notes: value.notes.trim() || undefined,
})

const handleSubmit = async () => {
  if (!form.value.ip_address.trim()) {
    toast.warning('Укажите IP-адрес')
    return
  }

  saving.value = true
  try {
    const payload = toRequest(form.value)
    if (isEditMode.value && editingId.value !== null) {
      await updateIpAddress(editingId.value, payload)
      toast.success('IP-адрес обновлён')
    } else {
      await createIpAddress(payload)
      toast.success('IP-адрес создан')
    }
    closeModal()
    await fetchAddresses()
  } catch (error) {
    toast.error('Ошибка сохранения IP-адреса: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

const deleteAddress = async (id: number) => {
  if (!confirm('Удалить IP-адрес?')) return
  try {
    await deleteIpAddress(id)
    toast.success('IP-адрес удалён')
    await fetchAddresses()
  } catch (error) {
    toast.error('Ошибка удаления IP-адреса: ' + getErrorMessage(error))
  }
}

onMounted(async () => {
  await Promise.all([fetchAddresses(), fetchSegments()])
})
</script>
