<template>
  <UiCard padding="md" class="bg-gradient-to-r from-gray-50 to-white">
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-3">
      <div class="lg:col-span-5">
        <UiInput
          :model-value="searchQuery"
          icon="ri:search-line"
          placeholder="Поиск по имени или сотруднику..."
          @update:model-value="$emit('update:searchQuery', $event)"
        />
      </div>
      
      <div class="lg:col-span-3">
        <UiSelect
          :model-value="interfaceId"
          full-width
          placeholder="Все интерфейсы"
          @update:model-value="$emit('update:interfaceId', $event)"
        >
          <option
            v-for="iface in interfaces"
            :key="iface.id"
            :value="iface.id"
          >
            {{ iface.name }}
          </option>
        </UiSelect>
      </div>
      
      <div class="lg:col-span-3">
        <UiSelect
          :model-value="status"
          full-width
          placeholder="Все статусы"
          @update:model-value="$emit('update:status', $event)"
        >
          <option value="active">Активен</option>
          <option value="disabled">Отключен</option>
          <option value="expired">Истек</option>
          <option value="revoked">Отозван</option>
        </UiSelect>
      </div>
      
      <div class="lg:col-span-1 flex items-end">
        <UiButton
          v-if="hasActiveFilters"
          variant="secondary"
          icon="ri:refresh-line"
          full-width
          @click="$emit('reset')"
        >
          Сбросить
        </UiButton>
      </div>
    </div>
  </UiCard>
</template>

<script setup lang="ts">
interface Props {
  searchQuery: string
  interfaceId?: number
  status?: string
  interfaces: Array<{ id: number; name: string }>
}

const props = defineProps<Props>()

defineEmits<{
  'update:searchQuery': [value: string]
  'update:interfaceId': [value: number | undefined]
  'update:status': [value: string | undefined]
  reset: []
}>()

const hasActiveFilters = computed(() => {
  return props.searchQuery || props.interfaceId || props.status
})
</script>
