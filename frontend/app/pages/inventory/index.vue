<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-teal-500 to-teal-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:task-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Инвентаризация</h1>
            <p class="mt-1 text-gray-600">Сверка учётных данных с фактическим наличием техники</p>
          </div>
        </div>
        <button class="btn btn-primary" @click="openCreateModal">
          <Icon name="ri:add-line" class="mr-2" />
          Начать инвентаризацию
        </button>
      </div>
    </div>

    <!-- Статистика -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
      <div class="card p-5">
        <div class="flex items-center gap-3">
          <div class="h-11 w-11 rounded-lg bg-teal-100 flex items-center justify-center">
            <Icon name="ri:file-list-2-line" class="text-xl text-teal-600" />
          </div>
          <div>
            <p class="text-xs text-gray-500">Всего сессий</p>
            <p class="text-2xl font-bold text-gray-900">{{ audits.length }}</p>
          </div>
        </div>
      </div>
      <div class="card p-5">
        <div class="flex items-center gap-3">
          <div class="h-11 w-11 rounded-lg bg-blue-100 flex items-center justify-center">
            <Icon name="ri:play-circle-line" class="text-xl text-blue-600" />
          </div>
          <div>
            <p class="text-xs text-gray-500">В работе</p>
            <p class="text-2xl font-bold text-blue-600">{{ stats.in_progress }}</p>
          </div>
        </div>
      </div>
      <div class="card p-5">
        <div class="flex items-center gap-3">
          <div class="h-11 w-11 rounded-lg bg-green-100 flex items-center justify-center">
            <Icon name="ri:checkbox-circle-line" class="text-xl text-green-600" />
          </div>
          <div>
            <p class="text-xs text-gray-500">Завершено</p>
            <p class="text-2xl font-bold text-green-600">{{ stats.completed }}</p>
          </div>
        </div>
      </div>
      <div class="card p-5">
        <div class="flex items-center gap-3">
          <div class="h-11 w-11 rounded-lg bg-gray-100 flex items-center justify-center">
            <Icon name="ri:close-circle-line" class="text-xl text-gray-600" />
          </div>
          <div>
            <p class="text-xs text-gray-500">Отменено</p>
            <p class="text-2xl font-bold text-gray-600">{{ stats.cancelled }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Фильтры -->
    <div class="card">
      <div class="p-4 bg-gradient-to-r from-gray-50 to-white">
        <div class="flex flex-col sm:flex-row items-stretch sm:items-center gap-4">
          <div class="flex-1 relative">
            <Icon
              name="ri:search-line"
              class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
            />
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Поиск по названию, расположению или автору..."
              class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            />
          </div>
          <select v-model="statusFilter" class="input-field sm:w-56">
            <option value="">Все статусы</option>
            <option value="in_progress">В работе</option>
            <option value="completed">Завершённые</option>
            <option value="cancelled">Отменённые</option>
          </select>
          <button
            v-if="searchQuery || statusFilter"
            class="btn btn-secondary whitespace-nowrap"
            @click="resetFilters"
          >
            <Icon name="ri:refresh-line" class="mr-2" />
            Сбросить
          </button>
        </div>
      </div>
    </div>

    <!-- Загрузка -->
    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-teal-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка сессий...</p>
    </div>

    <!-- Пусто -->
    <div v-else-if="filteredAudits.length === 0" class="card p-16 text-center">
      <div class="h-20 w-20 rounded-full bg-teal-100 flex items-center justify-center mx-auto mb-4">
        <Icon name="ri:task-line" class="text-4xl text-teal-600" />
      </div>
      <h3 class="text-lg font-semibold text-gray-900 mb-2">
        {{ audits.length === 0 ? 'Инвентаризаций пока нет' : 'Ничего не найдено' }}
      </h3>
      <p class="text-sm text-gray-500 mb-4">
        {{
          audits.length === 0
            ? 'Создайте сессию — система запомнит текущий список компьютеров'
            : 'Измените запрос или сбросьте фильтры'
        }}
      </p>
      <button v-if="audits.length === 0" class="btn btn-primary" @click="openCreateModal">
        <Icon name="ri:add-line" class="mr-2" />
        Начать инвентаризацию
      </button>
    </div>

    <!-- Список сессий -->
    <div v-else class="grid grid-cols-1 lg:grid-cols-2 gap-4">
      <NuxtLink
        v-for="audit in filteredAudits"
        :key="audit.id"
        :to="`/inventory/${audit.id}`"
        class="card card-hover group"
      >
        <div class="p-5">
          <div class="flex items-start justify-between gap-3 mb-3">
            <div class="min-w-0">
              <h3
                class="text-base font-semibold text-gray-900 truncate group-hover:text-teal-600 transition-colors"
              >
                {{ audit.audit_name }}
              </h3>
              <p class="mt-1 text-xs text-gray-500">
                {{ formatDate(audit.audit_date) }} · {{ AUDIT_TYPE_LABELS[audit.audit_type] }}
              </p>
            </div>
            <span :class="['badge whitespace-nowrap', AUDIT_STATUS_COLORS[audit.status]]">
              {{ AUDIT_STATUS_LABELS[audit.status] }}
            </span>
          </div>

          <div class="space-y-1.5 text-sm text-gray-600 mb-4">
            <div class="flex items-center gap-2">
              <Icon name="ri:map-pin-line" class="text-gray-400" />
              <span>{{ audit.location || 'Все расположения' }}</span>
            </div>
            <div class="flex items-center gap-2">
              <Icon name="ri:user-line" class="text-gray-400" />
              <span class="truncate">{{ audit.audited_by_email || 'Неизвестный автор' }}</span>
            </div>
          </div>

          <div class="flex items-center justify-between text-xs text-gray-500 mb-1.5">
            <span>Проверено {{ audit.counts.checked }} из {{ audit.counts.total }}</span>
            <span class="font-medium text-gray-700">{{ progressPercent(audit) }}%</span>
          </div>
          <div class="h-2 rounded-full bg-gray-100 overflow-hidden">
            <div
              :class="[
                'h-full rounded-full transition-all',
                audit.status === 'completed' ? 'bg-green-500' : 'bg-teal-500',
              ]"
              :style="{ width: `${progressPercent(audit)}%` }"
            ></div>
          </div>

          <div class="mt-4 pt-4 border-t border-gray-100 grid grid-cols-4 gap-2 text-center">
            <div>
              <p class="text-lg font-semibold text-green-600">{{ audit.counts.found }}</p>
              <p class="text-[11px] text-gray-500">Найдено</p>
            </div>
            <div>
              <p class="text-lg font-semibold text-red-600">{{ audit.counts.missing }}</p>
              <p class="text-[11px] text-gray-500">Нет</p>
            </div>
            <div>
              <p class="text-lg font-semibold text-amber-600">{{ audit.counts.damaged }}</p>
              <p class="text-[11px] text-gray-500">Повреждено</p>
            </div>
            <div>
              <p class="text-lg font-semibold text-gray-500">
                {{ audit.counts.total - audit.counts.checked }}
              </p>
              <p class="text-[11px] text-gray-500">Осталось</p>
            </div>
          </div>
        </div>
      </NuxtLink>
    </div>

    <p v-if="!loading && total > audits.length" class="text-center text-xs text-gray-500">
      Показаны последние {{ audits.length }} из {{ total }} сессий.
    </p>

    <!-- Создание сессии -->
    <UiModal v-model="showCreateModal" title="Новая инвентаризация">
      <form id="create-audit-form" class="space-y-4" @submit.prevent="handleCreate">
        <UiInput
          v-model="form.audit_name"
          label="Название"
          required
          placeholder="Например: Инвентаризация офиса, ноябрь 2026"
        />

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <UiSelect v-model="form.audit_type" label="Тип" required>
            <option v-for="(label, value) in AUDIT_TYPE_LABELS" :key="value" :value="value">
              {{ label }}
            </option>
          </UiSelect>

          <UiSelect
            v-model="form.department_id"
            label="Подразделение"
            placeholder="Все подразделения"
          >
            <option v-for="department in departments" :key="department.id" :value="department.id">
              {{ department.name }}
            </option>
          </UiSelect>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Расположение</label>
          <input
            v-model="form.location"
            list="inventory-locations"
            class="input-field"
            placeholder="Все расположения"
          />
          <datalist id="inventory-locations">
            <option v-for="location in locations" :key="location" :value="location"></option>
          </datalist>
          <p class="mt-1 text-xs text-gray-500">
            Совпадение точное, как в карточке компьютера. Пустое поле — взять все компьютеры.
          </p>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Заметки</label>
          <textarea
            v-model="form.notes"
            rows="3"
            class="input-field"
            placeholder="Комментарий к сессии..."
          ></textarea>
        </div>

        <div
          v-if="createError"
          class="p-4 rounded-lg bg-red-50 text-red-800 text-sm flex items-start gap-2"
        >
          <Icon name="ri:error-warning-line" class="text-lg flex-shrink-0 mt-0.5" />
          <span>{{ createError }}</span>
        </div>

        <p class="text-xs text-gray-500">
          В сессию попадёт снимок компьютеров на момент создания. Новые компьютеры, добавленные
          позже, в неё не войдут.
        </p>
      </form>

      <template #footer>
        <button type="button" class="btn btn-secondary" @click="showCreateModal = false">
          Отмена
        </button>
        <button type="submit" form="create-audit-form" class="btn btn-primary" :disabled="saving">
          <Icon
            :name="saving ? 'ri:loader-4-line' : 'ri:play-line'"
            :class="['mr-2', saving && 'animate-spin']"
          />
          {{ saving ? 'Создание...' : 'Начать' }}
        </button>
      </template>
    </UiModal>
  </div>
