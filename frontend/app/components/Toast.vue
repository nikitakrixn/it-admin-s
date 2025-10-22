<template>
    <Teleport to="body">
        <div class="fixed top-4 right-4 z-[9999] space-y-2">
            <TransitionGroup name="toast">
                <div
                    v-for="toast in toasts"
                    :key="toast.id"
                    :class="[
                        'flex items-start gap-3 p-4 rounded-xl shadow-lg backdrop-blur-sm border max-w-sm',
                        getToastClass(toast.type),
                    ]"
                >
                    <div class="flex-shrink-0">
                        <Icon :name="getIcon(toast.type)" class="text-xl" />
                    </div>
                    <div class="flex-1 min-w-0">
                        <p class="text-sm font-medium">{{ toast.message }}</p>
                    </div>
                    <button
                        @click="removeToast(toast.id)"
                        class="flex-shrink-0 opacity-70 hover:opacity-100"
                    >
                        <Icon name="ri:close-line" />
                    </button>
                </div>
            </TransitionGroup>
        </div>
    </Teleport>
</template>

<script setup lang="ts">
const toasts = useState<any[]>("toasts", () => []);

const removeToast = (id: number) => {
    const index = toasts.value.findIndex((t) => t.id === id);
    if (index > -1) {
        toasts.value.splice(index, 1);
    }
};

const getToastClass = (type: string) => {
    const classes = {
        success: "bg-green-50/90 border-green-200 text-green-900",
        error: "bg-red-50/90 border-red-200 text-red-900",
        warning: "bg-amber-50/90 border-amber-200 text-amber-900",
        info: "bg-blue-50/90 border-blue-200 text-blue-900",
    };
    return classes[type as keyof typeof classes] || classes.info;
};

const getIcon = (type: string) => {
    const icons = {
        success: "ri:checkbox-circle-fill",
        error: "ri:error-warning-fill",
        warning: "ri:alert-fill",
        info: "ri:information-fill",
    };
    return icons[type as keyof typeof icons] || icons.info;
};
</script>

<style scoped>
.toast-enter-active,
.toast-leave-active {
    transition: all 0.3s ease;
}

.toast-enter-from {
    opacity: 0;
    transform: translateX(100px);
}

.toast-leave-to {
    opacity: 0;
    transform: translateX(100px) scale(0.8);
}
</style>
