<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center gap-4 mb-6">
                <NuxtLink
                    to="/equipment"
                    class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                >
                    <Icon name="ri:arrow-left-line" class="text-xl" />
                </NuxtLink>
                <div class="flex-1 flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <div
                            :class="[
                                'h-16 w-16 rounded-2xl flex items-center justify-center shadow-lg',
                                getTypeGradient(equipment.type),
                            ]"
                        >
                            <Icon
                                :name="getTypeIcon(equipment.type)"
                                class="text-3xl text-white"
                            />
                        </div>
                        <div>
                            <div class="flex items-center gap-3">
                                <h1 class="text-3xl font-bold text-gray-900">
                                    {{ equipment.name }}
                                </h1>
                                <span
                                    :class="[
                                        'badge',
                                        getStatusColor(equipment.status),
                                    ]"
                                >
                                    {{ getStatusLabel(equipment.status) }}
                                </span>
                            </div>
                            <p class="mt-1 text-gray-600">
                                {{ equipment.inventory_number }}
                            </p>
                        </div>
                    </div>

                    <div class="flex items-center gap-2">
                        <NuxtLink
                            :to="`/equipment/${equipment.id}/edit`"
                            class="btn btn-secondary"
                        >
                            <Icon name="ri:edit-line" class="mr-2" />
                            Редактировать
                        </NuxtLink>
                        <button
                            @click="showDeleteModal = true"
                            class="btn bg-red-50 text-red-600 hover:bg-red-100"
                        >
                            <Icon name="ri:delete-bin-line" class="mr-2" />
                            Удалить
                        </button>
                    </div>
                </div>
            </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <div class="lg:col-span-2 space-y-6">
                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <h2 class="text-lg font-semibold text-gray-900">
                            Общая информация
                        </h2>
                    </div>
                    <div class="p-6">
                        <div class="grid grid-cols-2 gap-6">
                            <div>
                                <p class="text-xs text-gray-600 mb-1">
                                    Производитель
                                </p>
                                <p class="text-sm font-medium text-gray-900">
                                    {{ equipment.manufacturer }}
                                </p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">Модель</p>
                                <p class="text-sm font-medium text-gray-900">
                                    {{ equipment.model }}
                                </p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">
                                    Серийный номер
                                </p>
                                <p
                                    class="text-sm font-medium text-gray-900 font-mono"
                                >
                                    {{ equipment.serial_number }}
                                </p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">Тип</p>
                                <span class="badge bg-blue-100 text-blue-800">
                                    {{ getTypeLabel(equipment.type) }}
                                </span>
                            </div>
                            <div v-if="equipment.location">
                                <p class="text-xs text-gray-600 mb-1">
                                    Местоположение
                                </p>
                                <div class="flex items-center gap-2">
                                    <Icon
                                        name="ri:map-pin-line"
                                        class="text-gray-400"
                                    />
                                    <p
                                        class="text-sm font-medium text-gray-900"
                                    >
                                        {{ equipment.location }}
                                    </p>
                                </div>
                            </div>
                            <div v-if="equipment.ip_address">
                                <p class="text-xs text-gray-600 mb-1">
                                    IP-адрес
                                </p>
                                <p
                                    class="text-sm font-medium text-gray-900 font-mono"
                                >
                                    {{ equipment.ip_address }}
                                </p>
                            </div>
                        </div>
                        <div
                            v-if="equipment.notes"
                            class="mt-6 pt-6 border-t border-gray-200"
                        >
                            <p class="text-xs text-gray-600 mb-2">Примечания</p>
                            <p class="text-sm text-gray-700">
                                {{ equipment.notes }}
                            </p>
                        </div>
                    </div>
                </div>

                <div v-if="equipment.employee" class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <h2 class="text-lg font-semibold text-gray-900">
                            Назначение
                        </h2>
                    </div>
                    <div class="p-6">
                        <NuxtLink
                            :to="`/employees/${equipment.employee.id}`"
                            class="flex items-center gap-4 p-4 rounded-lg bg-blue-50 border border-blue-100 hover:bg-blue-100 transition-colors"
                        >
                            <div
                                class="h-12 w-12 rounded-full bg-blue-500 flex items-center justify-center"
                            >
                                <Icon
                                    name="ri:user-line"
                                    class="text-xl text-white"
                                />
                            </div>
                            <div>
                                <p class="text-sm font-medium text-gray-900">
                                    {{ equipment.employee.name }}
                                </p>
                                <p class="text-xs text-gray-600">
                                    {{ equipment.employee.position }}
                                </p>
                            </div>
                        </NuxtLink>
                    </div>
                </div>

                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <h2 class="text-lg font-semibold text-gray-900">
                            История обслуживания
                        </h2>
                    </div>
                    <div class="p-6">
                        <div class="space-y-4">
                            <div
                                v-for="(event, index) in history"
                                :key="event.id"
                                class="flex gap-4"
                            >
                                <div class="flex flex-col items-center">
                                    <div
                                        :class="[
                                            'h-8 w-8 rounded-full flex items-center justify-center',
                                            getEventColor(event.type),
                                        ]"
                                    >
                                        <Icon
                                            :name="getEventIcon(event.type)"
                                            class="text-sm text-white"
                                        />
                                    </div>
                                    <div
                                        v-if="index < history.length - 1"
                                        class="w-0.5 flex-1 bg-gray-200 mt-2"
                                    />
                                </div>
                                <div class="flex-1 pb-4">
                                    <p class="text-sm text-gray-900">
                                        <span class="font-semibold">{{
                                            event.user
                                        }}</span>
                                        {{ event.action }}
                                    </p>
                                    <p
                                        v-if="event.details"
                                        class="text-sm text-gray-600 mt-1"
                                    >
                                        {{ event.details }}
                                    </p>
                                    <p class="text-xs text-gray-500 mt-1">
                                        {{ event.created_at }}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="space-y-6">
                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <h3 class="text-sm font-semibold text-gray-900">
                            Статус
                        </h3>
                    </div>
                    <div class="p-6">
                        <select
                            v-model="equipment.status"
                            @change="updateStatus"
                            class="input-field"
                        >
                            <option value="active">Активное</option>
                            <option value="inactive">Неактивное</option>
                            <option value="repair">В ремонте</option>
                            <option value="storage">На складе</option>
                            <option value="decommissioned">Списано</option>
                        </select>
                    </div>
                </div>

                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <h3 class="text-sm font-semibold text-gray-900">
                            Гарантия
                        </h3>
                    </div>
                    <div class="p-6 space-y-4">
                        <div>
                            <p class="text-xs text-gray-600 mb-1">
                                Дата покупки
                            </p>
                            <p class="text-sm font-medium text-gray-900">
                                {{ equipment.purchase_date || "-" }}
                            </p>
                        </div>
                        <div>
                            <p class="text-xs text-gray-600 mb-1">
                                Гарантия до
                            </p>
                            <div class="flex items-center gap-2">
                                <p class="text-sm font-medium text-gray-900">
                                    {{ equipment.warranty_end_date || "-" }}
                                </p>
                                <span
                                    v-if="equipment.warranty_active"
                                    class="badge bg-green-100 text-green-800 text-xs"
                                >
                                    Активна
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <h3 class="text-sm font-semibold text-gray-900">
                            Временные метки
                        </h3>
                    </div>
                    <div class="p-6 space-y-3">
                        <div class="flex items-center gap-2 text-sm">
                            <Icon name="ri:time-line" class="text-gray-600" />
                            <div>
                                <p class="text-xs text-gray-600">Добавлено</p>
                                <p class="text-gray-900">
                                    {{ equipment.created_at }}
                                </p>
                            </div>
                        </div>
                        <div class="flex items-center gap-2 text-sm">
                            <Icon
                                name="ri:refresh-line"
                                class="text-gray-600"
                            />
                            <div>
                                <p class="text-xs text-gray-600">Обновлено</p>
                                <p class="text-gray-900">
                                    {{ equipment.updated_at }}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <Teleport to="body">
            <Transition name="modal">
                <div
                    v-if="showDeleteModal"
                    class="fixed inset-0 z-[60] overflow-y-auto"
                >
                    <div
                        class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm"
                        @click="showDeleteModal = false"
                    />
                    <div
                        class="relative min-h-screen flex items-center justify-center p-4"
                    >
                        <div
                            class="relative bg-white rounded-2xl shadow-2xl max-w-md w-full p-6"
                        >
                            <div class="text-center">
                                <div
                                    class="h-16 w-16 rounded-full bg-red-100 flex items-center justify-center mx-auto mb-4"
                                >
                                    <Icon
                                        name="ri:delete-bin-line"
                                        class="text-3xl text-red-600"
                                    />
                                </div>
                                <h3
                                    class="text-lg font-semibold text-gray-900 mb-2"
                                >
                                    Удалить оборудование?
                                </h3>
                                <p class="text-sm text-gray-600 mb-6">
                                    Это действие нельзя отменить.
                                </p>
                                <div class="flex gap-3">
                                    <button
                                        @click="showDeleteModal = false"
                                        class="flex-1 btn btn-secondary"
                                    >
                                        Отмена
                                    </button>
                                    <button
                                        @click="deleteEquipment"
                                        class="flex-1 btn bg-red-600 text-white hover:bg-red-700"
                                    >
                                        Удалить
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </Transition>
        </Teleport>
    </div>
