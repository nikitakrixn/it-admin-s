<template>
  <div class="space-y-6">
    <!-- Header -->
    <WireguardHeader
      :view-mode="viewMode"
      :is-admin="isAdmin"
      :syncing="syncing"
      @toggle-view="toggleView"
      @import="importFromMikrotik"
      @sync="syncAll"
      @create="openCreateModal"
    />

    <!-- Статистика -->
    <WireguardStats
      :stats="stats"
      :interfaces-count="interfaces.length"
      @filter-status="filterByStatus"
    />

    <!-- Фильтры -->
    <WireguardFilters
      v-model:search-query="searchQuery"
      v-model:interface-id="filters.interface_id"
      v-model:status="filters.status"
      :interfaces="interfaces"
      @reset="resetFilters"
    />

    <!-- Loading -->
    <div v-if="loading" class="space-y-4">
      <UiSkeleton height="200px" />
      <UiSkeleton height="400px" />
    </div>

    <!-- Список (таблица) -->
    <UiCard v-else-if="viewMode === 'list'" padding="none">
      <UiTable>
        <template #header>
          <tr>
            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Пир</th>
            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Сотрудник</th>
            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">IP адрес</th>
            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Статус</th>
            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Последнее подключение</th>
            <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Трафик</th>
            <th v-if="isAdmin" class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">Действия</th>
          </tr>
        </template>

        <tr v-if="filteredPeers.length === 0">
          <td :colspan="isAdmin ? 7 : 6">
            <UiEmptyState
              icon="ri:shield-user-line"
              badge="ri:search-line"
              :title="hasFilters ? 'Ничего не найдено' : 'Пиры отсутствуют'"
              :description="hasFilters ? 'Попробуйте изменить параметры поиска' : 'Создайте первый WireGuard пир'"
            >
              <template #actions>
                <UiButton v-if="hasFilters" variant="secondary" icon="ri:close-line" @click="resetFilters">
                  Сбросить фильтры
                </UiButton>
                <UiButton v-if="isAdmin" icon="ri:add-line" @click="openCreateModal">
                  Добавить пир
                </UiButton>
              </template>
            </UiEmptyState>
          </td>
        </tr>

        <WireguardPeerRow
          v-for="peer in filteredPeers"
          :key="peer.id"
          :peer="peer"
          :is-admin="isAdmin"
          @view="viewPeerConfig"
          @download="downloadPeerConfig"
          @edit="editPeer"
          @delete="confirmDelete"
        />
      </UiTable>
    </UiCard>

    <!-- Сетка -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
      <div
        v-for="peer in filteredPeers"
        :key="peer.id"
        class="card card-hover group cursor-pointer"
        @click="viewPeerConfig(peer)"
      >
        <div class="p-5">
          <div class="flex items-start justify-between mb-4">
            <div class="h-12 w-12 rounded-xl bg-gradient-to-br from-blue-500 to-blue-700 flex items-center justify-center relative">
              <Icon name="ri:shield-user-line" class="text-2xl text-white" />
              <span v-if="isOnline(peer)" class="absolute -top-1 -right-1 w-3 h-3 bg-green-500 border-2 border-white rounded-full" />
            </div>
            <UiBadge :variant="getStatusVariant(peer.status)">
              {{ getStatusLabel(peer.status) }}
            </UiBadge>
          </div>

          <h3 class="text-lg font-semibold text-gray-900 mb-1 group-hover:text-primary-600 transition-colors">
            {{ peer.name }}
          </h3>
          <p class="text-sm text-gray-500 mb-3">{{ peer.interface_name }}</p>

          <div class="space-y-2 mb-4">
            <div class="flex items-center gap-2 text-sm text-gray-600">
              <Icon name="ri:global-line" class="text-gray-400" />
              <code class="text-xs bg-gray-100 px-2 py-0.5 rounded font-mono">{{ peer.client_address }}</code>
            </div>
            <div v-if="peer.employee_name" class="flex items-center gap-2 text-sm text-gray-600">
              <Icon name="ri:user-line" class="text-gray-400" />
              <span>{{ peer.employee_name }}</span>
            </div>
            <div class="flex items-center gap-2 text-sm text-gray-600">
              <Icon name="ri:time-line" :class="isOnline(peer) ? 'text-green-600' : 'text-gray-400'" />
              <span :class="isOnline(peer) ? 'text-green-600 font-medium' : ''">
                {{ formatLastHandshake(peer.last_handshake) }}
              </span>
            </div>
          </div>

          <div class="pt-3 border-t border-gray-200 space-y-1">
            <div class="flex items-center justify-between text-xs text-gray-600">
              <span class="flex items-center gap-1">
                <Icon name="ri:download-line" class="text-blue-500" />
                RX
              </span>
              <span class="font-medium font-mono">{{ formatBytes(peer.rx_bytes) }}</span>
            </div>
            <div class="flex items-center justify-between text-xs text-gray-600">
              <span class="flex items-center gap-1">
                <Icon name="ri:upload-line" class="text-green-500" />
                TX
              </span>
              <span class="font-medium font-mono">{{ formatBytes(peer.tx_bytes) }}</span>
            </div>
          </div>

          <div v-if="isAdmin" class="mt-4 pt-3 border-t border-gray-200 flex gap-2" @click.stop>
            <UiButton size="sm" variant="ghost" icon="ri:download-line" full-width @click="downloadPeerConfig(peer.id)">
              Скачать
            </UiButton>
            <UiButton size="sm" variant="ghost" icon="ri:edit-line" @click="editPeer(peer)">
              Изменить
            </UiButton>
          </div>
        </div>
      </div>

      <div v-if="filteredPeers.length === 0" class="col-span-full">
        <UiEmptyState
          icon="ri:shield-user-line"
          badge="ri:search-line"
          :title="hasFilters ? 'Ничего не найдено' : 'Пиры отсутствуют'"
          :description="hasFilters ? 'Попробуйте изменить параметры поиска' : 'Создайте первый WireGuard пир'"
        >
          <template #actions>
            <UiButton v-if="hasFilters" variant="secondary" icon="ri:close-line" @click="resetFilters">
              Сбросить фильтры
            </UiButton>
            <UiButton v-if="isAdmin" icon="ri:add-line" @click="openCreateModal">
              Добавить пир
            </UiButton>
          </template>
        </UiEmptyState>
      </div>
    </div>

    <!-- Modals -->
    <UiModal v-model="showCreateModal" :title="showEditModal ? 'Редактировать пир' : 'Добавить WireGuard пир'" size="lg">
      <WireguardPeerForm
        v-model:form="form"
        :interfaces="interfaces"
        :employees="employees"
        :submitting="submitting"
        :submit-text="showEditModal ? 'Сохранить изменения' : 'Создать пир'"
        @submit="submitForm"
        @cancel="closeModals"
      />
    </UiModal>

    <WireguardConfigModal
      v-if="peerConfig"
      v-model="showConfigModal"
      :config="peerConfig.config"
      :peer-name="peerConfig.peer.name"
      @copy="copyConfigToClipboard"
      @download="downloadConfigFromModal"
    />

    <ConfirmModal
      :show="showDeleteModal"
      title="Удалить пир?"
      :message="`Вы уверены, что хотите удалить пир '${peerToDelete?.name}'?`"
      confirm-text="Удалить"
      @confirm="deletePeerConfirmed"
      @cancel="showDeleteModal = false"
    />
  </div>