</template>

<script setup lang="ts">
import type { InventoryAudit, InventoryAuditType } from '~/composables/useInventory'
import type { Department } from '~/composables/useEmployees'
import {
  AUDIT_STATUS_COLORS,
  AUDIT_STATUS_LABELS,
  AUDIT_TYPE_LABELS,
  useInventory,
} from '~/composables/useInventory'

definePageMeta({
  middleware: 'auth',
})

useHead({
  title: 'Инвентаризация',
})

const toast = useToast()
const { fetchAudits, createAudit } = useInventory()
const { fetchDepartments } = useEmployees()
const { fetchComputers } = useComputers()

const audits = ref<InventoryAudit[]>([])
const total = ref(0)
const departments = ref<Department[]>([])
const locations = ref<string[]>([])
const loading = ref(false)
const saving = ref(false)
const showCreateModal = ref(false)
const searchQuery = ref('')
const statusFilter = ref('')
const createError = ref('')

const form = ref({
  audit_name: '',
  audit_type: 'full' as InventoryAuditType,
  department_id: null as number | null,
  location: '',
  notes: '',
})

const stats = computed(() => ({
  in_progress: audits.value.filter((audit) => audit.status === 'in_progress').length,
  completed: audits.value.filter((audit) => audit.status === 'completed').length,
  cancelled: audits.value.filter((audit) => audit.status === 'cancelled').length,
}))

