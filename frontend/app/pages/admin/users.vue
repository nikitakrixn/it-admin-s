<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between gap-4">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-slate-600 to-slate-800 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:user-settings-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Пользователи</h1>
            <p class="mt-1 text-gray-600">Доступ к панели администрирования</p>
          </div>
        </div>

        <button class="btn btn-primary" @click="openCreateModal">
          <Icon name="ri:user-add-line" class="mr-2" />
          Добавить пользователя
        </button>
      </div>
    </div>

    <div class="card">
      <div v-if="loading" class="p-8 text-center text-gray-500">
        <Icon name="ri:loader-4-line" class="text-2xl animate-spin" />
      </div>

      <div v-else-if="users.length === 0" class="p-8 text-center text-gray-500">
        Пользователей пока нет
      </div>

      <div v-else class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Email</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">Роль</th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Статус
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Последний вход
              </th>
              <th class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase">
                Создан
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr v-for="item in users" :key="item.id" class="hover:bg-gray-50">
              <td class="px-4 py-3">
                <div class="text-sm font-medium text-gray-900">{{ item.email }}</div>
                <div v-if="item.id === currentUserId" class="text-xs text-gray-500">Это вы</div>
              </td>
              <td class="px-4 py-3">
                <select
                  :value="item.role"
                  :disabled="item.id === currentUserId || savingId === item.id"
                  class="input-field w-36"
                  @change="changeRole(item, ($event.target as HTMLSelectElement).value)"
                >
                  <option value="admin">Администратор</option>
                  <option value="user">Пользователь</option>
                </select>
              </td>
              <td class="px-4 py-3">
                <button
                  type="button"
                  :disabled="item.id === currentUserId || savingId === item.id"
                  class="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-xs font-medium transition-colors disabled:opacity-50"
                  :class="
                    item.is_active
                      ? 'bg-green-100 text-green-800 hover:bg-green-200'
                      : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                  "
                  @click="toggleActive(item)"
                >
                  <Icon :name="item.is_active ? 'ri:checkbox-circle-line' : 'ri:prohibited-line'" />
                  {{ item.is_active ? 'Активен' : 'Отключён' }}
                </button>
              </td>
              <td class="px-4 py-3 text-sm text-gray-600">
                {{ formatDate(item.last_login_at) }}
              </td>
              <td class="px-4 py-3 text-sm text-gray-600">
                {{ formatDate(item.created_at) }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <UiModal v-model="showCreateModal" title="Новый пользователь">
      <form id="create-user-form" class="space-y-4" @submit.prevent="createUser">
        <div>
          <label for="new-email" class="block text-sm font-medium text-gray-700 mb-1">Email</label>
          <input
            id="new-email"
            v-model="form.email"
            type="email"
            required
            class="input-field"
            placeholder="user@company.local"
          />
        </div>

        <div>
          <label for="new-password" class="block text-sm font-medium text-gray-700 mb-1">
            Пароль
          </label>
          <input
            id="new-password"
            v-model="form.password"
            type="password"
            required
            minlength="12"
            class="input-field"
            placeholder="Минимум 12 символов"
          />
          <p class="mt-1 text-xs text-gray-500">
            Не короче 12 символов, обязательно буква и цифра.
          </p>
        </div>

        <div>
          <label for="new-role" class="block text-sm font-medium text-gray-700 mb-1">Роль</label>
          <select id="new-role" v-model="form.role" class="input-field">
            <option value="user">Пользователь</option>
            <option value="admin">Администратор</option>
          </select>
        </div>
      </form>

      <template #footer>
        <button class="btn btn-secondary" type="button" @click="showCreateModal = false">
          Отмена
        </button>
        <button class="btn btn-primary" type="submit" form="create-user-form" :disabled="creating">
          <Icon v-if="creating" name="ri:loader-4-line" class="mr-2 animate-spin" />
          <Icon v-else name="ri:user-add-line" class="mr-2" />
          {{ creating ? 'Создание...' : 'Создать' }}
        </button>
      </template>
    </UiModal>
  </div>
</template>

<script setup lang="ts">
import type { User } from '~/composables/useAuth'
import type { UserRole } from '~/composables/useUsers'

definePageMeta({ middleware: ['auth', 'admin'] })
useHead({ title: 'Пользователи' })

const toast = useToast()
const { user: currentUser } = useAuth()
const { fetchUsers, createUser: createUserRequest, updateUser } = useUsers()

const users = ref<User[]>([])
const loading = ref(false)
const creating = ref(false)
const savingId = ref<string | null>(null)
const showCreateModal = ref(false)

const form = ref({
  email: '',
  password: '',
  role: 'user' as UserRole,
})

const currentUserId = computed(() => currentUser.value?.id)

const formatDate = (value: string | null) => {
  if (!value) return '—'
  const date = new Date(value)
  if (Number.isNaN(date.getTime())) return value

  return date.toLocaleString('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

const loadUsers = async () => {
  loading.value = true
  try {
    users.value = await fetchUsers()
  } catch (error) {
    toast.error('Не удалось загрузить пользователей: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const openCreateModal = () => {
  form.value = { email: '', password: '', role: 'user' }
  showCreateModal.value = true
}

const createUser = async () => {
  creating.value = true
  try {
    const created = await createUserRequest(form.value)
    users.value = [created, ...users.value]
    showCreateModal.value = false
    toast.success('Пользователь создан')
  } catch (error) {
    toast.error(getErrorMessage(error, 'Не удалось создать пользователя'))
  } finally {
    creating.value = false
  }
}

const changeRole = async (item: User, role: string) => {
  savingId.value = item.id
  try {
    const updated = await updateUser(item.id, { role: role as UserRole })
    replaceUser(updated)
    toast.success('Роль обновлена')
  } catch (error) {
    toast.error(getErrorMessage(error, 'Не удалось изменить роль'))
    await loadUsers()
  } finally {
    savingId.value = null
  }
}

const toggleActive = async (item: User) => {
  savingId.value = item.id
  try {
    const updated = await updateUser(item.id, { is_active: !item.is_active })
    replaceUser(updated)
    toast.success(updated.is_active ? 'Пользователь включён' : 'Пользователь отключён')
  } catch (error) {
    toast.error(getErrorMessage(error, 'Не удалось изменить статус'))
    await loadUsers()
  } finally {
    savingId.value = null
  }
}

const replaceUser = (updated: User) => {
  users.value = users.value.map((item) => (item.id === updated.id ? updated : item))
}

onMounted(loadUsers)
</script>
