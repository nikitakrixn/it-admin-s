<template>
  <div class="card hover:shadow-md transition-shadow group">
    <div class="p-6">
      <div class="flex items-start justify-between mb-4">
        <div class="flex items-center gap-3 min-w-0">
          <div
            class="h-10 w-10 rounded-lg bg-emerald-100 flex items-center justify-center flex-shrink-0"
          >
            <Icon name="ri:global-line" class="text-xl text-emerald-600" />
          </div>
          <div class="min-w-0">
            <p class="text-sm font-semibold text-gray-900 font-mono truncate">
              {{ address.ip_address }}
            </p>
            <p class="text-xs text-gray-500 truncate">
              {{ address.hostname || 'Имя хоста не задано' }}
            </p>
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
          <p class="text-xs text-gray-500 uppercase font-bold">MAC</p>
          <p class="text-sm font-medium text-gray-900 font-mono">
            {{ address.mac_address || '—' }}
          </p>
        </div>
        <div class="p-3 rounded-lg bg-gray-50">
          <p class="text-xs text-gray-500 uppercase font-bold">Назначение</p>
          <p class="text-sm font-medium text-gray-900">
            {{ address.assigned_to_type || '—' }}
          </p>
        </div>
      </div>

      <div class="flex items-center justify-between text-xs text-gray-500">
        <span>{{ address.is_reserved ? 'Зарезервирован' : 'Динамический' }}</span>
        <span class="badge" :class="statusClass">{{ statusLabel }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { IpAddress } from '~/composables/useNetwork'

const props = defineProps<{
  address: IpAddress
}>()

const emit = defineEmits<{
  edit: [address: IpAddress]
  delete: [id: number]
}>()

const statusLabels: Record<string, string> = {
  available: 'Свободен',
  assigned: 'Назначен',
  reserved: 'Резерв',
  offline: 'Не в сети',
  conflict: 'Конфликт',
}

const statusLabel = computed(
  () => statusLabels[props.address.status ?? ''] ?? (props.address.status || '—'),
)

const statusClass = computed(() => {
  switch (props.address.status) {
    case 'assigned':
      return 'bg-green-100 text-green-800'
    case 'reserved':
      return 'bg-amber-100 text-amber-800'
    case 'conflict':
      return 'bg-red-100 text-red-800'
    case 'offline':
      return 'bg-gray-100 text-gray-700'
    default:
      return 'bg-blue-100 text-blue-800'
  }
})

const onEdit = () => emit('edit', props.address)
const onDelete = () => emit('delete', props.address.id)
</script>
