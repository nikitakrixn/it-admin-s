<template>
  <div class="w-full">
    <label v-if="label" :for="inputId" class="block text-sm font-medium text-gray-700 mb-1">
      {{ label }}
      <span v-if="required" class="text-red-500">*</span>
    </label>
    
    <div class="relative">
      <div v-if="icon || $slots.icon" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400">
        <slot name="icon">
          <Icon v-if="icon" :name="icon" />
        </slot>
      </div>
      
      <input
        :id="inputId"
        :type="type"
        :value="modelValue"
        :placeholder="placeholder"
        :disabled="disabled"
        :required="required"
        :readonly="readonly"
        :class="inputClasses"
        @input="handleInput"
        @blur="$emit('blur', $event)"
        @focus="$emit('focus', $event)"
      />
      
      <div v-if="$slots.suffix" class="absolute right-3 top-1/2 -translate-y-1/2">
        <slot name="suffix" />
      </div>
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
  modelValue?: string | number
  type?: 'text' | 'email' | 'password' | 'number' | 'tel' | 'url'
  label?: string
  placeholder?: string
  hint?: string
  error?: string
  icon?: string
  disabled?: boolean
  required?: boolean
  readonly?: boolean
  size?: 'sm' | 'md' | 'lg'
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  size: 'md',
  disabled: false,
  required: false,
  readonly: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: string | number]
  blur: [event: FocusEvent]
  focus: [event: FocusEvent]
}>()

const inputId = useId()

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  const value = props.type === 'number' ? Number(target.value) : target.value
  emit('update:modelValue', value)
}

const slots = useSlots()

const inputClasses = computed(() => {
  const base = 'w-full border rounded-lg transition-all focus:outline-none focus:ring-2 focus:ring-offset-0'
  
  const sizes = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-3 py-2 text-base',
    lg: 'px-4 py-3 text-lg',
  }
  
  const states = props.error
    ? 'border-red-300 focus:border-red-500 focus:ring-red-500'
    : 'border-gray-300 focus:border-primary-500 focus:ring-primary-500'
  
  const disabled = props.disabled ? 'bg-gray-100 cursor-not-allowed' : 'bg-white'
  const readonly = props.readonly ? 'bg-gray-50' : ''
  const withIcon = props.icon || slots.icon ? 'pl-10' : ''
  const withSuffix = slots.suffix ? 'pr-10' : ''
  
  return [base, sizes[props.size], states, disabled, readonly, withIcon, withSuffix].join(' ')
})
</script>
