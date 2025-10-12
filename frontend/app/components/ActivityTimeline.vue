<template>
    <div class="flow-root">
        <ul role="list" class="-mb-8">
            <li v-for="(activity, idx) in activities" :key="activity.id">
                <div class="relative pb-8">
                    <span
                        v-if="idx !== activities.length - 1"
                        class="absolute left-5 top-5 -ml-px h-full w-0.5 bg-gray-200"
                        aria-hidden="true"
                    />
                    <div class="relative flex items-start space-x-3">
                        <div class="relative">
                            <div
                                :class="[
                                    'flex h-10 w-10 items-center justify-center rounded-full ring-8 ring-white',
                                    getColorClass(activity.color),
                                ]"
                            >
                                <Icon
                                    :name="activity.icon"
                                    class="h-5 w-5 text-white"
                                />
                            </div>
                        </div>
                        <div class="min-w-0 flex-1">
                            <div>
                                <p class="text-sm text-gray-900">
                                    {{ activity.message }}
                                </p>
                                <p class="mt-0.5 text-xs text-gray-500">
                                    {{ activity.time }}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </li>
        </ul>
    </div>
</template>

<script setup lang="ts">
defineProps<{
    activities: Array<{
        id: number;
        type: string;
        message: string;
        time: string;
        icon: string;
        color: string;
    }>;
}>();

const getColorClass = (color: string) => {
    const colors: Record<string, string> = {
        blue: "bg-blue-500",
        purple: "bg-purple-500",
        green: "bg-green-500",
        amber: "bg-amber-500",
        red: "bg-red-500",
    };
    return colors[color] || "bg-gray-500";
};
</script>
