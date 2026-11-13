<template>
  <div
    class="flex items-start gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors cursor-pointer"
  >
    <div
      class="flex-shrink-0 w-12 h-12 rounded-lg flex flex-col items-center justify-center"
      :class="['bg-' + iconColor + '-100 text-' + iconColor + '-600']"
    >
      <span class="text-xs font-medium">{{ dateMonth }}</span>
      <span class="text-lg font-bold">{{ dateDay }}</span>
    </div>
    <div class="flex-1 min-w-0">
      <p class="text-sm font-medium text-gray-900">{{ title }}</p>
      <p class="text-xs text-gray-500 mt-1">{{ description }}</p>
    </div>
    <span class="badge text-xs" :class="badgeClass">{{ badge }}</span>
  </div>
</template>

<script setup lang="ts">
interface Props {
  dateMonth: string
  dateDay: number | string
  title: string
  description: string
  badge: string
  badgeColor: string
  icon: string
  iconColor: string
}

const props = defineProps<Props>()

const badgeClass = computed(
  () => 'bg-' + props.badgeColor + '-100 text-' + props.badgeColor + '-800',
)
</script>
