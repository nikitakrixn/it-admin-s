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
                                name="ri:cpu-line"
                                class="text-3xl text-white"
                            />
                        </div>
                        <div>
                            <h1 class="text-3xl font-bold text-gray-900">
                                Компоненты
                            </h1>
                            <p class="mt-1 text-gray-600">{{ computerName }}</p>
                        </div>
                    </div>

                    <NuxtLink
                        :to="`/computers/${route.params.id}/components/add`"
                        class="btn btn-primary"
                    >
                        <Icon name="ri:add-line" class="mr-2" />
                        Добавить компонент
                    </NuxtLink>
                </div>
            </div>
        </div>

        <!-- Фильтры -->
        <div class="card">
            <div class="p-4">
                <div class="flex items-center gap-4">
                    <div class="flex-1 relative">
                        <Icon
                            name="ri:search-line"
                            class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                        />
                        <input
                            v-model="searchQuery"
                            type="text"
                            placeholder="Поиск компонентов..."
                            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                        />
                    </div>

                    <select v-model="selectedType" class="input-field w-48">
                        <option value="">Все типы</option>
                        <option value="processor">Процессор</option>
                        <option value="ram">Оперативная память</option>
                        <option value="storage">Накопитель</option>
                        <option value="gpu">Видеокарта</option>
                        <option value="motherboard">Материнская плата</option>
                        <option value="network">Сетевой адаптер</option>
                        <option value="monitor">Монитор</option>
                        <option value="peripheral">Периферия</option>
                    </select>
                </div>
            </div>
        </div>

        <!-- Список компонентов -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div
                v-for="component in filteredComponents"
                :key="component.id"
                class="card card-hover group cursor-pointer"
                @click="viewComponent(component)"
            >
                <div class="p-5">
                    <div class="flex items-start justify-between mb-4">
                        <div class="flex items-center gap-3">
                            <div
                                :class="[
                                    'h-12 w-12 rounded-xl flex items-center justify-center',
                                    getComponentColor(component.type),
                                ]"
                            >
                                <Icon
                                    :name="getComponentIcon(component.type)"
                                    class="text-2xl text-white"
                                />
                            </div>
                            <div>
                                <p
                                    class="text-sm font-semibold text-gray-900 group-hover:text-primary-600 transition-colors"
                                >
                                    {{ getComponentTypeLabel(component.type) }}
                                </p>
                                <p class="text-xs text-gray-500">
                                    {{ component.name }}
                                </p>
                            </div>
                        </div>
                        <div class="flex items-center gap-2">
                            <button
                                @click.stop="editComponent(component)"
                                class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                                title="Редактировать"
                            >
                                <Icon
                                    name="ri:edit-line"
                                    class="text-gray-600"
                                />
                            </button>
                            <button
                                @click.stop="deleteComponent(component)"
                                class="p-2 rounded-lg hover:bg-red-50 transition-colors"
                                title="Удалить"
                            >
                                <Icon
                                    name="ri:delete-bin-line"
                                    class="text-red-600"
                                />
                            </button>
                        </div>
                    </div>

                    <div class="space-y-2">
                        <div
                            v-for="(value, key) in component.specs"
                            :key="key"
                            class="flex items-center justify-between text-sm"
                        >
                            <span class="text-gray-600">{{ key }}</span>
                            <span class="font-medium text-gray-900">{{
                                value
                            }}</span>
                        </div>
                    </div>

                    <div
                        v-if="component.notes"
                        class="mt-3 pt-3 border-t border-gray-200"
                    >
                        <p class="text-xs text-gray-600">
                            {{ component.notes }}
                        </p>
                    </div>
                </div>
            </div>

            <div
                v-if="filteredComponents.length === 0"
                class="col-span-full p-16 text-center"
            >
                <div
                    class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mx-auto mb-4"
                >
                    <Icon name="ri:cpu-line" class="text-4xl text-gray-400" />
                </div>
                <h3 class="text-lg font-semibold text-gray-900 mb-2">
                    Компоненты не найдены
                </h3>
                <p class="text-sm text-gray-500 mb-4">
                    Добавьте первый компонент
                </p>
                <NuxtLink
                    :to="`/computers/${route.params.id}/components/add`"
                    class="btn btn-primary"
                >
                    <Icon name="ri:add-line" class="mr-2" />
                    Добавить компонент
                </NuxtLink>
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