</template>

<script setup lang="ts">
import type { WireguardPeer } from '~/composables/useWireguard'

definePageMeta({ middleware: 'auth' })
useHead({ title: 'WireGuard VPN' })

const config = useRuntimeConfig()
const { isAdmin } = useAuth()
const { fetchPeers, fetchInterfaces, importPeersFromMikrotik, createPeer, updatePeer, deletePeer, downloadConfig, syncAllPeers, formatBytes, formatLastHandshake, getStatusLabel } = useWireguard()
const toast = useToast()

// State
const loading = ref(true)
const syncing = ref(false)
const submitting = ref(false)
const viewMode = ref<'list' | 'grid'>('list')
const peers = ref<WireguardPeer[]>([])
const interfaces = ref<any[]>([])
const employees = ref<any[]>([])
const total = ref(0)
const currentPage = ref(1)
const perPage = ref(20)
const searchQuery = ref('')
const filters = ref({ interface_id: undefined as number | undefined, status: undefined as string | undefined })

const showCreateModal = ref(false)
const showEditModal = ref(false)
const showDeleteModal = ref(false)
const showConfigModal = ref(false)
const peerToDelete = ref<WireguardPeer | null>(null)
const peerToEdit = ref<WireguardPeer | null>(null)
const peerConfig = ref<{ peer: WireguardPeer; config: string } | null>(null)

