<template>
  <div class="max-w-4xl mx-auto space-y-6">
    <div class="mb-8">
      <div class="flex items-center gap-4 mb-6">
        <NuxtLink to="/requests" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
          <Icon name="ri:arrow-left-line" class="text-xl" />
        </NuxtLink>
        <div class="flex items-center gap-4">
          <div
            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-blue-500 to-blue-700 flex items-center justify-center shadow-lg"
          >
            <Icon name="ri:file-add-line" class="text-3xl text-white" />
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">Создание заявки</h1>
            <p class="mt-1 text-gray-600">Заполните форму для создания новой заявки</p>
          </div>
        </div>
      </div>
    </div>

    <form class="space-y-6" @submit.prevent="handleSubmit">
      <div class="card">
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <h2 class="text-lg font-semibold text-gray-900">Основная информация</h2>
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
          </div>
        </div>
      </div>

      <div class="card">
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <h2 class="text-lg font-semibold text-gray-900">Ресурсы</h2>
        </div>

        <div class="p-6 space-y-6">
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

          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
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
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-white">
          <div class="flex items-center gap-2">
            <Icon name="ri:information-line" class="text-blue-600" />
            <h2 class="text-lg font-semibold text-gray-900">Предварительный просмотр</h2>
          </div>
        </div>

        <div class="p-6">
          <div class="flex items-start gap-4">
            <div
              :class="[
                'h-12 w-12 rounded-xl flex items-center justify-center flex-shrink-0',
                getPriorityColor(form.priority),
              ]"
            >
              <Icon name="ri:flag-line" class="text-xl text-white" />
            </div>
            <div class="flex-1">
              <h3 class="text-lg font-semibold text-gray-900 mb-1">
                {{ form.title || 'Заголовок заявки' }}
              </h3>
              <p class="text-sm text-gray-600 mb-3">
                {{ form.description || 'Описание заявки' }}
              </p>
              <div class="flex flex-wrap gap-2">
                <span v-if="form.category" class="badge bg-purple-100 text-purple-800">
                  {{ getCategoryLabel(form.category) }}
                </span>
                <span v-if="form.priority" :class="['badge', getPriorityBadgeColor(form.priority)]">
                  {{ getPriorityLabel(form.priority) }}
                </span>
                <span v-if="requesterName" class="badge bg-blue-100 text-blue-800">
                  <Icon name="ri:user-line" class="mr-1" />
                  {{ requesterName }}
                </span>
                <span v-if="assigneeName" class="badge bg-green-100 text-green-800">
                  <Icon name="ri:user-settings-line" class="mr-1" />
                  {{ assigneeName }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div class="flex items-center justify-between gap-4">
        <NuxtLink to="/requests" class="btn btn-secondary">
          <Icon name="ri:close-line" class="mr-2" />
          Отмена
        </NuxtLink>

        <button
          type="submit"
          class="btn btn-primary disabled:opacity-50"
          :disabled="saving || !isFormValid"
        >
          <Icon name="ri:send-plane-line" class="mr-2" />
          {{ saving ? 'Создание...' : 'Создать заявку' }}
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

useHead({
  title: 'Создание заявки',
})

const router = useRouter()
const toast = useToast()

const { createRequest } = useRequests()
const { fetchEmployees } = useEmployees()
const { fetchComputers } = useComputers()
const { fetchEquipmentList } = useEquipment()

const saving = ref(false)
const employees = ref<Employee[]>([])
const computers = ref<Computer[]>([])
const equipment = ref<Equipment[]>([])

const form = ref({
  title: '',
  description: '',
  priority: 'medium',
  category: '',
  requester_id: null as number | null,
  assignee_id: null as number | null,
  computer_id: null as number | null,
  equipment_id: null as number | null,
  estimated_time: '',
})

const isFormValid = computed(() =>
  Boolean(form.value.title.trim() && form.value.description.trim() && form.value.requester_id),
)

const requesterName = computed(
  () => employees.value.find((item) => item.id === form.value.requester_id)?.full_name ?? '',
)

const assigneeName = computed(
  () => employees.value.find((item) => item.id === form.value.assignee_id)?.full_name ?? '',
)

const fetchData = async () => {
  try {
    const [employeeList, computerList, equipmentList] = await Promise.all([
      fetchEmployees({ per_page: 100 }),
      fetchComputers({ per_page: 100 }),
      fetchEquipmentList({ per_page: 100 }),
    ])
    employees.value = employeeList.employees
    computers.value = computerList.computers
    equipment.value = equipmentList.equipment
  } catch (error) {
    toast.error('Не удалось загрузить справочники: ' + getErrorMessage(error))
  }
}

const handleSubmit = async () => {
  if (!isFormValid.value || !form.value.requester_id) {
    toast.error('Заполните все обязательные поля')
    return
  }

  saving.value = true
  try {
    const created = await createRequest({
      title: form.value.title.trim(),
      description: form.value.description.trim(),
      priority: form.value.priority,
      category: form.value.category || undefined,
      requester_id: form.value.requester_id,
      assignee_id: form.value.assignee_id ?? undefined,
      computer_id: form.value.computer_id ?? undefined,
      equipment_id: form.value.equipment_id ?? undefined,
      estimated_time: toOptionalNumber(form.value.estimated_time),
    })

    toast.success('Заявка создана')
    router.push(`/requests/${created.id}`)
  } catch (error) {
    toast.error('Не удалось создать заявку: ' + getErrorMessage(error))
  } finally {
    saving.value = false
  }
}

const getPriorityLabel = (priority: string) => {
  const labels: Record<string, string> = {
    low: 'Низкий',
    medium: 'Средний',
    high: 'Высокий',
    critical: 'Критический',
  }
  return labels[priority] || priority
}

const getCategoryLabel = (category: string) => {
  const labels: Record<string, string> = {
    hardware: 'Оборудование',
    software: 'ПО',
    network: 'Сеть',
    access: 'Доступ',
    consultation: 'Консультация',
    other: 'Другое',
  }
  return labels[category] || category
}

const getPriorityColor = (priority: string) => {
  const colors: Record<string, string> = {
    low: 'bg-gray-500',
    medium: 'bg-blue-500',
    high: 'bg-amber-500',
    critical: 'bg-red-500',
  }
  return colors[priority] || 'bg-gray-500'
}

const getPriorityBadgeColor = (priority: string) => {
  const colors: Record<string, string> = {
    low: 'bg-gray-100 text-gray-800',
    medium: 'bg-blue-100 text-blue-800',
    high: 'bg-amber-100 text-amber-800',
    critical: 'bg-red-100 text-red-800',
  }
  return colors[priority] || 'bg-gray-100 text-gray-800'
}

onMounted(fetchData)
</script>
