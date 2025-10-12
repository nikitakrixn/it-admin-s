<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div class="h-16 w-16 rounded-2xl gradient-secondary flex items-center justify-center shadow-lg">
            <Icon name="ri:bar-chart-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Аналитика</h1>
            <p class="mt-1 text-gray-600">Детальная статистика и отчеты системы</p>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <select v-model="selectedPeriod" class="input-field w-48">
            <option value="7">Последние 7 дней</option>
            <option value="30">Последние 30 дней</option>
            <option value="90">Последние 90 дней</option>
          </select>
          <button @click="downloadReport" class="btn btn-primary whitespace-nowrap">
            <Icon name="ri:file-download-line" class="mr-2" />
            Скачать отчет
          </button>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
      <div class="card">
        <div class="p-6">
          <div class="flex justify-between items-start mb-4">
            <div class="flex-1">
              <p class="text-sm font-medium text-gray-600 mb-2">Всего сотрудников</p>
              <div class="flex items-baseline gap-2">
                <p class="text-3xl font-bold text-gray-900">156</p>
                <span class="text-sm font-medium text-green-600 flex items-center">
                  <Icon name="ri:arrow-up-line" class="mr-1" />
                  +12.5%
                </span>
              </div>
            </div>
            <div class="flex h-14 w-14 items-center justify-center rounded-xl bg-blue-100 text-blue-600">
              <Icon name="ri:user-line" class="text-2xl" />
            </div>
          </div>
          <div class="pt-4 border-t border-gray-100">
            <div class="flex items-center justify-between text-sm">
              <span class="text-gray-600">За месяц</span>
              <span class="font-semibold text-gray-900">+18</span>
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="p-6">
          <div class="flex justify-between items-start mb-4">
            <div class="flex-1">
              <p class="text-sm font-medium text-gray-600 mb-2">Активных устройств</p>
              <div class="flex items-baseline gap-2">
                <p class="text-3xl font-bold text-gray-900">142</p>
                <span class="text-sm font-medium text-green-600 flex items-center">
                  <Icon name="ri:arrow-up-line" class="mr-1" />
                  +8.2%
                </span>
              </div>
            </div>
            <div class="flex h-14 w-14 items-center justify-center rounded-xl bg-purple-100 text-purple-600">
              <Icon name="ri:computer-line" class="text-2xl" />
            </div>
          </div>
          <div class="pt-4 border-t border-gray-100">
            <div class="flex items-center justify-between text-sm">
              <span class="text-gray-600">Онлайн</span>
              <span class="font-semibold text-gray-900">134</span>
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="p-6">
          <div class="flex justify-between items-start mb-4">
            <div class="flex-1">
              <p class="text-sm font-medium text-gray-600 mb-2">Открытых заявок</p>
              <div class="flex items-baseline gap-2">
                <p class="text-3xl font-bold text-gray-900">23</p>
                <span class="text-sm font-medium text-red-600 flex items-center">
                  <Icon name="ri:arrow-down-line" class="mr-1" />
                  -3.1%
                </span>
              </div>
            </div>
            <div class="flex h-14 w-14 items-center justify-center rounded-xl bg-amber-100 text-amber-600">
              <Icon name="ri:time-line" class="text-2xl" />
            </div>
          </div>
          <div class="pt-4 border-t border-gray-100">
            <div class="flex items-center justify-between text-sm">
              <span class="text-gray-600">В работе</span>
              <span class="font-semibold text-gray-900">5</span>
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="p-6">
          <div class="flex justify-between items-start mb-4">
            <div class="flex-1">
              <p class="text-sm font-medium text-gray-600 mb-2">Uptime системы</p>
              <div class="flex items-baseline gap-2">
                <p class="text-3xl font-bold text-gray-900">98.5%</p>
                <span class="text-sm font-medium text-green-600 flex items-center">
                  <Icon name="ri:arrow-up-line" class="mr-1" />
                  +15.3%
                </span>
              </div>
            </div>
            <div class="flex h-14 w-14 items-center justify-center rounded-xl bg-green-100 text-green-600">
              <Icon name="ri:checkbox-circle-line" class="text-2xl" />
            </div>
          </div>
          <div class="pt-4 border-t border-gray-100">
            <div class="flex items-center justify-between text-sm">
              <span class="text-gray-600">Доступность</span>
              <span class="font-semibold text-green-600">Отлично</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div class="card">
        <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center gap-3">
            <div class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center">
              <Icon name="ri:pie-chart-line" class="text-blue-600 text-xl" />
            </div>
            <h2 class="text-lg font-semibold text-gray-900">Заявки по категориям</h2>
          </div>
        </div>
        <div class="p-6">
          <div class="space-y-4">
            <div v-for="category in requestCategories" :key="category.name">
              <div class="flex items-center justify-between mb-2">
                <div class="flex items-center gap-2">
                  <div :class="['w-3 h-3 rounded-full', category.color]"></div>
                  <span class="text-sm font-medium text-gray-700">{{ category.name }}</span>
                </div>
                <span class="text-sm text-gray-600">{{ category.count }}</span>
              </div>
              <div class="w-full bg-gray-200 rounded-full h-2">
                <div
                  :class="category.color"
                  :style="{ width: `${category.percentage}%` }"
                  class="h-2 rounded-full transition-all duration-500"
                />
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center gap-3">
            <div class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center">
              <Icon name="ri:time-line" class="text-purple-600 text-xl" />
            </div>
            <h2 class="text-lg font-semibold text-gray-900">Среднее время решения</h2>
          </div>
        </div>
        <div class="p-6">
          <div class="text-center mb-6">
            <div class="inline-flex items-center justify-center h-32 w-32 rounded-full bg-gradient-to-br from-primary-500 to-primary-700 text-white mb-4 shadow-lg">
              <div>
                <p class="text-4xl font-bold">2.4</p>
                <p class="text-sm">часа</p>
              </div>
            </div>
            <p class="text-sm text-gray-600">Среднее время закрытия заявки</p>
          </div>
          <div class="grid grid-cols-3 gap-4">
            <div class="text-center p-3 rounded-lg bg-gray-50">
              <p class="text-2xl font-bold text-gray-900">1.2ч</p>
              <p class="text-xs text-gray-600 mt-1">Низкий</p>
            </div>
            <div class="text-center p-3 rounded-lg bg-gray-50">
              <p class="text-2xl font-bold text-gray-900">2.4ч</p>
              <p class="text-xs text-gray-600 mt-1">Средний</p>
            </div>
            <div class="text-center p-3 rounded-lg bg-gray-50">
              <p class="text-2xl font-bold text-gray-900">0.8ч</p>
              <p class="text-xs text-gray-600 mt-1">Высокий</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="card">
      <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
        <div class="flex items-center gap-3">
          <div class="h-10 w-10 rounded-lg bg-amber-100 flex items-center justify-center">
            <Icon name="ri:fire-line" class="text-amber-600 text-xl" />
          </div>
          <h2 class="text-lg font-semibold text-gray-900">Топ проблем за период</h2>
        </div>
      </div>
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">#</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Проблема</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Количество</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Среднее время</th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase">Тренд</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="(issue, index) in topIssues" :key="index" class="hover:bg-gray-50 transition-colors">
              <td class="px-6 py-4">
                <div class="flex items-center justify-center w-8 h-8 rounded-lg bg-gray-100 text-gray-700 font-semibold text-sm">
                  {{ index + 1 }}
                </div>
              </td>
              <td class="px-6 py-4">
                <div class="flex items-center gap-3">
                  <div class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center">
                    <Icon name="ri:tools-line" class="text-blue-600" />
                  </div>
                  <span class="text-sm font-medium text-gray-900">{{ issue.title }}</span>
                </div>
              </td>
              <td class="px-6 py-4">
                <span class="badge bg-gray-100 text-gray-800">{{ issue.count }}</span>
              </td>
              <td class="px-6 py-4 text-sm text-gray-600">{{ issue.avgTime }}</td>
              <td class="px-6 py-4">
                <span :class="['badge', issue.trend === 'up' ? 'bg-red-100 text-red-800' : 'bg-green-100 text-green-800']">
                  <Icon :name="issue.trend === 'up' ? 'ri:arrow-up-line' : 'ri:arrow-down-line'" class="mr-1" />
                  {{ issue.trend === 'up' ? 'Растет' : 'Снижается' }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="card">
        <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center gap-3">
            <div class="h-10 w-10 rounded-lg bg-green-100 flex items-center justify-center">
              <Icon name="ri:trophy-line" class="text-green-600 text-xl" />
            </div>
            <h2 class="text-lg font-semibold text-gray-900">Лучшие показатели</h2>
          </div>
        </div>
        <div class="p-6 space-y-3">
          <div class="flex items-center justify-between p-3 rounded-lg bg-green-50">
            <div class="flex items-center gap-3">
              <Icon name="ri:speed-line" class="text-xl text-green-600" />
              <span class="text-sm font-medium text-gray-900">Быстрое решение</span>
            </div>
            <span class="text-sm font-bold text-green-600">0.5ч</span>
          </div>
          <div class="flex items-center justify-between p-3 rounded-lg bg-blue-50">
            <div class="flex items-center gap-3">
              <Icon name="ri:user-star-line" class="text-xl text-blue-600" />
              <span class="text-sm font-medium text-gray-900">Довольных клиентов</span>
            </div>
            <span class="text-sm font-bold text-blue-600">95%</span>
          </div>
          <div class="flex items-center justify-between p-3 rounded-lg bg-purple-50">
            <div class="flex items-center gap-3">
              <Icon name="ri:checkbox-multiple-line" class="text-xl text-purple-600" />
              <span class="text-sm font-medium text-gray-900">Закрыто за день</span>
            </div>
            <span class="text-sm font-bold text-purple-600">45</span>
          </div>
        </div>
      </div>

      <div class="card lg:col-span-2">
        <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center gap-3">
            <div class="h-10 w-10 rounded-lg bg-indigo-100 flex items-center justify-center">
              <Icon name="ri:calendar-line" class="text-indigo-600 text-xl" />
            </div>
            <h2 class="text-lg font-semibold text-gray-900">Активность по дням недели</h2>
          </div>
        </div>
        <div class="p-6">
          <div class="grid grid-cols-7 gap-2">
            <div v-for="day in weekActivity" :key="day.label" class="text-center">
              <div class="mb-2">
                <div class="h-24 bg-gray-100 rounded-lg flex items-end justify-center p-2">
                  <div
                    :style="{ height: `${(day.value / maxActivity) * 100}%` }"
                    class="w-full gradient-primary rounded transition-all duration-500"
                  />
                </div>
              </div>
              <p class="text-xs font-medium text-gray-600">{{ day.label }}</p>
              <p class="text-xs text-gray-500">{{ day.value }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in'
  }
})

