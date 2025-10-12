<template>
    <Teleport to="body">
        <Transition name="modal">
            <div v-if="show" class="fixed inset-0 z-50 overflow-y-auto">
                <div
                    class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm"
                    @click="$emit('close')"
                />

                <div
                    class="relative min-h-screen flex items-start justify-center p-4 pt-20"
                >
                    <div
                        class="relative bg-white rounded-2xl shadow-2xl w-full max-w-2xl"
                    >
                        <div class="p-4 border-b border-gray-200">
                            <div class="relative">
                                <Icon
                                    name="ri:search-line"
                                    class="absolute left-4 top-1/2 -translate-y-1/2 text-gray-400 text-xl"
                                />
                                <input
                                    ref="searchInput"
                                    v-model="query"
                                    type="text"
                                    placeholder="Поиск сотрудников, компьютеров, заявок..."
                                    class="w-full pl-12 pr-4 py-3 text-lg border-0 focus:ring-0 focus:outline-none"
                                    @keydown.esc="$emit('close')"
                                />
                            </div>
                        </div>

                        <div
                            v-if="query"
                            class="max-h-96 overflow-y-auto scrollbar-thin"
                        >
                            <div v-if="loading" class="p-8 text-center">
                                <Icon
                                    name="ri:loader-4-line"
                                    class="text-3xl text-primary-600 animate-spin mx-auto"
                                />
                            </div>

                            <div
                                v-else-if="results.length === 0"
                                class="p-8 text-center"
                            >
                                <Icon
                                    name="ri:search-line"
                                    class="text-4xl text-gray-300 mx-auto mb-2"
                                />
                                <p class="text-gray-500">Ничего не найдено</p>
                            </div>

                            <div v-else class="divide-y divide-gray-100">
                                <NuxtLink
                                    v-for="result in results"
                                    :key="result.id"
                                    :to="result.url"
                                    class="flex items-center gap-4 p-4 hover:bg-gray-50 transition-colors"
                                    @click="$emit('close')"
                                >
                                    <div
                                        :class="[
                                            'flex h-10 w-10 items-center justify-center rounded-lg',
                                            getTypeColor(result.type),
                                        ]"
                                    >
                                        <Icon
                                            :name="getTypeIcon(result.type)"
                                            class="text-lg text-white"
                                        />
                                    </div>
                                    <div class="flex-1 min-w-0">
                                        <p
                                            class="text-sm font-medium text-gray-900"
                                        >
                                            {{ result.title }}
                                        </p>
                                        <p class="text-xs text-gray-500">
                                            {{ result.subtitle }}
                                        </p>
                                    </div>
                                    <span
                                        class="badge bg-gray-100 text-gray-600 text-xs"
                                        >{{ getTypeLabel(result.type) }}</span
                                    >
                                </NuxtLink>
                            </div>
                        </div>

                        <div
                            v-else
                            class="p-8 text-center text-gray-500 text-sm"
                        >
                            <p>Начните вводить для поиска</p>
                            <div
                                class="mt-4 flex items-center justify-center gap-4 text-xs"
                            >
                                <span class="flex items-center gap-1">
                                    <kbd class="px-2 py-1 bg-gray-100 rounded"
                                        >ESC</kbd
                                    >
                                    закрыть
                                </span>
                                <span class="flex items-center gap-1">
                                    <kbd class="px-2 py-1 bg-gray-100 rounded"
                                        >↑↓</kbd
                                    >
                                    навигация
                                </span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Transition>
    </Teleport>
</template>

<script setup lang="ts">
const query = ref("");
const loading = ref(false);
const results = ref<any[]>([]);
const searchInput = ref<HTMLInputElement>();

watch(
    () => query.value,
    async (newQuery) => {
        if (!newQuery) {
            results.value = [];
            return;
        }

        loading.value = true;

        await new Promise((resolve) => setTimeout(resolve, 300));

        results.value = [
            {
                id: 1,
                type: "employee",
                title: "Иванов Иван Иванович",
                subtitle: "Системный администратор",
                url: "/employees/1",
            },
            {
                id: 2,
                type: "computer",
                title: "PC-2024-001",
                subtitle: "Windows 11 Pro",
                url: "/computers/1",
            },
            {
                id: 3,
                type: "request",
                title: "Заявка #1234",
                subtitle: "Не работает принтер",
                url: "/requests/1234",
            },
        ].filter(
            (item) =>
                item.title.toLowerCase().includes(newQuery.toLowerCase()) ||
                item.subtitle.toLowerCase().includes(newQuery.toLowerCase()),
        );

        loading.value = false;
    },
);

const getTypeColor = (type: string) => {
    const colors = {
        employee: "bg-blue-500",
        computer: "bg-purple-500",
        request: "bg-amber-500",
        equipment: "bg-green-500",
    };
    return colors[type as keyof typeof colors] || "bg-gray-500";
};

const getTypeIcon = (type: string) => {
    const icons = {
        employee: "ri:user-line",
        computer: "ri:computer-line",
        request: "ri:file-list-line",
        equipment: "ri:printer-line",
    };
    return icons[type as keyof typeof icons] || "ri:file-line";
};

const getTypeLabel = (type: string) => {
    const labels = {
        employee: "Сотрудник",
        computer: "Компьютер",
        request: "Заявка",
        equipment: "Оборудование",
    };
    return labels[type as keyof typeof labels] || type;
};

const emit = defineEmits<{
    close: [];
}>();

const props = defineProps<{
    show: boolean;
}>();

// Фокус на input при открытии модального окна
watch(
    () => props.show,
    (newValue) => {
        if (newValue) {
            nextTick(() => {
                searchInput.value?.focus();
            });
        } else {
            // Очистка при закрытии
            query.value = "";
            results.value = [];
        }
    },
);

// Обработка ESC на уровне документа
const handleEscape = (e: KeyboardEvent) => {
    if (e.key === "Escape" && props.show) {
        emit("close");
    }
};

onMounted(() => {
    document.addEventListener("keydown", handleEscape);
});

onUnmounted(() => {
    document.removeEventListener("keydown", handleEscape);
});
</script>

<style scoped>
.modal-enter-active,
.modal-leave-active {
    transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
    opacity: 0;
}

.modal-enter-active .relative,
.modal-leave-active .relative {
    transition: all 0.3s ease;
}

.modal-enter-from .relative {
    transform: scale(0.95) translateY(-20px);
}

.modal-leave-to .relative {
    transform: scale(0.95) translateY(-20px);
}

kbd {
    font-family: ui-monospace, monospace;
    font-size: 0.75rem;
}
</style>
