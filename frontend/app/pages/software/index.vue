<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl gradient-primary flex items-center justify-center shadow-lg"
                    >
                        <Icon name="ri:apps-line" class="text-3xl text-white" />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Каталог ПО
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Управление программным обеспечением
                        </p>
                    </div>
                </div>
                <button
                    v-if="isAdmin"
                    @click="openCreateModal"
                    class="btn btn-primary"
                >
                    <Icon name="ri:add-line" class="mr-2" />
                    Добавить ПО
                </button>
            </div>
        </div>

        <div class="card">
            <div
                class="p-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
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
                            placeholder="Поиск по названию, производителю..."
                            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                        />
                    </div>
                    <select
                        v-model="filters.category"
                        @change="loadSoftware"
                        class="input-field w-48"
                    >
                        <option value="">Все категории</option>
                        <option value="OS">Операционные системы</option>
                        <option value="Office">Офисные приложения</option>
                        <option value="Development">Разработка</option>
                        <option value="Design">Дизайн</option>
                        <option value="Security">Безопасность</option>
                        <option value="Other">Другое</option>
                    </select>
                </div>
            </div>

            <div v-if="pending" class="flex justify-center py-12">
                <Icon
                    name="ri:loader-4-line"
                    class="text-4xl text-primary-600 animate-spin"
                />
            </div>

            <div v-else-if="error" class="p-4 bg-red-50 text-red-800 text-sm">
                <Icon name="ri:error-warning-line" class="inline mr-2" />
                {{ error }}
            </div>

            <div v-else class="overflow-x-auto">
                <table class="min-w-full divide-y divide-gray-200">
                    <thead class="bg-gray-50">
                        <tr>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Название
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Производитель
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Категория
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Лицензия
                            </th>
                            <th
                                v-if="isAdmin"
                                class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase"
                            >
                                Действия
                            </th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-gray-200">
                        <tr v-if="!softwareList.length">
                            <td
                                :colspan="isAdmin ? 5 : 4"
                                class="px-6 py-16 text-center"
                            >
                                <div class="flex flex-col items-center">
                                    <div
                                        class="h-20 w-20 rounded-full bg-gray-100 flex items-center justify-center mb-4"
                                    >
                                        <Icon
                                            name="ri:apps-line"
                                            class="text-4xl text-gray-400"
                                        />
                                    </div>
                                    <h3
                                        class="text-lg font-semibold text-gray-900 mb-2"
                                    >
                                        ПО не найдено
                                    </h3>
                                    <p class="text-sm text-gray-500 mb-4">
                                        Добавьте первое программное обеспечение
                                    </p>
                                    <button
                                        v-if="isAdmin"
                                        @click="openCreateModal"
                                        class="btn btn-primary"
                                    >
                                        <Icon name="ri:add-line" class="mr-2" />
                                        Добавить ПО
                                    </button>
                                </div>
                            </td>
                        </tr>
                        <tr
                            v-for="software in softwareList"
                            :key="software.id"
                            class="hover:bg-gray-50 transition-colors"
                        >
                            <td class="px-4 py-4">
                                <div class="flex items-center gap-3">
                                    <div
                                        class="h-10 w-10 rounded-lg bg-gradient-to-br from-blue-500 to-blue-700 flex items-center justify-center text-white font-semibold text-sm shadow-sm"
                                    >
                                        <Icon name="ri:apps-line" />
                                    </div>
                                    <div>
                                        <div
                                            class="text-sm font-medium text-gray-900"
                                        >
                                            {{ software.name }}
                                        </div>
                                        <div
                                            v-if="software.website"
                                            class="text-xs text-gray-500"
                                        >
                                            <a
                                                :href="software.website"
                                                target="_blank"
                                                class="text-primary-600 hover:underline"
                                            >
                                                {{ software.website }}
                                            </a>
                                        </div>
                                    </div>
                                </div>
                            </td>
                            <td class="px-4 py-4 text-sm text-gray-900">
                                {{ software.manufacturer || "-" }}
                            </td>
                            <td class="px-4 py-4">
                                <span
                                    v-if="software.category"
                                    class="badge bg-blue-100 text-blue-800"
                                >
                                    {{ software.category }}
                                </span>
                                <span v-else class="text-sm text-gray-400"
                                    >-</span
                                >
                            </td>
                            <td class="px-4 py-4">
                                <span
                                    v-if="software.requires_license"
                                    class="badge bg-orange-100 text-orange-800"
                                >
                                    Требуется
                                </span>
                                <span v-else class="badge bg-green-100 text-green-800">
                                    Не требуется
                                </span>
                            </td>
                            <td v-if="isAdmin" class="px-4 py-4 text-right">
                                <div
                                    class="flex items-center justify-end gap-1"
                                >
                                    <button
                                        @click="editSoftware(software)"
                                        class="p-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
                                        title="Редактировать"
                                    >
                                        <Icon name="ri:edit-line" />
                                    </button>
                                    <button
                                        @click="openDeleteModal(software)"
                                        class="p-2 text-gray-600 hover:text-red-600 hover:bg-red-50 rounded-lg transition-colors"
                                        title="Удалить"
                                    >
                                        <Icon name="ri:delete-bin-line" />
                                    </button>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <div
                v-if="data && data.total > 0"
                class="px-6 py-4 border-t border-gray-200 flex items-center justify-between bg-gray-50"
            >
                <p class="text-sm text-gray-700">
                    Показано
                    <span class="font-medium">{{
                        Math.min((currentPage - 1) * perPage + 1, data.total)
                    }}</span>
                    -
                    <span class="font-medium">{{
                        Math.min(currentPage * perPage, data.total)
                    }}</span>
                    из
                    <span class="font-medium">{{ data.total }}</span>
                </p>
                <div class="flex gap-2">
                    <button
                        @click="prevPage"
                        :disabled="currentPage === 1"
                        class="btn btn-secondary disabled:opacity-50"
                    >
                        <Icon name="ri:arrow-left-s-line" />
                    </button>
                    <span
                        class="px-4 py-2 text-sm text-gray-700 flex items-center"
                    >
                        {{ currentPage }} / {{ totalPages }}
                    </span>
                    <button
                        @click="nextPage"
                        :disabled="currentPage >= totalPages"
                        class="btn btn-secondary disabled:opacity-50"
                    >
                        <Icon name="ri:arrow-right-s-line" />
                    </button>
                </div>
            </div>
        </div>

        <!-- Create/Edit Modal -->
        <Teleport to="body">
            <Transition name="modal">
                <div
                    v-if="showModal"
                    class="fixed inset-0 z-50 overflow-y-auto"
                >
                    <div
                        class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm"
                        @click="closeModal"
                    />
                    <div
                        class="flex min-h-full items-center justify-center p-4"
                    >
                        <div
                            class="relative bg-white rounded-2xl shadow-2xl w-full max-w-2xl"
                        >
                            <div
                                class="flex items-center justify-between p-6 border-b border-gray-200"
                            >
                                <h3 class="text-xl font-semibold text-gray-900">
                                    {{
                                        editingItem
                                            ? "Редактировать ПО"
                                            : "Добавить ПО"
                                    }}
                                </h3>
                                <button
                                    @click="closeModal"
                                    class="p-2 rounded-lg hover:bg-gray-100"
                                >
                                    <Icon name="ri:close-line" />
                                </button>
                            </div>

                            <form @submit.prevent="handleSubmit" class="p-6">
                                <div class="space-y-4">
                                    <div>
                                        <label class="label">Название *</label>
                                        <input
                                            v-model="form.name"
                                            type="text"
                                            required
                                            class="input-field"
                                            placeholder="Microsoft Office"
                                        />
                                    </div>

                                    <div class="grid grid-cols-2 gap-4">
                                        <div>
                                            <label class="label"
                                                >Производитель</label
                                            >
                                            <input
                                                v-model="form.manufacturer"
                                                type="text"
                                                class="input-field"
                                                placeholder="Microsoft"
                                            />
                                        </div>
                                        <div>
                                            <label class="label"
                                                >Категория</label
                                            >
                                            <select
                                                v-model="form.category"
                                                class="input-field"
                                            >
                                                <option value="">
                                                    Не выбрано
                                                </option>
                                                <option value="OS">
                                                    Операционные системы
                                                </option>
                                                <option value="Office">
                                                    Офисные приложения
                                                </option>
                                                <option value="Development">
                                                    Разработка
                                                </option>
                                                <option value="Design">
                                                    Дизайн
                                                </option>
                                                <option value="Security">
                                                    Безопасность
                                                </option>
                                                <option value="Other">
                                                    Другое
                                                </option>
                                            </select>
                                        </div>
                                    </div>

                                    <div>
                                        <label class="label">Веб-сайт</label>
                                        <input
                                            v-model="form.website"
                                            type="url"
                                            class="input-field"
                                            placeholder="https://example.com"
                                        />
                                    </div>

                                    <div>
                                        <label class="label">Описание</label>
                                        <textarea
                                            v-model="form.description"
                                            rows="3"
                                            class="input-field"
                                            placeholder="Описание программного обеспечения..."
                                        />
                                    </div>

                                    <div class="flex items-center gap-6">
                                        <label class="flex items-center gap-2">
                                            <input
                                                v-model="form.is_system"
                                                type="checkbox"
                                                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                                            />
                                            <span class="text-sm text-gray-700"
                                                >Системное ПО</span
                                            >
                                        </label>
                                        <label class="flex items-center gap-2">
                                            <input
                                                v-model="form.requires_license"
                                                type="checkbox"
                                                class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                                            />
                                            <span class="text-sm text-gray-700"
                                                >Требуется лицензия</span
                                            >
                                        </label>
                                    </div>
                                </div>

                                <div
                                    class="mt-6 flex items-center justify-end gap-3"
                                >
                                    <button
                                        type="button"
                                        @click="closeModal"
                                        class="btn btn-secondary"
                                    >
                                        Отмена
                                    </button>
                                    <button
                                        type="submit"
                                        :disabled="saving"
                                        class="btn btn-primary"
                                    >
                                        <Icon
                                            v-if="saving"
                                            name="ri:loader-4-line"
                                            class="mr-2 animate-spin"
                                        />
                                        {{
                                            saving
                                                ? "Сохранение..."
                                                : editingItem
                                                  ? "Сохранить"
                                                  : "Создать"
                                        }}
                                    </button>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </Transition>
        </Teleport>

        <!-- Delete Modal -->
        <ConfirmModal
            :show="showDeleteModal"
            :loading="deleting"
            title="Удалить ПО?"
            :message="
                softwareToDelete
                    ? `Вы уверены, что хотите удалить ${softwareToDelete.name}?`
                    : ''
            "
            confirm-text="Да, удалить"
            cancel-text="Отмена"
            loading-text="Удаление..."
            variant="danger"
            icon="ri:delete-bin-line"
            @confirm="confirmDelete"
            @cancel="closeDeleteModal"
        />
    </div>
