<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl bg-gradient-to-br from-green-500 to-teal-700 flex items-center justify-center shadow-lg"
                    >
                        <Icon name="ri:link" class="text-3xl text-white" />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Привязка сотрудников к AD
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Связь между сотрудниками системы и пользователями AD
                        </p>
                    </div>
                </div>
            </div>
        </div>

        <!-- Статистика -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="card p-6">
                <div class="flex items-center">
                    <div class="rounded-lg bg-green-100 p-3">
                        <Icon
                            name="ri:checkbox-circle-line"
                            class="text-2xl text-green-600"
                        />
                    </div>
                    <div class="ml-4">
                        <p class="text-sm text-gray-600">Привязано</p>
                        <p class="text-2xl font-bold text-gray-900">0</p>
                    </div>
                </div>
            </div>

            <div class="card p-6">
                <div class="flex items-center">
                    <div class="rounded-lg bg-yellow-100 p-3">
                        <Icon
                            name="ri:error-warning-line"
                            class="text-2xl text-yellow-600"
                        />
                    </div>
                    <div class="ml-4">
                        <p class="text-sm text-gray-600">Без привязки</p>
                        <p class="text-2xl font-bold text-gray-900">0</p>
                    </div>
                </div>
            </div>

            <div class="card p-6">
                <div class="flex items-center">
                    <div class="rounded-lg bg-blue-100 p-3">
                        <Icon
                            name="ri:user-line"
                            class="text-2xl text-blue-600"
                        />
                    </div>
                    <div class="ml-4">
                        <p class="text-sm text-gray-600">Всего сотрудников</p>
                        <p class="text-2xl font-bold text-gray-900">0</p>
                    </div>
                </div>
            </div>
        </div>

        <!-- Фильтры -->
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
                        placeholder="Поиск сотрудника..."
                        class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    />
                </div>
                <select v-model="filterStatus" class="input-field w-48">
                    <option value="">Все статусы</option>
                    <option value="linked">Привязаны</option>
                    <option value="unlinked">Не привязаны</option>
                </select>
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
                                Сотрудник
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Отдел / Должность
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Пользователь AD
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Статус привязки
                            </th>
                            <th class="px-4 py-3 text-right">Действия</th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-gray-200">
                        <tr>
                            <td colspan="5" class="px-6 py-16 text-center">
                                <div class="flex flex-col items-center">
                                    <div
                                        class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mb-4"
                                    >
                                        <Icon
                                            name="ri:link"
                                            class="text-4xl text-gray-400"
                                        />
                                    </div>
                                    <h3
                                        class="text-lg font-semibold text-gray-900 mb-2"
                                    >
                                        Нет данных
                                    </h3>
                                    <p class="text-sm text-gray-500 mb-4">
                                        Интеграция с AD находится в разработке
                                    </p>
                                </div>
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
    title: "Привязка сотрудников",
});

const searchQuery = ref("");
const filterStatus = ref("");
</script>