const form = ref({
  interface_id: null as number | null,
  employee_id: null as number | null,
  name: '',
  client_address: '',
  client_dns: '',
  endpoint_address: '',
  endpoint_port: null as number | null,
  allowed_ips: '0.0.0.0/0,::/0',
  persistent_keepalive: 25,
  notes: '',
})

// Computed
const filteredPeers = computed(() => {
  let result = peers.value
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(p => p.name.toLowerCase().includes(query) || p.employee_name?.toLowerCase().includes(query))
  }
  return result
})

const hasFilters = computed(() => searchQuery.value || filters.value.interface_id || filters.value.status)

const isOnline = (peer: WireguardPeer) => {
  if (!peer.last_handshake) return false
  const now = new Date()
  const fiveMinutesAgo = new Date(now.getTime() - 5 * 60 * 1000)
  const handshakeDate = new Date(peer.last_handshake + 'Z')
  return handshakeDate > fiveMinutesAgo
}

const stats = computed(() => ({
  total: peers.value.length,
  active: peers.value.filter(p => p.status === 'active').length,
  disabled: peers.value.filter(p => p.status === 'disabled').length,
  online: peers.value.filter(isOnline).length,
}))

const getStatusVariant = (status: string) => {
  const variants: Record<string, any> = { active: 'success', disabled: 'default', expired: 'danger', revoked: 'danger' }
  return variants[status] || 'default'
}

// Methods
const toggleView = () => { viewMode.value = viewMode.value === 'list' ? 'grid' : 'list' }
const filterByStatus = (status: string) => {
  filters.value.status = filters.value.status === status ? undefined : status
  currentPage.value = 1
  loadPeers()
}

const loadPeers = async () => {
  loading.value = true
  try {
    const data = await fetchPeers({ page: currentPage.value, per_page: perPage.value, interface_id: filters.value.interface_id, status: filters.value.status })
    peers.value = data.peers
    total.value = data.total
  } catch (error: any) {
    toast.error('Ошибка загрузки пиров: ' + error.message)
  } finally {
    loading.value = false
  }
}

const loadInterfaces = async () => {
  try { interfaces.value = await fetchInterfaces() } catch (error: any) { console.error('Failed to load interfaces:', error) }
}

const loadEmployees = async () => {
  try {
    const { $api } = useNuxtApp()
    const data = await $api<any>('/employees', { params: { page: 1, per_page: 1000 } })
    employees.value = data.employees || []
  } catch (error: any) { console.error('Failed to load employees:', error) }
}

const syncAll = async () => {
  syncing.value = true
  try {
    await syncAllPeers()
    toast.success('Синхронизация завершена')
    await loadPeers()
  } catch (error: any) {
    toast.error('Ошибка синхронизации: ' + error.message)
  } finally {
    syncing.value = false
  }
}

const importFromMikrotik = async () => {
  syncing.value = true
  try {
    const data = await importPeersFromMikrotik()
    toast.success(`Импортировано ${data.peers.length} пиров`)
    await loadPeers()
  } catch (error: any) {
    toast.error('Ошибка импорта: ' + error.message)
  } finally {
    syncing.value = false
  }
}

const viewPeerConfig = async (peer: WireguardPeer) => {
  try {
    const data = await downloadConfig(peer.id)
    peerConfig.value = { peer, config: data.config }
    showConfigModal.value = true
  } catch (error: any) {
    toast.error('Ошибка загрузки конфигурации: ' + error.message)
  }
}

