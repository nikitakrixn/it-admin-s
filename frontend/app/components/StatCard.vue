<template>
    <NuxtLink :to="to" class="card card-hover group cursor-pointer">
        <div class="p-6">
            <div class="flex items-center justify-between">
                <div class="flex-1">
                    <p class="text-sm font-medium text-gray-600">{{ title }}</p>
                    <div class="mt-2 flex items-baseline gap-2">
                        <p class="text-3xl font-bold text-gray-900">
                            {{ value }}
                        </p>
                        <span
                            v-if="trend"
                            :class="[
                                'text-sm font-medium',
                                trendDown ? 'text-red-600' : 'text-green-600',
                            ]"
                        >
                            {{ trend }}
                        </span>
                    </div>
                </div>
                <div
                    :class="[
                        'flex h-14 w-14 items-center justify-center rounded-xl transition-transform group-hover:scale-110',
                        colorClasses,
                    ]"
                >
                    <Icon :name="icon" class="text-2xl text-white" />
                </div>
            </div>
        </div>
    </NuxtLink>
</template>

<script setup lang="ts">
const props = defineProps<{
    title: string;
    value: number | string;
    icon: string;
    color: "blue" | "purple" | "amber" | "green" | "red";
    trend?: string;
    trendDown?: boolean;
    to: string;
}>();

const colorClasses = computed(() => {
    const colors = {
        blue: "bg-gradient-to-br from-blue-500 to-blue-600 shadow-blue-500/30",
        purple: "bg-gradient-to-br from-purple-500 to-purple-600 shadow-purple-500/30",
        amber: "bg-gradient-to-br from-amber-500 to-amber-600 shadow-amber-500/30",
        green: "bg-gradient-to-br from-green-500 to-green-600 shadow-green-500/30",
        red: "bg-gradient-to-br from-red-500 to-red-600 shadow-red-500/30",
    };
    return `${colors[props.color]} shadow-lg`;
});
</script>
