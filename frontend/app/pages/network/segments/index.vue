<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-blue-500 to-blue-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:map-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Сети и подсети</h1>
            <p class="mt-1 text-gray-600">Управление сетевыми сегментами</p>
          </div>
        </div>
        <button class="btn btn-primary" @click="openCreateModal">
          <Icon name="ri:add-line" class="mr-2" />
          Добавить сеть
        </button>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">Всего сетей</p>
            <p class="text-2xl font-bold text-blue-600">{{ segments.length }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center">
            <Icon name="ri:map-2-line" class="text-2xl text-blue-600" />
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">С DHCP</p>
            <p class="text-2xl font-bold text-green-600">{{ dhcpCount }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center">
            <Icon name="ri:server-line" class="text-2xl text-green-600" />
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">С VLAN</p>
            <p class="text-2xl font-bold text-purple-600">{{ vlanCount }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center">
            <Icon name="ri:git-branch-line" class="text-2xl text-purple-600" />
          </div>
        </div>
      </div>
    </div>

    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-blue-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка сетей...</p>
    </div>

    <div v-else-if="segments.length === 0" class="card p-16 text-center">
      <Icon name="ri:inbox-line" class="text-4xl text-gray-300 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Сети не найдены</h3>
      <p class="text-sm text-gray-500 mb-4">Создайте первый сетевой сегмент</p>
      <button class="btn btn-primary" @click="openCreateModal">
        <Icon name="ri:add-line" class="mr-2" />
        Добавить сеть
      </button>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <SegmentCard
        v-for="segment in segments"
        :key="segment.id"
        :segment="segment"
        @edit="openEditModal"
        @delete="deleteSegment"
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
              {{ isEditMode ? 'Редактировать сеть' : 'Новая сеть' }}
            </h2>
            <button class="p-2 rounded-lg hover:bg-gray-100" @click="closeModal">
              <Icon name="ri:close-line" class="text-xl" />
            </button>
          </div>

          <form class="p-6 space-y-4" @submit.prevent="handleSubmit">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="md:col-span-2">
                <label class="block text-sm font-medium text-gray-700 mb-1">Название *</label>
                <input v-model="form.name" type="text" class="input-field" required />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Адрес сети *</label>
                <input
                  v-model="form.network_address"
                  type="text"
                  class="input-field font-mono"
                  placeholder="192.168.1.0"
                  required
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Маска подсети *</label>
                <input
                  v-model="form.subnet_mask"
                  type="text"
                  class="input-field font-mono"
                  placeholder="255.255.255.0"
                  required
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Шлюз</label>
                <input
                  v-model="form.gateway"
                  type="text"
                  class="input-field font-mono"
                  placeholder="192.168.1.1"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">VLAN ID</label>
                <input
                  v-model.number="form.vlan_id"
                  type="number"
                  min="1"
                  max="4094"
                  class="input-field"
                />
              </div>
              <div class="md:col-span-2">
                <label class="block text-sm font-medium text-gray-700 mb-1">DNS-серверы</label>
                <input
                  v-model="form.dns_servers"
                  type="text"
                  class="input-field font-mono"
                  placeholder="8.8.8.8, 8.8.4.4"
                />
              </div>
              <div class="md:col-span-2">
                <label class="block text-sm font-medium text-gray-700 mb-1">Локация</label>
                <input
                  v-model="form.location"
                  type="text"
                  class="input-field"
                  placeholder="Офис, 2 этаж"
                />
              </div>
            </div>

            <div class="rounded-lg border border-gray-200 p-4">
              <label class="flex items-center gap-2 text-sm font-medium text-gray-700">
                <input v-model="form.dhcp_enabled" type="checkbox" class="rounded" />
                DHCP включён
              </label>
              <div v-if="form.dhcp_enabled" class="mt-3 grid grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm text-gray-600 mb-1">Начало диапазона</label>
                  <input
                    v-model="form.dhcp_range_start"
                    type="text"
                    class="input-field font-mono"
                    placeholder="192.168.1.100"
                  />
                </div>
                <div>
                  <label class="block text-sm text-gray-600 mb-1">Конец диапазона</label>
                  <input
                    v-model="form.dhcp_range_end"
                    type="text"
                    class="input-field font-mono"
                    placeholder="192.168.1.200"
                  />
                </div>
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Описание</label>
              <textarea v-model="form.description" rows="3" class="input-field"></textarea>
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
import type { NetworkSegment, NewNetworkSegmentRequest } from '~/composables/useNetwork'
import SegmentCard from '~/components/network/SegmentCard.vue'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

useHead({
  title: 'Сети и подсети',
})

const toast = useToast()
const { fetchNetworkSegments, createNetworkSegment, updateNetworkSegment, deleteNetworkSegment } =
  useNetwork()

interface SegmentForm {
  name: string
  network_address: string
  subnet_mask: string
  gateway: string
  vlan_id: number | null
  dns_servers: string
  dhcp_enabled: boolean
  dhcp_range_start: string
  dhcp_range_end: string
  description: string
  location: string
}

const emptyForm = (): SegmentForm => ({
  name: '',
  network_address: '',
  subnet_mask: '',
  gateway: '',
  vlan_id: null,
  dns_servers: '',
  dhcp_enabled: false,
  dhcp_range_start: '',
  dhcp_range_end: '',
  description: '',
  location: '',
})

const segments = ref<NetworkSegment[]>([])
const loading = ref(false)
const showModal = ref(false)
const isEditMode = ref(false)
const editingId = ref<number | null>(null)
const saving = ref(false)
const form = ref<SegmentForm>(emptyForm())

const dhcpCount = computed(() => segments.value.filter((segment) => segment.dhcp_enabled).length)
const vlanCount = computed(
  () => segments.value.filter((segment) => segment.vlan_id !== null).length,
)

const fetchSegments = async () => {
  loading.value = true
  try {
    const response = await fetchNetworkSegments({ per_page: 100 })
    segments.value = response.segments
  } catch (error) {
    toast.error('Ошибка загрузки сетей: ' + getErrorMessage(error))
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

const openEditModal = (segment: NetworkSegment) => {
  isEditMode.value = true
  editingId.value = segment.id
  form.value = {
    name: segment.name,
    network_address: segment.network_address,
    subnet_mask: segment.subnet_mask,
    gateway: segment.gateway ?? '',
    vlan_id: segment.vlan_id,
    dns_servers: segment.dns_servers ?? '',
    dhcp_enabled: segment.dhcp_enabled ?? false,
    dhcp_range_start: segment.dhcp_range_start ?? '',
    dhcp_range_end: segment.dhcp_range_end ?? '',
    description: segment.description ?? '',
    location: segment.location ?? '',
  }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  editingId.value = null
  form.value = emptyForm()
}

const toRequest = (value: SegmentForm): NewNetworkSegmentRequest => ({
  name: value.name.trim(),
  network_address: value.network_address.trim(),
  subnet_mask: value.subnet_mask.trim(),
  gateway: value.gateway.trim() || undefined,
  vlan_id: toOptionalNumber(value.vlan_id),
  dns_servers: value.dns_servers.trim() || undefined,
  dhcp_enabled: value.dhcp_enabled,
  dhcp_range_start: value.dhcp_enabled ? value.dhcp_range_start.trim() || undefined : undefined,
  dhcp_range_end: value.dhcp_enabled ? value.dhcp_range_end.trim() || undefined : undefined,
  description: value.description.trim() || undefined,
  location: value.location.trim() || undefined,
})

const handleSubmit = async () => {
  if (
    !form.value.name.trim() ||
    !form.value.network_address.trim() ||
    !form.value.subnet_mask.trim()
  ) {
    toast.warning('Заполните обязательные поля')
    return
  }

  saving.value = true
  try {
    const payload = toRequest(form.value)
    if (isEditMode.value && editingId.value !== null) {
      await updateNetworkSegment(editingId.value, payload)
      toast.success('Сеть обновлена')
    } else {
      await createNetworkSegment(payload)
      toast.success('Сеть создана')
    }
    closeModal()
    await fetchSegments()
  } catch (error) {
    toast.error('Ошибка сохранения сети: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

const deleteSegment = async (id: number) => {
  if (!confirm('Удалить сеть?')) return
  try {
    await deleteNetworkSegment(id)
    toast.success('Сеть удалена')
    await fetchSegments()
  } catch (error) {
    toast.error('Ошибка удаления сети: ' + getErrorMessage(error))
  }
}

onMounted(fetchSegments)
</script>
