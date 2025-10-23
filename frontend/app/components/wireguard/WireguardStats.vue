<template>
  <UiCard padding="md">
    <div class="grid grid-cols-5 gap-4">
      <div class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors" @click="$emit('filterStatus', 'active')">
        <div class="flex items-center gap-3">
          <div class="h-10 w-10 rounded-lg bg-green-100 flex items-center justify-center flex-shrink-0">
            <Icon name="ri:checkbox-circle-line" class="text-xl text-green-600" />
          </div>
          <div>
            <p class="text-xs text-gray-600">Активные</p>
            <p class="text-xl font-bold text-green-600">{{ stats.active }}</p>
          </div>
        </div>
      </div>

      <div class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors" @click="$emit('filterStatus', 'disabled')">
        <div class="flex items-center gap-3">
          <div class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center flex-shrink-0">
            <Icon name="ri:close-circle-line" class="text-xl text-gray-600" />
          </div>
          <div>
            <p class="text-xs text-gray-600">Отключенные</p>
            <p class="text-xl font-bold text-gray-600">{{ stats.disabled }}</p>
          </div>
        </div>
      </div>

      <div class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors">
        <div class="flex items-center gap-3">
          <div class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center flex-shrink-0 relative">
            <Icon name="ri:signal-wifi-line" class="text-xl text-purple-600" />
            <span v-if="stats.online > 0" class="absolute -top-1 -right-1 w-3 h-3 bg-green-500 rounded-full animate-pulse" />
          </div>
          <div>
            <p class="text-xs text-gray-600">Онлайн</p>
            <p class="text-xl font-bold text-purple-600">{{ stats.online }}</p>
          </div>
        </div>
      </div>

      <div class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors">
        <div class="flex items-center gap-3">
          <div class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center flex-shrink-0">
            <Icon name="ri:router-line" class="text-xl text-blue-600" />
          </div>
          <div>
            <p class="text-xs text-gray-600">Интерфейсы</p>
            <p class="text-xl font-bold text-blue-600">{{ interfacesCount }}</p>
          </div>
        </div>
      </div>

      <div class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors">
        <div class="flex items-center gap-3">
          <div class="h-10 w-10 rounded-lg bg-amber-100 flex items-center justify-center flex-shrink-0">
            <Icon name="ri:database-line" class="text-xl text-amber-600" />
          </div>
          <div>
            <p class="text-xs text-gray-600">Всего</p>
            <p class="text-xl font-bold text-amber-600">{{ stats.total }}</p>
          </div>
        </div>
      </div>
    </div>
  </UiCard>
</template>

<script setup lang="ts">
interface Stats {
  total: number
  active: number
  disabled: number
  online: number
}

interface Props {
  stats: Stats
  interfacesCount: number
}

defineProps<Props>()

defineEmits<{
  filterStatus: [status: string]
}>()
</script>
