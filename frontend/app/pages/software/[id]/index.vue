<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center gap-4 mb-6">
                <NuxtLink
                    to="/software"
                    class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                >
                    <Icon name="ri:arrow-left-line" class="text-xl" />
                </NuxtLink>
                <div class="flex-1 flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <div
                            class="h-16 w-16 rounded-2xl bg-gradient-to-br from-indigo-500 to-indigo-700 flex items-center justify-center shadow-lg"
                        >
                            <Icon
                                name="ri:apps-2-line"
                                class="text-3xl text-white"
                            />
                        </div>
                        <div>
                            <h1 class="text-3xl font-bold text-gray-900">
                                {{ software.name }}
                            </h1>
                            <p class="mt-1 text-gray-600">
                                {{ software.manufacturer }}
                            </p>
                        </div>
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
                            Информация
                        </h2>
                    </div>
                    <div class="p-6">
                        <div class="grid grid-cols-2 gap-6">
                            <div>
                                <p class="text-xs text-gray-600 mb-1">
                                    Категория
                                </p>
                                <span class="badge bg-blue-100 text-blue-800">{{
                                    software.category
                                }}</span>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">
                                    Лицензия
                                </p>
                                <span
                                    :class="[
                                        'badge',
                                        software.requires_license
                                            ? 'bg-green-100 text-green-800'
                                            : 'bg-gray-100 text-gray-800',
                                    ]"
                                >
                                    {{
                                        software.requires_license
                                            ? "Требуется"
                                            : "Бесплатное"
                                    }}
                                </span>
                            </div>
                            <div v-if="software.website">
                                <p class="text-xs text-gray-600 mb-1">Сайт</p>
                                <a
                                    :href="software.website"
                                    target="_blank"
                                    class="text-sm text-primary-600 hover:text-primary-700"
                                >
                                    {{ software.website }}
                                </a>
                            </div>
                        </div>
                        <div
                            v-if="software.description"
                            class="mt-4 pt-4 border-t border-gray-200"
                        >
                            <p class="text-sm text-gray-700">
                                {{ software.description }}
                            </p>
                        </div>
                    </div>
                </div>

                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <div class="flex items-center justify-between">
                            <h2 class="text-lg font-semibold text-gray-900">
                                Установлено на компьютерах
                            </h2>
                            <span
                                class="badge bg-primary-100 text-primary-800"
                                >{{ installations.length }}</span
                            >
                        </div>
                    </div>
                    <div class="p-6">
                        <div class="space-y-3">
                            <NuxtLink
                                v-for="install in installations"
                                :key="install.id"
                                :to="`/computers/${install.computer_id}`"
                                class="flex items-center justify-between p-3 rounded-lg hover:bg-gray-50 transition-colors"
                            >
                                <div class="flex items-center gap-3">
                                    <Icon
                                        name="ri:computer-line"
                                        class="text-gray-400"
                                    />
                                    <div>
                                        <p
                                            class="text-sm font-medium text-gray-900"
                                        >
                                            {{ install.computer_name }}
                                        </p>
                                        <p class="text-xs text-gray-500">
                                            Версия {{ install.version }}
                                        </p>
                                    </div>
                                </div>
                                <div class="text-right">
                                    <span
                                        v-if="install.license_type"
                                        class="badge bg-green-100 text-green-800 text-xs"
                                    >
                                        {{ install.license_type }}
                                    </span>
                                </div>
                            </NuxtLink>
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
                            Статистика
                        </h3>
                    </div>
                    <div class="p-6 space-y-4">
                        <div>
                            <p class="text-xs text-gray-600 mb-1">
                                Всего установок
                            </p>
                            <p class="text-2xl font-bold text-gray-900">
                                {{ installations.length }}
                            </p>
                        </div>
                        <div>
                            <p class="text-xs text-gray-600 mb-1">
                                Активных лицензий
                            </p>
                            <p class="text-2xl font-bold text-green-600">
                                {{ activeLicenses }}
                            </p>
                        </div>
                        <div>
                            <p class="text-xs text-gray-600 mb-1">Истекающих</p>
                            <p class="text-2xl font-bold text-amber-600">
                                {{ expiringLicenses }}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
definePageMeta({
    middleware: "auth",
});

const route = useRoute();

const software = ref({
    id: route.params.id,
    name: "Microsoft Office 365",
    manufacturer: "Microsoft Corporation",
    category: "Офисные пакеты",
    requires_license: true,
    website: "https://www.microsoft.com/office",
    description:
        "Набор офисных приложений для работы с документами, таблицами и презентациями",
});

const installations = ref([
    {
        id: 1,
        computer_id: 1,
        computer_name: "PC-ADMIN-001",
        version: "16.0.16827",
        license_type: "Subscription",
    },
    {
        id: 2,
        computer_id: 2,
        computer_name: "LAPTOP-USER-002",
        version: "16.0.16827",
        license_type: "Subscription",
    },
    {
        id: 3,
        computer_id: 3,
        computer_name: "PC-USER-003",
        version: "16.0.16827",
        license_type: "Volume",
    },
]);

const activeLicenses = computed(
    () => installations.value.filter((i) => i.license_type).length,
);
const expiringLicenses = computed(() => 2);

useHead({
    title: software.value.name,
});
</script>
