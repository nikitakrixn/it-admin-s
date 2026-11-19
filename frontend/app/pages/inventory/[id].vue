<template>
  <div class="space-y-6">
    <!-- Загрузка -->
    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-teal-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка инвентаризации...</p>
    </div>

    <!-- Не найдено -->
    <div v-else-if="notFound || !audit" class="card p-16 text-center">
      <div class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mx-auto mb-4">
        <Icon name="ri:file-unknow-line" class="text-4xl text-gray-400" />
      </div>
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Сессия не найдена</h3>
      <p class="text-sm text-gray-500 mb-4">Возможно, её удалили или ссылка устарела.</p>
      <NuxtLink to="/inventory" class="btn btn-primary">
        <Icon name="ri:arrow-left-line" class="mr-2" />
        К списку инвентаризаций
      </NuxtLink>
    </div>

    <template v-else>
      <!-- Header -->
      <div class="mb-2">
        <NuxtLink
          to="/inventory"
          class="inline-flex items-center gap-1 text-sm text-gray-500 hover:text-teal-600 mb-4"
        >
          <Icon name="ri:arrow-left-line" />
          Все инвентаризации
        </NuxtLink>
        <div class="flex flex-col lg:flex-row lg:items-center justify-between gap-4">
          <div class="flex items-center gap-4 min-w-0">
            <div
              class="h-16 w-16 rounded-2xl bg-gradient-to-br from-teal-500 to-teal-700 flex items-center justify-center shadow-lg flex-shrink-0"
            >
              <Icon name="ri:task-line" class="text-3xl text-white" />
            </div>
            <div class="min-w-0">
              <div class="flex items-center gap-3 flex-wrap">
                <h1 class="text-3xl font-bold text-gray-900 truncate">{{ audit.audit_name }}</h1>
                <span :class="['badge', AUDIT_STATUS_COLORS[audit.status]]">
                  {{ AUDIT_STATUS_LABELS[audit.status] }}
                </span>
              </div>
              <p class="mt-1 text-gray-600">
                {{ formatDate(audit.audit_date) }} · {{ AUDIT_TYPE_LABELS[audit.audit_type] }} ·
                {{ audit.location || 'все расположения' }}
              </p>
            </div>
          </div>

          <div class="flex items-center gap-3">
            <button class="btn btn-secondary" @click="exportReport">
              <Icon name="ri:download-2-line" class="mr-2" />
              Скачать CSV
            </button>
            <template v-if="isActive">
              <button class="btn btn-secondary" @click="showCancelModal = true">
                <Icon name="ri:close-circle-line" class="mr-2" />
                Отменить
              </button>
              <button class="btn btn-primary" @click="openCompleteModal">
                <Icon name="ri:checkbox-circle-line" class="mr-2" />
                Завершить
              </button>
            </template>
          </div>
        </div>
      </div>

      <!-- Сводка -->
      <div class="grid grid-cols-2 md:grid-cols-5 gap-4">
        <div class="card p-4">
          <p class="text-xs text-gray-500">Всего</p>
          <p class="mt-1 text-2xl font-bold text-gray-900">{{ counts.total }}</p>
        </div>
        <div class="card p-4">
          <p class="text-xs text-gray-500">Найдено</p>
          <p class="mt-1 text-2xl font-bold text-green-600">{{ counts.found }}</p>
        </div>
        <div class="card p-4">
          <p class="text-xs text-gray-500">Отсутствует</p>
          <p class="mt-1 text-2xl font-bold text-red-600">{{ counts.missing }}</p>
        </div>
        <div class="card p-4">
          <p class="text-xs text-gray-500">Повреждено</p>
          <p class="mt-1 text-2xl font-bold text-amber-600">{{ counts.damaged }}</p>
        </div>
        <div class="card p-4">
          <p class="text-xs text-gray-500">Не проверено</p>
          <p class="mt-1 text-2xl font-bold text-gray-500">{{ pendingCount }}</p>
        </div>
      </div>

      <!-- Прогресс и сведения -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
        <div class="card lg:col-span-2">
          <div class="p-5">
            <div class="flex items-center justify-between text-sm mb-2">
              <span class="font-medium text-gray-900">Прогресс проверки</span>
              <span class="text-gray-500">
                {{ counts.checked }} из {{ counts.total }} · {{ progressPercent }}%
              </span>
            </div>
            <div class="h-3 rounded-full bg-gray-100 overflow-hidden">
              <div
                :class="[
                  'h-full rounded-full transition-all',
                  progressPercent === 100 ? 'bg-green-500' : 'bg-teal-500',
                ]"
                :style="{ width: `${progressPercent}%` }"
              ></div>
            </div>
            <div v-if="audit.notes" class="mt-4 pt-4 border-t border-gray-100">
              <p class="text-xs font-medium text-gray-500 uppercase tracking-wide mb-1">Заметки</p>
              <p class="text-sm text-gray-700 whitespace-pre-line">{{ audit.notes }}</p>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="p-5 space-y-2.5 text-sm">
            <div class="flex items-center justify-between gap-3">
              <span class="text-gray-500">Автор</span>
              <span class="text-gray-900 truncate">{{ audit.audited_by_email || '—' }}</span>
            </div>
            <div class="flex items-center justify-between gap-3">
              <span class="text-gray-500">Подразделение</span>
              <span class="text-gray-900 truncate">{{ departmentName || 'Все' }}</span>
            </div>
            <div class="flex items-center justify-between gap-3">
              <span class="text-gray-500">Начата</span>
              <span class="text-gray-900">{{ formatDateTime(audit.started_at) }}</span>
            </div>
            <div class="flex items-center justify-between gap-3">
              <span class="text-gray-500">Завершена</span>
              <span class="text-gray-900">{{ formatDateTime(audit.completed_at) }}</span>
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
                placeholder="Поиск по названию, инвентарному номеру или расположению..."
                class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
              />
            </div>
            <select v-model="statusFilter" class="input-field sm:w-56">
              <option value="">Все статусы</option>
              <option v-for="(label, value) in ITEM_STATUS_LABELS" :key="value" :value="value">
                {{ label }}
              </option>
            </select>
            <span class="text-sm text-gray-500 whitespace-nowrap">
              {{ filteredItems.length }} из {{ items.length }}
            </span>
          </div>
        </div>
      </div>

      <!-- Позиции -->
      <div class="card overflow-hidden">
        <div class="overflow-x-auto">
          <table class="w-full min-w-[860px]">
            <thead>
              <tr class="border-b border-gray-200 bg-gray-50">
                <th class="px-5 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Компьютер
                </th>
                <th class="px-5 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Инв. номер
                </th>
                <th class="px-5 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Расположение
                </th>
                <th class="px-5 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                  Статус
                </th>
                <th class="px-5 py-3 text-right text-xs font-medium text-gray-500 uppercase">
                  Действия
                </th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100">
              <tr v-if="filteredItems.length === 0">
                <td colspan="5" class="px-5 py-12 text-center text-sm text-gray-500">
                  {{
                    items.length === 0
                      ? 'В этой сессии нет позиций'
                      : 'Ничего не найдено — измените запрос или статус'
                  }}
                </td>
              </tr>
              <tr
                v-for="item in filteredItems"
                :key="item.id"
                class="hover:bg-gray-50 transition-colors"
              >
                <td class="px-5 py-4">
                  <div class="flex items-center gap-3">
                    <div
                      class="h-10 w-10 rounded-lg bg-teal-100 flex items-center justify-center flex-shrink-0"
                    >
                      <Icon name="ri:computer-line" class="text-teal-600" />
                    </div>
                    <div class="min-w-0">
                      <p class="font-medium text-gray-900 truncate">
                        {{ item.entity_name || `Компьютер #${item.entity_id}` }}
                      </p>
                      <p
                        v-if="item.checked_at && item.status !== 'pending'"
                        class="text-xs text-gray-500 truncate"
                      >
                        Проверено {{ formatDateTime(item.checked_at) }}
                        <template v-if="item.checked_by_email">
                          · {{ item.checked_by_email }}
                        </template>
                      </p>
                    </div>
                  </div>
                </td>
                <td class="px-5 py-4 text-sm text-gray-700">
                  {{ item.entity_inventory_number || '—' }}
                </td>
                <td class="px-5 py-4 text-sm">
                  <p class="text-gray-700">{{ item.expected_location || '—' }}</p>
                  <p
                    v-if="item.actual_location && item.actual_location !== item.expected_location"
                    class="text-xs text-blue-600 flex items-center gap-1 mt-0.5"
                  >
                    <Icon name="ri:arrow-right-line" />
                    фактически: {{ item.actual_location }}
                  </p>
                </td>
                <td class="px-5 py-4">
                  <span :class="['badge', ITEM_STATUS_COLORS[item.status]]">
                    {{ ITEM_STATUS_LABELS[item.status] }}
                  </span>
                  <p v-if="item.discrepancy_notes" class="text-xs text-gray-500 mt-1 max-w-[220px]">
                    {{ item.discrepancy_notes }}
                  </p>
                </td>
                <td class="px-5 py-4">
                  <div class="flex items-center justify-end gap-1.5">
                    <template v-if="isActive">
                      <button
                        type="button"
                        class="p-2 rounded-lg bg-green-50 text-green-700 hover:bg-green-100 disabled:opacity-50"
                        title="Найдено"
                        :disabled="savingItemId === item.id"
                        @click="applyStatus(item, 'found')"
                      >
                        <Icon
                          :name="savingItemId === item.id ? 'ri:loader-4-line' : 'ri:check-line'"
                          :class="savingItemId === item.id && 'animate-spin'"
                        />
                      </button>
                      <button
                        type="button"
                        class="p-2 rounded-lg bg-red-50 text-red-700 hover:bg-red-100 disabled:opacity-50"
                        title="Отсутствует"
                        :disabled="savingItemId === item.id"
                        @click="applyStatus(item, 'missing')"
                      >
                        <Icon name="ri:close-line" />
                      </button>
                      <button
                        type="button"
                        class="p-2 rounded-lg bg-amber-50 text-amber-700 hover:bg-amber-100 disabled:opacity-50"
                        title="Повреждено"
                        :disabled="savingItemId === item.id"
                        @click="applyStatus(item, 'damaged')"
                      >
                        <Icon name="ri:error-warning-line" />
                      </button>
                    </template>
                    <button
                      type="button"
                      class="p-2 rounded-lg bg-gray-100 text-gray-600 hover:bg-gray-200"
                      title="Подробнее"
                      @click="openItemModal(item)"
                    >
                      <Icon name="ri:more-2-line" />
                    </button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </template>

    <!-- Детали позиции -->
    <UiModal v-model="showItemModal" title="Результат проверки">
      <form id="item-form" class="space-y-4" @submit.prevent="saveItemDetails">
        <div class="p-4 rounded-lg bg-gray-50">
          <p class="font-medium text-gray-900">
            {{ editingItem?.entity_name || `Компьютер #${editingItem?.entity_id}` }}
          </p>
          <p class="text-sm text-gray-500">
            {{ editingItem?.entity_inventory_number || 'Без инвентарного номера' }} · учётное место:
            {{ editingItem?.expected_location || '—' }}
          </p>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <UiSelect v-model="itemForm.status" label="Статус" required>
            <option v-for="(label, value) in ITEM_STATUS_LABELS" :key="value" :value="value">
              {{ label }}
            </option>
          </UiSelect>

          <UiSelect v-model="itemForm.actual_condition" label="Состояние" placeholder="Не указано">
            <option v-for="(label, value) in CONDITION_LABELS" :key="value" :value="value">
              {{ label }}
            </option>
          </UiSelect>
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1"
            >Фактическое расположение</label
          >
          <input
            v-model="itemForm.actual_location"
            class="input-field"
            placeholder="Кабинет, склад, у сотрудника..."
          />
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Расхождения</label>
          <textarea
            v-model="itemForm.discrepancy_notes"
            rows="3"
            class="input-field"
            placeholder="Что не совпало с учётными данными..."
          ></textarea>
        </div>

        <div
          v-if="itemError"
          class="p-4 rounded-lg bg-red-50 text-red-800 text-sm flex items-start gap-2"
        >
          <Icon name="ri:error-warning-line" class="text-lg flex-shrink-0 mt-0.5" />
          <span>{{ itemError }}</span>
        </div>

        <p v-if="!isActive" class="text-xs text-gray-500">
          Сессия завершена: отметки доступны только для просмотра.
        </p>
      </form>

      <template #footer>
        <button type="button" class="btn btn-secondary" @click="showItemModal = false">
          {{ isActive ? 'Отмена' : 'Закрыть' }}
        </button>
        <button
          v-if="isActive"
          type="submit"
          form="item-form"
          class="btn btn-primary"
          :disabled="savingItem"
        >
          <Icon
            :name="savingItem ? 'ri:loader-4-line' : 'ri:save-line'"
            :class="['mr-2', savingItem && 'animate-spin']"
          />
          {{ savingItem ? 'Сохранение...' : 'Сохранить' }}
        </button>
      </template>
    </UiModal>

    <!-- Завершение -->
    <UiModal
      v-model="showCompleteModal"
      title="Завершить инвентаризацию?"
      size="sm"
      :close-on-backdrop="!completing"
    >
      <div class="space-y-4">
        <p class="text-sm text-gray-600">
          Проверено {{ counts.checked }} из {{ counts.total }} позиций.
        </p>

        <div
          v-if="pendingCount > 0"
          class="p-4 rounded-lg bg-amber-50 border border-amber-200 text-sm text-amber-800"
        >
          <div class="flex items-start gap-2">
            <Icon name="ri:error-warning-line" class="text-lg flex-shrink-0 mt-0.5" />
            <div>
              <p class="font-medium">Осталось непроверенных: {{ pendingCount }}</p>
              <label class="mt-2 flex items-start gap-2 cursor-pointer">
                <input v-model="markPendingMissing" type="checkbox" class="mt-0.5" />
                <span>Отметить их как отсутствующие</span>
              </label>
            </div>
          </div>
        </div>

        <div
          v-else
          class="p-4 rounded-lg bg-green-50 border border-green-200 text-sm text-green-800 flex items-start gap-2"
        >
          <Icon name="ri:checkbox-circle-line" class="text-lg flex-shrink-0 mt-0.5" />
          <span>Все позиции проверены.</span>
        </div>

        <p class="text-xs text-gray-500">
          После завершения изменить отметки будет нельзя. Итоги сохранятся в истории сессии.
        </p>
      </div>

      <template #footer>
        <button
          type="button"
          class="btn btn-secondary"
          :disabled="completing"
          @click="showCompleteModal = false"
        >
          Отмена
        </button>
        <button
          type="button"
          class="btn btn-primary"
          :disabled="completing || (pendingCount > 0 && !markPendingMissing)"
          @click="confirmComplete"
        >
          <Icon
            :name="completing ? 'ri:loader-4-line' : 'ri:checkbox-circle-line'"
            :class="['mr-2', completing && 'animate-spin']"
          />
          {{ completing ? 'Завершение...' : 'Завершить' }}
        </button>
      </template>
    </UiModal>

    <!-- Отмена -->
    <ConfirmModal
      :show="showCancelModal"
      :loading="cancelling"
      title="Отменить инвентаризацию?"
      :message="`Сессия «${audit?.audit_name ?? ''}» будет отменена. Отметки сохранятся, но завершить её уже не получится.`"
      confirm-text="Да, отменить"
      cancel-text="Продолжить проверку"
      loading-text="Отмена..."
      variant="danger"
      icon="ri:close-circle-line"
      @confirm="confirmCancel"
      @cancel="showCancelModal = false"
    />
  </div>
