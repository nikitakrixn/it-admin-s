<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-green-500 to-teal-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:link" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Привязка сотрудников к AD</h1>
            <p class="mt-1 text-gray-600">Связь между сотрудниками системы и пользователями AD</p>
          </div>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="card p-6">
        <div class="flex items-center">
          <div class="rounded-lg bg-green-100 p-3">
            <Icon name="ri:checkbox-circle-line" class="text-2xl text-green-600" />
          </div>
          <div class="ml-4">
            <p class="text-sm text-gray-600">Привязано</p>
            <p class="text-2xl font-bold text-gray-900">{{ stats.linked }}</p>
          </div>
        </div>
      </div>

      <div class="card p-6">
        <div class="flex items-center">
          <div class="rounded-lg bg-yellow-100 p-3">
            <Icon name="ri:error-warning-line" class="text-2xl text-yellow-600" />
          </div>
          <div class="ml-4">
            <p class="text-sm text-gray-600">Без привязки</p>
            <p class="text-2xl font-bold text-gray-900">{{ stats.unlinked }}</p>
          </div>
        </div>
      </div>

      <div class="card p-6">
        <div class="flex items-center">
          <div class="rounded-lg bg-blue-100 p-3">
            <Icon name="ri:user-line" class="text-2xl text-blue-600" />
          </div>
          <div class="ml-4">
            <p class="text-sm text-gray-600">Всего сотрудников</p>
            <p class="text-2xl font-bold text-gray-900">{{ stats.total }}</p>
          </div>
        </div>
      </div>
    </div>

    <div class="card p-4">
      <div class="flex gap-4">
        <div class="flex-1 relative">
          <Icon
            name="ri:search-line"
            class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
          />
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Поиск сотрудника..."
            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          />
        </div>
        <select v-model="filterStatus" class="input-field w-48">
          <option value="">Все статусы</option>
          <option value="linked">Привязаны</option>
          <option value="unlinked">Не привязаны</option>
        </select>
      </div>
    </div>

    <div class="card">
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Сотрудник
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Отдел / Должность
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Пользователь AD
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Статус привязки
              </th>
              <th class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase">
                Действия
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-if="loading">
              <td colspan="5" class="px-6 py-12 text-center">
                <Icon name="ri:loader-4-line" class="text-3xl text-green-500 animate-spin mb-2" />
                <p class="text-sm text-gray-500">Загрузка сотрудников...</p>
              </td>
            </tr>
            <tr v-else-if="filteredEmployees.length === 0">
              <td colspan="5" class="px-6 py-16 text-center">
                <div class="flex flex-col items-center">
                  <div
                    class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mb-4"
                  >
                    <Icon name="ri:link" class="text-4xl text-gray-400" />
                  </div>
                  <h3 class="text-lg font-semibold text-gray-900 mb-2">Нет данных</h3>
                  <p class="text-sm text-gray-500">
                    {{
                      employees.length === 0 ? 'Сотрудников пока нет' : 'Измените параметры поиска'
                    }}
                  </p>
                </div>
              </td>
            </tr>
            <template v-else>
              <tr v-for="employee in filteredEmployees" :key="employee.id">
                <td class="px-4 py-4">
                  <div class="flex items-center gap-3">
                    <div class="h-9 w-9 rounded-full bg-green-100 flex items-center justify-center">
                      <Icon name="ri:user-line" class="text-green-700" />
                    </div>
                    <div>
                      <NuxtLink
                        :to="`/employees/${employee.id}`"
                        class="text-sm font-medium text-gray-900 hover:text-primary-600"
                      >
                        {{ employee.full_name }}
                      </NuxtLink>
                      <p class="text-xs text-gray-500">{{ employee.email || '—' }}</p>
                    </div>
                  </div>
                </td>
                <td class="px-4 py-4">
                  <p class="text-sm text-gray-900">{{ employee.department_name || '—' }}</p>
                  <p class="text-xs text-gray-500">{{ employee.position_name || '—' }}</p>
                </td>
                <td class="px-4 py-4">
                  <p v-if="accountFor(employee)" class="text-sm font-mono text-gray-900">
                    {{ accountFor(employee)?.username }}
                  </p>
                  <p v-else-if="employee.ad_username" class="text-sm font-mono text-gray-500">
                    {{ employee.ad_username }}
                  </p>
                  <p v-else class="text-sm text-gray-400">—</p>
                </td>
                <td class="px-4 py-4">
                  <span
                    :class="[
                      'badge',
                      isLinked(employee)
                        ? 'bg-green-100 text-green-800'
                        : 'bg-amber-100 text-amber-800',
                    ]"
                  >
                    {{ isLinked(employee) ? 'Привязан' : 'Без привязки' }}
                  </span>
                </td>
                <td class="px-4 py-4 text-right">
                  <button class="btn btn-secondary btn-sm" @click="openLinkModal(employee)">
                    <Icon name="ri:link" class="mr-1" />
                    {{ isLinked(employee) ? 'Изменить' : 'Привязать' }}
                  </button>
                </td>
              </tr>
            </template>
          </tbody>
        </table>
      </div>
    </div>

    <Teleport to="body">
      <Transition name="modal">
        <div v-if="linkModalOpen" class="fixed inset-0 z-[60] overflow-y-auto">
          <div
            class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm"
            @click="linkModalOpen = false"
          ></div>
          <div class="relative min-h-screen flex items-center justify-center p-4">
            <div class="relative bg-white rounded-2xl shadow-2xl max-w-md w-full p-6">
              <h3 class="text-lg font-semibold text-gray-900 mb-1">Привязка к AD</h3>
              <p class="text-sm text-gray-600 mb-4">Сотрудник: {{ selectedEmployee?.full_name }}</p>

              <label class="block text-sm font-medium text-gray-700 mb-2">Учётная запись</label>
              <select v-model="selectedAccountId" class="input-field">
                <option :value="null" disabled>Выберите учётную запись</option>
                <option v-for="account in accounts" :key="account.id" :value="account.id">
                  {{ account.username }}
                  <template v-if="account.display_name"> — {{ account.display_name }}</template>
                </option>
              </select>

              <div class="flex gap-3 mt-6">
                <button class="flex-1 btn btn-secondary" @click="linkModalOpen = false">
                  Отмена
                </button>
                <button
                  class="flex-1 btn btn-primary disabled:opacity-50"
                  :disabled="saving || selectedAccountId === null"
                  @click="linkAccount"
                >
                  {{ saving ? 'Сохранение...' : 'Привязать' }}
                </button>
              </div>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import type { DomainAccount } from '~/composables/useAd'
