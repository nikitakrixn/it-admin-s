<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl bg-gradient-to-br from-indigo-500 to-purple-700 flex items-center justify-center shadow-lg"
                    >
                        <Icon name="ri:group-line" class="text-3xl text-white" />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Группы Active Directory
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Управление группами безопасности и рассылки
                        </p>
                    </div>
                </div>
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
                        placeholder="Поиск по названию группы..."
                        class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                    />
                </div>
                <select v-model="filterType" class="input-field w-48">
                    <option value="">Все типы</option>
                    <option value="security">Безопасности</option>
                    <option value="distribution">Рассылки</option>
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
                                Группа
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Тип
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Участников
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Описание
                            </th>
                            <th class="px-4 py-3 text-right">Действия</th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-gray-200">
                        <tr v-if="groups.length === 0">
                            <td colspan="5" class="px-6 py-16 text-center">
                                <div class="flex flex-col items-center">
                                    <div
                                        class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mb-4"
                                    >
                                        <Icon
                                            name="ri:group-line"
                                            class="text-4xl text-gray-400"
                                        />
                                    </div>
                                    <h3
                                        class="text-lg font-semibold text-gray-900 mb-2"
                                    >
                                        Группы не найдены
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
    title: "Группы AD",
});

const searchQuery = ref("");
const filterType = ref("");
const groups = ref([]);
</script>