</template>

<script setup lang="ts">
import type { Software } from "~/composables/useSoftware";

definePageMeta({
    middleware: "auth",
});

useHead({
    title: "Каталог ПО",
});

const { isAdmin } = useAuth();
const { fetchSoftware, createSoftware, updateSoftware, deleteSoftware } =
    useSoftware();
const toast = useToast();

const searchQuery = ref("");
const currentPage = ref(1);
const perPage = ref(20);
const filters = ref({
    category: "",
});

const showModal = ref(false);
const showDeleteModal = ref(false);
const editingItem = ref<Software | null>(null);
const softwareToDelete = ref<Software | null>(null);
const saving = ref(false);
const deleting = ref(false);

const form = ref({
    name: "",
    manufacturer: "",
    category: "",
    description: "",
    website: "",
    is_system: false,
    requires_license: false,
});

const { data, pending, error, refresh } = await useAsyncData(
    "software",
    () =>
        fetchSoftware({
            page: currentPage.value,
            per_page: perPage.value,
            category: filters.value.category || undefined,
            search: searchQuery.value || undefined,
        }),
    {
        watch: [currentPage],
    },
);

const softwareList = computed(() => data.value?.software || []);
const totalPages = computed(() =>
    data.value ? Math.ceil(data.value.total / data.value.per_page) : 0,
);

