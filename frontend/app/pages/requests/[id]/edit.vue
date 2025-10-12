<template>
    <div class="max-w-4xl mx-auto space-y-6">
        <div class="mb-8">
            <div class="flex items-center gap-4 mb-6">
                <NuxtLink
                    :to="`/requests/${route.params.id}`"
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
                            Редактирование заявки #{{ route.params.id }}
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Внесите необходимые изменения
                        </p>
                    </div>
                </div>
            </div>
        </div>

        <form @submit.prevent="handleSubmit" class="space-y-6">
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
                                Статус <span class="text-red-500">*</span>
                            </label>
                            <select
                                v-model="form.status"
                                required
                                class="input-field"
                            >
                                <option value="new">Новая</option>
                                <option value="in_progress">В работе</option>
                                <option value="completed">Завершена</option>
                                <option value="cancelled">Отменена</option>
                            </select>
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Тип заявки <span class="text-red-500">*</span>
                            </label>
                            <select
                                v-model="form.type"
                                required
                                class="input-field"
                            >
                                <option value="hardware">Оборудование</option>
                                <option value="software">
                                    Программное обеспечение
                                </option>
                                <option value="network">Сеть</option>
                                <option value="access">Доступ</option>
                                <option value="other">Другое</option>
                            </select>
                        </div>
                    </div>

                    <div>
                        <label
                            class="block text-sm font-medium text-gray-700 mb-2"
                        >
                            Заголовок <span class="text-red-500">*</span>
                        </label>
                        <input
                            v-model="form.title"
                            type="text"
                            required
                            placeholder="Краткое описание проблемы"
                            class="input-field"
                        />
                    </div>

                    <div>
                        <label
                            class="block text-sm font-medium text-gray-700 mb-2"
                        >
                            Описание <span class="text-red-500">*</span>
                        </label>
                        <textarea
                            v-model="form.description"
                            required
                            rows="5"
                            placeholder="Подробное описание проблемы или запроса"
                            class="input-field resize-none"
                        />
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Приоритет <span class="text-red-500">*</span>
                            </label>
                            <select
                                v-model="form.priority"
                                required
                                class="input-field"
                            >
                                <option value="low">Низкий</option>
                                <option value="medium">Средний</option>
                                <option value="high">Высокий</option>
                            </select>
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Категория
                            </label>
                            <select v-model="form.category" class="input-field">
                                <option value="">Выберите категорию</option>
                                <option value="printer">Принтеры</option>
                                <option value="computer">Компьютеры</option>
                                <option value="phone">Телефония</option>
                                <option value="email">Электронная почта</option>
                                <option value="internet">Интернет</option>
                                <option value="software_install">
                                    Установка ПО
                                </option>
                                <option value="account">Учетные записи</option>
                            </select>
                        </div>
                    </div>
                </div>
            </div>

            <div class="card">
                <div
                    class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <h2 class="text-lg font-semibold text-gray-900">
                        Дополнительная информация
                    </h2>
                </div>

                <div class="p-6 space-y-6">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Заявитель
                            </label>
                            <input
                                v-model="form.requester"
                                type="text"
                                class="input-field"
                                readonly
                            />
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Исполнитель
                            </label>
                            <select
                                v-model="form.assigned_to"
                                class="input-field"
                            >
                                <option value="">Не назначен</option>
                                <option value="Петров П.П.">Петров П.П.</option>
                                <option value="Сидоров С.С.">
                                    Сидоров С.С.
                                </option>
                                <option value="Козлова А.В.">
                                    Козлова А.В.
                                </option>
                            </select>
                        </div>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Местоположение
                            </label>
                            <input
                                v-model="form.location"
                                type="text"
                                placeholder="Офис, кабинет, этаж"
                                class="input-field"
                            />
                        </div>

                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Контактный телефон
                            </label>
                            <input
                                v-model="form.phone"
                                type="tel"
                                placeholder="+7 (___) ___-__-__"
                                class="input-field"
                            />
                        </div>
                    </div>

                    <div>
                        <label
                            class="block text-sm font-medium text-gray-700 mb-2"
                        >
                            Оборудование/Устройство
                        </label>
                        <input
                            v-model="form.equipment"
                            type="text"
                            placeholder="Модель, инвентарный номер"
                            class="input-field"
                        />
                    </div>

                    <div>
                        <label
                            class="block text-sm font-medium text-gray-700 mb-2"
                        >
                            Прикрепленные файлы
                        </label>

                        <div
                            v-if="
                                form.existingFiles &&
                                form.existingFiles.length > 0
                            "
                            class="mb-3 space-y-2"
                        >
                            <div
                                v-for="(file, index) in form.existingFiles"
                                :key="file.id"
                                class="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
                            >
                                <div class="flex items-center gap-2">
                                    <Icon
                                        name="ri:file-line"
                                        class="text-gray-600"
                                    />
                                    <span class="text-sm text-gray-900">{{
                                        file.name
                                    }}</span>
                                    <span class="text-xs text-gray-500"
                                        >({{ file.size }})</span
                                    >
                                </div>
                                <button
                                    type="button"
                                    @click="removeExistingFile(index)"
                                    class="p-1 rounded hover:bg-gray-200 transition-colors"
                                >
                                    <Icon
                                        name="ri:close-line"
                                        class="text-gray-600"
                                    />
                                </button>
                            </div>
                        </div>

                        <div
                            class="border-2 border-dashed border-gray-300 rounded-lg p-6 text-center hover:border-primary-500 transition-colors cursor-pointer"
                        >
                            <input
                                type="file"
                                multiple
                                @change="handleFileUpload"
                                class="hidden"
                                ref="fileInput"
                            />
                            <div @click="$refs.fileInput.click()">
                                <Icon
                                    name="ri:upload-cloud-line"
                                    class="text-4xl text-gray-400 mx-auto mb-2"
                                />
                                <p class="text-sm text-gray-600 mb-1">
                                    Добавить новые файлы
                                </p>
                                <p class="text-xs text-gray-500">
                                    PNG, JPG, PDF до 10MB
                                </p>
                            </div>
                        </div>

                        <div
                            v-if="form.newFiles.length > 0"
                            class="mt-3 space-y-2"
                        >
                            <div
                                v-for="(file, index) in form.newFiles"
                                :key="index"
                                class="flex items-center justify-between p-3 bg-blue-50 rounded-lg border border-blue-200"
                            >
                                <div class="flex items-center gap-2">
                                    <Icon
                                        name="ri:file-add-line"
                                        class="text-blue-600"
                                    />
                                    <span class="text-sm text-gray-900">{{
                                        file.name
                                    }}</span>
                                    <span class="text-xs text-gray-500"
                                        >({{ formatFileSize(file.size) }})</span
                                    >
                                    <span class="text-xs text-blue-600"
                                        >Новый</span
                                    >
                                </div>
                                <button
                                    type="button"
                                    @click="removeNewFile(index)"
                                    class="p-1 rounded hover:bg-blue-100 transition-colors"
                                >
                                    <Icon
                                        name="ri:close-line"
                                        class="text-gray-600"
                                    />
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex items-center justify-between gap-4">
                <NuxtLink
                    :to="`/requests/${route.params.id}`"
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
const fileInput = ref<HTMLInputElement>();

