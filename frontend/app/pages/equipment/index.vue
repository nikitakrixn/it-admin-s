<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div class="h-16 w-16 rounded-2xl bg-gradient-to-br from-green-500 to-green-700 flex items-center justify-center shadow-lg">
            <Icon name="ri:printer-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Оборудование</h1>
            <p class="mt-1 text-gray-600">Оргтехника и периферийные устройства</p>
          </div>
        </div>
        <NuxtLink to="/equipment/create" class="btn btn-primary">
          <Icon name="ri:add-line" class="mr-2" />
          Добавить оборудование
        </NuxtLink>
      </div>
    </div>

    <!-- Статистика -->
    <div class="card">
      <div class="p-4">
        <div class="grid grid-cols-5 gap-4">
          <div class="p-3 cursor-pointer hover:bg-gray-50 rounded-lg" @click="filterByStatus('active')">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-green-100 flex items-center justify-center">
                <Icon name="ri:checkbox-circle-line" class="text-xl text-green-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">Активное</p>
                <p class="text-xl font-bold text-green-600">{{ stats.active }}</p>
              </div>
            </div>
          </div>
          <div class="p-3 cursor-pointer hover:bg-gray-50 rounded-lg" @click="filterByStatus('repair')">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-amber-100 flex items-center justify-center">
                <Icon name="ri:tools-line" class="text-xl text-amber-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">В ремонте</p>
                <p class="text-xl font-bold text-amber-600">{{ stats.repair }}</p>
              </div>
            </div>
          </div>
          <div class="p-3 cursor-pointer hover:bg-gray-50 rounded-lg" @click="filterByStatus('storage')">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center">
                <Icon name="ri:archive-line" class="text-xl text-blue-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">На складе</p>
                <p class="text-xl font-bold text-blue-600">{{ stats.storage }}</p>
              </div>
            </div>
          </div>
          <div class="p-3 cursor-pointer hover:bg-gray-50 rounded-lg" @click="filterByStatus('inactive')">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center">
                <Icon name="ri:pause-circle-line" class="text-xl text-gray-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">Неактивное</p>
                <p class="text-xl font-bold text-gray-600">{{ stats.inactive }}</p>
              </div>
            </div>
          </div>
          <div class="p-3">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center">
                <Icon name="ri:database-line" class="text-xl text-purple-600" />
              </div>
              <div>
                <p class="text-xs text-gray-600">Всего</p>
                <p class="text-xl font-bold text-purple-600">{{ stats.total }}</p>
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
            <input v-model="searchQuery" type="text" placeholder="Поиск оборудования..." class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500" />
          </div>
          <select v-model="selectedType" class="input-field w-48">
            <option value="">Все типы</option>
            <option value="printer">Принтеры</option>
            <option value="scanner">Сканеры</option>
            <option value="mfu">МФУ</option>
            <option value="network">Сетевое</option>
            <option value="ups">ИБП</option>
          </select>
          <select v-model="selectedStatus" class="input-field w-40">
            <option value="">Все статусы</option>
            <option value="active">Активное</option>
            <option value="repair">В ремонте</option>
            <option value="storage">На складе</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Список оборудования -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <NuxtLink v-for="item in filteredEquipment" :key="item.id" :to="`/equipment/${item.id}`" class="card card-hover group">
        <div class="p-5">
          <div class="flex items-start justify-between mb-4">
            <div class="flex items-center gap-3">
              <div :class="['h-12 w-12 rounded-xl flex items-center justify-center', getTypeColor(item.type)]">
                <Icon :name="getTypeIcon(item.type)" class="text-2xl text-white" />
              </div>
              <div>
                <p class="text-sm font-semibold text-gray-900 group-hover:text-primary-600 transition-colors">{{ item.name }}</p>
                <p class="text-xs text-gray-500">{{ item.inventory_number }}</p>
              </div>
            </div>
            <span :class="['badge', getStatusColor(item.status)]">
              {{ getStatusLabel(item.status) }}
            </span>
          </div>
          <div class="space-y-2">
            <div class="flex items-center justify-between text-sm">
              <span class="text-gray-600">Модель</span>
              <span class="font-medium text-gray-900">{{ item.model }}</span>
            </div>
            <div v-if="item.location" class="flex items-center gap-2 text-sm text-gray-600">
              <Icon name="ri:map-pin-line" class="text-gray-400" />
              <span>{{ item.location }}</span>
            </div>
            <div v-if="item.employee" class="flex items-center gap-2 text-sm text-gray-600">
              <Icon name="ri:user-line" class="text-gray-400" />
              <span>{{ item.employee }}</span>
            </div>
          </div>
        </div>
      </NuxtLink>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

useHead({
  title: 'Оборудование'
})

const searchQuery = ref('')
const selectedType = ref('')
const selectedStatus = ref('')

const stats = ref({
  active: 15,
  repair: 2,
  storage: 5,
  inactive: 3,
  total: 25
})

const equipment = ref([
  { id: 1, name: 'HP LaserJet Pro M404dn', inventory_number: 'EQ-2024-001', type: 'printer', model: 'M404dn', status: 'active', location: 'Офис 301', employee: 'Иванов И.И.' },
  { id: 2, name: 'Canon imageRUNNER', inventory_number: 'EQ-2024-002', type: 'mfu', model: 'iR2625i', status: 'active', location: 'Офис 201', employee: null },
  { id: 3, name: 'TP-Link TL-SG1024', inventory_number: 'EQ-2024-003', type: 'network', model: 'TL-SG1024', status: 'active', location: 'Серверная', employee: null },
  { id: 4, name: 'APC Smart-UPS', inventory_number: 'EQ-2024-004', type: 'ups', model: 'SMT1500I', status: 'repair', location: 'Серверная', employee: null }
])

const filteredEquipment = computed(() => {
  let filtered = equipment.value
  if (selectedType.value) filtered = filtered.filter(e => e.type === selectedType.value)
  if (selectedStatus.value) filtered = filtered.filter(e => e.status === selectedStatus.value)
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(e => e.name.toLowerCase().includes(query) || e.inventory_number.toLowerCase().includes(query))
  }
  return filtered
})

const filterByStatus = (status: string) => {
  selectedStatus.value = selectedStatus.value === status ? '' : status
}

const getTypeIcon = (type: string) => {
  const icons: Record<string, string> = {
    printer: 'ri:printer-line',
    scanner: 'ri:scan-line',
    mfu: 'ri:printer-line',
    network: 'ri:router-line',
    ups: 'ri:battery-charge-line'
  }
  return icons[type] || 'ri:device-line'
}

const getTypeColor = (type: string) => {
  const colors: Record<string, string> = {
    printer: 'bg-blue-500',
    scanner: 'bg-purple-500',
    mfu: 'bg-indigo-500',
    network: 'bg-cyan-500',
    ups: 'bg-green-500'
  }
  return colors[type] || 'bg-gray-500'
}

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    active: 'bg-green-100 text-green-800',
    repair: 'bg-amber-100 text-amber-800',
    storage: 'bg-blue-100 text-blue-800',
    inactive: 'bg-gray-100 text-gray-800'
  }
  return colors[status] || 'bg-gray-100 text-gray-800'
}

const getStatusLabel = (status: string) => {
  const labels: Record<string, string> = {
    active: 'Активное',
    repair: 'В ремонте',
    storage: 'На складе',
    inactive: 'Неактивное'
  }
  return labels[status] || status
}
</script>
