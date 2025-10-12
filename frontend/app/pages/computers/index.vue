<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl bg-gradient-to-br from-blue-500 to-blue-700 flex items-center justify-center shadow-lg"
                    >
                        <Icon
                            name="ri:computer-line"
                            class="text-3xl text-white"
                        />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Компьютеры
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Управление компьютерным парком
                        </p>
                    </div>
                </div>
                <div class="flex items-center gap-2">
                    <button @click="toggleView" class="btn btn-secondary">
                        <Icon
                            :name="
                                viewMode === 'grid'
                                    ? 'ri:list-check'
                                    : 'ri:grid-line'
                            "
                            class="mr-2"
                        />
                        {{ viewMode === "grid" ? "Список" : "Сетка" }}
                    </button>
                    <NuxtLink to="/computers/create" class="btn btn-primary">
                        <Icon name="ri:add-line" class="mr-2" />
                        Добавить компьютер
                    </NuxtLink>
                </div>
            </div>
        </div>

        <!-- Статистика -->
        <div class="card">
            <div class="p-4">
                <div class="grid grid-cols-5 gap-4">
                    <div
                        class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors"
                        @click="filterByStatus('active')"
                    >
                        <div class="flex items-center gap-3">
                            <div
                                class="h-10 w-10 rounded-lg bg-green-100 flex items-center justify-center flex-shrink-0"
                            >
                                <Icon
                                    name="ri:checkbox-circle-line"
                                    class="text-xl text-green-600"
                                />
                            </div>
                            <div>
                                <p class="text-xs text-gray-600">Активные</p>
                                <p class="text-xl font-bold text-green-600">
                                    {{ stats.active }}
                                </p>
                            </div>
                        </div>
                    </div>

                    <div
                        class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors"
                        @click="filterByStatus('repair')"
                    >
                        <div class="flex items-center gap-3">
                            <div
                                class="h-10 w-10 rounded-lg bg-amber-100 flex items-center justify-center flex-shrink-0"
                            >
                                <Icon
                                    name="ri:tools-line"
                                    class="text-xl text-amber-600"
                                />
                            </div>
                            <div>
                                <p class="text-xs text-gray-600">В ремонте</p>
                                <p class="text-xl font-bold text-amber-600">
                                    {{ stats.repair }}
                                </p>
                            </div>
                        </div>
                    </div>

                    <div
                        class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors"
                        @click="filterByStatus('storage')"
                    >
                        <div class="flex items-center gap-3">
                            <div
                                class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center flex-shrink-0"
                            >
                                <Icon
                                    name="ri:archive-line"
                                    class="text-xl text-blue-600"
                                />
                            </div>
                            <div>
                                <p class="text-xs text-gray-600">На складе</p>
                                <p class="text-xl font-bold text-blue-600">
                                    {{ stats.storage }}
                                </p>
                            </div>
                        </div>
                    </div>

                    <div
                        class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors"
                        @click="filterByStatus('inactive')"
                    >
                        <div class="flex items-center gap-3">
                            <div
                                class="h-10 w-10 rounded-lg bg-gray-100 flex items-center justify-center flex-shrink-0"
                            >
                                <Icon
                                    name="ri:pause-circle-line"
                                    class="text-xl text-gray-600"
                                />
                            </div>
                            <div>
                                <p class="text-xs text-gray-600">Неактивные</p>
                                <p class="text-xl font-bold text-gray-600">
                                    {{ stats.inactive }}
                                </p>
                            </div>
                        </div>
                    </div>

                    <div
                        class="cursor-pointer hover:bg-gray-50 rounded-lg p-3 transition-colors"
                    >
                        <div class="flex items-center gap-3">
                            <div
                                class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center flex-shrink-0"
                            >
                                <Icon
                                    name="ri:database-line"
                                    class="text-xl text-purple-600"
                                />
                            </div>
                            <div>
                                <p class="text-xs text-gray-600">Всего</p>
                                <p class="text-xl font-bold text-purple-600">
                                    {{ stats.total }}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Фильтры и поиск -->
        <div class="card">
            <div class="p-4 bg-gradient-to-r from-gray-50 to-white">
                <div class="flex flex-wrap items-center gap-4">
                    <div class="flex-1 min-w-[300px] relative">
                        <Icon
                            name="ri:search-line"
                            class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                        />
                        <input
                            v-model="searchQuery"
                            type="text"
                            placeholder="Поиск по имени, инвентарному номеру, сотруднику..."
                            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                        />
                    </div>

                    <select v-model="selectedStatus" class="input-field w-40">
                        <option value="">Все статусы</option>
                        <option value="active">Активные</option>
                        <option value="inactive">Неактивные</option>
                        <option value="repair">В ремонте</option>
                        <option value="storage">На складе</option>
                        <option value="decommissioned">Списаны</option>
                    </select>

                    <select v-model="selectedType" class="input-field w-40">
                        <option value="">Все типы</option>
                        <option value="desktop">Десктоп</option>
                        <option value="laptop">Ноутбук</option>
                        <option value="server">Сервер</option>
                        <option value="workstation">Рабочая станция</option>
                        <option value="thin_client">Тонкий клиент</option>
                    </select>

                    <button @click="resetFilters" class="btn btn-secondary">
                        <Icon name="ri:refresh-line" class="mr-2" />
                        Сбросить
                    </button>
                </div>
            </div>
        </div>

        <!-- Список компьютеров (таблица) -->
        <div v-if="viewMode === 'list'" class="card">
            <div class="overflow-x-auto">
                <table class="min-w-full divide-y divide-gray-200">
                    <thead class="bg-gray-50">
                        <tr>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Компьютер
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Тип
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Сотрудник
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                ОС
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Характеристики
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Статус
                            </th>
                            <th
                                class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase"
                            >
                                Действия
                            </th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-gray-200">
                        <tr
                            v-for="computer in filteredComputers"
                            :key="computer.id"
                            class="hover:bg-gray-50 transition-colors"
                        >
                            <td class="px-4 py-4">
                                <div class="flex items-center gap-3">
                                    <div
                                        :class="[
                                            'h-10 w-10 rounded-lg flex items-center justify-center',
                                            getTypeColor(
                                                computer.computer_type,
                                            ),
                                        ]"
                                    >
                                        <Icon
                                            :name="
                                                getTypeIcon(
                                                    computer.computer_type,
                                                )
                                            "
                                            class="text-xl text-white"
                                        />
                                    </div>
                                    <div>
                                        <p
                                            class="text-sm font-medium text-gray-900"
                                        >
                                            {{ computer.hostname }}
                                        </p>
                                        <p class="text-xs text-gray-500">
                                            {{ computer.inventory_number }}
                                        </p>
                                    </div>
                                </div>
                            </td>
                            <td class="px-4 py-4">
                                <span class="badge bg-blue-100 text-blue-800">
                                    {{ getTypeLabel(computer.computer_type) }}
                                </span>
                            </td>
                            <td class="px-4 py-4">
                                <div
                                    v-if="computer.employee"
                                    class="flex items-center gap-2"
                                >
                                    <Icon
                                        name="ri:user-line"
                                        class="text-gray-400"
                                    />
                                    <span class="text-sm text-gray-900">{{
                                        computer.employee
                                    }}</span>
                                </div>
                                <span v-else class="text-sm text-gray-400"
                                    >Не назначен</span
                                >
                            </td>
                            <td class="px-4 py-4">
                                <div class="flex items-center gap-2">
                                    <Icon
                                        :name="getOSIcon(computer.os)"
                                        class="text-gray-600"
                                    />
                                    <div>
                                        <p class="text-sm text-gray-900">
                                            {{ computer.os }}
                                        </p>
                                        <p class="text-xs text-gray-500">
                                            {{ computer.os_version }}
                                        </p>
                                    </div>
                                </div>
                            </td>
                            <td class="px-4 py-4">
                                <div class="text-xs text-gray-600 space-y-1">
                                    <div class="flex items-center gap-1">
                                        <Icon
                                            name="ri:cpu-line"
                                            class="text-gray-400"
                                        />
                                        <span>{{ computer.cpu }}</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <Icon
                                            name="ri:database-2-line"
                                            class="text-gray-400"
                                        />
                                        <span>{{ computer.ram }} ГБ RAM</span>
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <Icon
                                            name="ri:hard-drive-line"
                                            class="text-gray-400"
                                        />
                                        <span>{{ computer.storage }} ГБ</span>
                                    </div>
                                </div>
                            </td>
                            <td class="px-4 py-4">
                                <span
                                    :class="[
                                        'badge',
                                        getStatusColor(computer.status),
                                    ]"
                                >
                                    {{ getStatusLabel(computer.status) }}
                                </span>
                            </td>
                            <td class="px-4 py-4 text-right">
                                <div
                                    class="flex items-center justify-end gap-2"
                                >
                                    <NuxtLink
                                        :to="`/computers/${computer.id}`"
                                        class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                                        title="Просмотр"
                                    >
                                        <Icon
                                            name="ri:eye-line"
                                            class="text-gray-600"
                                        />
                                    </NuxtLink>
                                    <NuxtLink
                                        :to="`/computers/${computer.id}/edit`"
                                        class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                                        title="Редактировать"
                                    >
                                        <Icon
                                            name="ri:edit-line"
                                            class="text-gray-600"
                                        />
                                    </NuxtLink>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>

                <div
                    v-if="filteredComputers.length === 0"
                    class="p-16 text-center"
                >
                    <div
                        class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mx-auto mb-4"
                    >
                        <Icon
                            name="ri:computer-line"
                            class="text-4xl text-gray-400"
                        />
                    </div>
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">
                        Компьютеры не найдены
                    </h3>
                    <p class="text-sm text-gray-500 mb-4">
                        Попробуйте изменить параметры поиска
                    </p>
                </div>
            </div>
        </div>

        <!-- Сетка компьютеров -->
        <div
            v-else
            class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4"
        >
            <NuxtLink
                v-for="computer in filteredComputers"
                :key="computer.id"
                :to="`/computers/${computer.id}`"
                class="card card-hover group"
            >
                <div class="p-5">
                    <div class="flex items-start justify-between mb-4">
                        <div
                            :class="[
                                'h-12 w-12 rounded-xl flex items-center justify-center',
                                getTypeColor(computer.computer_type),
                            ]"
                        >
                            <Icon
                                :name="getTypeIcon(computer.computer_type)"
                                class="text-2xl text-white"
                            />
                        </div>
                        <span
                            :class="['badge', getStatusColor(computer.status)]"
                        >
                            {{ getStatusLabel(computer.status) }}
                        </span>
                    </div>

                    <h3
                        class="text-lg font-semibold text-gray-900 mb-1 group-hover:text-primary-600 transition-colors"
                    >
                        {{ computer.hostname }}
                    </h3>
                    <p class="text-sm text-gray-500 mb-3">
                        {{ computer.inventory_number }}
                    </p>

                    <div class="space-y-2 mb-4">
                        <div
                            class="flex items-center gap-2 text-sm text-gray-600"
                        >
                            <Icon
                                :name="getOSIcon(computer.os)"
                                class="text-gray-400"
                            />
                            <span
                                >{{ computer.os }}
                                {{ computer.os_version }}</span
                            >
                        </div>
                        <div
                            v-if="computer.employee"
                            class="flex items-center gap-2 text-sm text-gray-600"
                        >
                            <Icon name="ri:user-line" class="text-gray-400" />
                            <span>{{ computer.employee }}</span>
                        </div>
                    </div>

                    <div class="pt-3 border-t border-gray-200 space-y-1">
                        <div
                            class="flex items-center justify-between text-xs text-gray-600"
                        >
                            <span class="flex items-center gap-1">
                                <Icon name="ri:cpu-line" />
                                CPU
                            </span>
                            <span class="font-medium">{{ computer.cpu }}</span>
                        </div>
                        <div
                            class="flex items-center justify-between text-xs text-gray-600"
                        >
                            <span class="flex items-center gap-1">
                                <Icon name="ri:database-2-line" />
                                RAM
                            </span>
                            <span class="font-medium"
                                >{{ computer.ram }} ГБ</span
                            >
                        </div>
                        <div
                            class="flex items-center justify-between text-xs text-gray-600"
                        >
                            <span class="flex items-center gap-1">
                                <Icon name="ri:hard-drive-line" />
                                Storage
                            </span>
                            <span class="font-medium"
                                >{{ computer.storage }} ГБ</span
                            >
                        </div>
                    </div>
                </div>
            </NuxtLink>

            <div
                v-if="filteredComputers.length === 0"
                class="col-span-full p-16 text-center"
            >
                <div
                    class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mx-auto mb-4"
                >
                    <Icon
                        name="ri:computer-line"
                        class="text-4xl text-gray-400"
                    />
                </div>
                <h3 class="text-lg font-semibold text-gray-900 mb-2">
                    Компьютеры не найдены
                </h3>
                <p class="text-sm text-gray-500 mb-4">
                    Попробуйте изменить параметры поиска
                </p>
            </div>
        </div>
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
    title: "Компьютеры",
});

