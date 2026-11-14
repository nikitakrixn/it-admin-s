<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4">
        <div
          class="h-16 w-16 rounded-2xl bg-gradient-to-br from-amber-500 to-orange-600 flex items-center justify-center shadow-lg"
        >
          <Icon name="ri:lock-password-line" class="text-3xl text-white" />
        </div>
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Смена пароля</h1>
          <p class="mt-1 text-gray-600">Пароль для входа в панель администрирования</p>
        </div>
      </div>
    </div>

    <div class="card max-w-xl">
      <form class="p-6 space-y-4" @submit.prevent="handleSubmit">
        <div>
          <label for="current-password" class="block text-sm font-medium text-gray-700 mb-1">
            Текущий пароль
          </label>
          <input
            id="current-password"
            v-model="form.currentPassword"
            type="password"
            required
            autocomplete="current-password"
            class="input-field"
          />
        </div>

        <div>
          <label for="new-password" class="block text-sm font-medium text-gray-700 mb-1">
            Новый пароль
          </label>
          <input
            id="new-password"
            v-model="form.newPassword"
            type="password"
            required
            minlength="12"
            autocomplete="new-password"
            class="input-field"
          />
          <p class="mt-1 text-xs text-gray-500">
            Не короче 12 символов, обязательно буква и цифра.
          </p>
        </div>

        <div>
          <label for="confirm-password" class="block text-sm font-medium text-gray-700 mb-1">
            Повторите новый пароль
          </label>
          <input
            id="confirm-password"
            v-model="form.confirmPassword"
            type="password"
            required
            autocomplete="new-password"
            class="input-field"
          />
        </div>

        <div class="flex items-center gap-3 pt-2">
          <button type="submit" class="btn btn-primary" :disabled="saving">
            <Icon v-if="saving" name="ri:loader-4-line" class="mr-2 animate-spin" />
            <Icon v-else name="ri:save-line" class="mr-2" />
            {{ saving ? 'Сохранение...' : 'Изменить пароль' }}
          </button>
          <NuxtLink to="/" class="btn btn-secondary">Отмена</NuxtLink>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ middleware: 'auth' })
useHead({ title: 'Смена пароля' })

const toast = useToast()
const { changePassword } = useAuth()

const form = ref({
  currentPassword: '',
  newPassword: '',
  confirmPassword: '',
})

const saving = ref(false)

const handleSubmit = async () => {
  if (form.value.newPassword !== form.value.confirmPassword) {
    toast.error('Пароли не совпадают')
    return
  }

  saving.value = true
  try {
    await changePassword({
      currentPassword: form.value.currentPassword,
      newPassword: form.value.newPassword,
    })

    form.value = { currentPassword: '', newPassword: '', confirmPassword: '' }
    toast.success('Пароль изменён')
  } catch (error) {
    toast.error(getErrorMessage(error, 'Не удалось изменить пароль'))
  } finally {
    saving.value = false
  }
}
</script>
