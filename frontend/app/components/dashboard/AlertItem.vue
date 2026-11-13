<template>
  <div class="flex items-start gap-3 p-3 rounded-lg" :class="cardClass">
    <div class="flex-shrink-0">
      <Icon :name="icon" class="text-xl" :class="iconClass" />
    </div>
    <div class="flex-1 min-w-0">
      <p class="text-sm font-medium text-gray-900">{{ title }}</p>
      <p class="text-xs text-gray-600 mt-1">{{ description }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title: string
  description: string
  type?: 'error' | 'warning' | 'info'
  icon: string
}

const props = withDefaults(defineProps<Props>(), {
  type: 'info',
})

const cardClass = computed(() => {
  const base = 'p-3 rounded-lg border'
  const map: Record<string, string> = {
    error: 'bg-red-50 border-red-100',
    warning: 'bg-amber-50 border-amber-100',
    info: 'bg-blue-50 border-blue-100',
  }
  return base + ' ' + (map[props.type] || map.info)
})

const iconClass = computed(() => {
  const map: Record<string, string> = {
    error: 'text-red-600',
    warning: 'text-amber-600',
    info: 'text-blue-600',
  }
  return map[props.type] || map.info
})
</script>
