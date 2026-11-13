<template>
  <div class="p-5 rounded-xl border" :class="bgClass">
    <div class="flex items-center gap-3 mb-3">
      <div class="w-10 h-10 rounded-lg flex items-center justify-center" :class="iconBgClass">
        <Icon :name="icon" class="text-xl" :class="iconTextClass" />
      </div>
      <div>
        <p class="text-xs text-gray-600">{{ title }}</p>
        <p class="text-sm font-semibold text-gray-900">{{ value }}</p>
      </div>
    </div>
    <div class="flex items-center text-xs text-gray-500">
      <Icon name="ri:check-line" class="mr-1" />
      {{ subtitle }}
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title: string
  value: string
  subtitle: string
  icon: string
  color?: string
}

const props = withDefaults(defineProps<Props>(), {
  color: 'blue',
})

const bgClass = computed(
  () => 'bg-' + props.color + '-50 to-' + props.color + '-100 border-' + props.color + '-100',
)
const iconBgClass = computed(() => 'bg-' + props.color + '-100')
const iconTextClass = computed(() => 'text-' + props.color + '-600')
</script>
