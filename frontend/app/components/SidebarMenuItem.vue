<template>
    <NuxtLink
        :to="to"
        class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors"
        :class="
            isActive
                ? 'bg-primary-50 text-primary-700'
                : 'text-gray-700 hover:bg-gray-100'
        "
    >
        <Icon :name="icon" class="text-lg" />
        <span>{{ label }}</span>
    </NuxtLink>
</template>

<script setup lang="ts">
const props = defineProps<{
    to: string;
    icon: string;
    label: string;
}>();

const route = useRoute();
const isActive = computed(() => {
    if (props.to === "/") {
        return route.path === "/";
    }
    return route.path.startsWith(props.to);
});
</script>