const downloadPeerConfig = async (id: number) => {
  try {
    const data = await downloadConfig(id)
    const blob = new Blob([data.config], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `wireguard-peer-${id}.conf`
    a.click()
    URL.revokeObjectURL(url)
    toast.success('Конфигурация скачана')
  } catch (error: any) {
    toast.error('Ошибка скачивания: ' + error.message)
  }
}

const copyConfigToClipboard = async () => {
  if (!peerConfig.value) return
  try {
    await navigator.clipboard.writeText(peerConfig.value.config)
    toast.success('Конфигурация скопирована')
  } catch (error) {
    toast.error('Ошибка копирования')
  }
}

const downloadConfigFromModal = () => {
  if (!peerConfig.value) return
  const blob = new Blob([peerConfig.value.config], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `wireguard-${peerConfig.value.peer.name.replace(/\s+/g, '-')}.conf`
  a.click()
  URL.revokeObjectURL(url)
  toast.success('Конфигурация скачана')
}

const openCreateModal = () => {
  showEditModal.value = false
  peerToEdit.value = null
  form.value = {
    interface_id: null,
    employee_id: null,
    name: '',
    client_address: config.public.wgDefaultClientAddress || '',
    client_dns: config.public.wgDefaultClientDns || '',
    endpoint_address: '',
    endpoint_port: null,
    allowed_ips: config.public.wgDefaultAllowedIps || '0.0.0.0/0,::/0',
    persistent_keepalive: Number(config.public.wgDefaultPersistentKeepalive) || 25,
    notes: '',
  }
  showCreateModal.value = true
}

const editPeer = (peer: WireguardPeer) => {
  peerToEdit.value = peer
  form.value = {
    interface_id: peer.interface_id,
    employee_id: peer.employee_id,
    name: peer.name,
    client_address: peer.client_address,
    client_dns: peer.client_dns || '',
    endpoint_address: peer.endpoint_address || '',
    endpoint_port: peer.endpoint_port,
    allowed_ips: peer.allowed_ips,
    persistent_keepalive: peer.persistent_keepalive || 25,
    notes: peer.notes || '',
  }
  showEditModal.value = true
  showCreateModal.value = true
}

const confirmDelete = (peer: WireguardPeer) => {
  peerToDelete.value = peer
  showDeleteModal.value = true
}

const deletePeerConfirmed = async () => {
  if (!peerToDelete.value) return
  try {
    await deletePeer(peerToDelete.value.id)
    toast.success('Пир удален')
    showDeleteModal.value = false
    await loadPeers()
  } catch (error: any) {
    toast.error('Ошибка удаления: ' + error.message)
  }
}

const resetFilters = () => {
  filters.value = { interface_id: undefined, status: undefined }
  searchQuery.value = ''
  currentPage.value = 1
  loadPeers()
}

const closeModals = () => {
  if (!submitting.value) {
    showCreateModal.value = false
    showEditModal.value = false
    peerToEdit.value = null
    form.value = { interface_id: null, employee_id: null, name: '', client_address: '', client_dns: '', endpoint_address: '', endpoint_port: null, allowed_ips: '0.0.0.0/0,::/0', persistent_keepalive: 25, notes: '' }
  }
}

const submitForm = async () => {
  // Валидация
  if (!form.value.interface_id) {
    toast.error('Выберите интерфейс')
    return
  }
  if (!form.value.name) {
    toast.error('Введите название')
    return
  }
  if (!form.value.client_address) {
    toast.error('Введите IP адрес клиента')
    return
  }

  submitting.value = true
  try {
    const data = {
      interface_id: Number(form.value.interface_id),
      employee_id: form.value.employee_id ? Number(form.value.employee_id) : undefined,
      name: form.value.name,
      client_address: form.value.client_address,
      client_dns: form.value.client_dns || undefined,
      endpoint_address: form.value.endpoint_address || undefined,
      endpoint_port: form.value.endpoint_port ? Number(form.value.endpoint_port) : undefined,
      allowed_ips: form.value.allowed_ips || undefined,
      persistent_keepalive: form.value.persistent_keepalive ? Number(form.value.persistent_keepalive) : undefined,
      notes: form.value.notes || undefined,
    }

    if (showEditModal.value && peerToEdit.value) {
      await updatePeer(peerToEdit.value.id, data)
      toast.success('Пир успешно обновлен')
    } else {
      await createPeer(data)
      toast.success('Пир успешно создан')
    }

    submitting.value = false
    closeModals()
    await loadPeers()
  } catch (err: any) {
    submitting.value = false
    toast.error('Ошибка при сохранении: ' + (err.message || 'Неизвестная ошибка'))
  }
}

watch([currentPage, () => filters.value.interface_id, () => filters.value.status], () => { loadPeers() })

onMounted(async () => {
  await loadPeers()
  await loadInterfaces()
  await loadEmployees()
})
</script>
