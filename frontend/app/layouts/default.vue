<template>
  <div class="min-h-screen bg-gray-100">
    <!-- Mobile sidebar -->
    <div v-if="sidebarOpen" class="relative z-50 lg:hidden">
      <div class="fixed inset-0 bg-gray-900/80 backdrop-blur-sm" @click="sidebarOpen = false"></div>
      
      <div class="fixed inset-0 flex">
        <div class="relative mr-16 flex w-full max-w-xs flex-1 transform transition-transform duration-300">
          <div class="absolute left-full top-0 flex w-16 justify-center pt-5">
            <button @click="sidebarOpen = false" class="-m-2.5 p-2.5 text-white hover:bg-primary-700/70 rounded-full">
              <span class="sr-only">Закрыть меню</span>
              <Icon name="ri:close-line" class="text-2xl" />
            </button>
          </div>

          <div class="flex grow flex-col gap-y-5 overflow-y-auto bg-gradient-to-b from-primary-700 to-primary-900 px-6 pb-4">
            <div class="flex h-16 shrink-0 items-center">
              <h1 class="text-xl font-bold text-white">IT-Админ v2</h1>
            </div>
            <nav class="flex flex-1 flex-col">
              <ul role="list" class="flex flex-1 flex-col gap-y-7">
                <li>
                  <ul role="list" class="-mx-2 space-y-1">
                    <MenuItem to="/dashboard" icon="ri:dashboard-line" label="Главная" />
                    <MenuItem to="/employees" icon="ri:user-line" label="Сотрудники" />
                    <MenuItem to="/computers" icon="ri:computer-line" label="Компьютеры" />
                    <MenuItem to="/equipment" icon="ri:printer-line" label="Оргтехника" />
                    <MenuItem to="/consumables" icon="ri:ink-bottle-line" label="Расходники" />
                    <MenuItem to="/requests" icon="ri:file-list-line" label="Заявки" />
                    <MenuItem to="/network" icon="ri:router-line" label="Сеть" />
                    <MenuItem to="/ad-management" icon="ri:account-circle-line" label="Active Directory" />
                    <MenuItem to="/security" icon="ri:shield-line" label="Безопасность" />
                    <MenuItem to="/reports" icon="ri:bar-chart-line" label="Отчеты" />
                    <MenuItem to="/logs" icon="ri:history-line" label="Журнал активности" />
                  </ul>
                </li>
              </ul>
            </nav>
          </div>
        </div>
      </div>
    </div>

    <!-- Desktop sidebar -->
    <div class="hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-72 lg:flex-col">
      <div class="flex grow flex-col gap-y-5 overflow-y-auto bg-gradient-to-b from-primary-700 to-primary-900 px-6 pb-4">
        <div class="flex h-16 shrink-0 items-center">
          <h1 class="text-xl font-bold text-white">IT-Админ v2</h1>
        </div>
        <nav class="flex flex-1 flex-col">
          <ul role="list" class="flex flex-1 flex-col gap-y-7">
            <li>
              <ul role="list" class="-mx-2 space-y-1">
                <MenuItem to="/dashboard" icon="ri:dashboard-line" label="Главная" />
                <MenuItem to="/employees" icon="ri:user-line" label="Сотрудники" />
                <MenuItem to="/computers" icon="ri:computer-line" label="Компьютеры" />
                <MenuItem to="/equipment" icon="ri:printer-line" label="Оргтехника" />
                <MenuItem to="/consumables" icon="ri:ink-bottle-line" label="Расходники" />
                <MenuItem to="/requests" icon="ri:file-list-line" label="Заявки" />
                <MenuItem to="/network" icon="ri:router-line" label="Сеть" />
                <MenuItem to="/ad-management" icon="ri:account-circle-line" label="Active Directory" />
                <MenuItem to="/security" icon="ri:shield-line" label="Безопасность" />
                <MenuItem to="/reports" icon="ri:bar-chart-line" label="Отчеты" />
                <MenuItem to="/logs" icon="ri:history-line" label="Журнал активности" />
              </ul>
            </li>
            
            <!-- User profile -->
            <li class="mt-auto border-t border-primary-600/30 pt-4">
              <div class="group flex gap-x-3 rounded-md p-2 text-sm font-medium text-white">
                <div class="flex w-full items-center justify-between">
                  <div class="flex min-w-0 items-center gap-x-3">
                    <div class="h-9 w-9 rounded-full bg-primary-600 flex items-center justify-center text-white shadow-inner">
                      <span class="text-sm font-medium">{{ user?.email?.charAt(0).toUpperCase() || 'U' }}</span>
                    </div>
                    <div class="min-w-0 flex-1">
                      <div class="text-sm font-medium text-white truncate">{{ user?.email || 'Loading...' }}</div>
                      <button @click="handleLogout" class="text-xs text-primary-200 hover:text-white transition-colors">
                        Выйти
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            </li>
          </ul>
        </nav>
      </div>
    </div>

    <!-- Main content -->
    <div class="lg:pl-72">
      <!-- Top bar -->
      <div class="sticky top-0 z-40 flex h-16 shrink-0 items-center gap-x-4 border-b border-gray-200 bg-white px-4 shadow-sm sm:gap-x-6 sm:px-6 lg:px-8">
        <button @click="sidebarOpen = true" class="-m-2.5 p-2.5 text-gray-700 hover:bg-gray-100 rounded-md lg:hidden">
          <span class="sr-only">Открыть боковое меню</span>
          <Icon name="ri:menu-line" class="text-2xl" />
        </button>

        <div class="flex flex-1 gap-x-4 items-center justify-between lg:gap-x-6">
          <form class="relative flex flex-1 max-w-md" @submit.prevent>
            <label for="search-field" class="sr-only">Поиск</label>
            <Icon name="ri:search-line" class="pointer-events-none absolute inset-y-0 left-0 h-full w-5 text-gray-400 ml-3" />
            <input 
              id="search-field" 
              class="block h-10 w-full rounded-full border border-gray-200 bg-gray-50 py-0 pl-10 pr-4 text-gray-900 placeholder:text-gray-500 focus:border-primary-500 focus:ring-2 focus:ring-primary-500 focus:ring-opacity-50 sm:text-sm" 
              placeholder="Поиск..." 
              type="search"
            >
          </form>
          
          <!-- Notifications -->
          <div class="relative flex-shrink-0">
            <button class="rounded-full bg-white p-1.5 text-gray-500 hover:text-primary-600 hover:bg-gray-100 relative">
              <span class="sr-only">Просмотр уведомлений</span>
              <Icon name="ri:notification-3-line" class="text-2xl" />
            </button>
          </div>
        </div>
      </div>

      <!-- Page content -->
      <main class="py-6">
        <div class="px-4 sm:px-6 lg:px-8">
          <slot />
        </div>
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
const sidebarOpen = ref(false)
const { user, logout } = useAuth()

const handleLogout = () => {
  logout()
}
</script>

<style scoped>
/* Улучшенные стили для скроллбара */
::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track {
  background: #f1f5f9;
}

::-webkit-scrollbar-thumb {
  background: #cfd8e3;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}
</style>
