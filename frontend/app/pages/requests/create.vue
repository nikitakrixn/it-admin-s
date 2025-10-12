<template>
    <div class="max-w-4xl mx-auto space-y-6">
        <div class="mb-8">
            <div class="flex items-center gap-4 mb-6">
                <NuxtLink
                    to="/requests"
                    class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                >
                    <Icon name="ri:arrow-left-line" class="text-xl" />
                </NuxtLink>
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl bg-gradient-to-br from-blue-500 to-blue-700 flex items-center justify-center shadow-lg"
                    >
                        <Icon
                            name="ri:file-add-line"
                            class="text-3xl text-white"
                        />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Создание заявки
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Заполните форму для создания новой заявки
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
                            <option value="">Выберите тип заявки</option>
                            <option value="hardware">Оборудование</option>
                            <option value="software">
                                Программное обеспечение
                            </option>
                            <option value="network">Сеть</option>
                            <option value="access">Доступ</option>
                            <option value="other">Другое</option>
                        </select>
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
                                <option value="">Выберите приоритет</option>
                                <option value="low">Низкий</option>
                                <option value="medium">Средний</option>
                                <option value="high">Высокий</option>
                            </select>
                            <p class="mt-1 text-xs text-gray-500">
                                Высокий - критичные проблемы, требующие
                                немедленного решения
                            </p>
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
                            Прикрепить файлы
                        </label>
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
                                    Нажмите для выбора файлов или перетащите их
                                    сюда
                                </p>
                                <p class="text-xs text-gray-500">
                                    PNG, JPG, PDF до 10MB
                                </p>
                            </div>
                        </div>

                        <div
                            v-if="form.files.length > 0"
                            class="mt-3 space-y-2"
                        >
                            <div
                                v-for="(file, index) in form.files"
                                :key="index"
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
                                        >({{ formatFileSize(file.size) }})</span
                                    >
                                </div>
                                <button
                                    type="button"
                                    @click="removeFile(index)"
                                    class="p-1 rounded hover:bg-gray-200 transition-colors"
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

            <div class="card">
                <div
                    class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-white"
                >
                    <div class="flex items-center gap-2">
                        <Icon
                            name="ri:information-line"
                            class="text-blue-600"
                        />
                        <h2 class="text-lg font-semibold text-gray-900">
                            Предварительный просмотр
                        </h2>
                    </div>
                </div>

                <div class="p-6">
                    <div class="space-y-4">
                        <div class="flex items-start gap-4">
                            <div
                                :class="[
                                    'h-12 w-12 rounded-xl flex items-center justify-center flex-shrink-0',
                                    getPriorityColor(form.priority),
                                ]"
                            >
                                <Icon
                                    name="ri:flag-line"
                                    class="text-xl text-white"
                                />
                            </div>
                            <div class="flex-1">
                                <h3
                                    class="text-lg font-semibold text-gray-900 mb-1"
                                >
                                    {{ form.title || "Заголовок заявки" }}
                                </h3>
                                <p class="text-sm text-gray-600 mb-3">
                                    {{ form.description || "Описание заявки" }}
                                </p>
                                <div class="flex flex-wrap gap-2">
                                    <span
                                        v-if="form.type"
                                        class="badge bg-blue-100 text-blue-800"
                                    >
                                        {{ getTypeLabel(form.type) }}
                                    </span>
                                    <span
                                        v-if="form.priority"
                                        :class="[
                                            'badge',
                                            getPriorityBadgeColor(
                                                form.priority,
                                            ),
                                        ]"
                                    >
                                        {{ getPriorityLabel(form.priority) }}
                                    </span>
                                    <span
                                        v-if="form.category"
                                        class="badge bg-purple-100 text-purple-800"
                                    >
                                        {{ getCategoryLabel(form.category) }}
                                    </span>
                                    <span
                                        v-if="form.location"
                                        class="badge bg-gray-100 text-gray-800"
                                    >
                                        <Icon
                                            name="ri:map-pin-line"
                                            class="mr-1"
                                        />
                                        {{ form.location }}
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="flex items-center justify-between gap-4">
                <NuxtLink to="/requests" class="btn btn-secondary">
                    <Icon name="ri:close-line" class="mr-2" />
                    Отмена
                </NuxtLink>

                <div class="flex gap-2">
                    <button
                        type="button"
                        @click="saveDraft"
                        class="btn btn-secondary"
                    >
                        <Icon name="ri:save-line" class="mr-2" />
                        Сохранить черновик
                    </button>
                    <button
                        type="submit"
                        class="btn btn-primary"
                        :disabled="!isFormValid"
                    >
                        <Icon name="ri:send-plane-line" class="mr-2" />
                        Создать заявку
                    </button>
                </div>
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

