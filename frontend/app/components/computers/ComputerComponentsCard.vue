<template>
  <div class="grid grid-cols-2 gap-4">
    <component-card
      v-for="group in groups"
      :key="group.type"
      :title="group.label"
      :icon="group.icon"
      :color="group.color"
      :icon-bg="group.iconBg"
      :link="group.link"
      :edit="group.edit"
    >
      <template #value>
        <p class="text-sm font-semibold text-gray-900 truncate">
          {{ group.component ? group.component.name : 'Не установлено' }}
        </p>
        <p class="text-xs text-gray-600">{{ group.summary }}</p>
      </template>
      <template #stats>
        <div v-if="group.details.length" class="space-y-1">
          <div
            v-for="item in group.details"
            :key="item.label"
            class="flex items-center justify-between text-xs"
          >
            <span class="text-gray-600">{{ item.label }}</span>
            <span class="font-medium text-gray-900">{{ item.value }}</span>
          </div>
        </div>
        <p v-else class="text-xs text-gray-500 col-span-3">
          {{
            group.component
              ? 'Характеристики не заполнены'
              : 'Добавьте компонент, чтобы увидеть характеристики'
          }}
        </p>
      </template>
    </component-card>
  </div>
</template>

<script setup lang="ts">
import type { ComputerComponent } from '~/composables/useComputerComponents'
import ComponentCard from '~/components/computers/ComponentCard.vue'

interface Props {
  components: ComputerComponent[]
  computerId: number
}

const props = defineProps<Props>()

const emit = defineEmits<{
  edit: [componentType: string, componentId: number]
}>()

interface ComponentGroup {
  type: string
  label: string
  icon: string
  color: string
  iconBg: string
  component: ComputerComponent | null
  summary: string
  details: Array<{ label: string; value: string }>
  link: string
  edit?: () => void
}

const typeDefinitions = [
  {
    type: 'processor',
    label: 'Процессор',
    icon: 'ri:cpu-line',
    color: 'bg-blue-500 text-blue-600 border-blue-100 bg-blue-50',
    iconBg: 'bg-blue-500',
  },
  {
    type: 'ram',
    label: 'Оперативная память',
    icon: 'ri:database-2-line',
    color: 'bg-purple-500 text-purple-600 border-purple-100 bg-purple-50',
    iconBg: 'bg-purple-500',
  },
  {
    type: 'storage',
    label: 'Накопители',
    icon: 'ri:hard-drive-line',
    color: 'bg-green-500 text-green-600 border-green-100 bg-green-50',
    iconBg: 'bg-green-500',
  },
  {
    type: 'gpu',
    label: 'Видеокарта',
    icon: 'ri:gamepad-line',
    color: 'bg-amber-500 text-amber-600 border-amber-100 bg-amber-50',
    iconBg: 'bg-amber-500',
  },
  {
    type: 'motherboard',
    label: 'Материнская плата',
    icon: 'ri:cpu-line',
    color: 'bg-indigo-500 text-indigo-600 border-indigo-100 bg-indigo-50',
    iconBg: 'bg-indigo-500',
  },
  {
    type: 'network',
    label: 'Сетевые адаптеры',
    icon: 'ri:wifi-line',
    color: 'bg-cyan-500 text-cyan-600 border-cyan-100 bg-cyan-50',
    iconBg: 'bg-cyan-500',
  },
  {
    type: 'monitor',
    label: 'Мониторы',
    icon: 'ri:tv-line',
    color: 'bg-pink-500 text-pink-600 border-pink-100 bg-pink-50',
    iconBg: 'bg-pink-500',
  },
  {
    type: 'peripheral',
    label: 'Периферия',
    icon: 'ri:keyboard-line',
    color: 'bg-teal-500 text-teal-600 border-teal-100 bg-teal-50',
    iconBg: 'bg-teal-500',
  },
]

const plural = (count: number, one: string, few: string, many: string) => {
  const mod10 = count % 10
  const mod100 = count % 100
  if (mod10 === 1 && mod100 !== 11) return one
  if (mod10 >= 2 && mod10 <= 4 && (mod100 < 10 || mod100 >= 20)) return few
  return many
}

const groups = computed<ComponentGroup[]>(() =>
  typeDefinitions.map((definition) => {
    const items = props.components.filter((item) => item.component_type === definition.type)
    const component = items[0] ?? null
    let summary = 'Не установлено'

    if (items.length === 1) {
      summary = component?.manufacturer || component?.model || 'Установлено'
    } else if (items.length > 1) {
      summary = `${items.length} ${plural(items.length, 'компонент', 'компонента', 'компонентов')}`
    }

    return {
      ...definition,
      component,
      summary,
      details: component?.details.slice(0, 3) ?? [],
      link: component
        ? `/computers/${props.computerId}/components/${definition.type}/${component.id}`
        : `/computers/${props.computerId}/components/add`,
      edit: component ? () => emit('edit', definition.type, component.id) : undefined,
    }
  }),
)
</script>