</template>

<script setup lang="ts">
import type {
  InventoryAuditDetailResponse,
  InventoryCondition,
  InventoryItem,
  InventoryItemStatus,
} from '~/composables/useInventory'
import {
  AUDIT_STATUS_COLORS,
  AUDIT_STATUS_LABELS,
  AUDIT_TYPE_LABELS,
  CONDITION_LABELS,
  ITEM_STATUS_COLORS,
  ITEM_STATUS_LABELS,
  summarizeInventory,
  useInventory,
} from '~/composables/useInventory'
import { downloadCsv } from '~/utils/csv'
import { buildInventoryReportCsv } from '~/utils/inventoryReport'

definePageMeta({
  middleware: 'auth',
})

const route = useRoute()
const toast = useToast()
const { fetchAudit, updateItem, completeAudit, cancelAudit } = useInventory()
const { fetchDepartments } = useEmployees()

const auditId = Number(route.params.id)

const detail = ref<InventoryAuditDetailResponse | null>(null)
const loading = ref(true)
const notFound = ref(false)
const departmentName = ref('')
const searchQuery = ref('')
const statusFilter = ref('')
const savingItemId = ref<number | null>(null)

const showItemModal = ref(false)
const editingItem = ref<InventoryItem | null>(null)
const savingItem = ref(false)
const itemError = ref('')
const itemForm = ref({
  status: 'found' as InventoryItemStatus,
  actual_location: '',
  actual_condition: '' as InventoryCondition | '',
  discrepancy_notes: '',
})

