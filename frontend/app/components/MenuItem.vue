<template>
    <li>
        <NuxtLink
            :to="to"
            class="menu-item group flex items-center gap-x-3 rounded-md px-3 py-2 text-sm font-medium transition-all duration-200"
            :class="
                isActive
                    ? 'active bg-primary-800 text-white'
                    : 'text-primary-100 hover:bg-primary-800/70'
            "
        >
            <Icon :name="icon" class="text-lg flex-shrink-0" />
            <span>{{ label }}</span>
            <span
                v-if="badge"
                class="ml-auto inline-flex items-center justify-center px-2 py-0.5 text-xs font-medium rounded-full bg-red-500 text-white min-w-[1.5rem] shadow-sm"
            >
                {{ badge }}
            </span>
        </NuxtLink>
    </li>
</template>

<script setup lang="ts">
const props = defineProps<{
    to: string;
    icon: string;
    label: string;
    badge?: number | string;
}>();

const route = useRoute();
const isActive = computed(() => route.path.startsWith(props.to));
</script>

<style scoped>
.menu-item {
    transition: all 0.2s ease;
}

.menu-item.active {
    background-color: rgba(255, 255, 255, 0.1);
    font-weight: 500;
}

.menu-item:hover:not(.active) {
    background-color: rgba(255, 255, 255, 0.05);
}
</style>
