<template>
  <NuxtLink :to="link" class="card card-hover group">
    <div class="relative overflow-hidden">
      <div class="p-6">
        <div class="flex justify-between items-start">
          <div class="flex-1">
            <p class="text-sm font-medium text-gray-600 mb-2">{{ title }}</p>
            <div class="flex items-baseline gap-2 mb-3">
              <p class="text-3xl font-bold text-gray-900">{{ value }}</p>
              <span
                v-if="change"
                class="text-sm font-medium flex items-center"
                :class="changeColor"
              >
                <Icon :name="changeIcon" class="mr-1" />
                {{ change }}
              </span>
            </div>
            <div v-if="subtitle" class="flex items-center text-sm text-gray-500">
              <Icon :name="subtitleIcon" class="mr-1" />
              <span>{{ subtitle }}</span>
            </div>
          </div>
          <div
            class="flex h-14 w-14 items-center justify-center rounded-xl"
            :class="[
              'text-' + color + '-600',
              'bg-' + color + '-100',
              'group-hover:scale-110 transition-transform',
            ]"
          >
            <Icon :name="icon" class="text-2xl" />
          </div>
        </div>
        <div v-if="$slots.detail" class="mt-4 pt-4 border-t border-gray-100">
          <slot name="detail" />
        </div>
      </div>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
interface Props {
  title: string
  value: number | string
  change?: string
  changeColor?: string
  changeIcon?: string
  subtitle?: string
  subtitleIcon?: string
  icon: string
  color: string
  link: string
}

defineProps<Props>()
</script>