const searchQuery = ref("");
const selectedStatus = ref("");
const selectedType = ref("");
const viewMode = ref<"list" | "grid">("list");

// Мок данные
const computers = ref([
    {
        id: 1,
        hostname: "PC-ADMIN-001",
        inventory_number: "INV-2024-001",
        computer_type: "desktop",
        manufacturer: "HP",
        model: "EliteDesk 800 G6",
        serial_number: "SN123456",
        os: "Windows 11 Pro",
        os_version: "23H2",
        employee: "Иванов И.И.",
        cpu: "Intel Core i7-10700",
        ram: 16,
        storage: 512,
        status: "active",
    },
    {
        id: 2,
        hostname: "LAPTOP-USER-002",
        inventory_number: "INV-2024-002",
        computer_type: "laptop",
        manufacturer: "Dell",
        model: "Latitude 5420",
        serial_number: "SN123457",
        os: "Windows 10 Pro",
        os_version: "22H2",
        employee: "Петров П.П.",
        cpu: "Intel Core i5-1135G7",
        ram: 8,
        storage: 256,
        status: "active",
    },
    {
        id: 3,
        hostname: "SERVER-DC-001",
        inventory_number: "INV-2024-003",
        computer_type: "server",
        manufacturer: "Dell",
        model: "PowerEdge R740",
        serial_number: "SN123458",
        os: "Windows Server 2022",
        os_version: "Standard",
        employee: null,
        cpu: "Intel Xeon Gold 6230",
        ram: 64,
        storage: 2000,
        status: "active",
    },
    {
        id: 4,
        hostname: "PC-REPAIR-004",
        inventory_number: "INV-2024-004",
        computer_type: "desktop",
        manufacturer: "Lenovo",
        model: "ThinkCentre M720",
        serial_number: "SN123459",
        os: "Windows 10 Pro",
        os_version: "21H2",
        employee: null,
        cpu: "Intel Core i3-9100",
        ram: 8,
        storage: 256,
        status: "repair",
    },
    {
        id: 5,
        hostname: "LAPTOP-STORAGE-005",
        inventory_number: "INV-2024-005",
        computer_type: "laptop",
        manufacturer: "HP",
        model: "ProBook 450 G8",
        serial_number: "SN123460",
        os: "Windows 11 Pro",
        os_version: "22H2",
        employee: null,
        cpu: "Intel Core i5-1135G7",
        ram: 16,
        storage: 512,
        status: "storage",
    },
]);

