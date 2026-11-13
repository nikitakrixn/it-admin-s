<template>
  <div
    class="p-4 rounded-lg border group hover:shadow-md transition-all cursor-pointer relative"
    :class="[color]"
  >
    <NuxtLink :to="link" class="absolute inset-0" />
    <div class="flex items-center gap-3 mb-3">
      <div :class="['h-10 w-10 rounded-lg flex items-center justify-center flex-shrink-0', iconBg]">
        <Icon :name="icon" class="text-xl text-white" />
      </div>
      <div class="min-w-0 flex-1">
        <p
          class="text-sm font-semibold text-gray-900 group-hover:text-primary-600 transition-colors"
        >
          <slot name="value" />
        </p>
      </div>
      <button
        v-if="edit"
        class="relative z-10 p-1.5 rounded-lg hover:bg-primary-100 transition-colors opacity-0 group-hover:opacity-100"
        title="Редактировать"
        @click.prevent="edit"
      >
        <Icon :name="editIcon" class="text-primary-600" />
      </button>
    </div>
    <div v-if="$slots.stats" class="grid grid-cols-3 gap-2 text-xs">
      <slot name="stats" />
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title: string
  icon: string
  color: string
  iconBg?: string
  link?: string
  edit?: () => void
  editIcon?: string
  editColor?: string
}

withDefaults(defineProps<Props>(), {
  iconBg: 'bg-primary-500',
  link: '#',
  edit: undefined,
  editIcon: 'ri:edit-line',
  editColor: 'text-primary-600 hover:bg-primary-100',
})
</script>