</template>

<script setup lang="ts">
definePageMeta({ middleware: "auth" });

const route = useRoute();
const router = useRouter();
const toast = useToast();

const showDeleteModal = ref(false);

const equipment = ref({
    id: route.params.id,
    name: "HP LaserJet Pro M404dn",
    inventory_number: "EQ-2024-001",
    type: "printer",
    manufacturer: "HP",
    model: "M404dn",
    serial_number: "SN987654321",
    status: "active",
    location: "Офис 301, 3 этаж",
    ip_address: "192.168.1.50",
    purchase_date: "15.01.2024",
    warranty_end_date: "15.01.2027",
    warranty_active: true,
    notes: "Основной принтер для отдела",
    employee: {
        id: 1,
        name: "Иванов Иван Иванович",
        position: "Системный администратор",
    },
    created_at: "15.01.2024 10:00",
    updated_at: "10.12.2025 14:30",
});

const history = ref([
    {
        id: 1,
        type: "created",
        user: "Администратор",
        action: "добавил оборудование",
        details: null,
        created_at: "15.01.2024 10:00",
    },
    {
        id: 2,
        type: "assigned",
        user: "Администратор",
        action: "назначил оборудование",
        details: "Иванов И.И.",
        created_at: "15.01.2024 10:15",
    },
    {
        id: 3,
        type: "maintenance",
        user: "Техник",
        action: "провел обслуживание",
        details: "Замена картриджа",
        created_at: "20.06.2024 14:00",
    },
]);