const showCompleteModal = ref(false)
const completing = ref(false)
const markPendingMissing = ref(false)

const showCancelModal = ref(false)
const cancelling = ref(false)

const audit = computed(() => detail.value?.audit ?? null)
const items = computed(() => detail.value?.items ?? [])
const isActive = computed(() => audit.value?.status === 'in_progress')

// audit.counts заполняются только при завершении сессии, а таблица живёт
// локально: после каждой отметки сервер возвращает одну позицию.
const counts = computed(() => summarizeInventory(items.value))

const pendingCount = computed(() => counts.value.total - counts.value.checked)

const progressPercent = computed(() => {
  if (counts.value.total === 0) return 0
  return Math.round((counts.value.checked / counts.value.total) * 100)
})

const filteredItems = computed(() => {
  let result = items.value

  if (statusFilter.value) {
    result = result.filter((item) => item.status === statusFilter.value)
  }

  const query = searchQuery.value.trim().toLowerCase()
  if (query) {
    result = result.filter((item) =>
      [item.entity_name, item.entity_inventory_number, item.expected_location, item.actual_location]
        .filter((value): value is string => Boolean(value))
        .some((value) => value.toLowerCase().includes(query)),
    )
  }

  return result
})

const exportReport = () => {
  if (!audit.value) return

  const csv = buildInventoryReportCsv(audit.value, items.value)
  downloadCsv(`inventory-${audit.value.id}-${audit.value.audit_date}.csv`, csv)
  toast.success('Отчёт скачан')
}

