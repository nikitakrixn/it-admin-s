<template>
  <div>
    <!-- Шапка страницы -->
    <div class="mb-8 border-b border-gray-200 pb-5">
      <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between">
        <div class="flex-1 min-w-0">
          <div class="flex items-center">
            <div class="h-14 w-14 flex-shrink-0 rounded-lg bg-gradient-to-br from-primary-400 to-primary-700 flex items-center justify-center mr-4 shadow-md">
              <Icon name="ri:dashboard-line text-white text-xl" />
            </div>
            <div>
              <h1 class="text-2xl font-bold text-gray-900 sm:text-3xl sm:tracking-tight">Панель управления</h1>
              <p class="mt-1 text-sm text-gray-500 max-w-4xl">Обзор основных показателей и статистики системы</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Основные показатели -->
    <div class="mb-8 grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
      <!-- Сотрудники -->
      <div class="relative overflow-hidden rounded-lg border border-gray-200 bg-white shadow-sm transition-all duration-300 hover:shadow-md">
        <div class="absolute top-0 left-0 w-1 h-full bg-blue-500"></div>
        <div class="p-5">
          <div class="flex justify-between">
            <div>
              <p class="text-sm font-medium text-gray-500">Сотрудники</p>
              <div class="mt-1 flex items-baseline">
                <p class="text-2xl font-bold text-gray-900">0</p>
              </div>
            </div>
            <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-blue-50 text-blue-500">
              <Icon name="ri:user-line text-xl" />
            </div>
          </div>
          <div class="mt-3">
            <NuxtLink to="/employees" class="text-sm font-medium text-blue-600 hover:text-blue-800">
              Управление <Icon name="ri:arrow-right-line ml-1 inline-block" />
            </NuxtLink>
          </div>
        </div>
      </div>

      <!-- Компьютеры -->
      <div class="relative overflow-hidden rounded-lg border border-gray-200 bg-white shadow-sm transition-all duration-300 hover:shadow-md">
        <div class="absolute top-0 left-0 w-1 h-full bg-indigo-500"></div>
        <div class="p-5">
          <div class="flex justify-between">
            <div>
              <p class="text-sm font-medium text-gray-500">Компьютеры</p>
              <div class="mt-1 flex items-baseline">
                <p class="text-2xl font-bold text-gray-900">0</p>
              </div>
            </div>
            <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-indigo-50 text-indigo-500">
              <Icon name="ri:computer-line text-xl" />
            </div>
          </div>
          <div class="mt-3">
            <NuxtLink to="/computers" class="text-sm font-medium text-indigo-600 hover:text-indigo-800">
              Управление <Icon name="ri:arrow-right-line ml-1 inline-block" />
            </NuxtLink>
          </div>
        </div>
      </div>

      <!-- Оргтехника -->
      <div class="relative overflow-hidden rounded-lg border border-gray-200 bg-white shadow-sm transition-all duration-300 hover:shadow-md">
        <div class="absolute top-0 left-0 w-1 h-full bg-purple-500"></div>
        <div class="p-5">
          <div class="flex justify-between">
            <div>
              <p class="text-sm font-medium text-gray-500">Оргтехника</p>
              <div class="mt-1 flex items-baseline">
                <p class="text-2xl font-bold text-gray-900">0</p>
              </div>
            </div>
            <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-purple-50 text-purple-500">
              <Icon name="ri:printer-line text-xl" />
            </div>
          </div>
          <div class="mt-3">
            <NuxtLink to="/equipment" class="text-sm font-medium text-purple-600 hover:text-purple-800">
              Управление <Icon name="ri:arrow-right-line ml-1 inline-block" />
            </NuxtLink>
          </div>
        </div>
      </div>

      <!-- Заявки -->
      <div class="relative overflow-hidden rounded-lg border border-gray-200 bg-white shadow-sm transition-all duration-300 hover:shadow-md">
        <div class="absolute top-0 left-0 w-1 h-full bg-green-500"></div>
        <div class="p-5">
          <div class="flex justify-between">
            <div>
              <p class="text-sm font-medium text-gray-500">Активные заявки</p>
              <div class="mt-1 flex items-baseline">
                <p class="text-2xl font-bold text-gray-900">0</p>
              </div>
            </div>
            <div class="flex h-12 w-12 items-center justify-center rounded-lg bg-green-50 text-green-500">
              <Icon name="ri:file-list-line text-xl" />
            </div>
          </div>
          <div class="mt-3">
            <NuxtLink to="/requests" class="text-sm font-medium text-green-600 hover:text-green-800">
              Управление <Icon name="ri:arrow-right-line ml-1 inline-block" />
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>

    <!-- Быстрые действия -->
    <div class="mb-8">
      <h2 class="text-lg font-semibold text-gray-900 mb-3">Быстрые действия</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <NuxtLink to="/requests/create" class="group flex items-center p-4 bg-white rounded-lg border border-gray-200 shadow-sm hover:shadow-md hover:border-primary-200 transition-all duration-300">
          <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-primary-100 text-primary-600 group-hover:bg-primary-200">
            <Icon name="ri:add-line" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-900">Создать заявку</p>
            <p class="text-xs text-gray-500 mt-0.5">Новая заявка в систему</p>
          </div>
        </NuxtLink>

        <NuxtLink to="/employees/create" class="group flex items-center p-4 bg-white rounded-lg border border-gray-200 shadow-sm hover:shadow-md hover:border-blue-200 transition-all duration-300">
          <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-blue-100 text-blue-600 group-hover:bg-blue-200">
            <Icon name="ri:user-add-line" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-900">Добавить сотрудника</p>
            <p class="text-xs text-gray-500 mt-0.5">Новый профиль сотрудника</p>
          </div>
        </NuxtLink>

        <NuxtLink to="/computers/scan-network" class="group flex items-center p-4 bg-white rounded-lg border border-gray-200 shadow-sm hover:shadow-md hover:border-indigo-200 transition-all duration-300">
          <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-indigo-100 text-indigo-600 group-hover:bg-indigo-200">
            <Icon name="ri:scan-line" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-900">Сканировать сеть</p>
            <p class="text-xs text-gray-500 mt-0.5">Поиск устройств в сети</p>
          </div>
        </NuxtLink>

        <NuxtLink to="/reports" class="group flex items-center p-4 bg-white rounded-lg border border-gray-200 shadow-sm hover:shadow-md hover:border-amber-200 transition-all duration-300">
          <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-amber-100 text-amber-600 group-hover:bg-amber-200">
            <Icon name="ri:bar-chart-line" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-900">Отчеты</p>
            <p class="text-xs text-gray-500 mt-0.5">Аналитика и статистика</p>
          </div>
        </NuxtLink>
      </div>
    </div>

    <!-- Информация о системе -->
    <div class="bg-white rounded-lg border border-gray-200 shadow-sm overflow-hidden">
      <div class="px-5 py-4 border-b border-gray-200 bg-gray-50">
        <h2 class="text-base font-semibold text-gray-900">Информация о системе</h2>
      </div>
      
      <div class="p-5 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-5">
        <div class="p-4 bg-gray-50 rounded-lg border border-gray-100">
          <div class="flex items-center">
            <div class="w-8 h-8 rounded-full bg-green-100 flex items-center justify-center text-green-600 mr-3">
              <Icon name="ri:server-line" />
            </div>
            <div>
              <p class="text-xs text-gray-500">Статус сервера</p>
              <p class="text-sm font-medium text-gray-900">Активен</p>
            </div>
          </div>
        </div>
        
        <div class="p-4 bg-gray-50 rounded-lg border border-gray-100">
          <div class="flex items-center">
            <div class="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center text-blue-600 mr-3">
              <Icon name="ri:database-2-line" />
            </div>
            <div>
              <p class="text-xs text-gray-500">База данных</p>
              <p class="text-sm font-medium text-gray-900">Подключена</p>
            </div>
          </div>
        </div>
        
        <div class="p-4 bg-gray-50 rounded-lg border border-gray-100">
          <div class="flex items-center">
            <div class="w-8 h-8 rounded-full bg-purple-100 flex items-center justify-center text-purple-600 mr-3">
              <Icon name="ri:user-settings-line" />
            </div>
            <div>
              <p class="text-xs text-gray-500">Active Directory</p>
              <p class="text-sm font-medium text-gray-900">Готов</p>
            </div>
          </div>
        </div>
        
        <div class="p-4 bg-gray-50 rounded-lg border border-gray-100">
          <div class="flex items-center">
            <div class="w-8 h-8 rounded-full bg-amber-100 flex items-center justify-center text-amber-600 mr-3">
              <Icon name="ri:notification-3-line" />
            </div>
            <div>
              <p class="text-xs text-gray-500">Уведомления</p>
              <p class="text-sm font-medium text-gray-900">Включены</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
useHead({
  title: 'Панель управления',
})
</script>