useHead({
  title: 'Аналитика'
})

const toast = useToast()
const selectedPeriod = ref('30')

const requestCategories = ref([
  { name: 'Проблемы с ПО', count: 45, percentage: 35, color: 'bg-blue-500' },
  { name: 'Проблемы с оборудованием', count: 38, percentage: 30, color: 'bg-purple-500' },
  { name: 'Сетевые проблемы', count: 25, percentage: 20, color: 'bg-amber-500' },
  { name: 'Другое', count: 19, percentage: 15, color: 'bg-green-500' }
])

const topIssues = ref([
  { title: 'Не работает принтер', count: 23, avgTime: '1.5ч', trend: 'down' },
  { title: 'Проблемы с доступом к сети', count: 18, avgTime: '2.1ч', trend: 'up' },
  { title: 'Медленная работа ПК', count: 15, avgTime: '3.2ч', trend: 'down' },
  { title: 'Не запускается приложение', count: 12, avgTime: '1.8ч', trend: 'down' },
  { title: 'Проблемы с email', count: 9, avgTime: '0.9ч', trend: 'up' }
])

const weekActivity = ref([
  { label: 'Пн', value: 45 },
  { label: 'Вт', value: 62 },
  { label: 'Ср', value: 38 },
  { label: 'Чт', value: 71 },
  { label: 'Пт', value: 55 },
  { label: 'Сб', value: 12 },
  { label: 'Вс', value: 8 }
])

const maxActivity = computed(() => Math.max(...weekActivity.value.map(d => d.value)))

const downloadReport = () => {
  toast.success('Отчет формируется. Загрузка начнется через несколько секунд.')
}
</script>
