<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4 mb-6">
        <NuxtLink to="/consumables" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
          <Icon name="ri:arrow-left-line" class="text-xl" />
        </NuxtLink>
        <div class="flex-1 flex items-center justify-between">
          <div class="flex items-center gap-4">
            <div
              class="h-16 w-16 rounded-2xl bg-gradient-to-br from-orange-500 to-orange-700 flex items-center justify-center shadow-lg"
            >
              <Icon name="ri:inbox-line" class="text-3xl text-white" />
            </div>
            <div>
              <h1 class="text-3xl font-bold text-gray-900">
                {{ consumable?.name || 'Расходный материал' }}
              </h1>
              <p class="mt-1 text-gray-600">
                {{ consumable?.manufacturer || 'Производитель не указан' }}
              </p>
            </div>
          </div>
          <button v-if="consumable" class="btn btn-primary" @click="openMovementModal('purchase')">
            <Icon name="ri:add-line" class="mr-2" />
            Добавить движение
          </button>
        </div>
      </div>
    </div>

    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-orange-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка расходника...</p>
    </div>

    <div v-else-if="!consumable" class="card p-16 text-center">
      <Icon name="ri:error-warning-line" class="text-4xl text-red-400 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Расходник не найден</h3>
      <p class="text-sm text-gray-500 mb-4">Возможно, он был удалён</p>
      <NuxtLink to="/consumables" class="btn btn-primary">К списку расходников</NuxtLink>
    </div>

    <div v-else class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <div class="lg:col-span-2 space-y-6">
        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h2 class="text-lg font-semibold text-gray-900">Информация</h2>
          </div>
          <div class="p-6">
            <div class="grid grid-cols-2 gap-6">
              <div>
                <p class="text-xs text-gray-600 mb-1">Категория</p>
                <span v-if="consumable.category" class="badge bg-blue-100 text-blue-800">
                  {{ consumable.category }}
                </span>
                <span v-else class="text-sm text-gray-400">—</span>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Модель</p>
                <p class="text-sm font-medium text-gray-900">
                  {{ consumable.model || '—' }}
                </p>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Совместимость</p>
                <p class="text-sm text-gray-900">
                  {{ consumable.compatible_with_name || 'Универсальный' }}
                </p>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Единица измерения</p>
                <p class="text-sm font-medium text-gray-900">{{ consumable.unit || 'шт' }}</p>
              </div>
            </div>
            <div v-if="consumable.notes" class="mt-6 pt-6 border-t border-gray-200">
              <p class="text-xs text-gray-600 mb-2">Примечания</p>
              <p class="text-sm text-gray-700">
                {{ consumable.notes }}
              </p>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center justify-between">
              <h2 class="text-lg font-semibold text-gray-900">История движения</h2>
              <button class="btn btn-primary btn-sm" @click="openMovementModal('purchase')">
                <Icon name="ri:add-line" class="mr-2" />
                Добавить
              </button>
            </div>
          </div>
          <div class="p-6">
            <div v-if="movements.length === 0" class="py-8 text-center">
              <Icon name="ri:history-line" class="text-4xl text-gray-300 mb-3" />
              <p class="text-sm text-gray-500">Движений пока нет</p>
            </div>
            <div v-else class="space-y-3">
              <div
                v-for="movement in movements"
                :key="movement.id"
                class="flex items-center justify-between p-3 rounded-lg hover:bg-gray-100 transition-colors"
              >
                <div class="flex items-center gap-3">
                  <div
                    :class="[
                      'h-10 w-10 rounded-lg flex items-center justify-center',
                      getMovementColor(movement.movement_type),
                    ]"
                  >
                    <Icon
                      :name="getMovementIcon(movement.movement_type)"
                      class="text-xl text-white"
                    />
                  </div>
                  <div>
                    <p class="text-sm font-medium text-gray-900">
                      {{ getMovementLabel(movement.movement_type) }}
                    </p>
                    <p class="text-xs text-gray-600">
                      {{ formatDateTime(movement.created_at) }}
                      <span v-if="movement.performed_by_email">
                        · {{ movement.performed_by_email }}
                      </span>
                    </p>
                  </div>
                </div>
                <div class="text-right">
                  <p
                    :class="[
                      'text-sm font-medium',
                      movement.quantity > 0 ? 'text-green-600' : 'text-red-600',
                    ]"
                  >
                    {{ movement.quantity > 0 ? '+' : '' }}{{ movement.quantity }}
                    {{ consumable.unit || 'шт' }}
                  </p>
                  <p v-if="movement.notes" class="text-xs text-gray-500">
                    {{ movement.notes }}
                  </p>
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
              <p class="text-3xl font-bold text-gray-900">
                {{ consumable.quantity }} {{ consumable.unit || 'шт' }}
              </p>
            </div>
            <div>
              <p class="text-xs text-gray-600 mb-1">Минимальный остаток</p>
              <p class="text-lg font-medium text-gray-600">
                {{ consumable.min_quantity }}
                {{ consumable.unit || 'шт' }}
              </p>
            </div>
            <div>
              <p class="text-xs text-gray-600 mb-1">Статус</p>
              <span :class="['badge', stockStatus.color]">
                {{ stockStatus.label }}
              </span>
            </div>
            <div class="pt-4 border-t border-gray-200 space-y-2">
              <button class="w-full btn btn-primary btn-sm" @click="openMovementModal('purchase')">
                <Icon name="ri:add-line" class="mr-2" />
                Добавить
              </button>
              <button
                class="w-full btn btn-secondary btn-sm"
                :disabled="consumable.quantity === 0"
                @click="openMovementModal('usage')"
              >
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
              <p class="text-2xl font-bold text-green-600">
                {{ stats.total_in }} {{ consumable.unit || 'шт' }}
              </p>
            </div>
            <div>
              <p class="text-xs text-gray-600 mb-1">Всего списано</p>
              <p class="text-2xl font-bold text-red-600">
                {{ stats.total_out }} {{ consumable.unit || 'шт' }}
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <Teleport to="body">
      <div
        v-if="showMovementModal"
        class="fixed inset-0 z-50 flex items-center justify-center bg-gray-900/50 p-4"
        @click.self="closeMovementModal"
      >
        <div class="w-full max-w-lg rounded-xl bg-white shadow-2xl">
          <div class="flex items-center justify-between border-b border-gray-200 px-6 py-4">
            <h2 class="text-lg font-semibold text-gray-900">Новое движение</h2>
            <button class="p-2 rounded-lg hover:bg-gray-100" @click="closeMovementModal">
              <Icon name="ri:close-line" class="text-xl" />
            </button>
          </div>

          <form class="p-6 space-y-4" @submit.prevent="submitMovement">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Тип операции *</label>
              <select v-model="movementForm.movement_type" class="input-field" required>
                <option value="purchase">Поступление</option>
                <option value="usage">Использование</option>
                <option value="return">Возврат</option>
                <option value="write_off">Списание</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Количество *</label>
              <input
                v-model.number="movementForm.quantity"
                type="number"
                min="1"
                class="input-field"
                required
              />
              <p class="mt-1 text-xs text-gray-500">
                Остаток изменится на
                <span :class="movementDelta >= 0 ? 'text-green-600' : 'text-red-600'">
                  {{ movementDelta > 0 ? '+' : '' }}{{ movementDelta }}
                </span>
                {{ consumable?.unit || 'шт' }}
              </p>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Примечание</label>
              <textarea
                v-model="movementForm.notes"
                rows="3"
                class="input-field"
                placeholder="Например, замена картриджа в кабинете 301"
              ></textarea>
            </div>

            <div class="flex justify-end gap-3 border-t border-gray-200 pt-4">
              <button type="button" class="btn btn-secondary" @click="closeMovementModal">
                Отмена
              </button>
              <button type="submit" class="btn btn-primary" :disabled="savingMovement">
                <Icon v-if="savingMovement" name="ri:loader-4-line" class="mr-2 animate-spin" />
                {{ savingMovement ? 'Сохранение...' : 'Сохранить' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import type {
  Consumable,
  ConsumableMovement,
  ConsumableMovementType,
} from '~/composables/useConsumables'

definePageMeta({ middleware: 'auth' })

const route = useRoute()
const toast = useToast()
const { fetchConsumable, fetchMovements, createMovement } = useConsumables()

const consumableId = Number(route.params.id)

const consumable = ref<Consumable | null>(null)
const movements = ref<ConsumableMovement[]>([])
const stats = ref({ total_in: 0, total_out: 0 })
const loading = ref(true)

const showMovementModal = ref(false)
const savingMovement = ref(false)
const movementForm = ref<{
  movement_type: ConsumableMovementType
  quantity: number
  notes: string
}>({
  movement_type: 'purchase',
  quantity: 1,
  notes: '',
})

const movementDelta = computed(() =>
  movementForm.value.movement_type === 'purchase' || movementForm.value.movement_type === 'return'
    ? movementForm.value.quantity
    : -movementForm.value.quantity,
)

const stockStatus = computed(() => {
  if (!consumable.value) return { label: '—', color: 'bg-gray-100 text-gray-800' }
  if (consumable.value.quantity === 0)
    return { label: 'Нет в наличии', color: 'bg-red-100 text-red-800' }
  if (consumable.value.quantity <= consumable.value.min_quantity)
    return { label: 'Заканчивается', color: 'bg-amber-100 text-amber-800' }
  return { label: 'В наличии', color: 'bg-green-100 text-green-800' }
})

useHead({
  title: computed(() => consumable.value?.name || 'Расходный материал'),
})

const fetchData = async () => {
  loading.value = true
  try {
    const [item, movementResponse] = await Promise.all([
      fetchConsumable(consumableId),
      fetchMovements(consumableId, { per_page: 100 }),
    ])
    consumable.value = item
    movements.value = movementResponse.movements
    stats.value = {
      total_in: movementResponse.total_in,
      total_out: movementResponse.total_out,
    }
  } catch (error) {
    consumable.value = null
    toast.error('Ошибка загрузки расходника: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const openMovementModal = (type: ConsumableMovementType) => {
  movementForm.value = { movement_type: type, quantity: 1, notes: '' }
  showMovementModal.value = true
}

const closeMovementModal = () => {
  showMovementModal.value = false
}

const submitMovement = async () => {
  if (!consumable.value) return
  if (!Number.isFinite(movementForm.value.quantity) || movementForm.value.quantity <= 0) {
    toast.warning('Количество должно быть больше нуля')
    return
  }

  savingMovement.value = true
  try {
    const result = await createMovement(consumableId, {
      movement_type: movementForm.value.movement_type,
      quantity: movementDelta.value,
      notes: movementForm.value.notes.trim() || undefined,
    })
    consumable.value = result.consumable
    toast.success('Движение сохранено')
    closeMovementModal()
    await fetchData()
  } catch (error) {
    toast.error('Ошибка сохранения движения: ' + getErrorMessage(error))
  } finally {
    savingMovement.value = false
  }
}

const formatDateTime = (value: string) => {
  const date = new Date(value)
  return Number.isNaN(date.getTime())
    ? value
    : date.toLocaleString('ru-RU', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
      })
}

const getMovementIcon = (type: string) => {
  const icons: Record<string, string> = {
    purchase: 'ri:shopping-cart-line',
    usage: 'ri:arrow-right-line',
    return: 'ri:arrow-left-line',
    write_off: 'ri:delete-bin-line',
    transfer: 'ri:swap-line',
  }
  return icons[type] || 'ri:arrow-right-line'
}

const getMovementColor = (type: string) => {
  const colors: Record<string, string> = {
    purchase: 'bg-green-500',
    usage: 'bg-blue-500',
    return: 'bg-amber-500',
    write_off: 'bg-red-500',
    transfer: 'bg-purple-500',
  }
  return colors[type] || 'bg-gray-500'
}

const getMovementLabel = (type: string) => {
  const labels: Record<string, string> = {
    purchase: 'Закупка',
    usage: 'Использование',
    return: 'Возврат',
    write_off: 'Списание',
    transfer: 'Перемещение',
  }
  return labels[type] || type
}

onMounted(fetchData)
</script>
