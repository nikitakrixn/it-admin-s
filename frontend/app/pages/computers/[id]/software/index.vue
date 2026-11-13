<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4 mb-6">
        <NuxtLink
          :to="`/computers/${route.params.id}`"
          class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
        >
          <Icon name="ri:arrow-left-line" class="text-xl" />
        </NuxtLink>
        <div class="flex-1 flex items-center justify-between">
          <div class="flex items-center gap-4">
            <div
              class="h-16 w-16 rounded-2xl bg-gradient-to-br from-purple-500 to-purple-700 flex items-center justify-center shadow-lg"
            >
              <Icon name="ri:apps-line" class="text-3xl text-white" />
            </div>
            <div>
              <h1 class="text-3xl font-bold text-gray-900">Программное обеспечение</h1>
              <p class="mt-1 text-gray-600">{{ computerName }}</p>
            </div>
          </div>
          <button class="btn btn-primary" @click="openAddModal">
            <Icon name="ri:add-line" class="mr-2" />
            Добавить ПО
          </button>
        </div>
      </div>
    </div>

    <!-- Статистика -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Всего ПО</p>
              <p class="text-2xl font-bold text-purple-600">{{ softwareList.length }}</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center">
              <Icon name="ri:apps-line" class="text-2xl text-purple-600" />
            </div>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Лицензионное</p>
              <p class="text-2xl font-bold text-green-600">{{ licensedCount }}</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center">
              <Icon name="ri:shield-check-line" class="text-2xl text-green-600" />
            </div>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Истекает</p>
              <p class="text-2xl font-bold text-amber-600">{{ expiringCount }}</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-amber-100 flex items-center justify-center">
              <Icon name="ri:time-line" class="text-2xl text-amber-600" />
            </div>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Истекло</p>
              <p class="text-2xl font-bold text-red-600">{{ expiredCount }}</p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-red-100 flex items-center justify-center">
              <Icon name="ri:error-warning-line" class="text-2xl text-red-600" />
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Фильтры -->
    <div class="card">
      <div class="p-4">
        <div class="flex flex-col md:flex-row md:items-center gap-4">
          <div class="flex-1 relative">
            <Icon
              name="ri:search-line"
              class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
            />
            <input
              v-model="searchQuery"
              type="text"
              placeholder="Поиск по названию или производителю..."
              class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            />
          </div>
          <select v-model="selectedCategory" class="input-field md:w-56">
            <option value="">Все категории</option>
            <option v-for="category in categories" :key="category" :value="category">
              {{ category }}
            </option>
          </select>
          <select v-model="selectedStatus" class="input-field md:w-48">
            <option value="">Все статусы</option>
            <option value="active">Активно</option>
            <option value="inactive">Неактивно</option>
            <option value="trial">Пробный период</option>
            <option value="expired">Истекло</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Список ПО -->
    <div class="card">
      <div v-if="pending" class="flex justify-center py-16">
        <Icon name="ri:loader-4-line" class="text-3xl text-primary-500 animate-spin" />
      </div>

      <div v-else-if="filteredSoftware.length === 0" class="p-16 text-center">
        <div
          class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mx-auto mb-4"
        >
          <Icon name="ri:apps-line" class="text-4xl text-gray-400" />
        </div>
        <h3 class="text-lg font-semibold text-gray-900 mb-2">ПО не найдено</h3>
        <p class="text-sm text-gray-500 mb-4">
          {{
            softwareList.length
              ? 'Измените фильтры поиска'
              : 'Установите первое приложение из каталога'
          }}
        </p>
        <button class="btn btn-primary" @click="openAddModal">
          <Icon name="ri:add-line" class="mr-2" />
          Добавить ПО
        </button>
      </div>

      <div v-else class="overflow-x-auto">
        <table class="w-full">
          <thead class="bg-gray-50 border-b border-gray-200">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-semibold text-gray-600 uppercase">
                Название
              </th>
              <th class="px-6 py-3 text-left text-xs font-semibold text-gray-600 uppercase">
                Версия
              </th>
              <th class="px-6 py-3 text-left text-xs font-semibold text-gray-600 uppercase">
                Лицензия
              </th>
              <th class="px-6 py-3 text-left text-xs font-semibold text-gray-600 uppercase">
                Установлено
              </th>
              <th class="px-6 py-3 text-left text-xs font-semibold text-gray-600 uppercase">
                Статус
              </th>
              <th class="px-6 py-3 text-right text-xs font-semibold text-gray-600 uppercase">
                Действия
              </th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-200">
            <tr
              v-for="item in filteredSoftware"
              :key="item.id"
              class="hover:bg-gray-50 transition-colors"
            >
              <td class="px-6 py-4">
                <div class="flex items-center gap-3">
                  <div
                    class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center flex-shrink-0"
                  >
                    <Icon name="ri:apps-2-line" class="text-purple-600" />
                  </div>
                  <div class="min-w-0">
                    <NuxtLink
                      :to="`/software/${item.software_id}`"
                      class="text-sm font-medium text-gray-900 hover:text-primary-600 transition-colors"
                    >
                      {{ item.name }}
                    </NuxtLink>
                    <p class="text-xs text-gray-500">
                      {{ item.manufacturer || 'Производитель не указан' }}
                      <template v-if="item.category"> · {{ item.category }}</template>
                    </p>
                  </div>
                </div>
              </td>
              <td class="px-6 py-4 text-sm text-gray-900">{{ item.version || '—' }}</td>
              <td class="px-6 py-4">
                <p class="text-sm text-gray-900">{{ getLicenseLabel(item.license_type) }}</p>
                <p v-if="item.license_end_date" class="text-xs text-gray-500">
                  до {{ formatDate(item.license_end_date) }}
                </p>
              </td>
              <td class="px-6 py-4 text-sm text-gray-900">
                {{ formatDate(item.installation_date) }}
              </td>
              <td class="px-6 py-4">
                <span class="badge" :class="getStatusColor(item.status)">
                  {{ getStatusLabel(item.status) }}
                </span>
              </td>
              <td class="px-6 py-4 text-right">
                <button
                  class="p-2 rounded-lg hover:bg-red-50 transition-colors"
                  title="Удалить"
                  @click="uninstall(item)"
                >
                  <Icon name="ri:delete-bin-line" class="text-red-600" />
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Модальное окно добавления -->
    <Teleport to="body">
      <Transition name="modal">
        <div v-if="showAddModal" class="fixed inset-0 z-[60] overflow-y-auto">
          <div class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm" @click="closeAddModal"></div>

          <div class="relative min-h-screen flex items-center justify-center p-4">
            <div class="relative bg-white rounded-2xl shadow-2xl max-w-2xl w-full">
              <div
                class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
              >
                <div class="flex items-center justify-between">
                  <h2 class="text-xl font-semibold text-gray-900">Добавить ПО</h2>
                  <button
                    class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                    @click="closeAddModal"
                  >
                    <Icon name="ri:close-line" class="text-xl text-gray-600" />
                  </button>
                </div>
              </div>

              <form class="p-6 space-y-4" @submit.prevent="installSelected">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">
                    Приложение из каталога *
                  </label>
                  <input
                    v-model="catalogSearch"
                    type="text"
                    placeholder="Поиск в каталоге..."
                    class="input-field mb-2"
                  />
                  <div
                    class="max-h-56 overflow-y-auto border border-gray-200 rounded-lg divide-y divide-gray-100"
                  >
                    <button
                      v-for="software in filteredCatalog"
                      :key="software.id"
                      type="button"
                      :class="[
                        'w-full px-4 py-3 text-left hover:bg-gray-50 transition-colors',
                        form.software_catalog_id === software.id ? 'bg-primary-50' : '',
                      ]"
                      @click="selectSoftware(software)"
                    >
                      <p class="text-sm font-medium text-gray-900">{{ software.name }}</p>
                      <p class="text-xs text-gray-500">
                        {{ software.manufacturer || 'Производитель не указан' }}
                        <template v-if="software.category"> · {{ software.category }}</template>
                      </p>
                    </button>
                    <p v-if="filteredCatalog.length === 0" class="px-4 py-6 text-sm text-gray-500">
                      {{ catalogPending ? 'Загрузка каталога...' : 'Ничего не найдено' }}
                    </p>
                  </div>
                </div>

                <div class="grid grid-cols-2 gap-4">
                  <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">Версия</label>
                    <input v-model="form.version" type="text" class="input-field" />
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">Тип лицензии</label>
                    <select v-model="form.license_type" class="input-field">
                      <option value="">Не указана</option>
                      <option value="perpetual">Бессрочная</option>
                      <option value="subscription">Подписка</option>
                      <option value="trial">Пробная</option>
                      <option value="free">Бесплатная</option>
                      <option value="oem">OEM</option>
                      <option value="volume">Корпоративная</option>
                      <option value="unknown">Неизвестна</option>
                    </select>
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                      Дата установки
                    </label>
                    <input v-model="form.installation_date" type="date" class="input-field" />
                  </div>
                  <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                      Лицензия действует до
                    </label>
                    <input v-model="form.license_end_date" type="date" class="input-field" />
                  </div>
                </div>

                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">Примечания</label>
                  <textarea
                    v-model="form.notes"
                    rows="2"
                    class="input-field resize-none"
                    placeholder="Ключ, комментарий..."
                  ></textarea>
                </div>

                <div class="flex gap-3 pt-2">
                  <button type="button" class="flex-1 btn btn-secondary" @click="closeAddModal">
                    Отмена
                  </button>
                  <button
                    type="submit"
                    class="flex-1 btn btn-primary disabled:opacity-50"
                    :disabled="saving || !form.software_catalog_id"
                  >
                    <Icon v-if="saving" name="ri:loader-4-line" class="mr-2 animate-spin" />
                    <Icon v-else name="ri:download-line" class="mr-2" />
                    {{ saving ? 'Добавление...' : 'Установить' }}
                  </button>
                </div>
              </form>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import type {
  InstalledSoftware,
  NewInstalledSoftwareRequest,
} from '~/composables/useComputerComponents'
import type { Software } from '~/composables/useSoftware'

