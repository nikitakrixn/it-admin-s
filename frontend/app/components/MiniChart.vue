<template>
    <div class="flex items-end gap-1 h-8">
        <div
            v-for="(value, index) in data"
            :key="index"
            class="flex-1 bg-current rounded-t transition-all duration-300 hover:opacity-70"
            :style="{
                height: `${(value / max) * 100}%`,
                opacity: 0.3 + (value / max) * 0.7,
            }"
        />
    </div>
</template>

<script setup lang="ts">
const props = defineProps<{
    data: number[];
}>();

const max = computed(() => Math.max(...props.data));
</script>
