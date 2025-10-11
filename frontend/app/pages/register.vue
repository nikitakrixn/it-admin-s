<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <div class="mx-auto h-16 w-16 flex items-center justify-center rounded-full bg-primary-100">
          <Icon name="ri:user-add-line" class="h-10 w-10 text-primary-600" />
        </div>
        <h2 class="mt-6 text-center text-3xl font-bold text-gray-900">
          Регистрация
        </h2>
        <p class="mt-2 text-center text-sm text-gray-600">
          Создайте новый аккаунт
        </p>
      </div>

      <form class="mt-8 space-y-6" @submit.prevent="handleRegister">
        <div v-if="error" class="rounded-md bg-red-50 p-4">
          <div class="flex">
            <div class="flex-shrink-0">
              <Icon name="ri:error-warning-line" class="h-5 w-5 text-red-400" />
            </div>
            <div class="ml-3">
              <p class="text-sm font-medium text-red-800">{{ error }}</p>
            </div>
          </div>
        </div>

        <div class="rounded-md shadow-sm space-y-3">
          <div>
            <label for="name" class="sr-only">Имя</label>
            <input
              id="name"
              v-model="credentials.name"
              name="name"
              type="text"
              required
              class="appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-primary-500 focus:border-primary-500 focus:z-10 sm:text-sm"
              placeholder="Имя"
            />
          </div>
          <div>
            <label for="email" class="sr-only">Email</label>
            <input
              id="email"
              v-model="credentials.email"
              name="email"
              type="email"
              autocomplete="email"
              required
              class="appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-primary-500 focus:border-primary-500 focus:z-10 sm:text-sm"
              placeholder="Email"
            />
          </div>
          <div>
            <label for="password" class="sr-only">Пароль</label>
            <input
              id="password"
              v-model="credentials.password"
              name="password"
              type="password"
              autocomplete="new-password"
              required
              class="appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-primary-500 focus:border-primary-500 focus:z-10 sm:text-sm"
              placeholder="Пароль (минимум 8 символов)"
            />
          </div>
        </div>

        <div>
          <button
            type="submit"
            :disabled="loading"
            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <span v-if="loading" class="absolute left-0 inset-y-0 flex items-center pl-3">
              <Icon name="ri:loader-4-line" class="h-5 w-5 text-primary-300 animate-spin" />
            </span>
            <span v-else class="absolute left-0 inset-y-0 flex items-center pl-3">
              <Icon name="ri:user-add-line" class="h-5 w-5 text-primary-300 group-hover:text-primary-200" />
            </span>
            {{ loading ? 'Регистрация...' : 'Зарегистрироваться' }}
          </button>
        </div>

        <div class="text-center">
          <NuxtLink to="/login" class="text-sm font-medium text-primary-600 hover:text-primary-500">
            Уже есть аккаунт? Войти
          </NuxtLink>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'guest',
  layout: false
})

useHead({
  title: 'Регистрация'
})

const { register } = useAuth()

const credentials = ref({
  name: '',
  email: '',
  password: ''
})

const loading = ref(false)
const error = ref('')

const handleRegister = async () => {
  if (credentials.value.password.length < 8) {
    error.value = 'Пароль должен содержать минимум 8 символов'
    return
  }

  loading.value = true
  error.value = ''

  try {
    await register(credentials.value)
    await navigateTo('/')
  } catch (err: any) {
    error.value = err.message || 'Ошибка регистрации'
  } finally {
    loading.value = false
  }
}
</script>