// Загрузка данных заявки
const form = ref({
    status: "in_progress",
    type: "hardware",
    title: "Не работает принтер в офисе 301",
    description:
        'Принтер HP LaserJet не печатает документы. При попытке печати появляется ошибка "Printer offline". Проверил подключение - кабель подключен, индикаторы горят. Требуется срочная помощь, так как нужно распечатать важные документы для встречи.',
    priority: "high",
    category: "printer",
    requester: "Иванов Иван Иванович",
    assigned_to: "Петров П.П.",
    location: "Офис 301, 3 этаж",
    phone: "+7 (999) 123-45-67",
    equipment: "HP LaserJet Pro M404dn, инв. №12345",
    existingFiles: [
        { id: 1, name: "error_screenshot.png", size: "245 KB" },
        { id: 2, name: "printer_manual.pdf", size: "1.2 MB" },
    ],
    newFiles: [] as File[],
});

useHead({
    title: `Редактирование заявки #${route.params.id}`,
});

const handleFileUpload = (event: Event) => {
    const target = event.target as HTMLInputElement;
    if (target.files) {
        const newFiles = Array.from(target.files);
        form.value.newFiles.push(...newFiles);
    }
};

const removeExistingFile = (index: number) => {
    form.value.existingFiles.splice(index, 1);
    toast.info("Файл будет удален после сохранения");
};

const removeNewFile = (index: number) => {
    form.value.newFiles.splice(index, 1);
};

const formatFileSize = (bytes: number) => {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
};

const handleSubmit = async () => {
    try {
        // Здесь будет API запрос
        console.log("Обновление заявки:", form.value);

        toast.success("Заявка успешно обновлена!");
        router.push(`/requests/${route.params.id}`);
    } catch (error) {
        toast.error("Ошибка при обновлении заявки");
        console.error(error);
    }
};
</script>