useHead({
    title: "Создание заявки",
});

const router = useRouter();
const toast = useToast();
const fileInput = ref<HTMLInputElement>();

const form = ref({
    type: "",
    title: "",
    description: "",
    priority: "",
    category: "",
    location: "",
    phone: "",
    equipment: "",
    files: [] as File[],
});

const isFormValid = computed(() => {
    return (
        form.value.type &&
        form.value.title &&
        form.value.description &&
        form.value.priority
    );
});

const handleFileUpload = (event: Event) => {
    const target = event.target as HTMLInputElement;
    if (target.files) {
        const newFiles = Array.from(target.files);
        form.value.files.push(...newFiles);
    }
};

const removeFile = (index: number) => {
    form.value.files.splice(index, 1);
};

const formatFileSize = (bytes: number) => {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
};

const handleSubmit = async () => {
    if (!isFormValid.value) {
        toast.error("Заполните все обязательные поля");
        return;
    }

    try {
        // Здесь будет API запрос
        console.log("Создание заявки:", form.value);

        toast.success("Заявка успешно создана!");
        router.push("/requests");
    } catch (error) {
        toast.error("Ошибка при создании заявки");
        console.error(error);
    }
};

const saveDraft = () => {
    localStorage.setItem("request_draft", JSON.stringify(form.value));
    toast.success("Черновик сохранен");
};

// Загрузка черновика при монтировании
onMounted(() => {
    const draft = localStorage.getItem("request_draft");
    if (draft) {
        try {
            const parsed = JSON.parse(draft);
            form.value = { ...form.value, ...parsed, files: [] };
            toast.info("Загружен сохраненный черновик");
        } catch (e) {
            console.error("Ошибка загрузки черновика:", e);
        }
    }
});

const getTypeLabel = (type: string) => {
    const labels: Record<string, string> = {
        hardware: "Оборудование",
        software: "ПО",
        network: "Сеть",
        access: "Доступ",
        other: "Другое",
    };
    return labels[type] || type;
};

const getPriorityLabel = (priority: string) => {
    const labels: Record<string, string> = {
        low: "Низкий",
        medium: "Средний",
        high: "Высокий",
    };
    return labels[priority] || priority;
};

const getCategoryLabel = (category: string) => {
    const labels: Record<string, string> = {
        printer: "Принтеры",
        computer: "Компьютеры",
        phone: "Телефония",
        email: "Email",
        internet: "Интернет",
        software_install: "Установка ПО",
        account: "Учетные записи",
    };
    return labels[category] || category;
};

const getPriorityColor = (priority: string) => {
    const colors: Record<string, string> = {
        low: "bg-gray-500",
        medium: "bg-blue-500",
        high: "bg-red-500",
    };
    return colors[priority] || "bg-gray-500";
};

const getPriorityBadgeColor = (priority: string) => {
    const colors: Record<string, string> = {
        low: "bg-gray-100 text-gray-800",
        medium: "bg-blue-100 text-blue-800",
        high: "bg-red-100 text-red-800",
    };
    return colors[priority] || "bg-gray-100 text-gray-800";
};
</script>
