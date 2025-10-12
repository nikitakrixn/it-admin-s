<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl bg-gradient-to-br from-orange-500 to-orange-700 flex items-center justify-center shadow-lg"
                    >
                        <Icon
                            name="ri:inbox-line"
                            class="text-3xl text-white"
                        />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Расходные материалы
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Учет и управление расходниками
                        </p>
                    </div>
                </div>
                <button @click="showAddModal = true" class="btn btn-primary">
                    <Icon name="ri:add-line" class="mr-2" />
                    Добавить расходник
                </button>
            </div>
        </div>

        <!-- Статистика -->
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">
                                Всего позиций
                            </p>
                            <p class="text-2xl font-bold text-orange-600">
                                {{ stats.total }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-orange-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:inbox-line"
                                class="text-2xl text-orange-600"
                            />
                        </div>
                    </div>
                </div>
            </div>
            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">В наличии</p>
                            <p class="text-2xl font-bold text-green-600">
                                {{ stats.inStock }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:checkbox-circle-line"
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
                            <p class="text-sm text-gray-600 mb-1">
                                Заканчивается
                            </p>
                            <p class="text-2xl font-bold text-amber-600">
                                {{ stats.lowStock }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-amber-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:alert-line"
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
                            <p class="text-sm text-gray-600 mb-1">
                                Нет в наличии
                            </p>
                            <p class="text-2xl font-bold text-red-600">
                                {{ stats.outOfStock }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-red-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:close-circle-line"
                                class="text-2xl text-red-600"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Фильтры -->
        <div class="card">
            <div class="p-4 bg-gradient-to-r from-gray-50 to-white">
                <div class="flex items-center gap-4">
                    <div class="flex-1 relative">
                        <Icon
                            name="ri:search-line"
                            class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                        />
                        <input
                            v-model="searchQuery"
                            type="text"
                            placeholder="Поиск расходников..."
                            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                        />
                    </div>
                    <select v-model="selectedCategory" class="input-field w-48">
                        <option value="">Все категории</option>
                        <option value="cartridge">Картр��джи</option>
                        <option value="paper">Бумага</option>
                        <option value="toner">Тонеры</option>
                    </select>
                </div>
            </div>
        </div>

        <!-- Список расходников -->
        <div class="card">
            <div class="overflow-x-auto">
                <table class="min-w-full divide-y divide-gray-200">
                    <thead class="bg-gray-50">
                        <tr>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Наименование
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Категория
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Количество
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Мин. остаток
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
                            v-for="item in filteredConsumables"
                            :key="item.id"
                            class="hover:bg-gray-50"
                        >
                            <td class="px-4 py-4">
                                <div class="flex items-center gap-3">
                                    <Icon
                                        name="ri:inbox-line"
                                        class="text-gray-400"
                                    />
                                    <div>
                                        <p
                                            class="text-sm font-medium text-gray-900"
                                        >
                                            {{ item.name }}
                                        </p>
                                        <p class="text-xs text-gray-500">
                                            {{ item.manufacturer }}
                                        </p>
                                    </div>
                                </div>
                            </td>
                            <td class="px-4 py-4">
                                <span class="badge bg-blue-100 text-blue-800">{{
                                    item.category
                                }}</span>
                            </td>
                            <td class="px-4 py-4">
                                <p class="text-sm font-medium text-gray-900">
                                    {{ item.quantity }} {{ item.unit }}
                                </p>
                            </td>
                            <td class="px-4 py-4">
                                <p class="text-sm text-gray-600">
                                    {{ item.min_quantity }} {{ item.unit }}
                                </p>
                            </td>
                            <td class="px-4 py-4">
                                <span
                                    :class="[
                                        'badge',
                                        getStockStatus(item).color,
                                    ]"
                                >
                                    {{ getStockStatus(item).label }}
                                </span>
                            </td>
                            <td class="px-4 py-4 text-right">
                                <div
                                    class="flex items-center justify-end gap-2"
                                >
                                    <button
                                        @click="adjustStock(item, 1)"
                                        class="p-2 rounded-lg hover:bg-green-50 transition-colors"
                                        title="Добавить"
                                    >
                                        <Icon
                                            name="ri:add-line"
                                            class="text-green-600"
                                        />
                                    </button>
                                    <button
                                        @click="adjustStock(item, -1)"
                                        class="p-2 rounded-lg hover:bg-red-50 transition-colors"
                                        title="Списать"
                                    >
                                        <Icon
                                            name="ri:subtract-line"
                                            class="text-red-600"
                                        />
                                    </button>
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
    title: "Расходные материалы",
});

const toast = useToast();
const searchQuery = ref("");
const selectedCategory = ref("");
const showAddModal = ref(false);

const stats = ref({
    total: 25,
    inStock: 18,
    lowStock: 5,
    outOfStock: 2,
});

const consumables = ref([
    {
        id: 1,
        name: "Картридж HP 85A",
        manufacturer: "HP",
        category: "Картриджи",
        quantity: 5,
        min_quantity: 2,
        unit: "шт",
    },
    {
        id: 2,
        name: "Бумага A4",
        manufacturer: "Svetocopy",
        category: "Бумага",
        quantity: 50,
        min_quantity: 10,
        unit: "пачек",
    },
    {
        id: 3,
        name: "Тонер Canon C-EXV33",
        manufacturer: "Canon",
        category: "Тонеры",
        quantity: 1,
        min_quantity: 2,
        unit: "шт",
    },
    {
        id: 4,
        name: "Картридж Kyocera TK-1150",
        manufacturer: "Kyocera",
        category: "Картриджи",
        quantity: 0,
        min_quantity: 1,
        unit: "шт",
    },
]);

const filteredConsumables = computed(() => {
    let filtered = consumables.value;
    if (selectedCategory.value)
        filtered = filtered.filter(
            (c) => c.category === selectedCategory.value,
        );
    if (searchQuery.value) {
        const query = searchQuery.value.toLowerCase();
        filtered = filtered.filter((c) => c.name.toLowerCase().includes(query));
    }
    return filtered;
});

const getStockStatus = (item: any) => {
    if (item.quantity === 0)
        return { label: "Нет в наличии", color: "bg-red-100 text-red-800" };
    if (item.quantity <= item.min_quantity)
        return { label: "Заканчивается", color: "bg-amber-100 text-amber-800" };
    return { label: "В наличии", color: "bg-green-100 text-green-800" };
};

const adjustStock = (item: any, delta: number) => {
    item.quantity = Math.max(0, item.quantity + delta);
    toast.success(delta > 0 ? "Количество увеличено" : "Количество уменьшено");
};
</script>
