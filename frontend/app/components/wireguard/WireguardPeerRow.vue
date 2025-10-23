<template>
  <tr class="hover:bg-gray-50 transition-colors">
    <td class="px-4 py-4">
      <div class="flex items-center gap-3">
        <div class="relative">
          <div class="h-10 w-10 rounded-lg bg-gradient-to-br from-blue-500 to-blue-700 flex items-center justify-center text-white font-semibold text-lg shadow-sm">
            <Icon name="ri:shield-user-line" class="text-xl" />
          </div>
          <span
            v-if="isOnline"
            class="absolute -top-1 -right-1 w-3 h-3 bg-green-500 border-2 border-white rounded-full"
          />
        </div>
        <div>
          <div class="flex items-center gap-2">
            <span class="font-medium text-gray-900">{{ peer.name }}</span>
          </div>
          <div class="flex items-center gap-2 text-sm text-gray-500">
            <Icon name="ri:router-line" class="text-xs" />
            {{ peer.interface_name }}
          </div>
        </div>
      </div>
    </td>
    
    <td class="px-4 py-4">
      <div v-if="peer.employee_name" class="flex items-center gap-2">
        <div class="h-8 w-8 rounded-full bg-gradient-to-br from-gray-400 to-gray-600 flex items-center justify-center text-white text-xs font-semibold">
          {{ getInitials(peer.employee_name) }}
        </div>
        <span class="text-sm text-gray-900">{{ peer.employee_name }}</span>
      </div>
      <span v-else class="text-sm text-gray-400 italic">Не назначен</span>
    </td>
    
    <td class="px-4 py-4">
      <div class="flex items-center gap-2">
        <Icon name="ri:global-line" class="text-gray-400 text-sm" />
        <code class="text-sm bg-gray-100 px-2 py-1 rounded font-mono">
          {{ peer.client_address }}
        </code>
      </div>
    </td>
    
    <td class="px-4 py-4">
      <UiBadge :variant="getStatusVariant(peer.status)">
        {{ getStatusLabel(peer.status) }}
      </UiBadge>
    </td>
    
    <td class="px-4 py-4">
      <div class="flex items-center gap-2 text-sm">
        <Icon
          name="ri:time-line"
          :class="isOnline ? 'text-green-600' : 'text-gray-400'"
        />
        <span :class="isOnline ? 'text-green-600 font-medium' : 'text-gray-500'">
          {{ formatLastHandshake(peer.last_handshake) }}
        </span>
      </div>
    </td>
    
    <td class="px-4 py-4">
      <div class="space-y-1">
        <div class="flex items-center gap-2 text-sm text-gray-600">
          <Icon name="ri:download-line" class="text-xs text-blue-500" />
          <span class="font-mono">{{ formatBytes(peer.rx_bytes) }}</span>
        </div>
        <div class="flex items-center gap-2 text-sm text-gray-600">
          <Icon name="ri:upload-line" class="text-xs text-green-500" />
          <span class="font-mono">{{ formatBytes(peer.tx_bytes) }}</span>
        </div>
      </div>
    </td>
    
    <td v-if="isAdmin" class="px-4 py-4">
      <div class="flex items-center justify-end gap-1">
        <UiTooltip text="Просмотр конфигурации">
          <button
            @click="$emit('view', peer)"
            class="p-2 text-gray-600 hover:text-purple-600 hover:bg-purple-50 rounded-lg transition-colors"
          >
            <Icon name="ri:file-text-line" />
          </button>
        </UiTooltip>
        
        <UiTooltip text="Скачать конфиг">
          <button
            @click="$emit('download', peer.id)"
            class="p-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
          >
            <Icon name="ri:download-line" />
          </button>
        </UiTooltip>
        
        <UiTooltip text="Редактировать">
          <button
            @click="$emit('edit', peer)"
            class="p-2 text-gray-600 hover:text-amber-600 hover:bg-amber-50 rounded-lg transition-colors"
          >
            <Icon name="ri:edit-line" />
          </button>
        </UiTooltip>
        
        <UiTooltip text="Удалить">
          <button
            @click="$emit('delete', peer)"
            class="p-2 text-gray-600 hover:text-red-600 hover:bg-red-50 rounded-lg transition-colors"
          >
            <Icon name="ri:delete-bin-line" />
          </button>
        </UiTooltip>
      </div>
    </td>
  </tr>
</template>

<script setup lang="ts">
import type { WireguardPeer } from '~/composables/useWireguard'

interface Props {
  peer: WireguardPeer
  isAdmin: boolean
}

const props = defineProps<Props>()

defineEmits<{
  view: [peer: WireguardPeer]
  download: [id: number]
  edit: [peer: WireguardPeer]
  delete: [peer: WireguardPeer]
}>()

const { formatBytes, formatLastHandshake, getStatusLabel } = useWireguard()

const isOnline = computed(() => {
  if (!props.peer.last_handshake) return false
  const now = new Date()
  const fiveMinutesAgo = new Date(now.getTime() - 5 * 60 * 1000)
  const handshakeDate = new Date(props.peer.last_handshake + 'Z')
  return handshakeDate > fiveMinutesAgo
})

const getInitials = (name: string) => {
  return name.split(' ').map(n => n[0]).join('').slice(0, 2)
}

const getStatusVariant = (status: string) => {
  const variants: Record<string, any> = {
    active: 'success',
    disabled: 'default',
    expired: 'danger',
    revoked: 'danger',
  }
  return variants[status] || 'default'
}
</script>