const stats = computed(() => ({
    active: computers.value.filter((c) => c.status === "active").length,
    repair: computers.value.filter((c) => c.status === "repair").length,
    storage: computers.value.filter((c) => c.status === "storage").length,
    inactive: computers.value.filter((c) => c.status === "inactive").length,
    total: computers.value.length,
}));

const filteredComputers = computed(() => {
    let filtered = computers.value;

    if (selectedStatus.value) {
        filtered = filtered.filter((c) => c.status === selectedStatus.value);
    }

    if (selectedType.value) {
        filtered = filtered.filter(
            (c) => c.computer_type === selectedType.value,
        );
    }

    if (searchQuery.value) {
        const query = searchQuery.value.toLowerCase();
        filtered = filtered.filter(
            (c) =>
                c.hostname.toLowerCase().includes(query) ||
                c.inventory_number.toLowerCase().includes(query) ||
                c.employee?.toLowerCase().includes(query) ||
                c.manufacturer.toLowerCase().includes(query) ||
                c.model.toLowerCase().includes(query),
        );
    }

    return filtered;
});

const toggleView = () => {
    viewMode.value = viewMode.value === "list" ? "grid" : "list";
};

const filterByStatus = (status: string) => {
    selectedStatus.value = selectedStatus.value === status ? "" : status;
};

