<template>
  <div class="card hover:shadow-md transition-shadow">
    <div class="p-6">
      <div class="flex items-start justify-between mb-4">
        <div class="flex items-center gap-3 min-w-0">
          <div
            class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center flex-shrink-0"
          >
            <Icon name="ri:group-line" class="text-xl text-purple-600" />
          </div>
          <div class="min-w-0">
            <p class="text-sm font-semibold text-gray-900 truncate">{{ group.name }}</p>
            <p class="text-xs text-gray-500 font-mono truncate">
              {{ group.distinguished_name || '—' }}
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

      <div class="flex flex-wrap gap-2 mb-4">
        <span v-if="group.group_type" class="badge bg-blue-100 text-blue-800">
          {{ typeLabel(group.group_type) }}
        </span>
        <span v-if="group.group_scope" class="badge bg-indigo-100 text-indigo-800">
          {{ scopeLabel(group.group_scope) }}
        </span>
      </div>

      <p class="text-xs text-gray-500 mb-4 line-clamp-2">
        {{ group.description || 'Описание не указано' }}
      </p>

      <div class="flex items-center justify-between border-t border-gray-100 pt-4">
        <span class="text-xs text-gray-500">Участников</span>
        <span class="text-sm font-semibold text-gray-900">{{ group.member_count ?? 0 }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { AdGroup } from '~/composables/useAd'

const props = defineProps<{
  group: AdGroup
}>()

const emit = defineEmits<{
  edit: [group: AdGroup]
  delete: [id: number]
}>()

const typeLabels: Record<string, string> = {
  Security: 'Безопасности',
  Distribution: 'Рассылки',
}

const scopeLabels: Record<string, string> = {
  Global: 'Global',
  DomainLocal: 'Domain Local',
  Universal: 'Universal',
}

const typeLabel = (value: string) => typeLabels[value] ?? value
const scopeLabel = (value: string) => scopeLabels[value] ?? value

const onEdit = () => emit('edit', props.group)
const onDelete = () => emit('delete', props.group.id)
</script>
