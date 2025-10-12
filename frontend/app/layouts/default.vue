<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-50 via-white to-gray-50">
    <div v-if="sidebarOpen" class="fixed inset-0 z-50 lg:hidden">
      <div class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm" @click="sidebarOpen = false" />
      <div class="fixed inset-y-0 left-0 w-72 bg-white shadow-2xl">
        <div class="flex h-full flex-col">
          <div class="flex h-16 items-center justify-between px-6 border-b border-gray-200">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-xl gradient-primary flex items-center justify-center">
                <Icon name="ri:shield-check-line" class="text-xl text-white" />
              </div>
              <div>
                <h1 class="text-lg font-bold text-gray-900">IT-Admin</h1>
                <p class="text-xs text-gray-500">v2.0</p>
              </div>
            </div>
            <button @click="sidebarOpen = false" class="p-2 rounded-lg hover:bg-gray-100">
              <Icon name="ri:close-line" class="text-xl" />
            </button>
          </div>
          <nav class="flex-1 overflow-y-auto p-4 scrollbar-thin">
            <SidebarMenu @navigate="sidebarOpen = false" />
          </nav>
          <div class="border-t border-gray-200 p-4">
            <UserProfile />
          </div>
        </div>
      </div>
    </div>

    <div class="hidden lg:fixed lg:inset-y-0 lg:flex lg:w-64 lg:flex-col">
      <div class="flex flex-col flex-1 bg-white border-r border-gray-200 shadow-sm">
        <div class="flex h-16 items-center px-6 border-b border-gray-200">
          <div class="flex items-center gap-3">
            <div class="h-10 w-10 rounded-xl gradient-primary flex items-center justify-center shadow-lg">
              <Icon name="ri:shield-check-line" class="text-xl text-white" />
            </div>
            <div>
              <h1 class="text-lg font-bold text-gray-900">IT-Admin</h1>
              <p class="text-xs text-gray-500">v2.0</p>
            </div>
          </div>
        </div>
        <nav class="flex-1 overflow-y-auto p-4 scrollbar-thin">
          <SidebarMenu />
        </nav>
        <div class="border-t border-gray-200 p-4">
          <UserProfile />
        </div>
      </div>
    </div>

    <div class="lg:pl-64">
      <header class="sticky top-0 z-40 flex h-16 items-center gap-4 border-b border-gray-200 bg-white/80 backdrop-blur-md px-4 shadow-sm lg:px-6">
        <button @click="sidebarOpen = true" class="p-2 rounded-lg hover:bg-gray-100 lg:hidden">
          <Icon name="ri:menu-line" class="text-xl" />
        </button>

        <div class="flex flex-1 items-center justify-between gap-4">
          <button
            @click="showSearch = true"
            class="flex-1 max-w-md flex items-center gap-3 px-4 py-2 bg-gray-50 hover:bg-gray-100 rounded-lg transition-colors text-left"
          >
            <Icon name="ri:search-line" class="text-gray-400" />
            <span class="text-sm text-gray-500">Поиск...</span>
            <kbd class="ml-auto px-2 py-1 text-xs bg-white rounded border border-gray-200">Ctrl K</kbd>
          </button>

          <div class="flex items-center gap-2">
            <button
              @click="toggleTheme"
              class="relative p-2 rounded-lg hover:bg-gray-100 transition-colors"
              title="Переключить тему"
            >
              <Icon :name="isDark ? 'ri:sun-line' : 'ri:moon-line'" class="text-xl text-gray-600" />
            </button>

            <button
              @click="showNotifications = !showNotifications"
              class="relative p-2 rounded-lg hover:bg-gray-100 transition-colors"
              title="Уведомления"
            >
              <Icon name="ri:notification-3-line" class="text-xl text-gray-600" />
              <span class="absolute top-1 right-1 h-2 w-2 rounded-full bg-red-500 animate-pulse" />
            </button>
          </div>
        </div>
      </header>

      <main class="p-4 lg:p-6">
        <slot />
      </main>
    </div>

    <SearchModal :show="showSearch" @close="showSearch = false" />
    
    <Teleport to="body">
      <Transition name="slide-left">
        <div v-if="showNotifications" class="fixed inset-y-0 right-0 w-96 bg-white shadow-2xl z-50 border-l border-gray-200">
          <div class="flex h-full flex-col">
            <div class="flex items-center justify-between p-4 border-b border-gray-200">
              <h2 class="text-lg font-semibold text-gray-900">Уведомления</h2>
              <button @click="showNotifications = false" class="p-2 rounded-lg hover:bg-gray-100">
                <Icon name="ri:close-line" class="text-xl" />
              </button>
            </div>
            <div class="flex-1 overflow-y-auto p-4 space-y-3 scrollbar-thin">
              <NotificationItem
                type="info"
                message="Обновление системы доступно"
                time="5 минут назад"
              />
              <NotificationItem
                type="warning"
                message="Истекает срок лицензии Office 365"
                time="1 час назад"
              />
              <NotificationItem
                type="success"
                message="Резервная копия завершена"
                time="2 часа назад"
              />
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <Toast />
  </div>
</template>

<script setup lang="ts">
const sidebarOpen = ref(false)
const showSearch = ref(false)
const showNotifications = ref(false)
const isDark = ref(false)

const toggleTheme = () => {
  isDark.value = !isDark.value
  const toast = useToast()
  toast.info(`Тема ${isDark.value ? 'темная' : 'светлая'} будет доступна в следующей версии`)
}

onMounted(() => {
  const handleKeydown = (e: KeyboardEvent) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
      e.preventDefault()
      showSearch.value = true
    }
  }
  
  window.addEventListener('keydown', handleKeydown)
  
  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeydown)
  })
})
</script>

<style scoped>
.slide-left-enter-active,
.slide-left-leave-active {
  transition: transform 0.3s ease;
}

.slide-left-enter-from,
.slide-left-leave-to {
  transform: translateX(100%);
}
</style>
