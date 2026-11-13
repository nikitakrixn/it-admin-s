<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-blue-500 to-indigo-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:account-circle-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Управление Active Directory</h1>
            <p class="mt-1 text-gray-600">Создание и управление учетными записями пользователей</p>
          </div>
        </div>
        <div class="flex gap-2">
          <NuxtLink to="/ad/users" class="btn btn-secondary">
            <Icon name="ri:user-search-line" class="mr-2" />
            Список пользователей
          </NuxtLink>
          <NuxtLink to="/ad/groups" class="btn btn-secondary">
            <Icon name="ri:group-line" class="mr-2" />
            Группы
          </NuxtLink>
          <NuxtLink to="/ad/linked-employees" class="btn btn-secondary">
            <Icon name="ri:link" class="mr-2" />
            Привязка
          </NuxtLink>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
      <div class="card">
        <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex items-center">
            <div
              class="h-10 w-10 rounded-full bg-primary-100 flex items-center justify-center mr-3"
            >
              <Icon name="ri:user-add-line" class="text-primary-600" />
            </div>
            <div>
              <h3 class="text-lg font-medium text-gray-900">Создание пользователя AD</h3>
              <p class="text-sm text-gray-500">Заполните форму для создания нового пользователя</p>
            </div>
          </div>
        </div>

        <form class="p-6 space-y-6" @submit.prevent="handleSubmit">
          <div>
            <h4 class="text-sm font-medium text-gray-700 mb-4 pb-2 border-b">Учетные данные</h4>
            <div class="space-y-4">
              <div>
                <label class="label">Имя пользователя *</label>
                <div class="relative">
                  <Icon
                    name="ri:user-3-line"
                    class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                  />
                  <input
                    v-model="form.username"
                    type="text"
                    required
                    class="input-field pl-10"
                    placeholder="ivanov"
                  />
                </div>
              </div>

              <div>
                <label class="label">Отображаемое имя</label>
                <input
                  v-model="form.display_name"
                  type="text"
                  class="input-field"
                  placeholder="Иванов Иван"
                />
              </div>

              <div>
                <label class="label">Email</label>
                <div class="relative">
                  <Icon
                    name="ri:mail-line"
                    class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                  />
                  <input
                    v-model="form.email"
                    type="email"
                    class="input-field pl-10"
                    placeholder="ivanov@company.com"
                  />
                </div>
              </div>

              <div>
                <label class="label">Тип учётной записи *</label>
                <select v-model="form.account_type" required class="input-field">
                  <option value="user">Пользователь</option>
                  <option value="admin">Администратор</option>
                  <option value="service">Сервисная</option>
                  <option value="computer">Компьютер</option>
                  <option value="guest">Гостевая</option>
                </select>
              </div>
            </div>
          </div>

          <div>
            <h4 class="text-sm font-medium text-gray-700 mb-4 pb-2 border-b">Организация</h4>
            <div>
              <label class="label">Сотрудник</label>
              <select v-model="form.employee_id" class="input-field">
                <option :value="null">Не привязан</option>
                <option v-for="employee in employees" :key="employee.id" :value="employee.id">
                  {{ employee.full_name }}
                </option>
              </select>
              <p class="mt-1 text-xs text-gray-500">
                Учётную запись можно привязать к сотруднику позже на странице «Привязка»
              </p>
            </div>
          </div>

          <div class="flex justify-end">
            <button type="submit" :disabled="saving" class="btn btn-primary disabled:opacity-50">
              <Icon v-if="saving" name="ri:loader-4-line" class="mr-2 animate-spin" />
              <Icon v-else name="ri:user-add-line" class="mr-2" />
              {{ saving ? 'Создание...' : 'Создать пользователя' }}
            </button>
          </div>
        </form>
      </div>

      <div class="space-y-6">
        <div class="card">
          <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center">
              <div
                class="h-10 w-10 rounded-full bg-indigo-100 flex items-center justify-center mr-3"
              >
                <Icon name="ri:server-line" class="text-indigo-600" />
              </div>
              <div>
                <h3 class="text-lg font-medium text-gray-900">Интеграция с AD</h3>
                <p class="text-sm text-gray-500">Как устроена работа с доменом</p>
              </div>
            </div>
          </div>

          <div class="p-6 space-y-4 text-sm text-gray-600">
            <p>
              Учётные записи и группы хранятся в локальной базе данных. Синхронизация с контроллером
              домена выполняется отдельно и не входит в эту панель.
            </p>
            <div class="flex items-center gap-2">
              <span class="h-2 w-2 rounded-full bg-amber-400"></span>
              <span>Данные вводятся вручную или импортируются</span>
            </div>
          </div>
        </div>

        <div class="card">
          <div class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex items-center">
              <div
                class="h-10 w-10 rounded-full bg-green-100 flex items-center justify-center mr-3"
              >
                <Icon name="ri:pie-chart-line" class="text-green-600" />
              </div>
              <div>
                <h3 class="text-lg font-medium text-gray-900">Статистика Active Directory</h3>
                <p class="text-sm text-gray-500">Данные из локальной базы</p>
              </div>
            </div>
          </div>

          <div class="p-6">
            <div class="grid grid-cols-2 gap-4">
              <NuxtLink to="/ad/users" class="bg-gray-50 rounded-lg p-4 hover:bg-gray-100">
                <div class="flex items-center">
                  <div class="rounded-md bg-primary-500 p-3">
                    <Icon name="ri:user-line" class="text-white text-xl" />
                  </div>
                  <div class="ml-4">
                    <div class="text-sm font-medium text-gray-500">Пользователей</div>
                    <div class="text-2xl font-bold text-gray-900">{{ stats.accounts }}</div>
                  </div>
                </div>
              </NuxtLink>

              <NuxtLink to="/ad/groups" class="bg-gray-50 rounded-lg p-4 hover:bg-gray-100">
                <div class="flex items-center">
                  <div class="rounded-md bg-indigo-500 p-3">
                    <Icon name="ri:group-line" class="text-white text-xl" />
                  </div>
                  <div class="ml-4">
                    <div class="text-sm font-medium text-gray-500">Групп</div>
                    <div class="text-2xl font-bold text-gray-900">{{ stats.groups }}</div>
                  </div>
                </div>
              </NuxtLink>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Employee } from '~/composables/useEmployees'