definePageMeta({
  middleware: 'auth',
})

const route = useRoute()
const toast = useToast()
const { fetchComputer } = useComputers()
const { fetchInstalledSoftware, installSoftware, uninstallSoftware } = useComputerComponents()
const { fetchSoftware } = useSoftware()

const computerId = computed(() => Number(route.params.id))
const computerName = ref('')
const softwareList = ref<InstalledSoftware[]>([])
const pending = ref(true)

const searchQuery = ref('')
const selectedCategory = ref('')
const selectedStatus = ref('')

const showAddModal = ref(false)
const saving = ref(false)
const catalog = ref<Software[]>([])
const catalogPending = ref(false)
const catalogLoaded = ref(false)
const catalogSearch = ref('')

const form = ref<{
  software_catalog_id: number | null
  version: string
  license_type: string
  installation_date: string
  license_end_date: string
  notes: string
}>({
  software_catalog_id: null,
  version: '',
  license_type: '',
  installation_date: '',
  license_end_date: '',
  notes: '',
})

const categories = computed(() => {
  const unique = new Set<string>()
  softwareList.value.forEach((item) => {
    if (item.category) unique.add(item.category)
  })
  return Array.from(unique).sort((a, b) => a.localeCompare(b, 'ru'))
})

const filteredSoftware = computed(() => {
  let filtered = softwareList.value

  if (selectedCategory.value) {
    filtered = filtered.filter((item) => item.category === selectedCategory.value)
  }

  if (selectedStatus.value) {
    filtered = filtered.filter((item) => item.status === selectedStatus.value)
  }

  const query = searchQuery.value.trim().toLowerCase()
  if (query) {
    filtered = filtered.filter(
      (item) =>
        item.name.toLowerCase().includes(query) ||
        (item.manufacturer || '').toLowerCase().includes(query) ||
        (item.category || '').toLowerCase().includes(query),
    )
  }

  return filtered
})

