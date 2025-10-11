<template>
  <Teleport to="body">
    <Transition
      enter-active-class="transition ease-out duration-200"
      enter-from-class="opacity-0"
      enter-to-class="opacity-100"
      leave-active-class="transition ease-in duration-150"
      leave-from-class="opacity-100"
      leave-to-class="opacity-0"
    >
      <div
        v-if="show"
        class="fixed inset-0 z-[60] overflow-y-auto"
        @click.self="onCancel"
      >
        <!-- Backdrop -->
        <div class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm"></div>

        <!-- Modal Container -->
        <div class="flex min-h-full items-center justify-center p-4">
          <Transition
            enter-active-class="transition ease-out duration-200"
            enter-from-class="opacity-0 scale-95"
            enter-to-class="opacity-100 scale-100"
            leave-active-class="transition ease-in duration-150"
            leave-from-class="opacity-100 scale-100"
            leave-to-class="opacity-0 scale-95"
          >
            <div
              v-if="show"
              class="relative bg-white rounded-2xl shadow-2xl max-w-md w-full overflow-hidden"
              @click.stop
            >
              <!-- Icon -->
              <div class="pt-8 pb-4 px-6 text-center">
                <div
                  :class="[
                    'mx-auto h-16 w-16 rounded-full flex items-center justify-center mb-4',
                    variant === 'danger' ? 'bg-red-100' : 'bg-yellow-100'
                  ]"
                >
                  <Icon
                    :name="icon"
                    :class="[
                      'text-4xl',
                      variant === 'danger' ? 'text-red-600' : 'text-yellow-600'
                    ]"
                  />
                </div>

                <!-- Title -->
                <h3 class="text-xl font-bold text-gray-900 mb-2">
                  {{ title }}
                </h3>

                <!-- Message -->
                <p class="text-sm text-gray-600">
                  {{ message }}
                </p>
              </div>

              <!-- Actions -->
              <div class="bg-gray-50 px-6 py-4 flex flex-col-reverse sm:flex-row sm:justify-end gap-3">
                <button
                  type="button"
                  @click="onCancel"
                  :disabled="loading"
                  class="inline-flex justify-center items-center px-5 py-2.5 border border-gray-300 rounded-lg shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
                >
                  {{ cancelText }}
                </button>
                <button
                  type="button"
                  @click="onConfirm"
                  :disabled="loading"
                  :class="[
                    'inline-flex justify-center items-center px-5 py-2.5 border border-transparent rounded-lg shadow-sm text-sm font-medium text-white focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transition-all',
                    variant === 'danger'
                      ? 'bg-gradient-to-r from-red-600 to-red-700 hover:from-red-700 hover:to-red-800 focus:ring-red-500'
                      : 'bg-gradient-to-r from-yellow-600 to-yellow-700 hover:from-yellow-700 hover:to-yellow-800 focus:ring-yellow-500'
                  ]"
                >
                  <Icon v-if="loading" name="ri:loader-4-line" class="mr-2 animate-spin" />
                  {{ loading ? loadingText : confirmText }}
                </button>
              </div>
            </div>
          </Transition>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
interface Props {
  show: boolean
  title?: string
  message?: string
  confirmText?: string
  cancelText?: string
  loadingText?: string
  loading?: boolean
  variant?: 'danger' | 'warning'
  icon?: string
}

const props = withDefaults(defineProps<Props>(), {
  title: 'Подтвердите действие',
  message: 'Вы уверены, что хотите выполнить это действие?',
  confirmText: 'Подтвердить',
  cancelText: 'Отмена',
  loadingText: 'Выполнение...',
  loading: false,
  variant: 'danger',
  icon: 'ri:error-warning-line'
})

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()

const onConfirm = () => {
  if (!props.loading) {
    emit('confirm')
  }
}

const onCancel = () => {
  if (!props.loading) {
    emit('cancel')
  }
}

// Close on Escape key
onMounted(() => {
  const handleEscape = (e: KeyboardEvent) => {
    if (e.key === 'Escape' && props.show && !props.loading) {
      onCancel()
    }
  }
  window.addEventListener('keydown', handleEscape)
  onUnmounted(() => {
    window.removeEventListener('keydown', handleEscape)
  })
})
</script>