const route = useRoute();
const router = useRouter();
const toast = useToast();

const computerName = ref("PC-ADMIN-001");
const searchQuery = ref("");
const selectedType = ref("");

useHead({
    title: `Компоненты - ${computerName.value}`,
});

// Мок данные компонентов
const components = ref([
    {
        id: 1,
        type: "processor",
        name: "Intel Core i7-10700",
        specs: {
            Ядра: "8",
            Потоки: "16",
            Частота: "2.90 GHz",
        },
        notes: null,
    },
    {
        id: 2,
        type: "ram",
        name: "Kingston DDR4",
        specs: {
            Объем: "8 ГБ",
            Тип: "DDR4",
            Частота: "3200 MHz",
            Слот: "1",
        },
        notes: null,
    },
    {
        id: 3,
        type: "ram",
        name: "Kingston DDR4",
        specs: {
            Объем: "8 ГБ",
            Тип: "DDR4",
            Частота: "3200 MHz",
            Слот: "2",
        },
        notes: null,
    },
    {
        id: 4,
        type: "storage",
        name: "Samsung 970 EVO",
        specs: {
            Тип: "NVMe",
            Объем: "512 ГБ",
            Интерфейс: "M.2",
        },
        notes: "Системный диск",
    },
    {
        id: 5,
        type: "gpu",
        name: "NVIDIA GeForce GTX 1650",
        specs: {
            Память: "4 ГБ",
            Драйвер: "531.68",
        },
        notes: null,
    },
    {
        id: 6,
        type: "motherboard",
        name: "HP 8704",
        specs: {
            Чипсет: "Intel Q470",
            BIOS: "S09 Ver. 02.06.00",
        },
        notes: null,
    },
    {
        id: 7,
        type: "network",
        name: "Intel I219-LM",
        specs: {
            Тип: "Ethernet",
            Скорость: "1 Гбит/с",
        },
        notes: null,
    },
    {
        id: 8,
        type: "monitor",
        name: "Dell P2419H",
        specs: {
            Размер: '24"',
            Разрешение: "1920x1080",
        },
        notes: null,
    },
]);

const filteredComponents = computed(() => {
    let filtered = components.value;

    if (selectedType.value) {
        filtered = filtered.filter((c) => c.type === selectedType.value);
    }

    if (searchQuery.value) {
        const query = searchQuery.value.toLowerCase();
        filtered = filtered.filter(
            (c) =>
                c.name.toLowerCase().includes(query) ||
                getComponentTypeLabel(c.type).toLowerCase().includes(query),
        );
    }

    return filtered;
});

const viewComponent = (component: any) => {
    router.push(`/computers/${route.params.id}/components/${component.id}`);
};

const editComponent = (component: any) => {
    router.push(
        `/computers/${route.params.id}/components/${component.id}/edit`,
    );
};

const deleteComponent = (component: any) => {
    if (confirm(`Удалить компонент "${component.name}"?`)) {
        components.value = components.value.filter(
            (c) => c.id !== component.id,
        );
        toast.success("Компонент удален");
    }
};

const getComponentIcon = (type: string) => {
    const icons: Record<string, string> = {
        processor: "ri:cpu-line",
        ram: "ri:database-2-line",
        storage: "ri:hard-drive-line",
        gpu: "ri:gamepad-line",
        motherboard: "ri:circuit-board-line",
        network: "ri:wifi-line",
        monitor: "ri:monitor-line",
        peripheral: "ri:keyboard-line",
    };
    return icons[type] || "ri:cpu-line";
};

const getComponentColor = (type: string) => {
    const colors: Record<string, string> = {
        processor: "bg-blue-500",
        ram: "bg-purple-500",
        storage: "bg-green-500",
        gpu: "bg-amber-500",
        motherboard: "bg-indigo-500",
        network: "bg-cyan-500",
        monitor: "bg-pink-500",
        peripheral: "bg-teal-500",
    };
    return colors[type] || "bg-gray-500";
};

const getComponentTypeLabel = (type: string) => {
    const labels: Record<string, string> = {
        processor: "Процессор",
        ram: "Оперативная память",
        storage: "Накопитель",
        gpu: "Видеокарта",
        motherboard: "Материнская плата",
        network: "Сетевой адаптер",
        monitor: "Монитор",
        peripheral: "Периферия",
    };
    return labels[type] || type;
};
</script>
