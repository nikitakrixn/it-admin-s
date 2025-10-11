<template>
  <div>
    <div class="mb-6">
      <NuxtLink to="/employees" class="text-sm text-gray-500 hover:text-gray-700 flex items-center mb-2">
        <Icon name="ri:arrow-left-line" class="mr-1" />
        Назад к списку
      </NuxtLink>
    </div>

    <div v-if="pending" class="flex justify-center py-12">
      <Icon name="ri:loader-4-line" class="text-4xl text-primary-600 animate-spin" />
    </div>

    <div v-else-if="error" class="rounded-md bg-red-50 p-4">
      <div class="flex">
        <Icon name="ri:error-warning-line" class="h-5 w-5 text-red-400" />
        <div class="ml-3">
          <p class="text-sm font-medium text-red-800">Ошибка загрузки данных сотрудника</p>
        </div>
      </div>
    </div>

    <div v-else-if="employee" class="space-y-6">
      <div class="bg-white shadow-sm rounded-lg border border-gray-200 overflow-hidden">
        <div class="bg-gradient-to-r from-primary-500 to-primary-600 px-6 py-8">
          <div class="flex items-center">
            <div class="h-20 w-20 rounded-full bg-white flex items-center justify-center text-primary-600 font-bold text-2xl shadow-lg">
              {{ employee.first_name.charAt(0) }}{{ employee.last_name.charAt(0) }}
            </div>
            <div class="ml-6 text-white">
              <h1 class="text-3xl font-bold">{{ employee.full_name }}</h1>
              <p class="mt-1 text-primary-100">{{ employee.position_name || 'Должность не указана' }}</p>
            </div>
          </div>
        </div>
        
        <div class="px-6 py-4 bg-gray-50 border-t border-gray-200 flex justify-end space-x-3">
          <NuxtLink
            :to="`/employees/${employee.id}/edit`"
            class="inline-flex items-center px-4 py-2 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50"
          >
            <Icon name="ri:edit-line" class="mr-2" />
            Редактировать
          </NuxtLink>
          <button
            @click="confirmDelete"
            class="inline-flex items-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-red-600 hover:bg-red-700"
          >
            <Icon name="ri:delete-bin-line" class="mr-2" />
            Удалить
          </button>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div class="bg-white shadow-sm rounded-lg border border-gray-200 overflow-hidden">
          <div class="px-6 py-4 border-b border-gray-200 bg-gray-50">
            <h3 class="text-lg font-medium text-gray-900">Личная информация</h3>
          </div>
          <div class="px-6 py-4 space-y-4">
            <div>
              <dt class="text-sm font-medium text-gray-500">ФИО</dt>
              <dd class="mt-1 text-sm text-gray-900">{{ employee.full_name }}</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500">Email</dt>
              <dd class="mt-1 text-sm text-gray-900">{{ employee.email || '-' }}</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500">Телефон</dt>
              <dd class="mt-1 text-sm text-gray-900">{{ employee.phone || '-' }}</dd>
            </div>
          </div>
        </div>

        <div class="bg-white shadow-sm rounded-lg border border-gray-200 overflow-hidden">
          <div class="px-6 py-4 border-b border-gray-200 bg-gray-50">
            <h3 class="text-lg font-medium text-gray-900">Рабочая информация</h3>
          </div>
          <div class="px-6 py-4 space-y-4">
            <div>
              <dt class="text-sm font-medium text-gray-500">Отдел</dt>
              <dd class="mt-1 text-sm text-gray-900">{{ employee.department_name || '-' }}</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500">Должность</dt>
              <dd class="mt-1 text-sm text-gray-900">{{ employee.position_name || '-' }}</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500">Статус</dt>
              <dd class="mt-1">
                <span :class="{
                  'bg-green-100 text-green-800': employee.status === 'active',
                  'bg-gray-100 text-gray-800': employee.status === 'inactive',
                  'bg-red-100 text-red-800': employee.status === 'terminated'
                }" class="px-2 py-1 text-xs font-medium rounded-full">
                  {{ statusLabel(employee.status) }}
                </span>
              </dd>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  middleware: 'auth'
})

const route = useRoute()
const employeeId = parseInt(route.params.id as string)

const { fetchEmployee, deleteEmployee } = useEmployees()

const { data: employee, pending, error } = await useAsyncData(
  `employee-${employeeId}`,
  () => fetchEmployee(employeeId)
)

useHead({
  title: computed(() => employee.value ? employee.value.full_name : 'Сотрудник')
})

const statusLabel = (status: string) => {
  const labels: Record<string, string> = {
    active: 'Активен',
    inactive: 'Неактивен',
    terminated: 'Уволен'
  }
  return labels[status] || status
}

const confirmDelete = async () => {
  if (!employee.value) return
  
  if (confirm(`Вы уверены, что хотите удалить сотрудника ${employee.value.full_name}?`)) {
    try {
      await deleteEmployee(employee.value.id)
      
      // Инвалидируем кеш списка сотрудников
      await refreshNuxtData('employees')
      
      await navigateTo('/employees')
    } catch (err) {
      alert('Ошибка при удалении сотрудника')
    }
  }
}
</script>
