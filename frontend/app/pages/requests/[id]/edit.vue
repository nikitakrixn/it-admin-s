<template>
  <div class="max-w-4xl mx-auto space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4 mb-6">
        <NuxtLink
          :to="`/requests/${requestId}`"
          class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
        >
          <Icon name="ri:arrow-left-line" class="text-xl" />
        </NuxtLink>
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-amber-500 to-amber-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:edit-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Редактирование заявки #{{ requestId }}</h1>
            <p class="mt-1 text-gray-600">Внесите необходимые изменения</p>
          </div>
        </div>
      </div>
    </div>

    <div v-if="loading" class="card p-16 text-center">
      <Icon name="ri:loader-4-line" class="text-4xl text-amber-500 animate-spin mb-4" />
      <p class="text-sm text-gray-500">Загрузка заявки...</p>
    </div>

    <div v-else-if="!loaded" class="card p-16 text-center">
      <Icon name="ri:error-warning-line" class="text-4xl text-gray-300 mb-4" />
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Заявка не найдена</h3>
      <NuxtLink to="/requests" class="btn btn-primary mt-4">
        <Icon name="ri:arrow-left-line" class="mr-2" />
        К списку заявок
      </NuxtLink>
    </div>

    <form v-else class="space-y-6" @submit.prevent="handleSubmit">
      <div class="card">
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <h2 class="text-lg font-semibold text-gray-900">Основная информация</h2>
        </div>

        <div class="p-6 space-y-6">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">
                Статус <span class="text-red-500">*</span>
              </label>
              <select v-model="form.status" required class="input-field">
                <option value="new">Новая</option>
                <option value="in_progress">В работе</option>
                <option value="waiting">Ожидание</option>
                <option value="resolved">Решена</option>
                <option value="closed">Закрыта</option>
                <option value="cancelled">Отменена</option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">
                Приоритет <span class="text-red-500">*</span>
              </label>
              <select v-model="form.priority" required class="input-field">
                <option value="low">Низкий</option>
                <option value="medium">Средний</option>
                <option value="high">Высокий</option>
                <option value="critical">Критический</option>
              </select>
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              Заголовок <span class="text-red-500">*</span>
            </label>
            <input
              v-model="form.title"
              type="text"
              required
              placeholder="Краткое описание проблемы"
              class="input-field"
            />
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">
              Описание <span class="text-red-500">*</span>
            </label>
            <textarea
              v-model="form.description"
              required
              rows="5"
              placeholder="Подробное описание проблемы или запроса"
              class="input-field resize-none"
            ></textarea>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2"> Категория </label>
              <select v-model="form.category" class="input-field">
                <option value="">Без категории</option>
                <option value="hardware">Оборудование</option>
                <option value="software">Программное обеспечение</option>
                <option value="network">Сеть</option>
                <option value="access">Доступ</option>
                <option value="consultation">Консультация</option>
                <option value="other">Другое</option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2"> Оценка, мин </label>
              <input
                v-model="form.estimated_time"
                type="number"
                min="0"
                placeholder="Например, 60"
                class="input-field"
              />
            </div>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <h2 class="text-lg font-semibold text-gray-900">Участники и ресурсы</h2>
        </div>

        <div class="p-6 space-y-6">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">
                Заявитель <span class="text-red-500">*</span>
              </label>
              <select v-model="form.requester_id" required class="input-field">
                <option :value="null" disabled>Выберите сотрудника</option>
                <option v-for="employee in employees" :key="employee.id" :value="employee.id">
                  {{ employee.full_name }}
                </option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2"> Исполнитель </label>
              <select v-model="form.assignee_id" class="input-field">
                <option :value="null">Не назначен</option>
                <option v-for="employee in employees" :key="employee.id" :value="employee.id">
                  {{ employee.full_name }}
                </option>
              </select>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2"> Компьютер </label>
              <select v-model="form.computer_id" class="input-field">
                <option :value="null">Не указан</option>
                <option v-for="item in computers" :key="item.id" :value="item.id">
                  {{ item.hostname || `Компьютер #${item.id}` }}
                </option>
              </select>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2"> Оборудование </label>
              <select v-model="form.equipment_id" class="input-field">
                <option :value="null">Не указано</option>
                <option v-for="item in equipment" :key="item.id" :value="item.id">
                  {{ item.name }}
                </option>
              </select>
            </div>
          </div>
        </div>
      </div>

      <div class="flex items-center justify-between gap-4">
        <NuxtLink :to="`/requests/${requestId}`" class="btn btn-secondary">
          <Icon name="ri:close-line" class="mr-2" />
          Отмена
        </NuxtLink>

        <button type="submit" class="btn btn-primary disabled:opacity-50" :disabled="saving">
          <Icon name="ri:save-line" class="mr-2" />
          {{ saving ? 'Сохранение...' : 'Сохранить изменения' }}
        </button>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import type { Computer } from '~/composables/useComputers'
