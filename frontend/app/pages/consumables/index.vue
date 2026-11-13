<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-orange-500 to-orange-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:inbox-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Расходные материалы</h1>
            <p class="mt-1 text-gray-600">Учет и управление расходниками</p>
          </div>
        </div>
        <button class="btn btn-primary" @click="openCreateModal">
          <Icon name="ri:add-line" class="mr-2" />
          Добавить расходник
        </button>
      </div>
    </div>

    <!-- Статистика -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Всего позиций</p>
              <p class="text-2xl font-bold text-orange-600">
                {{ stats.total }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-orange-100 flex items-center justify-center">
              <Icon name="ri:inbox-line" class="text-2xl text-orange-600" />
            </div>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">В наличии</p>
              <p class="text-2xl font-bold text-green-600">
                {{ stats.inStock }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center">
              <Icon name="ri:checkbox-circle-line" class="text-2xl text-green-600" />
            </div>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Заканчивается</p>
              <p class="text-2xl font-bold text-amber-600">
                {{ stats.lowStock }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-amber-100 flex items-center justify-center">
              <Icon name="ri:alert-line" class="text-2xl text-amber-600" />
            </div>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Нет в наличии</p>
              <p class="text-2xl font-bold text-red-600">
                {{ stats.outOfStock }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-red-100 flex items-center justify-center">
              <Icon name="ri:close-circle-line" class="text-2xl text-red-600" />
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
            <Icon
              name="ri:search-line"
              class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
            />
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Поиск по названию, производителю или модели..."
              class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            />
          </div>
          <select v-model="selectedCategory" class="input-field w-56">
            <option value="">Все категории</option>
            <option v-for="category in categories" :key="category" :value="category">
              {{ category }}
            </option>
          </select>
        </div>
      </div>
    </div>

    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-orange-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка расходников...</p>
    </div>

    <div v-else-if="consumables.length === 0" class="card p-16 text-center">
      <Icon name="ri:inbox-line" class="text-4xl text-gray-300 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Расходники не найдены</h3>
      <p class="text-sm text-gray-500 mb-4">Добавьте первый расходный материал</p>
      <button class="btn btn-primary" @click="openCreateModal">
        <Icon name="ri:add-line" class="mr-2" />
        Добавить расходник
      </button>
    </div>

    <div v-else-if="filteredConsumables.length === 0" class="card p-16 text-center">
      <Icon name="ri:search-line" class="text-4xl text-gray-300 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Ничего не найдено</h3>
      <p class="text-sm text-gray-500">Измените параметры поиска</p>
    </div>

    <!-- Список расходников -->
    <div v-else class="card">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Наименование
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Категория
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Совместимость
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Количество
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Мин. остаток
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Статус
              </th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">
                Действия
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="item in filteredConsumables" :key="item.id" class="hover:bg-gray-50">
              <td class="px-4 py-4">
                <NuxtLink :to="`/consumables/${item.id}`" class="flex items-center gap-3 group">
                  <Icon name="ri:inbox-line" class="text-gray-400" />
                  <div>
                    <p
                      class="text-sm font-medium text-gray-900 group-hover:text-primary-600 transition-colors"
                    >
                      {{ item.name }}
                    </p>
                    <p class="text-xs text-gray-500">
                      {{ item.manufacturer || 'Производитель не указан' }}
                    </p>
                  </div>
                </NuxtLink>
              </td>
              <td class="px-4 py-4">
                <span v-if="item.category" class="badge bg-blue-100 text-blue-800">
                  {{ item.category }}
                </span>
                <span v-else class="text-sm text-gray-400">—</span>
              </td>
              <td class="px-4 py-4">
                <p class="text-sm text-gray-600">
                  {{ item.compatible_with_name || '—' }}
                </p>
              </td>
              <td class="px-4 py-4">
                <p class="text-sm font-medium text-gray-900">
                  {{ item.quantity }} {{ item.unit || 'шт' }}
                </p>
              </td>
              <td class="px-4 py-4">
                <p class="text-sm text-gray-600">{{ item.min_quantity }} {{ item.unit || 'шт' }}</p>
              </td>
              <td class="px-4 py-4">
                <span :class="['badge', getStockStatus(item).color]">
                  {{ getStockStatus(item).label }}
                </span>
              </td>
              <td class="px-4 py-4 text-right">
                <div class="flex items-center justify-end gap-1">
                  <button
                    class="p-2 rounded-lg hover:bg-green-50 transition-colors"
                    title="Добавить 1"
                    @click="adjustStock(item, 1)"
                  >
                    <Icon name="ri:add-line" class="text-green-600" />
                  </button>
                  <button
                    class="p-2 rounded-lg hover:bg-red-50 transition-colors disabled:opacity-40"
                    title="Списать 1"
                    :disabled="item.quantity === 0"
                    @click="adjustStock(item, -1)"
                  >
                    <Icon name="ri:subtract-line" class="text-red-600" />
                  </button>
                  <button
                    class="p-2 rounded-lg hover:bg-blue-50 transition-colors"
                    title="Редактировать"
                    @click="openEditModal(item)"
                  >
                    <Icon name="ri:edit-line" class="text-blue-600" />
                  </button>
                  <button
                    class="p-2 rounded-lg hover:bg-red-50 transition-colors"
                    title="Удалить"
                    @click="deleteConsumableItem(item)"
                  >
                    <Icon name="ri:delete-bin-line" class="text-red-600" />
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <Teleport to="body">
      <div
        v-if="showModal"
        class="fixed inset-0 z-50 flex items-center justify-center bg-gray-900/50 p-4"
        @click.self="closeModal"
      >
        <div class="w-full max-w-2xl rounded-xl bg-white shadow-2xl">
          <div class="flex items-center justify-between border-b border-gray-200 px-6 py-4">
            <h2 class="text-lg font-semibold text-gray-900">
              {{ isEditMode ? 'Редактировать расходник' : 'Новый расходник' }}
            </h2>
            <button class="p-2 rounded-lg hover:bg-gray-100" @click="closeModal">
              <Icon name="ri:close-line" class="text-xl" />
            </button>
          </div>

          <form class="p-6 space-y-4" @submit.prevent="handleSubmit">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="md:col-span-2">
                <label class="block text-sm font-medium text-gray-700 mb-1">Название *</label>
                <input v-model="form.name" type="text" class="input-field" required />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Производитель</label>
                <input v-model="form.manufacturer" type="text" class="input-field" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Модель</label>
                <input v-model="form.model" type="text" class="input-field" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Категория</label>
                <input
                  v-model="form.category"
                  type="text"
                  class="input-field"
                  placeholder="Картриджи"
                  list="consumable-categories"
                />
                <datalist id="consumable-categories">
                  <option v-for="category in categories" :key="category" :value="category"></option>
                </datalist>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Единица</label>
                <input v-model="form.unit" type="text" class="input-field" placeholder="шт" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Количество *</label>
                <input v-model.number="form.quantity" type="number" min="0" class="input-field" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Мин. остаток *</label>
                <input
                  v-model.number="form.min_quantity"
                  type="number"
                  min="0"
                  class="input-field"
                />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Примечания</label>
              <textarea v-model="form.notes" rows="3" class="input-field"></textarea>
            </div>

            <div class="flex justify-end gap-3 border-t border-gray-200 pt-4">
              <button type="button" class="btn btn-secondary" @click="closeModal">Отмена</button>
              <button type="submit" class="btn btn-primary" :disabled="saving">
                <Icon v-if="saving" name="ri:loader-4-line" class="mr-2 animate-spin" />
                {{ saving ? 'Сохранение...' : 'Сохранить' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import type { Consumable, NewConsumableRequest } from '~/composables/useConsumables'

definePageMeta({
  middleware: 'auth',
})

useHead({
  title: 'Расходные материалы',
})

const toast = useToast()
const { fetchConsumables, createConsumable, updateConsumable, deleteConsumable, createMovement } =
  useConsumables()

interface ConsumableForm {
  name: string
  manufacturer: string
  model: string
  category: string
  quantity: number
  min_quantity: number
  unit: string
  notes: string
}

const emptyForm = (): ConsumableForm => ({
  name: '',
  manufacturer: '',
  model: '',
  category: '',
  quantity: 0,
  min_quantity: 0,
  unit: 'шт',
  notes: '',
})

const searchQuery = ref('')
const selectedCategory = ref('')
const consumables = ref<Consumable[]>([])
const loading = ref(false)
const showModal = ref(false)
const isEditMode = ref(false)
const editingId = ref<number | null>(null)
const saving = ref(false)
const form = ref<ConsumableForm>(emptyForm())

const categories = computed(() =>
  [...new Set(consumables.value.map((item) => item.category).filter(Boolean) as string[])].sort(
    (a, b) => a.localeCompare(b, 'ru'),
  ),
)

const stats = computed(() => ({
  total: consumables.value.length,
  inStock: consumables.value.filter((item) => item.quantity > item.min_quantity).length,
  lowStock: consumables.value.filter(
    (item) => item.quantity > 0 && item.quantity <= item.min_quantity,
  ).length,
  outOfStock: consumables.value.filter((item) => item.quantity === 0).length,
}))

const filteredConsumables = computed(() => {
  let filtered = consumables.value
  if (selectedCategory.value) {
    filtered = filtered.filter((item) => item.category === selectedCategory.value)
  }
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter((item) =>
      [item.name, item.manufacturer, item.model, item.category]
        .filter((value): value is string => Boolean(value))
        .some((value) => value.toLowerCase().includes(query)),
    )
  }
  return filtered
})

const fetchData = async () => {
  loading.value = true
  try {
    const response = await fetchConsumables({ per_page: 100 })
    consumables.value = response.consumables
  } catch (error) {
    toast.error('Ошибка загрузки расходников: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const getStockStatus = (item: Consumable) => {
  if (item.quantity === 0) return { label: 'Нет в наличии', color: 'bg-red-100 text-red-800' }
  if (item.quantity <= item.min_quantity)
    return { label: 'Заканчивается', color: 'bg-amber-100 text-amber-800' }
  return { label: 'В наличии', color: 'bg-green-100 text-green-800' }
}

const openCreateModal = () => {
  isEditMode.value = false
  editingId.value = null
  form.value = emptyForm()
  showModal.value = true
}

const openEditModal = (item: Consumable) => {
  isEditMode.value = true
  editingId.value = item.id
  form.value = {
    name: item.name,
    manufacturer: item.manufacturer ?? '',
    model: item.model ?? '',
    category: item.category ?? '',
    quantity: item.quantity,
    min_quantity: item.min_quantity,
    unit: item.unit ?? '',
    notes: item.notes ?? '',
  }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  editingId.value = null
  form.value = emptyForm()
}

const handleSubmit = async () => {
  if (!form.value.name.trim()) {
    toast.warning('Укажите название расходника')
    return
  }
  if (form.value.quantity < 0 || form.value.min_quantity < 0) {
    toast.warning('Количество не может быть отрицательным')
    return
  }

  const payload: NewConsumableRequest = {
    name: form.value.name.trim(),
    manufacturer: form.value.manufacturer.trim() || undefined,
    model: form.value.model.trim() || undefined,
    category: form.value.category.trim() || undefined,
    quantity: form.value.quantity,
    min_quantity: form.value.min_quantity,
    unit: form.value.unit.trim() || undefined,
    notes: form.value.notes.trim() || undefined,
  }

  saving.value = true
  try {
    if (isEditMode.value && editingId.value !== null) {
      await updateConsumable(editingId.value, payload)
      toast.success('Расходник обновлён')
    } else {
      await createConsumable(payload)
      toast.success('Расходник добавлен')
    }
    closeModal()
    await fetchData()
  } catch (error) {
    toast.error('Ошибка сохранения расходника: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

const adjustStock = async (item: Consumable, direction: 1 | -1) => {
  try {
    await createMovement(item.id, {
      movement_type: direction > 0 ? 'purchase' : 'usage',
      quantity: direction,
      notes: direction > 0 ? 'Быстрое добавление' : 'Быстрое списание',
    })
    toast.success(direction > 0 ? 'Количество увеличено' : 'Количество уменьшено')
    await fetchData()
  } catch (error) {
    toast.error('Ошибка изменения остатка: ' + getErrorMessage(error))
  }
}

const deleteConsumableItem = async (item: Consumable) => {
  if (!confirm(`Удалить «${item.name}»?`)) return
  try {
    await deleteConsumable(item.id)
    toast.success('Расходник удалён')
    await fetchData()
  } catch (error) {
    toast.error('Ошибка удаления расходника: ' + getErrorMessage(error))
  }
}

onMounted(fetchData)
</script>