const filteredCatalog = computed(() => {
  const installed = new Set(softwareList.value.map((item) => item.software_id))
  const query = catalogSearch.value.trim().toLowerCase()

  return catalog.value
    .filter((item) => !installed.has(item.id))
    .filter(
      (item) =>
        !query ||
        item.name.toLowerCase().includes(query) ||
        (item.manufacturer || '').toLowerCase().includes(query) ||
        (item.category || '').toLowerCase().includes(query),
    )
})

const licensedCount = computed(
  () =>
    softwareList.value.filter((item) => item.license_type && item.license_type !== 'free').length,
)

const expiringCount = computed(() => {
  const now = Date.now()
  const inMonth = now + 30 * 24 * 60 * 60 * 1000
  return softwareList.value.filter((item) => {
    if (!item.license_end_date) return false
    const end = new Date(item.license_end_date).getTime()
    return !Number.isNaN(end) && end >= now && end <= inMonth
  }).length
})

const expiredCount = computed(() => {
  const now = Date.now()
  return softwareList.value.filter((item) => {
    if (!item.license_end_date) return false
    const end = new Date(item.license_end_date).getTime()
    return !Number.isNaN(end) && end < now
  }).length
})

const loadData = async () => {
  pending.value = true
  try {
    const [computer, software] = await Promise.all([
      fetchComputer(computerId.value),
      fetchInstalledSoftware(computerId.value),
    ])
    computerName.value = computer.hostname || `Компьютер #${computer.id}`
    softwareList.value = software.software
  } catch (error) {
    toast.error('Не удалось загрузить ПО: ' + getErrorMessage(error))
  } finally {
    pending.value = false
  }
}

