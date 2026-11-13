<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4 mb-6">
        <NuxtLink to="/software" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
          <Icon name="ri:arrow-left-line" class="text-xl" />
        </NuxtLink>
        <div class="flex-1 flex items-center justify-between">
          <div class="flex items-center gap-4">
            <div
              class="h-16 w-16 rounded-2xl bg-gradient-to-br from-indigo-500 to-indigo-700 flex items-center justify-center shadow-lg"
            >
              <Icon name="ri:apps-2-line" class="text-3xl text-white" />
            </div>
            <div>
              <h1 class="text-3xl font-bold text-gray-900">
                {{ software?.name || 'Программа' }}
              </h1>
              <p class="mt-1 text-gray-600">
                {{ software?.manufacturer || 'Производитель не указан' }}
              </p>
            </div>
          </div>
          <NuxtLink v-if="software" to="/licenses" class="btn btn-secondary">
            <Icon name="ri:key-line" class="mr-2" />
            Лицензии
          </NuxtLink>
        </div>
      </div>
    </div>

    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-indigo-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка программы...</p>
    </div>

    <div v-else-if="!software" class="card p-16 text-center">
      <Icon name="ri:error-warning-line" class="text-4xl text-red-400 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Программа не найдена</h3>
      <p class="text-sm text-gray-500 mb-4">Возможно, запись была удалена</p>
      <NuxtLink to="/software" class="btn btn-primary">К каталогу программ</NuxtLink>
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
                <span v-if="software.category" class="badge bg-blue-100 text-blue-800">
                  {{ software.category }}
                </span>
                <span v-else class="text-sm text-gray-400">—</span>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Лицензия</p>
                <span
                  :class="[
                    'badge',
                    software.requires_license
                      ? 'bg-green-100 text-green-800'
                      : 'bg-gray-100 text-gray-800',
                  ]"
                >
                  {{ software.requires_license ? 'Требуется' : 'Не требуется' }}
                </span>
              </div>
              <div>
                <p class="text-xs text-gray-600 mb-1">Системное ПО</p>
                <span
                  :class="[
                    'badge',
                    software.is_system
                      ? 'bg-indigo-100 text-indigo-800'
                      : 'bg-gray-100 text-gray-800',
                  ]"
                >
                  {{ software.is_system ? 'Да' : 'Нет' }}
                </span>
              </div>
              <div v-if="software.website">
                <p class="text-xs text-gray-600 mb-1">Сайт</p>
                <a
                  :href="software.website"
                  target="_blank"
                  rel="noopener noreferrer"
                  class="text-sm text-primary-600 hover:text-primary-700 break-all"
                >
                  {{ software.website }}
                </a>
              </div>
            </div>
            <div v-if="software.description" class="mt-4 pt-4 border-t border-gray-200">
              <p class="text-xs text-gray-600 mb-2">Описание</p>
              <p class="text-sm text-gray-700">
                {{ software.description }}
              </p>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center justify-between">
              <h2 class="text-lg font-semibold text-gray-900">Установлено на компьютерах</h2>
              <span class="badge bg-primary-100 text-primary-800">{{ installations.length }}</span>
            </div>
          </div>
          <div class="p-6">
            <div v-if="installations.length === 0" class="py-8 text-center">
              <Icon name="ri:computer-line" class="text-4xl text-gray-300 mb-3" />
              <p class="text-sm text-gray-500">Программа пока нигде не установлена</p>
            </div>
            <div v-else class="space-y-3">
              <NuxtLink
                v-for="install in installations"
                :key="install.id"
                :to="`/computers/${install.computer_id}`"
                class="flex items-center justify-between p-3 rounded-lg hover:bg-gray-50 transition-colors"
              >
                <div class="flex items-center gap-3">
                  <div
                    class="h-10 w-10 rounded-lg bg-indigo-100 flex items-center justify-center flex-shrink-0"
                  >
                    <Icon name="ri:computer-line" class="text-indigo-600" />
                  </div>
                  <div>
                    <p class="text-sm font-medium text-gray-900">
                      {{ install.computer_name }}
                    </p>
                    <p class="text-xs text-gray-500">
                      <span v-if="install.inventory_number">{{ install.inventory_number }}</span>
                      <span v-if="install.inventory_number && install.employee_name"> · </span>
                      <span v-if="install.employee_name">{{ install.employee_name }}</span>
                      <span v-if="install.version"> · версия {{ install.version }} </span>
                    </p>
                  </div>
                </div>
                <div class="text-right">
                  <span
                    v-if="install.license_type"
                    class="badge bg-green-100 text-green-800 text-xs"
                  >
                    {{ licenseTypeLabel(install.license_type) }}
                  </span>
                  <p v-if="install.license_end_date" class="text-xs text-gray-500 mt-1">
                    до {{ formatDate(install.license_end_date) }}
                  </p>
                </div>
              </NuxtLink>
            </div>
          </div>
        </div>
      </div>

      <div class="space-y-6">
        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h3 class="text-sm font-semibold text-gray-900">Статистика</h3>
          </div>
          <div class="p-6 space-y-4">
            <div>
              <p class="text-xs text-gray-600 mb-1">Всего установок</p>
              <p class="text-2xl font-bold text-gray-900">
                {{ installations.length }}
              </p>
            </div>
            <div>
              <p class="text-xs text-gray-600 mb-1">Лицензий в пулах</p>
              <p class="text-2xl font-bold text-gray-900">{{ stats.totalLicenses }}</p>
            </div>
            <div>
              <p class="text-xs text-gray-600 mb-1">Использовано</p>
              <p class="text-2xl font-bold text-indigo-600">{{ stats.usedLicenses }}</p>
            </div>
            <div>
              <p class="text-xs text-gray-600 mb-1">Истекает в ближайшие 30 дней</p>
              <p class="text-2xl font-bold text-amber-600">{{ stats.expiringSoon }}</p>
            </div>
            <div v-if="stats.expired > 0">
              <p class="text-xs text-gray-600 mb-1">Просрочено</p>
              <p class="text-2xl font-bold text-red-600">{{ stats.expired }}</p>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center justify-between">
              <h3 class="text-sm font-semibold text-gray-900">Лицензионные пулы</h3>
              <NuxtLink to="/licenses" class="text-xs text-primary-600 hover:text-primary-700">
                Все
              </NuxtLink>
            </div>
          </div>
          <div class="p-6">
            <div v-if="licenses.length === 0" class="py-6 text-center">
              <Icon name="ri:key-line" class="text-3xl text-gray-300 mb-2" />
              <p class="text-sm text-gray-500">Лицензии не заведены</p>
            </div>
            <div v-else class="space-y-4">
              <div
                v-for="pool in licenses"
                :key="pool.id"
                class="border-b border-gray-100 pb-4 last:border-0 last:pb-0"
              >
                <div class="flex items-center justify-between mb-1">
                  <p class="text-sm font-medium text-gray-900">
                    {{ licenseTypeLabel(pool.license_type) }}
                  </p>
                  <span :class="['badge text-xs', poolStatus(pool).color]">
                    {{ poolStatus(pool).label }}
                  </span>
                </div>
                <p class="text-xs text-gray-600">
                  {{ pool.used_licenses ?? 0 }} из {{ pool.total_licenses }} занято
                </p>
                <p v-if="pool.expiration_date" class="text-xs text-gray-500 mt-1">
                  до {{ formatDate(pool.expiration_date) }}
                </p>
              </div>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <h3 class="text-sm font-semibold text-gray-900">Запись в каталоге</h3>
          </div>
          <div class="p-6 space-y-3 text-sm">
            <div class="flex justify-between">
              <span class="text-gray-600">Добавлена</span>
              <span class="text-gray-900">{{ formatDate(software.created_at) }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-gray-600">Обновлена</span>
              <span class="text-gray-900">{{ formatDate(software.updated_at) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { LicensePool } from '~/composables/useLicenses'
import type { Software, SoftwareInstallation } from '~/composables/useSoftware'

definePageMeta({
  middleware: 'auth',
})

const route = useRoute()
const toast = useToast()
const { fetchSoftwareById, fetchSoftwareInstallations } = useSoftware()
const { fetchLicenses } = useLicenses()

const softwareId = Number(route.params.id)

const software = ref<Software | null>(null)
const installations = ref<SoftwareInstallation[]>([])
const licenses = ref<LicensePool[]>([])
const loading = ref(true)

const stats = computed(() => {
  const today = new Date()
  const inThirtyDays = new Date(today.getTime() + 30 * 24 * 60 * 60 * 1000)

  let totalLicenses = 0
  let usedLicenses = 0
  let expiringSoon = 0
  let expired = 0

  for (const pool of licenses.value) {
    totalLicenses += pool.total_licenses
    usedLicenses += pool.used_licenses ?? 0

    if (!pool.expiration_date) continue

    const expiration = new Date(pool.expiration_date)
    if (Number.isNaN(expiration.getTime())) continue

    if (expiration < today) expired += 1
    else if (expiration <= inThirtyDays) expiringSoon += 1
  }

  return { totalLicenses, usedLicenses, expiringSoon, expired }
})

useHead({
  title: computed(() => software.value?.name || 'Программа'),
})

const fetchData = async () => {
  loading.value = true
  try {
    const [softwareResponse, installationResponse, licenseResponse] = await Promise.all([
      fetchSoftwareById(softwareId),
      fetchSoftwareInstallations(softwareId),
      fetchLicenses({ software_catalog_id: softwareId, per_page: 100 }),
    ])

    software.value = softwareResponse
    installations.value = installationResponse.installations
    licenses.value = licenseResponse.licenses
  } catch (error) {
    software.value = null
    toast.error('Ошибка загрузки программы: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const licenseTypeLabel = (value: string | null) => {
  if (!value) return '—'

  const labels: Record<string, string> = {
    perpetual: 'Бессрочная',
    subscription: 'Подписка',
    volume: 'Корпоративная',
    oem: 'OEM',
    trial: 'Пробная',
    educational: 'Образовательная',
    free: 'Бесплатная',
    unknown: 'Не указана',
  }

  return labels[value] || value
}

const poolStatus = (pool: LicensePool) => {
  if (!pool.expiration_date) {
    return { label: 'Бессрочная', color: 'bg-green-100 text-green-800' }
  }

  const expiration = new Date(pool.expiration_date)
  if (Number.isNaN(expiration.getTime())) {
    return { label: 'Не указано', color: 'bg-gray-100 text-gray-800' }
  }

  const now = new Date()
  const inThirtyDays = new Date(now.getTime() + 30 * 24 * 60 * 60 * 1000)

  if (expiration < now) return { label: 'Просрочена', color: 'bg-red-100 text-red-800' }
  if (expiration <= inThirtyDays) {
    return { label: 'Истекает', color: 'bg-amber-100 text-amber-800' }
  }

  return { label: 'Активна', color: 'bg-green-100 text-green-800' }
}

const formatDate = (value: string | null) => {
  if (!value) return '—'
  const date = new Date(value)
  return Number.isNaN(date.getTime()) ? value : date.toLocaleDateString('ru-RU')
}

onMounted(fetchData)
</script>
