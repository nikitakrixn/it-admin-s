<template>
  <div :class="fullWidth ? 'w-full' : 'inline-block'">
    <label v-if="label" :for="selectId" class="block text-sm font-medium text-gray-700 mb-1">
      {{ label }}
      <span v-if="required" class="text-red-500">*</span>
    </label>
    
    <div class="relative">
      <select
        :id="selectId"
        :value="modelValue"
        :disabled="disabled"
        :required="required"
        :class="selectClasses"
        @change="handleChange"
      >
        <option v-if="placeholder" value="">{{ placeholder }}</option>
        <slot />
      </select>
    </div>
    
    <p v-if="hint" class="mt-1 text-xs text-gray-500">
      {{ hint }}
    </p>
    
    <p v-if="error" class="mt-1 text-xs text-red-600">
      {{ error }}
    </p>
  </div>
</template>

<script setup lang="ts">
interface Props {
  modelValue?: string | number | null
  label?: string
  placeholder?: string
  hint?: string
  error?: string
  disabled?: boolean
  required?: boolean
  size?: 'sm' | 'md' | 'lg'
  fullWidth?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  size: 'md',
  disabled: false,
  required: false,
  fullWidth: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: string | number | null]
}>()

const selectId = useId()

const handleChange = (event: Event) => {
  const target = event.target as HTMLSelectElement
  const value = target.value === '' ? null : target.value
  emit('update:modelValue', value)
}

const selectClasses = computed(() => {
  const base = 'border rounded-lg transition-all focus:outline-none focus:ring-2 focus:ring-offset-0 appearance-none pr-10'
  const width = props.fullWidth ? 'w-full' : ''
  
  const sizes = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-3 py-2 text-base',
    lg: 'px-4 py-3 text-lg',
  }
  
  const states = props.error
    ? 'border-red-300 focus:border-red-500 focus:ring-red-500'
    : 'border-gray-300 focus:border-primary-500 focus:ring-primary-500'
  
  const disabled = props.disabled ? 'bg-gray-100 cursor-not-allowed' : ''
  
  return [base, width, sizes[props.size], states, disabled].join(' ')
})
</script>