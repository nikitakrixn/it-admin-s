<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl bg-gradient-to-br from-blue-500 to-indigo-700 flex items-center justify-center shadow-lg"
                    >
                        <Icon
                            name="ri:user-search-line"
                            class="text-3xl text-white"
                        />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Пользователи Active Directory
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Управление учетными записями пользователей в домене
                        </p>
                    </div>
                </div>
                <NuxtLink to="/ad" class="btn btn-primary">
                    <Icon name="ri:user-add-line" class="mr-2" />
                    Создать пользователя
                </NuxtLink>
            </div>
        </div>

        <!-- Поиск -->
        <div class="card p-4">
            <div class="flex gap-4">
                <div class="flex-1 relative">
                    <Icon
                        name="ri:search-line"
                        class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                    />
                    <input
                        v-model="searchQuery"
                        type="text"
                        placeholder="Поиск по имени, фамилии, имени пользователя..."
                        class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    />
                </div>
                <button class="btn btn-secondary">
                    <Icon name="ri:filter-line" class="mr-2" />
                    Применить
                </button>
            </div>
        </div>

        <!-- Таблица -->
        <div class="card">
            <div class="overflow-x-auto">
                <table class="min-w-full divide-y divide-gray-200">
                    <thead class="bg-gray-50">
                        <tr>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Пользователь
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Email
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Отдел / Должность
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Статус
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Привязка
                            </th>
                            <th class="px-4 py-3 text-right">Действия</th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-gray-200">
                        <tr v-if="users.length === 0">
                            <td colspan="6" class="px-6 py-16 text-center">
                                <div class="flex flex-col items-center">
                                    <div
                                        class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mb-4"
                                    >
                                        <Icon
                                            name="ri:user-search-line"
                                            class="text-4xl text-gray-400"
                                        />
                                    </div>
                                    <h3
                                        class="text-lg font-semibold text-gray-900 mb-2"
                                    >
                                        Пользователи не найдены
                                    </h3>
                                    <p class="text-sm text-gray-500 mb-4">
                                        Интеграция с AD находится в разработке
                                    </p>
                                </div>
                            </td>
                        </tr>
                        <tr
                            v-for="user in users"
                            :key="user.id"
                            class="hover:bg-gray-50"
                        >
                            <td class="px-4 py-4">
                                <div class="flex items-center gap-3">
                                    <div
                                        class="h-10 w-10 rounded-full bg-primary-100 flex items-center justify-center text-primary-800 font-semibold"
                                    >
                                        {{ user.initials }}
                                    </div>
                                    <div>
                                        <div
                                            class="text-sm font-medium text-gray-900"
                                        >
                                            {{ user.name }}
                                        </div>
                                        <div class="text-sm text-gray-500">
                                            {{ user.username }}
                                        </div>
                                    </div>
                                </div>
                            </td>
                            <td class="px-4 py-4 text-sm text-gray-900">
                                {{ user.email || "-" }}
                            </td>
                            <td class="px-4 py-4">
                                <div class="text-sm text-gray-900">
                                    {{ user.department || "-" }}
                                </div>
                                <div class="text-xs text-gray-500">
                                    {{ user.position || "-" }}
                                </div>
                            </td>
                            <td class="px-4 py-4">
                                <span
                                    :class="
                                        user.enabled
                                            ? 'bg-green-100 text-green-800'
                                            : 'bg-red-100 text-red-800'
                                    "
                                    class="badge"
                                >
                                    {{ user.enabled ? "Активен" : "Отключен" }}
                                </span>
                            </td>
                            <td class="px-4 py-4">
                                <span
                                    v-if="user.linked"
                                    class="badge bg-blue-100 text-blue-800"
                                >
                                    <Icon name="ri:link" class="mr-1" />
                                    Привязан
                                </span>
                                <span v-else class="text-sm text-gray-400"
                                    >Не привязан</span
                                >
                            </td>
                            <td class="px-4 py-4 text-right">
                                <button
                                    class="p-2 text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded-lg"
                                    title="Просмотр"
                                >
                                    <Icon name="ri:eye-line" />
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

useHead({
    title: "Пользователи AD",
});

const searchQuery = ref("");

// Демо данные
const users = ref([
    // Пустой массив - интеграция в разработке
]);
</script>
