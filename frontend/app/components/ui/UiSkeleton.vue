<template>
  <div :class="classes"></div>
</template>

<script setup lang="ts">
interface Props {
  width?: string
  height?: string
  circle?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  width: undefined,
  height: undefined,
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
