<template>
  <div :class="cardClasses">
    <div v-if="$slots.header || title" :class="headerClasses">
      <slot name="header">
        <h3 class="text-lg font-semibold text-gray-900">{{ title }}</h3>
      </slot>
    </div>
    
    <div :class="bodyClasses">
      <slot />
    </div>
    
    <div v-if="$slots.footer" :class="footerClasses">
      <slot name="footer" />
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title?: string
  variant?: 'default' | 'bordered' | 'elevated'
  padding?: 'none' | 'sm' | 'md' | 'lg'
  hoverable?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'default',
  padding: 'md',
  hoverable: false,
})

const cardClasses = computed(() => {
  const base = 'bg-white rounded-lg overflow-hidden border border-gray-200'
  
  const variants = {
    default: '',
    bordered: 'border-2',
    elevated: 'shadow-lg',
  }
  
  const hover = props.hoverable ? 'hover:shadow-md transition-shadow cursor-pointer' : ''
  
  return [base, variants[props.variant], hover].join(' ')
})

const headerClasses = computed(() => {
  const paddings = {
    none: '',
    sm: 'p-3',
    md: 'p-4',
    lg: 'p-6',
  }
  return `border-b border-gray-200 ${paddings[props.padding]}`
})

const bodyClasses = computed(() => {
  const paddings = {
    none: '',
    sm: 'p-3',
    md: 'p-4',
    lg: 'p-6',
  }
  return paddings[props.padding]
})

const footerClasses = computed(() => {
  const paddings = {
    none: '',
    sm: 'p-3',
    md: 'p-4',
    lg: 'p-6',
  }
  return `border-t border-gray-200 bg-gray-50 ${paddings[props.padding]}`
})
</script>
