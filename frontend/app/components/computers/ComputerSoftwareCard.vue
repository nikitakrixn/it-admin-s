<template>
  <div class="card">
    <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <Icon name="ri:apps-line" class="text-gray-600" />
          <h2 class="text-lg font-semibold text-gray-900">Установленное ПО</h2>
          <span class="badge bg-primary-100 text-primary-800">{{ software.length }}</span>
        </div>
        <NuxtLink :to="`/computers/${computerId}/software`" class="btn btn-secondary btn-sm">
          <Icon name="ri:list-check" class="mr-2" />
          Управление
        </NuxtLink>
      </div>
    </div>
    <div class="p-6">
      <div v-if="software.length" class="space-y-2">
        <NuxtLink
          v-for="item in software"
          :key="item.id"
          :to="`/software/${item.software_id}`"
          class="flex items-center justify-between p-3 rounded-lg hover:bg-gray-50 transition-colors"
        >
          <div class="flex items-center gap-3">
            <Icon name="ri:apps-2-line" class="text-gray-400" />
            <div>
              <p class="text-sm font-medium text-gray-900 hover:text-primary-600 transition-colors">
                {{ item.name }}
              </p>
              <p class="text-xs text-gray-500">
                {{ item.manufacturer || 'Производитель не указан' }}
              </p>
            </div>
          </div>
          <div class="text-right">
            <p class="text-sm font-medium text-gray-900">{{ item.version || '—' }}</p>
            <span v-if="item.license_type" class="text-xs text-gray-500">
              {{ getLicenseLabel(item.license_type) }}
            </span>
          </div>
        </NuxtLink>
      </div>
      <div v-else class="text-center py-6">
        <Icon name="ri:apps-line" class="text-3xl text-gray-300 mb-2" />
        <p class="text-sm text-gray-500">На компьютер ещё не установлено ПО</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { InstalledSoftware } from '~/composables/useComputerComponents'

interface Props {
  software: InstalledSoftware[]
  computerId: number
}

defineProps<Props>()

const getLicenseLabel = (type: string) => {
  const labels: Record<string, string> = {
    perpetual: 'Бессрочная',
    subscription: 'Подписка',
    trial: 'Пробная',
    free: 'Бесплатная',
    oem: 'OEM',
    volume: 'Корпоративная',
    unknown: 'Не указана',
  }
  return labels[type] || type
}
</script>