useHead({
    title: equipment.value.name,
});

const updateStatus = () => {
    toast.success(
        `Статус изменен на "${getStatusLabel(equipment.value.status)}"`,
    );
};

const deleteEquipment = () => {
    toast.success("Оборудование удалено");
    router.push("/equipment");
};

const getTypeIcon = (type: string) => {
    const icons: Record<string, string> = {
        printer: "ri:printer-line",
        scanner: "ri:scan-line",
        mfu: "ri:printer-line",
        network: "ri:router-line",
        ups: "ri:battery-charge-line",
    };
    return icons[type] || "ri:device-line";
};

const getTypeGradient = (type: string) => {
    const gradients: Record<string, string> = {
        printer: "bg-gradient-to-br from-blue-500 to-blue-700",
        scanner: "bg-gradient-to-br from-purple-500 to-purple-700",
        mfu: "bg-gradient-to-br from-indigo-500 to-indigo-700",
        network: "bg-gradient-to-br from-cyan-500 to-cyan-700",
        ups: "bg-gradient-to-br from-green-500 to-green-700",
    };
    return gradients[type] || "bg-gradient-to-br from-gray-500 to-gray-700";
};

const getTypeLabel = (type: string) => {
    const labels: Record<string, string> = {
        printer: "Принтер",
        scanner: "Сканер",
        mfu: "МФУ",
        network: "Сетевое оборудование",
        ups: "ИБП",
    };
    return labels[type] || type;
};

const getStatusColor = (status: string) => {
    const colors: Record<string, string> = {
        active: "bg-green-100 text-green-800",
        inactive: "bg-gray-100 text-gray-800",
        repair: "bg-amber-100 text-amber-800",
        storage: "bg-blue-100 text-blue-800",
        decommissioned: "bg-red-100 text-red-800",
    };
    return colors[status] || "bg-gray-100 text-gray-800";
};

const getStatusLabel = (status: string) => {
    const labels: Record<string, string> = {
        active: "Активное",
        inactive: "Неактивное",
        repair: "В ремонте",
        storage: "На складе",
        decommissioned: "Списано",
    };
    return labels[status] || status;
};

const getEventIcon = (type: string) => {
    const icons: Record<string, string> = {
        created: "ri:add-line",
        assigned: "ri:user-add-line",
        maintenance: "ri:tools-line",
        status_change: "ri:refresh-line",
    };
    return icons[type] || "ri:record-circle-line";
};

const getEventColor = (type: string) => {
    const colors: Record<string, string> = {
        created: "bg-blue-500",
        assigned: "bg-green-500",
        maintenance: "bg-amber-500",
        status_change: "bg-purple-500",
    };
    return colors[type] || "bg-gray-500";
};
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
</style>