import type { Employee } from '~/composables/useEmployees'
import type { Equipment } from '~/composables/useEquipment'
import { toOptionalNumber } from '~/utils/numbers'

definePageMeta({
  middleware: 'auth',
  pageTransition: {
    name: 'slide-up',
    mode: 'out-in',
  },
})

const route = useRoute()
const router = useRouter()
const toast = useToast()

const { fetchRequest, updateRequest } = useRequests()
const { fetchEmployees } = useEmployees()
const { fetchComputers } = useComputers()
const { fetchEquipmentList } = useEquipment()

const requestId = Number(route.params.id)
const loaded = ref(false)
const loading = ref(true)
const saving = ref(false)
const employees = ref<Employee[]>([])
const computers = ref<Computer[]>([])
const equipment = ref<Equipment[]>([])

const form = ref({
  title: '',
  description: '',
  status: 'new',
  priority: 'medium',
  category: '',
  requester_id: null as number | null,
  assignee_id: null as number | null,
  computer_id: null as number | null,
  equipment_id: null as number | null,
  estimated_time: '',
})

const fetchData = async () => {
  if (!Number.isFinite(requestId)) {
    loading.value = false
    return
  }

  loading.value = true
  try {
    const [item, employeeList, computerList, equipmentList] = await Promise.all([
      fetchRequest(requestId),
      fetchEmployees({ per_page: 100 }),
      fetchComputers({ per_page: 100 }),
      fetchEquipmentList({ per_page: 100 }),
    ])

    employees.value = employeeList.employees
    computers.value = computerList.computers
    equipment.value = equipmentList.equipment

    form.value = {
      title: item.title,
      description: item.description,
      status: item.status,
      priority: item.priority,
      category: item.category ?? '',
      requester_id: item.requester_id,
      assignee_id: item.assignee_id,
      computer_id: item.computer_id,
      equipment_id: item.equipment_id,
      estimated_time: item.estimated_time?.toString() ?? '',
    }
    loaded.value = true
  } catch (error) {
    toast.error('Не удалось загрузить заявку: ' + getErrorMessage(error))
  } finally {
    loading.value = false
  }
}

const handleSubmit = async () => {
  if (!form.value.requester_id) {
    toast.warning('Выберите заявителя')
    return
  }

  saving.value = true
  try {
    await updateRequest(requestId, {
      title: form.value.title.trim(),
      description: form.value.description.trim(),
      status: form.value.status,
      priority: form.value.priority,
      category: form.value.category || undefined,
      requester_id: form.value.requester_id,
      assignee_id: form.value.assignee_id ?? undefined,
      computer_id: form.value.computer_id ?? undefined,
      equipment_id: form.value.equipment_id ?? undefined,
      estimated_time: toOptionalNumber(form.value.estimated_time),
    })

    toast.success('Заявка обновлена')
    router.push(`/requests/${requestId}`)
  } catch (error) {
    toast.error('Не удалось обновить заявку: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

useHead({
  title: `Редактирование заявки #${requestId}`,
})

onMounted(fetchData)
</script>
