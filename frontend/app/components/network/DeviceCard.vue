<template>
  <div class="card hover:shadow-md transition-shadow group">
    <div class="p-6">
      <div class="flex items-start justify-between mb-4">
        <div class="flex items-center gap-3 min-w-0">
          <div
            class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center flex-shrink-0"
          >
            <Icon :name="deviceIcon" class="text-xl text-purple-600" />
          </div>
          <div class="min-w-0">
            <p class="text-sm font-semibold text-gray-900 truncate">{{ device.name }}</p>
            <p class="text-xs text-gray-500">{{ deviceTypeLabel }}</p>
          </div>
        </div>
        <div class="flex gap-1 flex-shrink-0">
          <button
            class="p-1 text-gray-400 hover:text-blue-600 hover:bg-gray-100 rounded"
            title="Редактировать"
            @click.stop="onEdit"
          >
            <Icon name="ri:edit-line" class="text-sm" />
          </button>
          <button
            class="p-1 text-gray-400 hover:text-red-600 hover:bg-red-50 rounded"
            title="Удалить"
            @click.stop="onDelete"
          >
            <Icon name="ri:delete-bin-line" class="text-sm" />
          </button>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-3 mb-3">
        <div class="p-3 rounded-lg bg-gray-50">
          <p class="text-xs text-gray-500 uppercase font-bold">IP-адрес</p>
          <p class="text-sm font-medium text-gray-900 font-mono">
            {{ device.ip_address || '—' }}
          </p>
        </div>
        <div class="p-3 rounded-lg bg-gray-50">
          <p class="text-xs text-gray-500 uppercase font-bold">MAC</p>
          <p class="text-sm font-medium text-gray-900 font-mono">
            {{ device.mac_address || '—' }}
          </p>
        </div>
      </div>

      <div class="flex items-center justify-between text-xs text-gray-500">
        <span class="truncate">{{ device.location || device.manufacturer || '—' }}</span>
        <span class="badge" :class="statusClass">{{ statusLabel }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { NetworkDevice } from '~/composables/useNetwork'

const props = defineProps<{
  device: NetworkDevice
}>()

const emit = defineEmits<{
  edit: [device: NetworkDevice]
  delete: [id: number]
}>()

const deviceIcons: Record<string, string> = {
  switch: 'ri:switch-line',
  router: 'ri:router-line',
  firewall: 'ri:shield-line',
  access_point: 'ri:wifi-line',
  load_balancer: 'ri:scales-3-line',
  vpn_gateway: 'ri:lock-line',
}

const deviceLabels: Record<string, string> = {
  switch: 'Коммутатор',
  router: 'Маршрутизатор',
  firewall: 'Межсетевой экран',
  access_point: 'Точка доступа',
  load_balancer: 'Балансировщик',
  vpn_gateway: 'VPN-шлюз',
}

const deviceIcon = computed(() => deviceIcons[props.device.device_type] ?? 'ri:hard-drive-line')
const deviceTypeLabel = computed(
  () => deviceLabels[props.device.device_type] ?? props.device.device_type,
)

const statusLabels: Record<string, string> = {
  active: 'Активно',
  inactive: 'Неактивно',
  maintenance: 'Обслуживание',
  failed: 'Сбой',
}

const statusLabel = computed(
  () => statusLabels[props.device.status ?? ''] ?? (props.device.status || '—'),
)

const statusClass = computed(() => {
  switch (props.device.status) {
    case 'active':
      return 'bg-green-100 text-green-800'
    case 'maintenance':
      return 'bg-amber-100 text-amber-800'
    case 'inactive':
      return 'bg-gray-100 text-gray-700'
    default:
      return 'bg-red-100 text-red-800'
  }
})

const onEdit = () => emit('edit', props.device)
const onDelete = () => emit('delete', props.device.id)
</script>