const loadSoftware = () => {
    currentPage.value = 1;
    refresh();
};

watch(searchQuery, () => {
    loadSoftware();
});

const nextPage = () => {
    if (currentPage.value < totalPages.value) {
        currentPage.value++;
    }
};

const prevPage = () => {
    if (currentPage.value > 1) {
        currentPage.value--;
    }
};

const openCreateModal = () => {
    editingItem.value = null;
    form.value = {
        name: "",
        manufacturer: "",
        category: "",
        description: "",
        website: "",
        is_system: false,
        requires_license: false,
    };
    showModal.value = true;
};

const editSoftware = (software: Software) => {
    editingItem.value = software;
    form.value = {
        name: software.name,
        manufacturer: software.manufacturer || "",
        category: software.category || "",
        description: software.description || "",
        website: software.website || "",
        is_system: software.is_system,
        requires_license: software.requires_license,
    };
    showModal.value = true;
};

const closeModal = () => {
    if (!saving.value) {
        showModal.value = false;
        editingItem.value = null;
    }
};

const handleSubmit = async () => {
    saving.value = true;
    try {
        const data = {
            name: form.value.name,
            manufacturer: form.value.manufacturer || undefined,
            category: form.value.category || undefined,
            description: form.value.description || undefined,
            website: form.value.website || undefined,
            is_system: form.value.is_system,
            requires_license: form.value.requires_license,
        };

        if (editingItem.value) {
            await updateSoftware(editingItem.value.id, data);
            toast.success("ПО успешно обновлено");
        } else {
            await createSoftware(data);
            toast.success("ПО успешно создано");
        }

        await refresh();
        showModal.value = false;
        editingItem.value = null;
    } catch (err: any) {
        if (err.statusCode === 401) {
            toast.error("Недостаточно прав. Требуется роль администратора.");
        } else {
            toast.error("Ошибка при сохранении: " + (err.message || "Неизвестная ошибка"));
        }
    } finally {
        saving.value = false;
    }
};

const openDeleteModal = (software: Software) => {
    softwareToDelete.value = software;
    showDeleteModal.value = true;
};

const closeDeleteModal = () => {
    if (!deleting.value) {
        showDeleteModal.value = false;
        softwareToDelete.value = null;
    }
};

const confirmDelete = async () => {
    if (!softwareToDelete.value) return;

    deleting.value = true;
    try {
        await deleteSoftware(softwareToDelete.value.id);
        toast.success("ПО успешно удалено");
        await refresh();
        showDeleteModal.value = false;
        softwareToDelete.value = null;
    } catch (err: any) {
        if (err.statusCode === 401) {
            toast.error("Недостаточно прав. Требуется роль администратора.");
        } else {
            toast.error("Ошибка при удалении: " + (err.message || "Неизвестная ошибка"));
        }
    } finally {
        deleting.value = false;
    }
};
</script>
