<template>
  <div class="card hover:shadow-md transition-shadow group">
    <div class="p-6">
      <div class="flex items-start justify-between mb-4">
        <div class="flex items-center gap-3 min-w-0">
          <div
            class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center flex-shrink-0"
          >
            <Icon name="ri:map-line" class="text-xl text-blue-600" />
          </div>
          <div class="min-w-0">
            <p class="text-sm font-semibold text-gray-900 truncate">
              {{ segment.name }}
            </p>
            <p class="text-xs text-gray-500 font-mono">
              {{ segment.network_address }} / {{ segment.subnet_mask }}
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
          <p class="text-xs text-gray-500 uppercase font-bold">Шлюз</p>
          <p class="text-sm font-medium text-gray-900 font-mono">
            {{ segment.gateway || '—' }}
          </p>
        </div>
        <div class="p-3 rounded-lg bg-gray-50">
          <p class="text-xs text-gray-500 uppercase font-bold">VLAN</p>
          <p class="text-sm font-medium text-gray-900">
            {{ segment.vlan_id ?? '—' }}
          </p>
        </div>
      </div>

      <div class="flex items-center justify-between text-xs text-gray-500">
        <span>{{ segment.location || 'Локация не указана' }}</span>
        <span
          class="badge"
          :class="
            segment.dhcp_enabled ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-700'
          "
        >
          DHCP {{ segment.dhcp_enabled ? 'вкл' : 'выкл' }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { NetworkSegment } from '~/composables/useNetwork'

const props = defineProps<{
  segment: NetworkSegment
}>()

const emit = defineEmits<{
  edit: [segment: NetworkSegment]
  delete: [id: number]
}>()

const onEdit = () => emit('edit', props.segment)
const onDelete = () => emit('delete', props.segment.id)
</script>