const replaceItem = (updated: InventoryItem) => {
  if (!detail.value) return
  const index = detail.value.items.findIndex((item) => item.id === updated.id)
  if (index >= 0) {
    detail.value.items[index] = updated
  }
}

const loadDetail = async () => {
  loading.value = true
  notFound.value = false

  try {
    detail.value = await fetchAudit(auditId)
  } catch (error) {
    if (getErrorStatus(error) === 404) {
      notFound.value = true
    } else {
      toast.error('Не удалось загрузить инвентаризацию: ' + getErrorMessage(error))
    }
  } finally {
    loading.value = false
  }
}

const loadDepartmentName = async () => {
  const departmentId = detail.value?.audit.department_id
  if (!departmentId) return

  try {
    const departments = await fetchDepartments()
    departmentName.value =
      departments.find((department) => department.id === departmentId)?.name ?? ''
  } catch {
    // Название подразделения — необязательная деталь.
  }
}

const applyStatus = async (item: InventoryItem, status: InventoryItemStatus) => {
  if (!audit.value || !isActive.value) return

  savingItemId.value = item.id
  try {
    const updated = await updateItem(audit.value.id, item.id, { status })
    replaceItem(updated)
  } catch (error) {
    toast.error(getErrorMessage(error, 'Не удалось обновить позицию'))
  } finally {
    savingItemId.value = null
  }
}

