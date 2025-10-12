<template>
    <NuxtLink
        :to="to"
        class="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors group"
    >
        <div
            class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary-100 text-primary-600 group-hover:bg-primary-200 transition-colors"
        >
            <Icon :name="icon" class="text-xl" />
        </div>
        <span class="text-sm font-medium text-gray-900">{{ label }}</span>
        <Icon
            name="ri:arrow-right-s-line"
            class="ml-auto text-gray-400 group-hover:text-primary-600 transition-colors"
        />
    </NuxtLink>
</template>

<script setup lang="ts">
defineProps<{
    icon: string;
    label: string;
    to: string;
}>();
</script>