const filteredAudits = computed(() => {
  let result = audits.value

  if (statusFilter.value) {
    result = result.filter((audit) => audit.status === statusFilter.value)
  }

  const query = searchQuery.value.trim().toLowerCase()
  if (query) {
    result = result.filter((audit) =>
      [audit.audit_name, audit.location, audit.audited_by_email]
        .filter((value): value is string => Boolean(value))
        .some((value) => value.toLowerCase().includes(query)),
    )
  }

  return result
})

const formatDate = (value: string | null) => {
  if (!value) return '—'
  const [date] = value.split('T')
  const [year, month, day] = date.split('-')
  return year && month && day ? `${day}.${month}.${year}` : date
}

const progressPercent = (audit: InventoryAudit) => {
  if (audit.counts.total === 0) return 0
  return Math.round((audit.counts.checked / audit.counts.total) * 100)
}

const resetFilters = () => {
  searchQuery.value = ''
  statusFilter.value = ''
}

const loadAudits = async () => {
  loading.value = true
  try {
    const response = await fetchAudits({ per_page: 100 })
    audits.value = response.audits
    total.value = response.total
  } catch (error) {
    toast.error('Не удалось загрузить инвентаризации: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const loadReferenceData = async () => {
  try {
    const [departmentList, computers] = await Promise.all([
      fetchDepartments(),
      fetchComputers({ per_page: 100 }),
    ])
    departments.value = departmentList

    const unique = new Set<string>()
    for (const computer of computers.computers) {
      if (computer.location) unique.add(computer.location)
    }
    locations.value = [...unique].sort((a, b) => a.localeCompare(b, 'ru'))
  } catch {
    // Справочники нужны только для подсказок — без них форму всё равно можно заполнить.
  }
}

const openCreateModal = () => {
  form.value = {
    audit_name: '',
    audit_type: 'full',
    department_id: null,
    location: '',
    notes: '',
  }
  createError.value = ''
  showCreateModal.value = true
}

const handleCreate = async () => {
  const name = form.value.audit_name.trim()
  if (!name) {
    createError.value = 'Укажите название инвентаризации'
    return
  }

  saving.value = true
  createError.value = ''

  try {
    const detail = await createAudit({
      audit_name: name,
      audit_type: form.value.audit_type,
      department_id: form.value.department_id ?? undefined,
      location: form.value.location.trim() || undefined,
      notes: form.value.notes.trim() || undefined,
    })

    showCreateModal.value = false
    toast.success(`Инвентаризация создана: ${detail.items.length} позиций`)
    await navigateTo(`/inventory/${detail.audit.id}`)
  } catch (error) {
    createError.value = getErrorMessage(error, 'Не удалось создать инвентаризацию')
  } finally {
    saving.value = false
  }
}

onMounted(() => {
  loadAudits()
  loadReferenceData()
})
</script>
