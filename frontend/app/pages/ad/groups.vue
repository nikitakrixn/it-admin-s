<template>
  <div class="space-y-6">
    <div class="mb-8">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-indigo-500 to-purple-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:group-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Группы Active Directory</h1>
            <p class="mt-1 text-gray-600">Управление группами безопасности и рассылки</p>
          </div>
        </div>
        <div class="flex gap-2">
          <NuxtLink to="/ad" class="btn btn-secondary">
            <Icon name="ri:arrow-left-line" class="mr-2" />
            Назад
          </NuxtLink>
          <button class="btn btn-primary" @click="openCreateModal">
            <Icon name="ri:add-line" class="mr-2" />
            Создать группу
          </button>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">Всего групп</p>
            <p class="text-2xl font-bold text-indigo-600">{{ groups.length }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-indigo-100 flex items-center justify-center">
            <Icon name="ri:group-2-line" class="text-2xl text-indigo-600" />
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">Участников всего</p>
            <p class="text-2xl font-bold text-blue-600">{{ totalMembers }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center">
            <Icon name="ri:team-line" class="text-2xl text-blue-600" />
          </div>
        </div>
      </div>
      <div class="card">
        <div class="p-5 flex items-center justify-between">
          <div>
            <p class="text-sm text-gray-600 mb-1">Групп безопасности</p>
            <p class="text-2xl font-bold text-purple-600">{{ securityCount }}</p>
          </div>
          <div class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center">
            <Icon name="ri:shield-keyhole-line" class="text-2xl text-purple-600" />
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
            placeholder="Поиск по названию группы..."
            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          />
        </div>
        <select v-model="filterType" class="input-field md:w-56">
          <option value="">Все типы</option>
          <option value="Security">Безопасности</option>
          <option value="Distribution">Рассылки</option>
        </select>
      </div>
    </div>

    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-indigo-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка групп...</p>
    </div>

    <div v-else-if="filteredGroups.length === 0" class="card p-16 text-center">
      <Icon name="ri:inbox-line" class="text-4xl text-gray-300 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Группы не найдены</h3>
      <p class="text-sm text-gray-500 mb-4">
        {{ groups.length === 0 ? 'Создайте первую группу AD' : 'Измените параметры поиска' }}
      </p>
      <button v-if="groups.length === 0" class="btn btn-primary" @click="openCreateModal">
        <Icon name="ri:add-line" class="mr-2" />
        Создать группу
      </button>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <GroupCard
        v-for="group in filteredGroups"
        :key="group.id"
        :group="group"
        @edit="openEditModal"
        @delete="deleteGroup"
      />
    </div>

    <Teleport to="body">
      <div
        v-if="showModal"
        class="fixed inset-0 z-50 flex items-center justify-center bg-gray-900/50 p-4"
        @click.self="closeModal"
      >
        <div class="w-full max-w-xl rounded-xl bg-white shadow-2xl">
          <div class="flex items-center justify-between border-b border-gray-200 px-6 py-4">
            <h2 class="text-lg font-semibold text-gray-900">
              {{ isEditMode ? 'Редактировать группу' : 'Новая группа' }}
            </h2>
            <button class="p-2 rounded-lg hover:bg-gray-100" @click="closeModal">
              <Icon name="ri:close-line" class="text-xl" />
            </button>
          </div>

          <form class="p-6 space-y-4" @submit.prevent="handleSubmit">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Название *</label>
              <input v-model="form.name" type="text" class="input-field" required />
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">
                Distinguished Name
              </label>
              <input
                v-model="form.distinguished_name"
                type="text"
                class="input-field font-mono"
                placeholder="CN=IT,OU=Groups,DC=example,DC=local"
              />
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Тип группы</label>
                <select v-model="form.group_type" class="input-field">
                  <option value="">Не указан</option>
                  <option value="Security">Безопасности</option>
                  <option value="Distribution">Рассылки</option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Область действия</label>
                <select v-model="form.group_scope" class="input-field">
                  <option value="">Не указана</option>
                  <option value="Global">Global</option>
                  <option value="DomainLocal">Domain Local</option>
                  <option value="Universal">Universal</option>
                </select>
              </div>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Описание</label>
              <textarea v-model="form.description" rows="3" class="input-field"></textarea>
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
import type { AdGroup, NewAdGroupRequest } from '~/composables/useAd'
import GroupCard from '~/components/ad/GroupCard.vue'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

useHead({
  title: 'Группы AD',
})

const toast = useToast()
const { fetchAdGroups, createAdGroup, updateAdGroup, deleteAdGroup } = useAd()

interface GroupForm {
  name: string
  distinguished_name: string
  description: string
  group_type: string
  group_scope: string
}

const emptyForm = (): GroupForm => ({
  name: '',
  distinguished_name: '',
  description: '',
  group_type: 'Security',
  group_scope: 'Global',
})

const groups = ref<AdGroup[]>([])
const loading = ref(false)
const showModal = ref(false)
const isEditMode = ref(false)
const editingId = ref<number | null>(null)
const saving = ref(false)
const searchQuery = ref('')
const filterType = ref('')
const form = ref<GroupForm>(emptyForm())

const totalMembers = computed(() =>
  groups.value.reduce((sum, group) => sum + (group.member_count ?? 0), 0),
)
const securityCount = computed(
  () => groups.value.filter((group) => group.group_type === 'Security').length,
)

const filteredGroups = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()

  return groups.value.filter((group) => {
    if (filterType.value && group.group_type !== filterType.value) return false
    if (!query) return true

    return [group.name, group.distinguished_name, group.description]
      .filter((value): value is string => Boolean(value))
      .some((value) => value.toLowerCase().includes(query))
  })
})

const fetchGroups = async () => {
  loading.value = true
  try {
    const response = await fetchAdGroups({ per_page: 100 })
    groups.value = response.groups
  } catch (error) {
    toast.error('Ошибка загрузки групп: ' + getErrorMessage(error))
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

const openEditModal = (group: AdGroup) => {
  isEditMode.value = true
  editingId.value = group.id
  form.value = {
    name: group.name,
    distinguished_name: group.distinguished_name ?? '',
    description: group.description ?? '',
    group_type: group.group_type ?? '',
    group_scope: group.group_scope ?? '',
  }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  editingId.value = null
  form.value = emptyForm()
}

const toRequest = (value: GroupForm): NewAdGroupRequest => ({
  name: value.name.trim(),
  distinguished_name: value.distinguished_name.trim() || undefined,
  description: value.description.trim() || undefined,
  group_type: value.group_type || undefined,
  group_scope: value.group_scope || undefined,
})

const handleSubmit = async () => {
  if (!form.value.name.trim()) {
    toast.warning('Укажите название группы')
    return
  }

  saving.value = true
  try {
    const payload = toRequest(form.value)
    if (isEditMode.value && editingId.value !== null) {
      await updateAdGroup(editingId.value, payload)
      toast.success('Группа обновлена')
    } else {
      await createAdGroup(payload)
      toast.success('Группа создана')
    }
    closeModal()
    await fetchGroups()
  } catch (error) {
    toast.error('Ошибка сохранения группы: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

const deleteGroup = async (id: number) => {
  if (!confirm('Удалить группу?')) return
  try {
    await deleteAdGroup(id)
    toast.success('Группа удалена')
    await fetchGroups()
  } catch (error) {
    toast.error('Ошибка удаления группы: ' + getErrorMessage(error))
  }
}

onMounted(fetchGroups)
</script>
