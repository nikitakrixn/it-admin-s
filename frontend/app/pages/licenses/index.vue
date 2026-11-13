<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4">
        <div
          class="h-16 w-16 rounded-2xl bg-gradient-to-br from-emerald-500 to-emerald-700 flex items-center justify-center shadow-lg"
        >
          <Icon name="ri:shield-check-line" class="text-3xl text-white" />
        </div>
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Лицензии</h1>
          <p class="mt-1 text-gray-600">Управление лицензиями ПО</p>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Всего лицензий</p>
              <p class="text-2xl font-bold text-emerald-600">
                {{ stats.total }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-emerald-100 flex items-center justify-center">
              <Icon name="ri:shield-check-line" class="text-2xl text-emerald-600" />
            </div>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-gray-600 mb-1">Активные</p>
              <p class="text-2xl font-bold text-green-600">
                {{ stats.active }}
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
              <p class="text-sm text-gray-600 mb-1">Истекает (30 дней)</p>
              <p class="text-2xl font-bold text-amber-600">
                {{ stats.expiring }}
              </p>
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
              <p class="text-sm text-gray-600 mb-1">Истекшие</p>
              <p class="text-2xl font-bold text-red-600">
                {{ stats.expired }}
              </p>
            </div>
            <div class="h-12 w-12 rounded-xl bg-red-100 flex items-center justify-center">
              <Icon name="ri:close-circle-line" class="text-2xl text-red-600" />
            </div>
          </div>
        </div>
      </div>
    </div>

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
              placeholder="Поиск лицензий..."
              class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
            />
          </div>
          <select v-model="selectedType" class="input-field w-48">
            <option value="">Все типы</option>
            <option value="subscription">Подписка</option>
            <option value="perpetual">Бессрочная</option>
            <option value="volume">Корпоративная</option>
            <option value="oem">OEM</option>
            <option value="trial">Пробная</option>
            <option value="educational">Образовательная</option>
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
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Истекает
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Использовано
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Статус
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-if="loading">
              <td colspan="6" class="px-4 py-12 text-center">
                <Icon name="ri:loader-4-line" class="text-3xl text-emerald-500 animate-spin mb-2" />
                <p class="text-sm text-gray-500">Загрузка лицензий...</p>
              </td>
            </tr>
            <tr v-else-if="filteredLicenses.length === 0">
              <td colspan="6" class="px-4 py-12 text-center">
                <Icon name="ri:shield-check-line" class="text-3xl text-gray-300 mb-2" />
                <p class="text-sm text-gray-500">Лицензии не найдены</p>
              </td>
            </tr>
            <template v-else>
              <tr v-for="license in filteredLicenses" :key="license.id" class="hover:bg-gray-50">
                <td class="px-4 py-4">
                  <div class="flex items-center gap-3">
                    <Icon name="ri:apps-2-line" class="text-gray-400" />
                    <div>
                      <p class="text-sm font-medium text-gray-900">
                        {{ softwareName(license) }}
                      </p>
                      <p class="text-xs text-gray-500">
                        {{ softwareManufacturer(license) }}
                      </p>
                    </div>
                  </div>
                </td>
                <td class="px-4 py-4">
                  <span class="badge bg-blue-100 text-blue-800">
                    {{ getTypeLabel(license.license_type) }}
                  </span>
                </td>
                <td class="px-4 py-4">
                  <p class="text-sm font-mono text-gray-900">
                    {{ license.license_key || '—' }}
                  </p>
                </td>
                <td class="px-4 py-4">
                  <p class="text-sm text-gray-900">
                    {{ formatDate(license.expiration_date) }}
                  </p>
                </td>
                <td class="px-4 py-4">
                  <p class="text-sm text-gray-900">
                    {{ license.used_licenses ?? 0 }} / {{ license.total_licenses }}
                  </p>
                </td>
                <td class="px-4 py-4">
                  <span :class="['badge', getLicenseStatus(license).color]">
                    {{ getLicenseStatus(license).label }}
                  </span>
                </td>
              </tr>
            </template>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { LicensePool } from '~/composables/useLicenses'
import type { Software } from '~/composables/useSoftware'

definePageMeta({ middleware: 'auth' })
useHead({ title: 'Лицензии' })

const toast = useToast()
const { fetchLicenses } = useLicenses()
const { fetchSoftware } = useSoftware()

const searchQuery = ref('')
const selectedType = ref('')
const licenses = ref<LicensePool[]>([])
const software = ref<Software[]>([])
const loading = ref(false)

const fetchData = async () => {
  loading.value = true
  try {
    const [licenseList, softwareList] = await Promise.all([
      fetchLicenses({ per_page: 100 }),
      fetchSoftware({ per_page: 100 }),
    ])
    licenses.value = licenseList.licenses
    software.value = softwareList.software
  } catch (error) {
    toast.error('Не удалось загрузить лицензии: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const softwareName = (license: LicensePool) => {
  const item = software.value.find((entry) => entry.id === license.software_catalog_id)
  return item?.name ?? `ПО #${license.software_catalog_id}`
}

const softwareManufacturer = (license: LicensePool) => {
  const item = software.value.find((entry) => entry.id === license.software_catalog_id)
  return item?.manufacturer || 'Производитель не указан'
}

const formatDate = (value: string | null) => {
  if (!value) return 'Бессрочно'
  const [date] = value.split('T')
  const [year, month, day] = date.split('-')
  return day && month && year ? `${day}.${month}.${year}` : value
}

const stats = computed(() => {
  const now = Date.now()
  const monthAhead = now + 30 * 24 * 60 * 60 * 1000
  let active = 0
  let expiring = 0
  let expired = 0

  for (const license of licenses.value) {
    if (!license.expiration_date) {
      active += 1
      continue
    }
    const time = new Date(license.expiration_date).getTime()
    if (Number.isNaN(time)) continue
    if (time < now) expired += 1
    else if (time < monthAhead) expiring += 1
    else active += 1
  }

  return { total: licenses.value.length, active, expiring, expired }
})

const filteredLicenses = computed(() => {
  let filtered = licenses.value
  if (selectedType.value) {
    filtered = filtered.filter((license) => license.license_type === selectedType.value)
  }
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter((license) =>
      [softwareName(license), softwareManufacturer(license), license.license_key ?? ''].some(
        (value) => value.toLowerCase().includes(query),
      ),
    )
  }
  return filtered
})

const getTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    perpetual: 'Бессрочная',
    subscription: 'Подписка',
    volume: 'Корпоративная',
    oem: 'OEM',
    trial: 'Пробная',
    educational: 'Образовательная',
  }
  return labels[type] || type
}

const getLicenseStatus = (license: LicensePool) => {
  if (!license.expiration_date) {
    return { label: 'Активна', color: 'bg-green-100 text-green-800' }
  }

  const expiryDate = new Date(license.expiration_date)
  const daysLeft = Math.floor((expiryDate.getTime() - Date.now()) / (1000 * 60 * 60 * 24))
  if (daysLeft < 0) return { label: 'Истекла', color: 'bg-red-100 text-red-800' }
  if (daysLeft < 30) return { label: 'Истекает', color: 'bg-amber-100 text-amber-800' }
  return { label: 'Активна', color: 'bg-green-100 text-green-800' }
}

onMounted(fetchData)
</script>
