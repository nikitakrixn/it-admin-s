<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl bg-gradient-to-br from-indigo-500 to-indigo-700 flex items-center justify-center shadow-lg"
                    >
                        <Icon
                            name="ri:history-line"
                            class="text-3xl text-white"
                        />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Журнал активности
                        </h1>
                        <p class="mt-1 text-gray-600">
                            История действий в системе
                        </p>
                    </div>
                </div>
                <div class="flex items-center gap-2">
                    <button @click="refresh" class="btn btn-secondary">
                        <Icon name="ri:refresh-line" class="mr-2" />
                        Обновить
                    </button>
                    <button @click="exportLogs" class="btn btn-primary">
                        <Icon name="ri:download-line" class="mr-2" />
                        Экспорт
                    </button>
                </div>
            </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
            <div
                class="card cursor-pointer hover:shadow-md transition-shadow"
                @click="filterByAction('create')"
            >
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Создано</p>
                            <p class="text-2xl font-bold text-green-600">
                                {{ createCount }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:add-circle-line"
                                class="text-2xl text-green-600"
                            />
                        </div>
                    </div>
                </div>
            </div>

            <div
                class="card cursor-pointer hover:shadow-md transition-shadow"
                @click="filterByAction('update')"
            >
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Изменено</p>
                            <p class="text-2xl font-bold text-blue-600">
                                {{ updateCount }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:edit-line"
                                class="text-2xl text-blue-600"
                            />
                        </div>
                    </div>
                </div>
            </div>

            <div
                class="card cursor-pointer hover:shadow-md transition-shadow"
                @click="filterByAction('delete')"
            >
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Удалено</p>
                            <p class="text-2xl font-bold text-red-600">
                                {{ deleteCount }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-red-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:delete-bin-line"
                                class="text-2xl text-red-600"
                            />
                        </div>
                    </div>
                </div>
            </div>

            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">
                                Всего записей
                            </p>
                            <p class="text-2xl font-bold text-gray-900">
                                {{ data?.total || 0 }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:file-list-line"
                                class="text-2xl text-purple-600"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="card">
            <div
                class="p-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
            >
                <div class="flex items-center gap-4">
                    <div class="flex-1 relative">
                        <Icon
                            name="ri:search-line"
                            class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                        />
                        <input
                            v-model="searchQuery"
                            type="text"
                            placeholder="Поиск в журнале..."
                            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                        />
                    </div>
                    <select
                        v-model="filters.action_type"
                        @change="loadLogs"
                        class="input-field w-48"
                    >
                        <option value="">Все действия</option>
                        <option value="create">Создание</option>
                        <option value="update">Изменение</option>
                        <option value="delete">Удаление</option>
                    </select>
                    <select
                        v-model="filters.entity_type"
                        @change="loadLogs"
                        class="input-field w-48"
                    >
                        <option value="">Все объекты</option>
                        <option value="employee">Сотрудники</option>
                        <option value="computer">Компьютеры</option>
                        <option value="equipment">Оборудование</option>
                    </select>
                    <button
                        v-if="filters.action_type || filters.entity_type"
                        @click="resetFilters"
                        class="btn btn-secondary"
                    >
                        <Icon name="ri:refresh-line" />
                    </button>
                </div>
            </div>

            <div v-if="pending" class="flex justify-center py-12">
                <Icon
                    name="ri:loader-4-line"
                    class="text-4xl text-primary-600 animate-spin"
                />
            </div>

            <div v-else-if="error" class="p-4 bg-red-50 text-red-800 text-sm">
                <Icon name="ri:error-warning-line" class="inline mr-2" />
                {{ error }}
            </div>

            <div v-else class="divide-y divide-gray-200">
                <div
                    v-for="log in logs"
                    :key="log.id"
                    class="p-4 hover:bg-gray-50 transition-colors cursor-pointer group"
                    @click="openDetailModal(log)"
                >
                    <div class="flex items-start gap-4">
                        <div
                            :class="[
                                'flex h-12 w-12 items-center justify-center rounded-xl flex-shrink-0 group-hover:scale-110 transition-transform',
                                getActionColor(log.action),
                            ]"
                        >
                            <Icon
                                :name="getActionIcon(log.action)"
                                class="text-xl text-white"
                            />
                        </div>
                        <div class="flex-1 min-w-0">
                            <div class="flex items-start justify-between gap-4">
                                <div class="flex-1">
                                    <p
                                        class="text-sm font-medium text-gray-900"
                                    >
                                        {{ getActionDescription(log) }}
                                    </p>
                                    <div
                                        class="mt-2 flex items-center gap-4 text-xs text-gray-500"
                                    >
                                        <span class="flex items-center gap-1">
                                            <Icon name="ri:user-line" />
                                            {{ log.user_email || "Система" }}
                                        </span>
                                        <span class="flex items-center gap-1">
                                            <Icon name="ri:time-line" />
                                            {{ formatDate(log.created_at) }}
                                        </span>
                                        <span
                                            :class="[
                                                'badge',
                                                getEntityColor(log.entity_type),
                                            ]"
                                        >
                                            {{
                                                getEntityLabel(log.entity_type)
                                            }}
                                        </span>
                                    </div>
                                </div>
                                <Icon
                                    name="ri:arrow-right-s-line"
                                    class="text-gray-400 group-hover:text-primary-600 transition-colors"
                                />
                            </div>
                        </div>
                    </div>
                </div>

                <div v-if="!logs.length" class="p-16 text-center">
                    <div
                        class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mx-auto mb-4"
                    >
                        <Icon
                            name="ri:file-list-line"
                            class="text-4xl text-gray-400"
                        />
                    </div>
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">
                        Записи не найдены
                    </h3>
                    <p class="text-sm text-gray-500 mb-4">
                        Попробуйте изменить параметры фильтров
                    </p>
                    <button @click="resetFilters" class="btn btn-secondary">
                        <Icon name="ri:refresh-line" class="mr-2" />
                        Сбросить фильтры
                    </button>
                </div>
            </div>

            <div
                v-if="data && data.total > 0"
                class="px-6 py-4 border-t border-gray-200 flex items-center justify-between bg-gray-50"
            >
                <p class="text-sm text-gray-700">
                    Показано
                    <span class="font-medium">{{
                        Math.min((currentPage - 1) * perPage + 1, data.total)
                    }}</span>
                    -
                    <span class="font-medium">{{
                        Math.min(currentPage * perPage, data.total)
                    }}</span>
                    из
                    <span class="font-medium">{{ data.total }}</span>
                </p>
                <div class="flex gap-2">
                    <button
                        @click="prevPage"
                        :disabled="currentPage === 1"
                        class="btn btn-secondary disabled:opacity-50"
                    >
                        <Icon name="ri:arrow-left-s-line" />
                    </button>
                    <span class="px-4 py-2 text-sm text-gray-700">
                        {{ currentPage }} / {{ totalPages }}
                    </span>
                    <button
                        @click="nextPage"
                        :disabled="currentPage >= totalPages"
                        class="btn btn-secondary disabled:opacity-50"
                    >
                        <Icon name="ri:arrow-right-s-line" />
                    </button>
                </div>
            </div>
        </div>

        <ActivityLogDetailModal
            :show="showDetailModal"
            :log="selectedLog"
            @close="closeDetailModal"
        />
    </div>
</template>

<script setup lang="ts">
definePageMeta({
    middleware: "auth",
    pageTransition: {
        name: "slide-up",
        mode: "out-in",
    },
});

useHead({
    title: "Журнал активности",
});

const toast = useToast();
const { fetchActivityLogs } = useActivityLog();

const searchQuery = ref("");
const currentPage = ref(1);
const perPage = ref(20);
const filters = ref({
    action_type: "",
    entity_type: "",
});

const showDetailModal = ref(false);
const selectedLog = ref<any>(null);

const createCount = computed(
    () =>
        logs.value.filter(
            (l) =>
                l.action.toLowerCase() === "create" ||
                l.action.toLowerCase() === "created",
        ).length,
);
const updateCount = computed(
    () =>
        logs.value.filter(
            (l) =>
                l.action.toLowerCase() === "update" ||
                l.action.toLowerCase() === "updated",
        ).length,
);
const deleteCount = computed(
    () =>
        logs.value.filter(
            (l) =>
                l.action.toLowerCase() === "delete" ||
                l.action.toLowerCase() === "deleted",
        ).length,
);

const { data, pending, error, refresh } = await useAsyncData(
    "activity-logs",
    () =>
        fetchActivityLogs({
            page: currentPage.value,
            per_page: perPage.value,
            action: filters.value.action_type || undefined,
            entity_type: filters.value.entity_type || undefined,
        }),
    {
        watch: [currentPage],
    },
);

const logs = computed(() => data.value?.logs || []);
const totalPages = computed(() =>
    data.value ? Math.ceil(data.value.total / data.value.per_page) : 0,
);

const loadLogs = () => {
    currentPage.value = 1;
    refresh();
};

const nextPage = () => {
    if (currentPage.value < totalPages.value) {
        currentPage.value++;
    }
};

const prevPage = () => {
    if (currentPage.value > 1) {
        currentPage.value--;
    }
};

const resetFilters = () => {
    filters.value.action_type = "";
    filters.value.entity_type = "";
    loadLogs();
};

const filterByAction = (action: string) => {
    filters.value.action_type = action;
    loadLogs();
};

const openDetailModal = (log: any) => {
    selectedLog.value = log;
    showDetailModal.value = true;
};

const closeDetailModal = () => {
    showDetailModal.value = false;
    selectedLog.value = null;
};

const exportLogs = () => {
    toast.success(
        "Экспорт журнала начат. Файл будет загружен через несколько секунд.",
    );
};

const getActionIcon = (action: string) => {
    const icons: Record<string, string> = {
        create: "ri:add-circle-line",
        created: "ri:add-circle-line",
        update: "ri:edit-line",
        updated: "ri:edit-line",
        delete: "ri:delete-bin-line",
        deleted: "ri:delete-bin-line",
        downloaded_config: "ri:download-line",
        synced_all_stats: "ri:refresh-line",
        synced_interfaces: "ri:router-line",
        imported_peers: "ri:download-cloud-line",
    };
    return icons[action.toLowerCase()] || "ri:file-line";
};

const getActionColor = (action: string) => {
    const colors: Record<string, string> = {
        create: "bg-green-500",
        created: "bg-green-500",
        update: "bg-blue-500",
        updated: "bg-blue-500",
        delete: "bg-red-500",
        deleted: "bg-red-500",
        downloaded_config: "bg-purple-500",
        synced_all_stats: "bg-indigo-500",
        synced_interfaces: "bg-cyan-500",
        imported_peers: "bg-teal-500",
    };
    return colors[action.toLowerCase()] || "bg-gray-500";
};

const getEntityColor = (entity: string) => {
    const colors: Record<string, string> = {
        employee: "bg-blue-100 text-blue-800",
        computer: "bg-purple-100 text-purple-800",
        equipment: "bg-amber-100 text-amber-800",
        wireguard_peer: "bg-green-100 text-green-800",
        wireguard_interface: "bg-cyan-100 text-cyan-800",
    };
    return colors[entity] || "bg-gray-100 text-gray-800";
};

const getEntityLabel = (entity: string) => {
    const labels: Record<string, string> = {
        employee: "Сотрудник",
        computer: "Компьютер",
        equipment: "Оборудование",
        wireguard_peer: "WireGuard",
        wireguard_interface: "WireGuard",
    };
    return labels[entity] || entity;
};

const formatDate = (date: string) => {
    return new Date(date).toLocaleString("ru-RU", {
        day: "2-digit",
        month: "2-digit",
        year: "numeric",
        hour: "2-digit",
        minute: "2-digit",
    });
};

const getActionDescription = (log: any) => {
    const actionLabels: Record<string, string> = {
        create: "Создан",
        created: "Создан",
        update: "Изменен",
        updated: "Изменен",
        delete: "Удален",
        deleted: "Удален",
        downloaded_config: "Скачана конфигурация",
        synced_all_stats: "Синхронизирована статистика",
        synced_interfaces: "Синхронизированы интерфейсы",
        imported_peers: "Импортированы пиры",
    };

    const entityLabels: Record<string, string> = {
        employee: "сотрудник",
        employees: "сотрудник",
        computer: "компьютер",
        computers: "компьютер",
        equipment: "оборудование",
        request: "заявка",
        requests: "заявка",
        department: "отдел",
        departments: "отдел",
        wireguard_peer: "WireGuard пир",
        wireguard_interface: "WireGuard интерфейс",
    };

    const action = actionLabels[log.action.toLowerCase()] || log.action;
    const entity =
        entityLabels[log.entity_type.toLowerCase()] || log.entity_type;
    
    // Специальная обработка для WireGuard действий
    if (log.action === "downloaded_config") {
        return `Скачана конфигурация для пира #${log.entity_id}`;
    }
    if (log.action === "synced_all_stats") {
        const count = log.details?.synced_count || 0;
        return `Синхронизирована статистика (${count} пиров)`;
    }
    if (log.action === "synced_interfaces") {
        const count = log.details?.synced_count || 0;
        return `Синхронизированы интерфейсы (${count} шт.)`;
    }
    if (log.action === "imported_peers") {
        const count = log.details?.imported_count || 0;
        return `Импортированы пиры из MikroTik (${count} шт.)`;
    }
    
    const entityId = log.entity_id ? ` #${log.entity_id}` : "";
    return `${action} ${entity}${entityId}`;
};
</script>
