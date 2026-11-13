<template>
  <div class="card">
    <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
      <div class="flex items-center gap-2">
        <Icon :name="osIcon" class="text-gray-600" />
        <h2 class="text-lg font-semibold text-gray-900">Операционная система</h2>
      </div>
    </div>
    <div class="p-6">
      <div class="grid grid-cols-2 gap-6">
        <div>
          <p class="text-xs text-gray-600 mb-1">ОС</p>
          <p class="text-sm font-medium text-gray-900">{{ computer.os || '—' }}</p>
        </div>
        <div>
          <p class="text-xs text-gray-600 mb-1">Версия</p>
          <p class="text-sm font-medium text-gray-900">{{ computer.os_version || '—' }}</p>
        </div>
        <div>
          <p class="text-xs text-gray-600 mb-1">Сборка</p>
          <p class="text-sm font-medium text-gray-900">{{ computer.os_build || '-' }}</p>
        </div>
        <div>
          <p class="text-xs text-gray-600 mb-1">Архитектура</p>
          <p class="text-sm font-medium text-gray-900">{{ computer.os_architecture || '-' }}</p>
        </div>
        <div>
          <p class="text-xs text-gray-600 mb-1">Последняя загрузка</p>
          <p class="text-sm font-medium text-gray-900">{{ computer.last_boot_time || '-' }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface ComputerProps {
  computer: {
    os: string | null
    os_version: string | null
    os_build: string | null
    os_architecture: string | null
    last_boot_time: string | null
  }
}

const props = withDefaults(defineProps<ComputerProps>(), {})

const osIcon = computed(() => {
  const os = props.computer.os || ''
  if (os.includes('Windows')) return 'ri:windows-line'
  if (os.includes('Linux')) return 'ri:ubuntu-line'
  if (os.includes('macOS')) return 'ri:apple-line'
  return 'ri:computer-line'
})
</script>
