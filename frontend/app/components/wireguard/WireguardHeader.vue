<template>
  <div class="mb-8">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-4">
        <div class="h-16 w-16 rounded-2xl bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center shadow-lg">
          <Icon name="ri:shield-user-line" class="text-3xl text-white" />
        </div>
        <div>
          <h1 class="text-3xl font-bold text-gray-900">WireGuard VPN</h1>
          <p class="mt-1 text-gray-600">Управление VPN подключениями сотрудников</p>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <UiButton
          variant="secondary"
          :icon="viewMode === 'grid' ? 'ri:list-check' : 'ri:grid-line'"
          @click="$emit('toggleView')"
        >
          {{ viewMode === 'grid' ? 'Список' : 'Сетка' }}
        </UiButton>
        <UiButton
          v-if="isAdmin"
          variant="secondary"
          icon="ri:download-cloud-line"
          :loading="syncing"
          @click="$emit('import')"
        >
          Импорт
        </UiButton>
        <UiButton
          v-if="isAdmin"
          variant="secondary"
          icon="ri:refresh-line"
          :loading="syncing"
          @click="$emit('sync')"
        >
          Синхронизировать
        </UiButton>
        <UiButton
          v-if="isAdmin"
          variant="primary"
          icon="ri:add-line"
          @click="$emit('create')"
        >
          Добавить пир
        </UiButton>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  viewMode: 'list' | 'grid'
  isAdmin: boolean
  syncing: boolean
}

defineProps<Props>()

defineEmits<{
  toggleView: []
  import: []
  sync: []
  create: []
}>()
</script>