const loadCatalog = async () => {
  if (catalogLoaded.value || catalogPending.value) return

  catalogPending.value = true
  try {
    const response = await fetchSoftware({ per_page: 100 })
    catalog.value = response.software
    catalogLoaded.value = true
  } catch (error) {
    toast.error('Не удалось загрузить каталог ПО: ' + getErrorMessage(error))
  } finally {
    catalogPending.value = false
  }
}

const openAddModal = () => {
  form.value = {
    software_catalog_id: null,
    version: '',
    license_type: '',
    installation_date: new Date().toISOString().slice(0, 10),
    license_end_date: '',
    notes: '',
  }
  catalogSearch.value = ''
  showAddModal.value = true
  loadCatalog()
}

const closeAddModal = () => {
  showAddModal.value = false
}

const selectSoftware = (software: Software) => {
  form.value.software_catalog_id = software.id
}

const installSelected = async () => {
  if (!form.value.software_catalog_id) {
    toast.warning('Выберите приложение из каталога')
    return
  }

  const request: NewInstalledSoftwareRequest = {
    software_catalog_id: form.value.software_catalog_id,
    version: form.value.version.trim() || undefined,
    license_type: form.value.license_type || undefined,
    installation_date: form.value.installation_date || undefined,
    license_end_date: form.value.license_end_date || undefined,
    notes: form.value.notes.trim() || undefined,
  }

  saving.value = true
  try {
    const installed = await installSoftware(computerId.value, request)
    softwareList.value = [...softwareList.value, installed].sort((a, b) =>
      a.name.localeCompare(b.name, 'ru'),
    )
    toast.success('ПО добавлено')
    showAddModal.value = false
  } catch (error) {
    toast.error('Не удалось добавить ПО: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

const uninstall = async (item: InstalledSoftware) => {
  if (!confirm(`Удалить «${item.name}» с компьютера?`)) return

  try {
    await uninstallSoftware(computerId.value, item.id)
    softwareList.value = softwareList.value.filter((entry) => entry.id !== item.id)
    toast.success('ПО удалено')
  } catch (error) {
    toast.error('Не удалось удалить ПО: ' + getErrorMessage(error))
  }
}

const formatDate = (value: string | null) => {
  if (!value) return '—'
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleDateString('ru-RU')
}

const getLicenseLabel = (type: string | null) => {
  const labels: Record<string, string> = {
    perpetual: 'Бессрочная',
    subscription: 'Подписка',
    trial: 'Пробная',
    free: 'Бесплатная',
    oem: 'OEM',
    volume: 'Корпоративная',
    unknown: 'Не указана',
  }
  return type ? labels[type] || type : 'Не указана'
}

const getStatusLabel = (status: string | null) => {
  const labels: Record<string, string> = {
    active: 'Активно',
    inactive: 'Неактивно',
    trial: 'Пробный период',
    expired: 'Истекло',
  }
  return status ? labels[status] || status : '—'
}

const getStatusColor = (status: string | null) => {
  const colors: Record<string, string> = {
    active: 'bg-green-100 text-green-800',
    inactive: 'bg-gray-100 text-gray-800',
    trial: 'bg-amber-100 text-amber-800',
    expired: 'bg-red-100 text-red-800',
  }
  return (status && colors[status]) || 'bg-gray-100 text-gray-800'
}

useHead({
  title: computed(() => `ПО - ${computerName.value}`),
})

onMounted(loadData)
</script>

<style scoped>
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}
</style>
