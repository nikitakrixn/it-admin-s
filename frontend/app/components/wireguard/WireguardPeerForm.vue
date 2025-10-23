<template>
  <form @submit.prevent="$emit('submit')" class="space-y-6">
    <!-- Основная информация -->
    <div class="space-y-4">
      <h4 class="text-sm font-semibold text-gray-700 flex items-center gap-2">
        <Icon name="ri:information-line" class="text-blue-600" />
        Основная информация
      </h4>
      
      <UiInput
        :model-value="form.name"
        label="Название"
        required
        placeholder="VPN для Иванова И.И."
        @update:model-value="updateField('name', $event)"
      />

      <div class="grid grid-cols-2 gap-4">
        <UiSelect
          :model-value="form.interface_id"
          label="Интерфейс"
          required
          full-width
          placeholder="Выберите интерфейс"
          @update:model-value="updateField('interface_id', $event)"
        >
          <option v-for="iface in interfaces" :key="iface.id" :value="iface.id">
            {{ iface.name }}
          </option>
        </UiSelect>
        
        <UiSelect
          :model-value="form.employee_id"
          label="Сотрудник"
          full-width
          placeholder="Не назначен"
          @update:model-value="updateField('employee_id', $event)"
        >
          <option v-for="employee in employees" :key="employee.id" :value="employee.id">
            {{ employee.last_name }} {{ employee.first_name }}
          </option>
        </UiSelect>
      </div>
    </div>

    <!-- Сетевые настройки -->
    <div class="space-y-4 pt-4 border-t border-gray-200">
      <h4 class="text-sm font-semibold text-gray-700 flex items-center gap-2">
        <Icon name="ri:global-line" class="text-green-600" />
        Сетевые настройки
      </h4>
      
      <div class="grid grid-cols-2 gap-4">
        <UiInput
          :model-value="form.client_address"
          label="IP адрес клиента"
          required
          placeholder="192.168.99.10/30"
          @update:model-value="updateField('client_address', $event)"
        />
        
        <UiInput
          :model-value="form.client_dns"
          label="DNS сервер"
          placeholder="192.168.78.254"
          @update:model-value="updateField('client_dns', $event)"
        />
      </div>

      <UiInput
        :model-value="form.allowed_ips"
        label="Allowed IPs"
        placeholder="0.0.0.0/0,::/0"
        hint="Разрешенные IP-адреса для маршрутизации через VPN"
        @update:model-value="updateField('allowed_ips', $event)"
      />
    </div>

    <!-- Настройки подключения -->
    <div class="space-y-4 pt-4 border-t border-gray-200">
      <h4 class="text-sm font-semibold text-gray-700 flex items-center gap-2">
        <Icon name="ri:settings-3-line" class="text-purple-600" />
        Настройки подключения
      </h4>
      
      <div class="grid grid-cols-2 gap-4">
        <UiInput
          :model-value="form.endpoint_address"
          label="Endpoint адрес"
          placeholder="vpn.example.com"
          @update:model-value="updateField('endpoint_address', $event)"
        />
        
        <UiInput
          :model-value="form.endpoint_port"
          type="number"
          label="Endpoint порт"
          placeholder="51821"
          @update:model-value="updateField('endpoint_port', $event)"
        />
      </div>

      <UiInput
        :model-value="form.persistent_keepalive"
        type="number"
        label="Persistent Keepalive (секунды)"
        placeholder="25"
        hint="Интервал отправки keepalive пакетов (рекомендуется 25)"
        @update:model-value="updateField('persistent_keepalive', $event)"
      />
    </div>

    <!-- Дополнительно -->
    <div class="space-y-4 pt-4 border-t border-gray-200">
      <h4 class="text-sm font-semibold text-gray-700 flex items-center gap-2">
        <Icon name="ri:file-text-line" class="text-amber-600" />
        Дополнительно
      </h4>
      
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Заметки</label>
        <textarea
          :value="form.notes"
          rows="3"
          class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
          placeholder="Дополнительная информация о конфигурации..."
          @input="updateField('notes', ($event.target as HTMLTextAreaElement).value)"
        />
      </div>
    </div>

    <div class="flex items-center justify-end gap-3 pt-6 border-t border-gray-200">
      <UiButton variant="secondary" type="button" @click="$emit('cancel')">
        Отмена
      </UiButton>
      <UiButton type="submit" :loading="submitting">
        {{ submitText }}
      </UiButton>
    </div>
  </form>
</template>

<script setup lang="ts">
interface FormData {
  interface_id: number | null
  employee_id: number | null
  name: string
  client_address: string
  client_dns: string
  endpoint_address: string
  endpoint_port: number | null
  allowed_ips: string
  persistent_keepalive: number
  notes: string
}

interface Props {
  form: FormData
  interfaces: Array<{ id: number; name: string }>
  employees: Array<{ id: number; first_name: string; last_name: string }>
  submitting: boolean
  submitText?: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:form': [form: FormData]
  submit: []
  cancel: []
}>()

const updateField = (field: keyof FormData, value: any) => {
  const updatedForm = { ...props.form, [field]: value }
  emit('update:form', updatedForm)
}
</script>
