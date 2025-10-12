<template>
  <div class="flex items-start gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors">
    <div
      :class="[
        'flex h-8 w-8 items-center justify-center rounded-lg flex-shrink-0',
        iconBgColor
      ]"
    >
      <Icon :name="iconName" :class="['text-lg', iconColor]" />
    </div>
    <div class="flex-1 min-w-0">
      <p class="text-sm text-gray-900">{{ message }}</p>
      <p class="text-xs text-gray-500 mt-0.5">{{ time }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
const props = defineProps<{
  type: 'info' | 'warning' | 'success' | 'error'
  message: string
  time: string
}>()

const iconName = computed(() => {
  const icons = {
    info: 'ri:information-line',
    warning: 'ri:alert-line',
    success: 'ri:checkbox-circle-line',
    error: 'ri:error-warning-line'
  }
  return icons[props.type]
})

const iconBgColor = computed(() => {
  const colors = {
    info: 'bg-blue-100',
    warning: 'bg-amber-100',
    success: 'bg-green-100',
    error: 'bg-red-100'
  }
  return colors[props.type]
})

const iconColor = computed(() => {
  const colors = {
    info: 'text-blue-600',
    warning: 'text-amber-600',
    success: 'text-green-600',
    error: 'text-red-600'
  }
  return colors[props.type]
})
</script>
