<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4 mb-6">
        <NuxtLink to="/consumables" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
          <Icon name="ri:arrow-left-line" class="text-xl" />
        </NuxtLink>
        <div class="flex-1 flex items-center justify-between">
          <div class="flex items-center gap-4">
            <div class="h-16 w-16 rounded-2xl bg-gradient-to-br from-orange-500 to-orange-700 flex items-center justify-center shadow-lg">
              <Icon name="ri:inbox-line" class="text-3xl text-white" />
            </div>
            <div>
              <h1 class="text-3xl font-bold text-gray-900">{{ consumable.name }}</h1>
              <p class="mt-1 text-gray-600">{{ consumable.manufacturer }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="lg:col-span-2 space-y-6">
        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h2 class="text-lg font-semibold text-gray-900">Информация</h2>
          </div>
          <div class="p-6">
            <div class="grid grid-cols-2 gap-6">
              <div>
                <p class="text-xs text-gray-600 mb-1">Категория</p>
                <span class="badge bg-blue-100 text-blue-800">{{ consumable.category }}</span>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Модель</p>
                <p class="text-sm font-medium text-gray-900">{{ consumable.model }}</p>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Совместимость</p>
                <p class="text-sm text-gray-900">{{ consumable.compatible_with }}</p>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Цена за единицу</p>
                <p class="text-sm font-medium text-gray-900">{{ consumable.price_per_unit }} ₽</p>
              </div>
            </div>
            <div v-if="consumable.notes" class="mt-6 pt-6 border-t border-gray-200">
              <p class="text-xs text-gray-600 mb-2">Примечания</p>
              <p class="text-sm text-gray-700">{{ consumable.notes }}</p>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center justify-between">
              <h2 class="text-lg font-semibold text-gray-900">История движения</h2>
              <button @click="showMovementModal = true" class="btn btn-primary btn-sm">
                <Icon name="ri:add-line" class="mr-2" />
                Добавить
              </button>
            </div>
          </div>
          <div class="p-6">
            <div class="space-y-3">
              <div v-for="movement in movements" :key="movement.id" class="flex items-center justify-between p-3 rounded-lg hover:bg-gray-100 transition-colors">
                <div class="flex items-center gap-3">
                  <div :class="['h-10 w-10 rounded-lg flex items-center justify-center', getMovementColor(movement.type)]">
                    <Icon :name="getMovementIcon(movement.type)" class="text-xl text-white" />
                  </div>
                  <div>
                    <p class="text-sm font-medium text-gray-900">{{ getMovementLabel(movement.type) }}</p>
                    <p class="text-xs text-gray-600">{{ movement.created_at }}</p>
                  </div>
                </div>
                <div class="text-right">
                  <p :class="['text-sm font-medium', movement.quantity > 0 ? 'text-green-600' : 'text-red-600']">
                    {{ movement.quantity > 0 ? '+' : '' }}{{ movement.quantity }} {{ consumable.unit }}
                  </p>
                  <p v-if="movement.notes" class="text-xs text-gray-500">{{ movement.notes }}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="space-y-6">
        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h3 class="text-sm font-semibold text-gray-900">Остаток</h3>
          </div>
          <div class="p-6 space-y-4">
            <div>
              <p class="text-xs text-gray-600 mb-1">Текущее количество</p>
              <p class="text-3xl font-bold text-gray-900">{{ consumable.quantity }} {{ consumable.unit }}</p>
            </div>
            <div>
              <p class="text-xs text-gray-600 mb-1">Минимальный остаток</p>
              <p class="text-lg font-medium text-gray-600">{{ consumable.min_quantity }} {{ consumable.unit }}</p>
            </div>
            <div>
              <p class="text-xs text-gray-600 mb-1">Статус</p>
              <span :class="['badge', getStockStatus().color]">
                {{ getStockStatus().label }}
              </span>
            </div>
            <div class="pt-4 border-t border-gray-200 space-y-2">
              <button @click="adjustStock(1)" class="w-full btn btn-primary btn-sm">
                <Icon name="ri:add-line" class="mr-2" />
                Добавить
              </button>
              <button @click="adjustStock(-1)" class="w-full btn btn-secondary btn-sm">
                <Icon name="ri:subtract-line" class="mr-2" />
                Списать
              </button>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h3 class="text-sm font-semibold text-gray-900">Статистика</h3>
          </div>
          <div class="p-6 space-y-4">
            <div>
              <p class="text-xs text-gray-600 mb-1">Всего поступило</p>
              <p class="text-2xl font-bold text-green-600">{{ stats.total_in }}</p>
            </div>
            <div>
              <p class="text-xs text-gray-600 mb-1">Всего списано</p>
              <p class="text-2xl font-bold text-red-600">{{ stats.total_out }}</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ middleware: 'auth' })

const route = useRoute()
const toast = useToast()

const showMovementModal = ref(false)

const consumable = ref({
  id: route.params.id,
  name: 'Картридж HP 85A',
  manufacturer: 'HP',
  model: 'CE285A',
  category: 'Картриджи',
  compatible_with: 'HP LaserJet Pro M1132/M1212',
  quantity: 5,
  min_quantity: 2,
  unit: 'шт',
  price_per_unit: 3500,
  notes: 'Оригинальный картридж'
})

const movements = ref([
  { id: 1, type: 'purchase', quantity: 10, notes: 'Закупка', created_at: '01.12.2025 10:00' },
  { id: 2, type: 'usage', quantity: -3, notes: 'Замена в принтере офис 301', created_at: '05.12.2025 14:30' },
  { id: 3, type: 'usage', quantity: -2, notes: 'Замена в принтере офис 201', created_at: '08.12.2025 11:00' }
])

const stats = ref({
  total_in: 10,
  total_out: 5
})

useHead({
  title: consumable.value.name
})

const getStockStatus = () => {
  if (consumable.value.quantity === 0) return { label: 'Нет в наличии', color: 'bg-red-100 text-red-800' }
  if (consumable.value.quantity <= consumable.value.min_quantity) return { label: 'Заканчивается', color: 'bg-amber-100 text-amber-800' }
  return { label: 'В наличии', color: 'bg-green-100 text-green-800' }
}

const adjustStock = (delta: number) => {
  consumable.value.quantity = Math.max(0, consumable.value.quantity + delta)
  movements.value.unshift({
    id: movements.value.length + 1,
    type: delta > 0 ? 'purchase' : 'usage',
    quantity: delta,
    notes: delta > 0 ? 'Добавление' : 'Списание',
    created_at: new Date().toLocaleString('ru-RU')
  })
  toast.success(delta > 0 ? 'Количество увеличено' : 'Количество уменьшено')
}

const getMovementIcon = (type: string) => {
  const icons: Record<string, string> = {
    purchase: 'ri:shopping-cart-line',
    usage: 'ri:arrow-right-line',
    return: 'ri:arrow-left-line',
    write_off: 'ri:delete-bin-line'
  }
  return icons[type] || 'ri:arrow-right-line'
}

const getMovementColor = (type: string) => {
  const colors: Record<string, string> = {
    purchase: 'bg-green-500',
    usage: 'bg-blue-500',
    return: 'bg-amber-500',
    write_off: 'bg-red-500'
  }
  return colors[type] || 'bg-gray-500'
}

const getMovementLabel = (type: string) => {
  const labels: Record<string, string> = {
    purchase: 'Закупка',
    usage: 'Использование',
    return: 'Возврат',
    write_off: 'Списание'
  }
  return labels[type] || type
}
</script>