definePageMeta({
  middleware: 'auth',
})

useHead({
  title: 'Active Directory',
})

const toast = useToast()
const { createDomainAccount, fetchDomainAccounts, fetchAdGroups } = useAd()
const { fetchEmployees } = useEmployees()

const saving = ref(false)
const employees = ref<Employee[]>([])
const stats = ref({ accounts: 0, groups: 0 })

const emptyForm = () => ({
  username: '',
  display_name: '',
  email: '',
  account_type: 'user',
  employee_id: null as number | null,
})

const form = ref(emptyForm())

const fetchData = async () => {
  try {
    const [accountList, groupList, employeeList] = await Promise.all([
      fetchDomainAccounts({ per_page: 1 }),
      fetchAdGroups({ per_page: 1 }),
      fetchEmployees({ per_page: 100 }),
    ])
    stats.value = { accounts: accountList.total, groups: groupList.total }
    employees.value = employeeList.employees
  } catch (error) {
    toast.error('Не удалось загрузить данные AD: ' + getErrorMessage(error))
  }
}

const handleSubmit = async () => {
  if (!form.value.username.trim()) {
    toast.warning('Укажите имя пользователя')
    return
  }

  saving.value = true
  try {
    await createDomainAccount({
      username: form.value.username.trim(),
      display_name: form.value.display_name.trim() || undefined,
      email: form.value.email.trim() || undefined,
      account_type: form.value.account_type,
      employee_id: form.value.employee_id ?? undefined,
    })
    toast.success('Учётная запись создана')
    form.value = emptyForm()
    await fetchData()
  } catch (error) {
    toast.error('Не удалось создать учётную запись: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

onMounted(fetchData)
</script>
