<template>
  <NuxtLink :to="link" class="card card-hover group">
    <div class="p-5 flex items-center gap-4">
      <div
        class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl"
        :class="[
          'bg-' + color + '-100 text-' + color + '-600',
          'group-hover:bg-' + color + '-200 transition-colors',
        ]"
      >
        <Icon :name="icon" class="text-2xl" />
      </div>
      <div>
        <p class="text-sm font-semibold text-gray-900">{{ title }}</p>
        <p class="text-xs text-gray-500 mt-0.5">{{ description }}</p>
      </div>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
interface Props {
  icon: string
  title: string
  description: string
  color: string
  link: string
}

defineProps<Props>()
</script>
