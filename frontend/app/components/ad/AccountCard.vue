<template>
  <div class="card hover:shadow-md transition-shadow">
    <div class="p-6">
      <div class="flex items-start justify-between mb-4">
        <div class="flex items-center gap-3 min-w-0">
          <div
            class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center flex-shrink-0"
          >
            <Icon name="ri:user-line" class="text-xl text-blue-600" />
          </div>
          <div class="min-w-0">
            <p class="text-sm font-semibold text-gray-900 truncate">{{ account.username }}</p>
            <p class="text-xs text-gray-500 truncate">{{ account.display_name || '—' }}</p>
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

      <p class="text-xs text-gray-600 mb-3 truncate">{{ account.email || 'Email не указан' }}</p>

      <div class="flex flex-wrap gap-2 mb-4">
        <span class="badge bg-blue-100 text-blue-800">{{ account.account_type }}</span>
        <span
          class="badge"
          :class="account.is_enabled ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'"
        >
          {{ account.is_enabled ? 'Активен' : 'Отключён' }}
        </span>
        <span v-if="account.is_locked" class="badge bg-amber-100 text-amber-800">Заблокирован</span>
      </div>

      <div class="flex items-center justify-between border-t border-gray-100 pt-4">
        <span class="text-xs text-gray-500">Создан</span>
        <span class="text-xs font-medium text-gray-900">{{ formatDate(account.created_at) }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { DomainAccount } from '~/composables/useAd'

const props = defineProps<{
  account: DomainAccount
}>()

const emit = defineEmits<{
  edit: [account: DomainAccount]
  delete: [id: number]
}>()

const formatDate = (value: string): string => {
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleDateString('ru-RU')
}

const onEdit = () => emit('edit', props.account)
const onDelete = () => emit('delete', props.account.id)
</script>
