<template>
  <div class="flex flex-col items-center justify-center py-12 px-4">
    <div class="relative mb-6">
      <div
        class="h-24 w-24 rounded-2xl bg-gradient-to-br from-blue-100 to-purple-100 flex items-center justify-center"
      >
        <Icon
          :name="icon"
          class="text-5xl text-blue-600"
        />
      </div>
      <div v-if="badge" class="absolute -bottom-2 -right-2 h-8 w-8 rounded-full bg-yellow-400 flex items-center justify-center shadow-lg">
        <Icon :name="badge" class="text-lg text-yellow-900" />
      </div>
    </div>
    
    <h3 class="text-xl font-bold text-gray-900 mb-2">
      {{ title }}
    </h3>
    
    <p class="text-sm text-gray-500 mb-6 max-w-md text-center">
      {{ description }}
    </p>
    
    <div class="flex gap-3">
      <slot name="actions" />
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  icon?: string
  badge?: string
  title: string
  description: string
}

withDefaults(defineProps<Props>(), {
  icon: 'ri:inbox-line',
})
</script>
