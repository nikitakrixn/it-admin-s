<template>
  <div :class="classes" />
</template>

<script setup lang="ts">
interface Props {
  width?: string
  height?: string
  circle?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  circle: false,
})

const classes = computed(() => {
  const base = 'animate-pulse bg-gray-200 rounded'
  const result = [base]
  
  if (props.circle) {
    result.push('rounded-full')
  }
  
  return result.join(' ')
})
</script>