import type { Employee } from '~/composables/useEmployees'

definePageMeta({
  middleware: 'auth',
})

useHead({
  title: 'Привязка сотрудников',
})

const toast = useToast()
const { fetchDomainAccounts, updateDomainAccount } = useAd()
const { fetchEmployees } = useEmployees()

const searchQuery = ref('')
const filterStatus = ref('')
const employees = ref<Employee[]>([])
const accounts = ref<DomainAccount[]>([])
const loading = ref(false)
const saving = ref(false)
const linkModalOpen = ref(false)
const selectedEmployee = ref<Employee | null>(null)
const selectedAccountId = ref<number | null>(null)

const fetchData = async () => {
  loading.value = true
  try {
    const [employeeList, accountList] = await Promise.all([
      fetchEmployees({ per_page: 100 }),
      fetchDomainAccounts({ per_page: 100 }),
    ])
    employees.value = employeeList.employees
    accounts.value = accountList.accounts
  } catch (error) {
    toast.error('Не удалось загрузить данные: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const accountFor = (employee: Employee) => {
  return (
    accounts.value.find((account) => account.employee_id === employee.id) ??
    accounts.value.find(
      (account) => employee.ad_username && account.username === employee.ad_username,
    )
  )
}

const isLinked = (employee: Employee) => Boolean(accountFor(employee))

const stats = computed(() => {
  const linked = employees.value.filter((employee) => isLinked(employee)).length
  return {
    linked,
    unlinked: employees.value.length - linked,
    total: employees.value.length,
  }
})

const filteredEmployees = computed(() => {
  let filtered = employees.value

  if (filterStatus.value === 'linked') {
    filtered = filtered.filter((employee) => isLinked(employee))
  } else if (filterStatus.value === 'unlinked') {
    filtered = filtered.filter((employee) => !isLinked(employee))
  }

  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter((employee) =>
      [
        employee.full_name,
        employee.email,
        employee.ad_username,
        employee.department_name,
        employee.position_name,
      ]
        .filter((value): value is string => Boolean(value))
        .some((value) => value.toLowerCase().includes(query)),
    )
  }

  return filtered
})

const openLinkModal = (employee: Employee) => {
  selectedEmployee.value = employee
  selectedAccountId.value = accountFor(employee)?.id ?? null
  linkModalOpen.value = true
}

const linkAccount = async () => {
  if (!selectedEmployee.value || selectedAccountId.value === null) return

  saving.value = true
  try {
    await updateDomainAccount(selectedAccountId.value, { employee_id: selectedEmployee.value.id })
    toast.success('Учётная запись привязана')
    linkModalOpen.value = false
    await fetchData()
  } catch (error) {
    toast.error('Не удалось привязать учётную запись: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

onMounted(fetchData)
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
