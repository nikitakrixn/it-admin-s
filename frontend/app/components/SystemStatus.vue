<template>
    <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
            <div :class="['h-3 w-3 rounded-full', statusColor]" />
            <div>
                <p class="text-sm font-medium text-gray-900">{{ label }}</p>
                <p v-if="message" class="text-xs text-gray-500">
                    {{ message }}
                </p>
            </div>
        </div>
        <span :class="['badge', statusBadgeColor]">
            {{ statusText }}
        </span>
    </div>
</template>

<script setup lang="ts">
const props = defineProps<{
    label: string;
    status: "online" | "offline" | "warning";
    message?: string;
}>();

const statusColor = computed(() => {
    const colors = {
        online: "bg-green-500 animate-pulse",
        offline: "bg-red-500",
        warning: "bg-amber-500 animate-pulse",
    };
    return colors[props.status];
});

const statusBadgeColor = computed(() => {
    const colors = {
        online: "bg-green-100 text-green-800",
        offline: "bg-red-100 text-red-800",
        warning: "bg-amber-100 text-amber-800",
    };
    return colors[props.status];
});

const statusText = computed(() => {
    const texts = {
        online: "Онлайн",
        offline: "Офлайн",
        warning: "Внимание",
    };
    return texts[props.status];
});
</script>