const openItemModal = (item: InventoryItem) => {
  editingItem.value = item
  itemForm.value = {
    status: item.status,
    actual_location: item.actual_location ?? item.expected_location ?? '',
    actual_condition: item.actual_condition ?? '',
    discrepancy_notes: item.discrepancy_notes ?? '',
  }
  itemError.value = ''
  showItemModal.value = true
}

const saveItemDetails = async () => {
  if (!audit.value || !editingItem.value) return

  savingItem.value = true
  itemError.value = ''

  try {
    const updated = await updateItem(audit.value.id, editingItem.value.id, {
      status: itemForm.value.status,
      // Пустые строки отправляем явно: бэкенд трактует их как «очистить»,
      // а отсутствие поля — как «не трогать» (так работают быстрые кнопки).
      actual_location: itemForm.value.actual_location.trim(),
      actual_condition: itemForm.value.actual_condition,
      discrepancy_notes: itemForm.value.discrepancy_notes.trim(),
    })

    replaceItem(updated)
    showItemModal.value = false
    toast.success('Позиция обновлена')
  } catch (error) {
    itemError.value = getErrorMessage(error, 'Не удалось сохранить позицию')
  } finally {
    savingItem.value = false
  }
}

const openCompleteModal = () => {
  markPendingMissing.value = false
  showCompleteModal.value = true
}

const confirmComplete = async () => {
  if (!audit.value) return

  completing.value = true
  try {
    detail.value = await completeAudit(audit.value.id, markPendingMissing.value)
    showCompleteModal.value = false
    toast.success(
      `Инвентаризация завершена: найдено ${counts.value.found}, отсутствует ${counts.value.missing}`,
    )
  } catch (error) {
    toast.error(getErrorMessage(error, 'Не удалось завершить инвентаризацию'))
  } finally {
    completing.value = false
  }
}

const confirmCancel = async () => {
  if (!audit.value) return

  cancelling.value = true
  try {
    detail.value = await cancelAudit(audit.value.id)
    showCancelModal.value = false
    toast.success('Инвентаризация отменена')
  } catch (error) {
    toast.error(getErrorMessage(error, 'Не удалось отменить инвентаризацию'))
  } finally {
    cancelling.value = false
  }
}

onMounted(async () => {
  await loadDetail()
  await loadDepartmentName()
})
</script>
