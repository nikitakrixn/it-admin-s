<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div class="h-16 w-16 rounded-2xl bg-gradient-to-br from-indigo-500 to-indigo-700 flex items-center justify-center shadow-lg">
            <Icon name="ri:apps-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Программное обеспечение</h1>
            <p class="mt-1 text-gray-600">Каталог и управление ПО</p>
          </div>
        </div>
        <NuxtLink to="/software/create" class="btn btn-primary">
          <Icon name="ri:add-line" class="mr-2" />
          Добавить ПО
        </NuxtLink>
      </div>
    </div>

    <!-- Статистика -->
    <div class="card">
      <div class="p-4">
        <div class="grid grid-cols-5 gap-4">
          <div class="p-3">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-indigo-100 flex items-center justify-center">
                <Icon name="ri:apps-line" class="text-xl text-indigo-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">Всего ПО</p>
                <p class="text-xl font-bold text-indigo-600">{{ stats.total }}</p>
              </div>
            </div>
          </div>
          <div class="p-3">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-green-100 flex items-center justify-center">
                <Icon name="ri:shield-check-line" class="text-xl text-green-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">Лицензионное</p>
                <p class="text-xl font-bold text-green-600">{{ stats.licensed }}</p>
              </div>
            </div>
          </div>
          <div class="p-3">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center">
                <Icon name="ri:gift-line" class="text-xl text-blue-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">Бесплатное</p>
                <p class="text-xl font-bold text-blue-600">{{ stats.free }}</p>
              </div>
            </div>
          </div>
          <div class="p-3">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-amber-100 flex items-center justify-center">
                <Icon name="ri:time-line" class="text-xl text-amber-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">Истекает</p>
                <p class="text-xl font-bold text-amber-600">{{ stats.expiring }}</p>
              </div>
            </div>
          </div>
          <div class="p-3">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center">
                <Icon name="ri:computer-line" class="text-xl text-purple-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">Установок</p>
                <p class="text-xl font-bold text-purple-600">{{ stats.installations }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Фильтры -->
    <div class="card">
      <div class="p-4 bg-gradient-to-r from-gray-50 to-white">
        <div class="flex items-center gap-4">
          <div class="flex-1 relative">
            <Icon name="ri:search-line" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Поиск ПО..."
              class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            />
          </div>
          <select v-model="selectedCategory" class="input-field w-48">
            <option value="">Все категории</option>
            <option value="office">Офисные пакеты</option>
            <option value="browser">Браузеры</option>
            <option value="antivirus">Антивирусы</option>
            <option value="development">Разработка</option>
            <option value="media">Медиа</option>
            <option value="communication">Коммуникации</option>
          </select>
          <select v-model="selectedLicense" class="input-field w-48">
            <option value="">Все типы</option>
            <option value="licensed">Лицензионное</option>
            <option value="free">Бесплатное</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Список ПО -->
    <div class="card">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">ПО</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Категория</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Лицензия</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Установлено</th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">Действия</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="software in filteredSoftware" :key="software.id" class="hover:bg-gray-50 transition-colors">
              <td class="px-4 py-4">
                <div class="flex items-center gap-3">
                  <div class="h-10 w-10 rounded-lg bg-indigo-100 flex items-center justify-center">
                    <Icon name="ri:apps-2-line" class="text-xl text-indigo-600" />
                  </div>
                  <div>
                    <p class="text-sm font-medium text-gray-900">{{ software.name }}</p>
                    <p class="text-xs text-gray-500">{{ software.manufacturer }}</p>
                  </div>
                </div>
              </td>
              <td class="px-4 py-4">
                <span class="badge bg-blue-100 text-blue-800">
                  {{ getCategoryLabel(software.category) }}
                </span>
              </td>
              <td class="px-4 py-4">
                <span :class="['badge', software.requires_license ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800']">
                  {{ software.requires_license ? 'Требует лицензию' : 'Бесплатное' }}
                </span>
              </td>
              <td class="px-4 py-4">
                <div class="flex items-center gap-2">
                  <Icon name="ri:computer-line" class="text-gray-400" />
                  <span class="text-sm font-medium text-gray-900">{{ software.installations }}</span>
                </div>
              </td>
              <td class="px-4 py-4 text-right">
                <div class="flex items-center justify-end gap-2">
                  <NuxtLink :to="`/software/${software.id}`" class="p-2 rounded-lg hover:bg-gray-100 transition-colors" title="Просмотр">
                    <Icon name="ri:eye-line" class="text-gray-600" />
                  </NuxtLink>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

useHead({
  title: 'Программное обеспечение'
})

const searchQuery = ref('')
const selectedCategory = ref('')
const selectedLicense = ref('')

const stats = ref({
  total: 47,
  licensed: 23,
  free: 24,
  expiring: 5,
  installations: 156
})

const software = ref([
  { id: 1, name: 'Microsoft Office 365', manufacturer: 'Microsoft', category: 'office', requires_license: true, installations: 45 },
  { id: 2, name: 'Google Chrome', manufacturer: 'Google', category: 'browser', requires_license: false, installations: 52 },
  { id: 3, name: 'Kaspersky Endpoint Security', manufacturer: 'Kaspersky', category: 'antivirus', requires_license: true, installations: 48 },
  { id: 4, name: 'Visual Studio Code', manufacturer: 'Microsoft', category: 'development', requires_license: false, installations: 15 },
  { id: 5, name: 'Adobe Acrobat Reader', manufacturer: 'Adobe', category: 'office', requires_license: false, installations: 50 },
  { id: 6, name: 'Microsoft Teams', manufacturer: 'Microsoft', category: 'communication', requires_license: true, installations: 42 },
  { id: 7, name: 'VLC Media Player', manufacturer: 'VideoLAN', category: 'media', requires_license: false, installations: 30 }
])

const filteredSoftware = computed(() => {
  let filtered = software.value

  if (selectedCategory.value) {
    filtered = filtered.filter(s => s.category === selectedCategory.value)
  }

  if (selectedLicense.value === 'licensed') {
    filtered = filtered.filter(s => s.requires_license)
  } else if (selectedLicense.value === 'free') {
    filtered = filtered.filter(s => !s.requires_license)
  }

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(s =>
      s.name.toLowerCase().includes(query) ||
      s.manufacturer.toLowerCase().includes(query)
    )
  }

  return filtered
})

const getCategoryLabel = (category: string) => {
  const labels: Record<string, string> = {
    office: 'Офисные',
    browser: 'Браузеры',
    antivirus: 'Антивирусы',
    development: 'Разработка',
    media: 'Медиа',
    communication: 'Коммуникации'
  }
  return labels[category] || category
}
</script>
