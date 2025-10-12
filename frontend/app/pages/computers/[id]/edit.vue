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
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl bg-gradient-to-br from-amber-500 to-amber-700 flex items-center justify-center shadow-lg"
                    >
                        <Icon name="ri:edit-line" class="text-3xl text-white" />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Редактирование компьютера
                        </h1>
                        <p class="mt-1 text-gray-600">{{ form.hostname }}</p>
                    </div>
                </div>
            </div>
        </div>

        <form @submit.prevent="handleSubmit" class="space-y-6">
            <!-- Основная информация -->
            <div class="card">
                <div
                    class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <h2 class="text-lg font-semibold text-gray-900">
                        Основная информация
                    </h2>
                </div>
                <div class="p-6 space-y-6">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Имя компьютера (Hostname)
                                <span class="text-red-500">*</span>
                            </label>
                            <input
                                v-model="form.hostname"
                                type="text"
                                required
                                class="input-field"
                            />
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Инвентарный номер
                            </label>
                            <input
                                v-model="form.inventory_number"
                                type="text"
                                class="input-field"
                            />
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Тип компьютера
                                <span class="text-red-500">*</span>
                            </label>
                            <select
                                v-model="form.computer_type"
                                required
                                class="input-field"
                            >
                                <option value="desktop">Десктоп</option>
                                <option value="laptop">Ноутбук</option>
                                <option value="server">Сервер</option>
                                <option value="workstation">
                                    Рабочая станция
                                </option>
                                <option value="thin_client">
                                    Тонкий клиент
                                </option>
                            </select>
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Статус <span class="text-red-500">*</span>
                            </label>
                            <select
                                v-model="form.status"
                                required
                                class="input-field"
                            >
                                <option value="active">Активный</option>
                                <option value="inactive">Неактивный</option>
                                <option value="repair">В ремонте</option>
                                <option value="storage">На складе</option>
                                <option value="decommissioned">Списан</option>
                            </select>
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Производитель
                            </label>
                            <input
                                v-model="form.manufacturer"
                                type="text"
                                class="input-field"
                            />
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Модель
                            </label>
                            <input
                                v-model="form.model"
                                type="text"
                                class="input-field"
                            />
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Серийный номер
                            </label>
                            <input
                                v-model="form.serial_number"
                                type="text"
                                class="input-field"
                            />
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Местоположение
                            </label>
                            <input
                                v-model="form.location"
                                type="text"
                                class="input-field"
                            />
                        </div>
                    </div>
                </div>
            </div>

            <!-- Операционная система -->
            <div class="card">
                <div
                    class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <h2 class="text-lg font-semibold text-gray-900">
                        Операционная система
                    </h2>
                </div>
                <div class="p-6 space-y-6">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                ОС
                            </label>
                            <input
                                v-model="form.os"
                                type="text"
                                class="input-field"
                            />
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Версия ОС
                            </label>
                            <input
                                v-model="form.os_version"
                                type="text"
                                class="input-field"
                            />
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Сборка
                            </label>
                            <input
                                v-model="form.os_build"
                                type="text"
                                class="input-field"
                            />
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Архитектура
                            </label>
                            <select
                                v-model="form.os_architecture"
                                class="input-field"
                            >
                                <option value="">Выберите архитектуру</option>
                                <option value="x64">x64 (64-bit)</option>
                                <option value="x86">x86 (32-bit)</option>
                                <option value="ARM64">ARM64</option>
                            </select>
                        </div>

                        <div class="flex items-center gap-2">
                            <input
                                v-model="form.domain_joined"
                                type="checkbox"
                                id="domain-joined"
                                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                            />
                            <label
                                for="domain-joined"
                                class="text-sm text-gray-700"
                                >Присоединен к домену</label
                            >
                        </div>
                    </div>
                </div>
            </div>

            <!-- Назначение -->
            <div class="card">
                <div
                    class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <h2 class="text-lg font-semibold text-gray-900">
                        Назначение
                    </h2>
                </div>
                <div class="p-6">
                    <div>
                        <label
                            class="block text-sm font-medium text-gray-700 mb-2"
                        >
                            Сотрудник
                        </label>
                        <select v-model="form.employee_id" class="input-field">
                            <option value="">Не назначен</option>
                            <option value="1">Иванов Иван Иванович</option>
                            <option value="2">Петров Петр Петрович</option>
                            <option value="3">Сидоров Сергей Сергеевич</option>
                        </select>
                    </div>
                </div>
            </div>

            <!-- Гарантия и покупка -->
            <div class="card">
                <div
                    class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <h2 class="text-lg font-semibold text-gray-900">
                        Гарантия и покупка
                    </h2>
                </div>
                <div class="p-6">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Дата покупки
                            </label>
                            <input
                                v-model="form.purchase_date"
                                type="date"
                                class="input-field"
                            />
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Гарантия до
                            </label>
                            <input
                                v-model="form.warranty_end_date"
                                type="date"
                                class="input-field"
                            />
                        </div>
                    </div>
                </div>
            </div>

            <!-- Примечания -->
            <div class="card">
                <div
                    class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <h2 class="text-lg font-semibold text-gray-900">
                        Примечания
                    </h2>
                </div>
                <div class="p-6">
                    <textarea
                        v-model="form.notes"
                        rows="4"
                        placeholder="Дополнительная информация о компьютере..."
                        class="input-field resize-none"
                    />
                </div>
            </div>

            <!-- Кнопки -->
            <div class="flex items-center justify-between gap-4">
                <NuxtLink
                    :to="`/computers/${route.params.id}`"
                    class="btn btn-secondary"
                >
                    <Icon name="ri:close-line" class="mr-2" />
                    Отмена
                </NuxtLink>

                <button type="submit" class="btn btn-primary">
                    <Icon name="ri:save-line" class="mr-2" />
                    Сохранить изменения
                </button>
            </div>
        </form>
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

// Загрузка данных компьютера
const form = ref({
    hostname: "PC-ADMIN-001",
    inventory_number: "INV-2024-001",
    computer_type: "desktop",
    status: "active",
    manufacturer: "HP",
    model: "EliteDesk 800 G6",
    serial_number: "SN123456789",
    location: "Офис 301, 3 этаж",
    os: "Windows 11 Pro",
    os_version: "23H2",
    os_build: "22631.3007",
    os_architecture: "x64",
    domain_joined: true,
    employee_id: "1",
    purchase_date: "2024-01-15",
    warranty_end_date: "2027-01-15",
    notes: "",
});

useHead({
    title: `Редактирование ${form.value.hostname}`,
});

const handleSubmit = async () => {
    try {
        // Здесь будет API запрос
        console.log("Обновление компьютера:", form.value);

        toast.success("Компьютер успешно обновлен!");
        router.push(`/computers/${route.params.id}`);
    } catch (error) {
        toast.error("Ошибка при обновлении компьютера");
        console.error(error);
    }
};
</script>