const resetFilters = () => {
    searchQuery.value = "";
    selectedStatus.value = "";
    selectedType.value = "";
};

const getTypeIcon = (type: string) => {
    const icons: Record<string, string> = {
        desktop: "ri:computer-line",
        laptop: "ri:macbook-line",
        server: "ri:server-line",
        workstation: "ri:dashboard-line",
        thin_client: "ri:tv-line",
    };
    return icons[type] || "ri:computer-line";
};

const getTypeColor = (type: string) => {
    const colors: Record<string, string> = {
        desktop: "bg-blue-500",
        laptop: "bg-purple-500",
        server: "bg-red-500",
        workstation: "bg-green-500",
        thin_client: "bg-gray-500",
    };
    return colors[type] || "bg-gray-500";
};

const getTypeLabel = (type: string) => {
    const labels: Record<string, string> = {
        desktop: "Десктоп",
        laptop: "Ноутбук",
        server: "Сервер",
        workstation: "Рабочая станция",
        thin_client: "Тонкий клиент",
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
        active: "Активный",
        inactive: "Неактивный",
        repair: "В ремонте",
        storage: "На складе",
        decommissioned: "Списан",
    };
    return labels[status] || status;
};

const getOSIcon = (os: string) => {
    if (os.includes("Windows")) return "ri:windows-line";
    if (os.includes("Linux")) return "ri:ubuntu-line";
    if (os.includes("macOS")) return "ri:apple-line";
    return "ri:computer-line";
};
</script>
