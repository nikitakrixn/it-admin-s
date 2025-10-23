<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="modelValue" class="fixed inset-0 z-50 overflow-y-auto">
        <div
          class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm transition-opacity"
          @click="handleBackdropClick"
        />
        <div class="flex min-h-full items-center justify-center p-4">
          <div
            :class="modalClasses"
            class="relative bg-white rounded-2xl shadow-2xl transform transition-all"
          >
            <div
              v-if="$slots.header || title"
              class="flex items-center justify-between p-6 border-b border-gray-200"
            >
              <slot name="header">
                <h3 class="text-xl font-semibold text-gray-900">
                  {{ title }}
                </h3>
              </slot>
              <button
                v-if="closable"
                @click="close"
                class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
              >
                <Icon name="ri:close-line" class="text-xl" />
              </button>
            </div>

            <div :class="bodyClasses">
              <slot />
            </div>

            <div
              v-if="$slots.footer"
              class="flex items-center justify-end gap-3 p-6 border-t border-gray-200 bg-gray-50"
            >
              <slot name="footer" />
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
interface Props {
  modelValue: boolean
  title?: string
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full'
  closable?: boolean
  closeOnBackdrop?: boolean
  noPadding?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  size: 'md',
  closable: true,
  closeOnBackdrop: true,
  noPadding: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  close: []
}>()

const close = () => {
  emit('update:modelValue', false)
  emit('close')
}

const handleBackdropClick = () => {
  if (props.closeOnBackdrop) {
    close()
  }
}

const modalClasses = computed(() => {
  const sizes = {
    sm: 'max-w-md',
    md: 'max-w-2xl',
    lg: 'max-w-4xl',
    xl: 'max-w-6xl',
    full: 'max-w-full mx-4',
  }
  return `w-full ${sizes[props.size]}`
})

const bodyClasses = computed(() => {
  return props.noPadding ? '' : 'p-6'
})

watch(() => props.modelValue, (isOpen) => {
  if (isOpen) {
    document.body.style.overflow = 'hidden'
  } else {
    document.body.style.overflow = ''
  }
})

onUnmounted(() => {
  document.body.style.overflow = ''
})
</script>

<style scoped>
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>
