<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4">
        <div class="h-16 w-16 rounded-2xl bg-gradient-to-br from-emerald-500 to-emerald-700 flex items-center justify-center shadow-lg">
          <Icon name="ri:shield-check-line" class="text-3xl text-white" />
        </div>
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Лицензии</h1>
          <p class="mt-1 text-gray-600">Управление лицензиями ПО</p>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <div class="card"><div class="p-5"><div class="flex items-center justify-between"><div><p class="text-sm text-gray-600 mb-1">Всего лицензий</p><p class="text-2xl font-bold text-emerald-600">{{ stats.total }}</p></div><div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center"><Icon name="ri:shield-check-line" class="text-2xl text-emerald-600" /></div></div></div></div>
      <div class="card"><div class="p-5"><div class="flex items-center justify-between"><div><p class="text-sm text-gray-600 mb-1">Активные</p><p class="text-2xl font-bold text-green-600">{{ stats.active }}</p></div><div class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center"><Icon name="ri:checkbox-circle-line" class="text-2xl text-green-600" /></div></div></div></div>
      <div class="card"><div class="p-5"><div class="flex items-center justify-between"><div><p class="text-sm text-gray-600 mb-1">Истекает (30 дней)</p><p class="text-2xl font-bold text-amber-600">{{ stats.expiring }}</p></div><div class="h-12 w-12 rounded-xl bg-amber-100 flex items-center justify-center"><Icon name="ri:time-line" class="text-2xl text-amber-600" /></div></div></div></div>
      <div class="card"><div class="p-5"><div class="flex items-center justify-between"><div><p class="text-sm text-gray-600 mb-1">Истекшие</p><p class="text-2xl font-bold text-red-600">{{ stats.expired }}</p></div><div class="h-12 w-12 rounded-xl bg-red-100 flex items-center justify-center"><Icon name="ri:close-circle-line" class="text-2xl text-red-600" /></div></div></div></div>
    </div>

    <div class="card">
      <div class="p-4 bg-gradient-to-r from-gray-50 to-white">
        <div class="flex items-center gap-4">
          <div class="flex-1 relative">
            <Icon name="ri:search-line" class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
            <input v-model="searchQuery" type="text" placeholder="Поиск лицензий..." class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500" />
          </div>
          <select v-model="selectedType" class="input-field w-48">
            <option value="">Все типы</option>
            <option value="subscription">Подписка</option>
            <option value="perpetual">Бессрочная</option>
            <option value="volume">Корпоративная</option>
          </select>
        </div>
      </div>
    </div>

    <div class="card">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">ПО</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Тип</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Ключ</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Истекает</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Использовано</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Статус</th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="license in filteredLicenses" :key="license.id" class="hover:bg-gray-50">
              <td class="px-4 py-4"><div class="flex items-center gap-3"><Icon name="ri:apps-2-line" class="text-gray-400" /><div><p class="text-sm font-medium text-gray-900">{{ license.software }}</p><p class="text-xs text-gray-500">{{ license.manufacturer }}</p></div></div></td>
              <td class="px-4 py-4"><span class="badge bg-blue-100 text-blue-800">{{ license.type }}</span></td>
              <td class="px-4 py-4"><p class="text-sm font-mono text-gray-900">{{ license.key }}</p></td>
              <td class="px-4 py-4"><p class="text-sm text-gray-900">{{ license.expires }}</p></td>
              <td class="px-4 py-4"><p class="text-sm text-gray-900">{{ license.used }} / {{ license.total }}</p></td>
              <td class="px-4 py-4"><span :class="['badge', getLicenseStatus(license).color]">{{ getLicenseStatus(license).label }}</span></td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ middleware: 'auth' })
useHead({ title: 'Лицензии' })

const searchQuery = ref('')
const selectedType = ref('')

const stats = ref({ total: 45, active: 38, expiring: 5, expired: 2 })

const licenses = ref([
  { id: 1, software: 'Microsoft Office 365', manufacturer: 'Microsoft', type: 'Подписка', key: 'XXXXX-XXXXX-XXXXX', expires: '15.01.2026', used: 45, total: 50 },
  { id: 2, software: 'Kaspersky Endpoint', manufacturer: 'Kaspersky', type: 'Подписка', key: 'XXXXX-XXXXX-XXXXX', expires: '20.12.2025', used: 48, total: 50 },
  { id: 3, software: 'Windows 11 Pro', manufacturer: 'Microsoft', type: 'Корпоративная', key: 'XXXXX-XXXXX-XXXXX', expires: '-', used: 52, total: 100 }
])

const filteredLicenses = computed(() => {
  let filtered = licenses.value
  if (selectedType.value) filtered = filtered.filter(l => l.type === selectedType.value)
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(l => l.software.toLowerCase().includes(query))
  }
  return filtered
})

const getLicenseStatus = (license: any) => {
  if (license.expires === '-') return { label: 'Активна', color: 'bg-green-100 text-green-800' }
  const expiryDate = new Date(license.expires.split('.').reverse().join('-'))
  const now = new Date()
  const daysLeft = Math.floor((expiryDate.getTime() - now.getTime()) / (1000 * 60 * 60 * 24))
  if (daysLeft < 0) return { label: 'Истекла', color: 'bg-red-100 text-red-800' }
  if (daysLeft < 30) return { label: 'Истекает', color: 'bg-amber-100 text-amber-800' }
  return { label: 'Активна', color: 'bg-green-100 text-green-800' }
}
</script>
