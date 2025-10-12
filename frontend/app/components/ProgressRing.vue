<template>
    <div class="relative inline-flex items-center justify-center">
        <svg :width="size" :height="size" class="transform -rotate-90">
            <circle
                :cx="size / 2"
                :cy="size / 2"
                :r="radius"
                :stroke="bgColor"
                :stroke-width="strokeWidth"
                fill="none"
            />
            <circle
                :cx="size / 2"
                :cy="size / 2"
                :r="radius"
                :stroke="color"
                :stroke-width="strokeWidth"
                fill="none"
                :stroke-dasharray="circumference"
                :stroke-dashoffset="dashOffset"
                stroke-linecap="round"
                class="transition-all duration-500"
            />
        </svg>
        <div class="absolute inset-0 flex items-center justify-center">
            <slot />
        </div>
    </div>
</template>

<script setup lang="ts">
const props = withDefaults(
    defineProps<{
        percentage: number;
        size?: number;
        strokeWidth?: number;
        color?: string;
        bgColor?: string;
    }>(),
    {
        size: 120,
        strokeWidth: 8,
        color: "#3b82f6",
        bgColor: "#e5e7eb",
    },
);

const radius = computed(() => (props.size - props.strokeWidth) / 2);
const circumference = computed(() => 2 * Math.PI * radius.value);
const dashOffset = computed(
    () => circumference.value * (1 - props.percentage / 100),
);
</script>
