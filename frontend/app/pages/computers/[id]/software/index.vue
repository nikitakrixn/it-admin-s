<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center gap-4 mb-6">
                <NuxtLink
                    :to="`/computers/${route.params.id}`"
                    class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                >
                    <Icon name="ri:arrow-left-line" class="text-xl" />
                </NuxtLink>
                <div class="flex-1 flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <div
                            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-purple-500 to-purple-700 flex items-center justify-center shadow-lg"
                        >
                            <Icon
                                name="ri:apps-line"
                                class="text-3xl text-white"
                            />
                        </div>
                        <div>
                            <h1 class="text-3xl font-bold text-gray-900">
                                Программное обеспечение
                            </h1>
                            <p class="mt-1 text-gray-600">{{ computerName }}</p>
                        </div>
                    </div>
                    <button @click="scanSoftware" class="btn btn-primary">
                        <Icon name="ri:refresh-line" class="mr-2" />
                        Сканировать
                    </button>
                </div>
            </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Всего ПО</p>
                            <p class="text-2xl font-bold text-purple-600">
                                {{ softwareList.length }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:apps-line"
                                class="text-2xl text-purple-600"
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
                                Лицензионное
                            </p>
                            <p class="text-2xl font-bold text-green-600">
                                {{ licensedCount }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:shield-check-line"
                                class="text-2xl text-green-600"
                            />
                        </div>
                    </div>
                </div>
            </div>
            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Истекает</p>
                            <p class="text-2xl font-bold text-amber-600">
                                {{ expiringCount }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-amber-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:time-line"
                                class="text-2xl text-amber-600"
                            />
                        </div>
                    </div>
                </div>
            </div>
            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Обновлено</p>
                            <p class="text-sm font-medium text-gray-900">
                                {{ lastScan }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:refresh-line"
                                class="text-2xl text-blue-600"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="card">
            <div
                class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
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
                            placeholder="Поиск ПО..."
                            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                        />
                    </div>
                    <select v-model="selectedCategory" class="input-field w-48">
                        <option value="">Все категории</option>
                        <option value="office">Офисные</option>
                        <option value="browser">Браузеры</option>
                        <option value="development">Разработка</option>
                    </select>
                </div>
            </div>
            <div class="overflow-x-auto">
                <table class="min-w-full divide-y divide-gray-200">
                    <thead class="bg-gray-50">
                        <tr>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                ПО
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Версия
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Лицензия
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Установлено
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
                            v-for="sw in filteredSoftware"
                            :key="sw.id"
                            class="hover:bg-gray-50"
                        >
                            <td class="px-4 py-4">
                                <div class="flex items-center gap-3">
                                    <Icon
                                        name="ri:apps-2-line"
                                        class="text-gray-400"
                                    />
                                    <div>
                                        <p
                                            class="text-sm font-medium text-gray-900"
                                        >
                                            {{ sw.name }}
                                        </p>
                                        <p class="text-xs text-gray-500">
                                            {{ sw.manufacturer }}
                                        </p>
                                    </div>
                                </div>
                            </td>
                            <td class="px-4 py-4">
                                <p class="text-sm text-gray-900">
                                    {{ sw.version }}
                                </p>
                            </td>
                            <td class="px-4 py-4">
                                <span
                                    v-if="sw.license_type"
                                    :class="[
                                        'badge',
                                        getLicenseColor(sw.license_type),
                                    ]"
                                >
                                    {{ getLicenseLabel(sw.license_type) }}
                                </span>
                                <span v-else class="text-sm text-gray-400"
                                    >-</span
                                >
                            </td>
                            <td class="px-4 py-4">
                                <p class="text-sm text-gray-600">
                                    {{ sw.installation_date }}
                                </p>
                            </td>
                            <td class="px-4 py-4 text-right">
                                <button
                                    @click="removeSoftware(sw)"
                                    class="p-2 rounded-lg hover:bg-red-50 transition-colors"
                                    title="Удалить"
                                >
                                    <Icon
                                        name="ri:delete-bin-line"
                                        class="text-red-600"
                                    />
                                </button>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
definePageMeta({
    middleware: "auth",
});

const route = useRoute();
const toast = useToast();

const computerName = ref("PC-ADMIN-001");
const searchQuery = ref("");
const selectedCategory = ref("");
const lastScan = ref("10.12.2025 14:30");

const softwareList = ref([
    {
        id: 1,
        name: "Microsoft Office 365",
        manufacturer: "Microsoft",
        version: "16.0.16827",
        license_type: "subscription",
        installation_date: "15.01.2024",
        category: "office",
    },
    {
        id: 2,
        name: "Google Chrome",
        manufacturer: "Google",
        version: "120.0.6099",
        license_type: "free",
        installation_date: "15.01.2024",
        category: "browser",
    },
    {
        id: 3,
        name: "Visual Studio Code",
        manufacturer: "Microsoft",
        version: "1.85.1",
        license_type: "free",
        installation_date: "20.03.2024",
        category: "development",
    },
]);

const filteredSoftware = computed(() => {
    let filtered = softwareList.value;
    if (selectedCategory.value) {
        filtered = filtered.filter(
            (s) => s.category === selectedCategory.value,
        );
    }
    if (searchQuery.value) {
        const query = searchQuery.value.toLowerCase();
        filtered = filtered.filter((s) => s.name.toLowerCase().includes(query));
    }
    return filtered;
});

const licensedCount = computed(
    () => softwareList.value.filter((s) => s.license_type !== "free").length,
);
const expiringCount = computed(() => 2);

const scanSoftware = () => {
    toast.info("Запущено сканирование ПО...");
    setTimeout(() => {
        lastScan.value = new Date().toLocaleString("ru-RU");
        toast.success("Сканирование завершено");
    }, 2000);
};

const removeSoftware = (sw: any) => {
    if (confirm(`Удалить ${sw.name}?`)) {
        softwareList.value = softwareList.value.filter((s) => s.id !== sw.id);
        toast.success("ПО удалено");
    }
};

const getLicenseColor = (type: string) => {
    const colors: Record<string, string> = {
        subscription: "bg-green-100 text-green-800",
        perpetual: "bg-blue-100 text-blue-800",
        trial: "bg-amber-100 text-amber-800",
        free: "bg-gray-100 text-gray-800",
    };
    return colors[type] || "bg-gray-100 text-gray-800";
};

const getLicenseLabel = (type: string) => {
    const labels: Record<string, string> = {
        subscription: "Подписка",
        perpetual: "Бессрочная",
        trial: "Пробная",
        free: "Бесплатная",
    };
    return labels[type] || type;
};

useHead({
    title: `ПО - ${computerName.value}`,
});
</script>
