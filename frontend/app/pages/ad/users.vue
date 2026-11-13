<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-blue-500 to-indigo-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:user-search-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Учётные записи AD</h1>
            <p class="mt-1 text-gray-600">Пользователи, сервисные и административные аккаунты</p>
          </div>
        </div>
        <div class="flex gap-2">
          <NuxtLink to="/ad" class="btn btn-secondary">
            <Icon name="ri:arrow-left-line" class="mr-2" />
            Назад
          </NuxtLink>
          <button class="btn btn-primary" @click="openCreateModal">
            <Icon name="ri:user-add-line" class="mr-2" />
            Создать учётную запись
          </button>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">Всего записей</p>
            <p class="text-2xl font-bold text-blue-600">{{ accounts.length }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center">
            <Icon name="ri:user-settings-line" class="text-2xl text-blue-600" />
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">Активные</p>
            <p class="text-2xl font-bold text-green-600">{{ activeCount }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center">
            <Icon name="ri:user-follow-line" class="text-2xl text-green-600" />
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">Заблокированы</p>
            <p class="text-2xl font-bold text-red-600">{{ lockedCount }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-red-100 flex items-center justify-center">
            <Icon name="ri:lock-line" class="text-2xl text-red-600" />
          </div>
        </div>
      </div>
    </div>

    <div class="card p-4">
      <div class="flex flex-col gap-4 md:flex-row">
        <div class="flex-1 relative">
          <Icon
            name="ri:search-line"
            class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
          />
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Поиск по логину, имени или email..."
            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          />
        </div>
        <select v-model="filterStatus" class="input-field md:w-56">
          <option value="">Все статусы</option>
          <option value="active">Активные</option>
          <option value="disabled">Отключённые</option>
          <option value="locked">Заблокированные</option>
        </select>
      </div>
    </div>

    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-blue-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка учётных записей...</p>
    </div>

    <div v-else-if="filteredAccounts.length === 0" class="card p-16 text-center">
      <Icon name="ri:inbox-line" class="text-4xl text-gray-300 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Учётные записи не найдены</h3>
      <p class="text-sm text-gray-500 mb-4">
        {{ accounts.length === 0 ? 'Создайте первую учётную запись' : 'Измените параметры поиска' }}
      </p>
      <button v-if="accounts.length === 0" class="btn btn-primary" @click="openCreateModal">
        <Icon name="ri:user-add-line" class="mr-2" />
        Создать учётную запись
      </button>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <AccountCard
        v-for="account in filteredAccounts"
        :key="account.id"
        :account="account"
        @edit="openEditModal"
        @delete="deleteAccount"
      />
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
              {{ isEditMode ? 'Редактировать учётную запись' : 'Новая учётная запись' }}
            </h2>
            <button class="p-2 rounded-lg hover:bg-gray-100" @click="closeModal">
              <Icon name="ri:close-line" class="text-xl" />
            </button>
          </div>

          <form class="p-6 space-y-4" @submit.prevent="handleSubmit">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Логин *</label>
                <input v-model="form.username" type="text" class="input-field" required />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Тип записи *</label>
                <select v-model="form.account_type" class="input-field" required>
                  <option value="user">Пользователь</option>
                  <option value="service">Сервисная</option>
                  <option value="admin">Администратор</option>
                  <option value="computer">Компьютер</option>
                  <option value="guest">Гостевая</option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Отображаемое имя</label>
                <input v-model="form.display_name" type="text" class="input-field" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Email</label>
                <input v-model="form.email" type="email" class="input-field" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">UPN</label>
                <input
                  v-model="form.user_principal_name"
                  type="text"
                  class="input-field font-mono"
                  placeholder="user@example.local"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">SAM-имя</label>
                <input v-model="form.sam_account_name" type="text" class="input-field font-mono" />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">ID сотрудника</label>
                <input
                  v-model.number="form.employee_id"
                  type="number"
                  min="1"
                  class="input-field"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">
                  Distinguished Name
                </label>
                <input
                  v-model="form.distinguished_name"
                  type="text"
                  class="input-field font-mono"
                />
              </div>
            </div>

            <div class="flex flex-wrap gap-6 rounded-lg border border-gray-200 p-4">
              <label class="flex items-center gap-2 text-sm text-gray-700">
                <input v-model="form.is_enabled" type="checkbox" class="rounded" />
                Включена
              </label>
              <label class="flex items-center gap-2 text-sm text-gray-700">
                <input v-model="form.is_locked" type="checkbox" class="rounded" />
                Заблокирована
              </label>
              <label class="flex items-center gap-2 text-sm text-gray-700">
                <input v-model="form.must_change_password" type="checkbox" class="rounded" />
                Сменить пароль при входе
              </label>
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
import { computed, onMounted, ref } from 'vue'
import { useAd } from '~/composables/useAd'
import type { DomainAccount, NewDomainAccountRequest } from '~/composables/useAd'
import AccountCard from '~/components/ad/AccountCard.vue'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

useHead({
  title: 'Пользователи AD',
})

const toast = useToast()
const { fetchDomainAccounts, createDomainAccount, updateDomainAccount, deleteDomainAccount } =
  useAd()

interface AccountForm {
  username: string
  display_name: string
  email: string
  account_type: string
  employee_id: number | null
  distinguished_name: string
  user_principal_name: string
  sam_account_name: string
  is_enabled: boolean
  is_locked: boolean
  must_change_password: boolean
}

const emptyForm = (): AccountForm => ({
  username: '',
  display_name: '',
  email: '',
  account_type: 'user',
  employee_id: null,
  distinguished_name: '',
  user_principal_name: '',
  sam_account_name: '',
  is_enabled: true,
  is_locked: false,
  must_change_password: false,
})

const accounts = ref<DomainAccount[]>([])
const loading = ref(false)
const showModal = ref(false)
const isEditMode = ref(false)
const editingId = ref<number | null>(null)
const saving = ref(false)
const searchQuery = ref('')
const filterStatus = ref('')
const form = ref<AccountForm>(emptyForm())

const activeCount = computed(() => accounts.value.filter((item) => item.is_enabled).length)
const lockedCount = computed(() => accounts.value.filter((item) => item.is_locked).length)

const filteredAccounts = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()

  return accounts.value.filter((account) => {
    if (filterStatus.value === 'active' && !account.is_enabled) return false
    if (filterStatus.value === 'disabled' && account.is_enabled) return false
    if (filterStatus.value === 'locked' && !account.is_locked) return false
    if (!query) return true

    return [account.username, account.display_name, account.email]
      .filter((value): value is string => Boolean(value))
      .some((value) => value.toLowerCase().includes(query))
  })
})

const fetchAccounts = async () => {
  loading.value = true
  try {
    const response = await fetchDomainAccounts({ per_page: 100 })
    accounts.value = response.accounts
  } catch (error) {
    toast.error('Ошибка загрузки учётных записей: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const openCreateModal = () => {
  isEditMode.value = false
  editingId.value = null
  form.value = emptyForm()
  showModal.value = true
}

const openEditModal = (account: DomainAccount) => {
  isEditMode.value = true
  editingId.value = account.id
  form.value = {
    username: account.username,
    display_name: account.display_name ?? '',
    email: account.email ?? '',
    account_type: account.account_type,
    employee_id: account.employee_id,
    distinguished_name: account.distinguished_name ?? '',
    user_principal_name: account.user_principal_name ?? '',
    sam_account_name: account.sam_account_name ?? '',
    is_enabled: account.is_enabled ?? true,
    is_locked: account.is_locked ?? false,
    must_change_password: account.must_change_password ?? false,
  }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  editingId.value = null
  form.value = emptyForm()
}

const toRequest = (value: AccountForm): NewDomainAccountRequest => ({
  username: value.username.trim(),
  display_name: value.display_name.trim() || undefined,
  email: value.email.trim() || undefined,
  account_type: value.account_type,
  employee_id: toOptionalNumber(value.employee_id),
  distinguished_name: value.distinguished_name.trim() || undefined,
  user_principal_name: value.user_principal_name.trim() || undefined,
  sam_account_name: value.sam_account_name.trim() || undefined,
  is_enabled: value.is_enabled,
  is_locked: value.is_locked,
  must_change_password: value.must_change_password,
})

const handleSubmit = async () => {
  if (!form.value.username.trim()) {
    toast.warning('Укажите логин учётной записи')
    return
  }

  saving.value = true
  try {
    const payload = toRequest(form.value)
    if (isEditMode.value && editingId.value !== null) {
      await updateDomainAccount(editingId.value, payload)
      toast.success('Учётная запись обновлена')
    } else {
      await createDomainAccount(payload)
      toast.success('Учётная запись создана')
    }
    closeModal()
    await fetchAccounts()
  } catch (error) {
    toast.error('Ошибка сохранения учётной записи: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

const deleteAccount = async (id: number) => {
  if (!confirm('Удалить учётную запись?')) return
  try {
    await deleteDomainAccount(id)
    toast.success('Учётная запись удалена')
    await fetchAccounts()
  } catch (error) {
    toast.error('Ошибка удаления учётной записи: ' + getErrorMessage(error))
  }
}

onMounted(fetchAccounts)
</script>
