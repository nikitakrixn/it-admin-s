<template>
    <div class="space-y-6">
        <!-- Header -->
        <div class="mb-8">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <NuxtLink
                        to="/employees"
                        class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                    >
                        <Icon name="ri:arrow-left-line" class="text-xl" />
                    </NuxtLink>
                    <div
                        class="h-16 w-16 rounded-2xl bg-gradient-to-br from-purple-500 to-purple-700 flex items-center justify-center shadow-lg"
                    >
                        <Icon
                            :name="
                                activeTab === 'departments'
                                    ? 'ri:building-line'
                                    : 'ri:briefcase-line'
                            "
                            class="text-3xl text-white"
                        />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            {{
                                activeTab === "departments"
                                    ? "Отделы"
                                    : "Должности"
                            }}
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Управление структурой организации
                        </p>
                    </div>
                </div>
                <button @click="openCreateModal" class="btn btn-primary">
                    <Icon name="ri:add-line" class="mr-2" />
                    {{
                        activeTab === "departments"
                            ? "Добавить отдел"
                            : "Добавить должность"
                    }}
                </button>
            </div>
        </div>

        <!-- Tabs -->
        <div class="border-b border-gray-200">
            <nav class="-mb-px flex space-x-8">
                <button
                    @click="activeTab = 'departments'"
                    :class="[
                        'flex items-center gap-2 py-4 px-1 border-b-2 font-medium text-sm transition-colors',
                        activeTab === 'departments'
                            ? 'border-purple-500 text-purple-600'
                            : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300',
                    ]"
                >
                    <Icon name="ri:building-line" class="text-lg" />
                    Отделы
                    <span
                        :class="[
                            'ml-2 py-0.5 px-2.5 rounded-full text-xs font-medium',
                            activeTab === 'departments'
                                ? 'bg-purple-100 text-purple-600'
                                : 'bg-gray-100 text-gray-600',
                        ]"
                    >
                        {{ departments.length }}
                    </span>
                </button>
                <button
                    @click="activeTab = 'positions'"
                    :class="[
                        'flex items-center gap-2 py-4 px-1 border-b-2 font-medium text-sm transition-colors',
                        activeTab === 'positions'
                            ? 'border-purple-500 text-purple-600'
                            : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300',
                    ]"
                >
                    <Icon name="ri:briefcase-line" class="text-lg" />
                    Должности
                    <span
                        :class="[
                            'ml-2 py-0.5 px-2.5 rounded-full text-xs font-medium',
                            activeTab === 'positions'
                                ? 'bg-purple-100 text-purple-600'
                                : 'bg-gray-100 text-gray-600',
                        ]"
                    >
                        {{ positions.length }}
                    </span>
                </button>
            </nav>
        </div>

        <!-- Stats - Departments -->
        <div
            v-if="activeTab === 'departments'"
            class="grid grid-cols-1 md:grid-cols-3 gap-4"
        >
            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">
                                Всего отделов
                            </p>
                            <p class="text-2xl font-bold text-gray-900">
                                {{ departments.length }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:building-line"
                                class="text-2xl text-purple-600"
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
                                Всего сотрудников
                            </p>
                            <p class="text-2xl font-bold text-gray-900">
                                {{ totalEmployees }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:team-line"
                                class="text-2xl text-blue-600"
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
                                Средний размер
                            </p>
                            <p class="text-2xl font-bold text-gray-900">
                                {{ avgDepartmentSize }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:group-line"
                                class="text-2xl text-green-600"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Stats - Positions -->
        <div v-else class="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">
                                Всего должностей
                            </p>
                            <p class="text-2xl font-bold text-gray-900">
                                {{ positions.length }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:briefcase-line"
                                class="text-2xl text-purple-600"
                            />
                        </div>
                    </div>
                </div>
            </div>

            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">С отделами</p>
                            <p class="text-2xl font-bold text-gray-900">
                                {{ positionsWithDepartment }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:links-line"
                                class="text-2xl text-blue-600"
                            />
                        </div>
                    </div>
                </div>
            </div>

            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Без отдела</p>
                            <p class="text-2xl font-bold text-gray-900">
                                {{ positionsWithoutDepartment }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-amber-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:question-line"
                                class="text-2xl text-amber-600"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Loading -->
        <div v-if="loading" class="flex justify-center py-12">
            <Icon
                name="ri:loader-4-line"
                class="text-4xl text-primary-600 animate-spin"
            />
        </div>

        <!-- Departments List -->
        <template v-else-if="activeTab === 'departments'">
            <div v-if="departments.length === 0" class="card">
                <div class="p-16 text-center">
                    <div
                        class="h-20 w-20 rounded-full bg-purple-100 flex items-center justify-center mx-auto mb-4"
                    >
                        <Icon
                            name="ri:building-line"
                            class="text-4xl text-purple-600"
                        />
                    </div>
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">
                        Отделы не найдены
                    </h3>
                    <p class="text-sm text-gray-500 mb-4">
                        Создайте первый отдел организации
                    </p>
                    <button @click="openCreateModal" class="btn btn-primary">
                        <Icon name="ri:add-line" class="mr-2" />
                        Добавить отдел
                    </button>
                </div>
            </div>

            <div
                v-else
                class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"
            >
                <div
                    v-for="department in departments"
                    :key="department.id"
                    class="card card-hover group cursor-pointer"
                    @click="selectDepartment(department)"
                >
                    <div class="p-6">
                        <div class="flex items-start justify-between mb-4">
                            <div class="flex-1">
                                <h3
                                    class="text-lg font-semibold text-gray-900 mb-1 group-hover:text-primary-600 transition-colors"
                                >
                                    {{ department.name }}
                                </h3>
                                <p
                                    v-if="department.description"
                                    class="text-sm text-gray-600 line-clamp-2"
                                >
                                    {{ department.description }}
                                </p>
                            </div>
                            <div
                                class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center group-hover:scale-110 transition-transform"
                            >
                                <Icon
                                    name="ri:building-line"
                                    class="text-2xl text-purple-600"
                                />
                            </div>
                        </div>

                        <div class="pt-4 border-t border-gray-100 space-y-3">
                            <div
                                class="flex items-center justify-between text-sm"
                            >
                                <span
                                    class="text-gray-600 flex items-center gap-2"
                                >
                                    <Icon
                                        name="ri:user-line"
                                        class="text-gray-400"
                                    />
                                    Сотрудников
                                </span>
                                <span class="font-semibold text-gray-900">
                                    {{ department.employee_count || 0 }}
                                </span>
                            </div>

                            <div
                                class="flex items-center justify-between text-sm"
                            >
                                <span
                                    class="text-gray-600 flex items-center gap-2"
                                >
                                    <Icon
                                        name="ri:briefcase-line"
                                        class="text-gray-400"
                                    />
                                    Должностей
                                </span>
                                <span class="font-semibold text-gray-900">
                                    {{
                                        getPositionsCountForDepartment(
                                            department.id,
                                        )
                                    }}
                                </span>
                            </div>
                        </div>

                        <div
                            class="mt-4 pt-4 border-t border-gray-100 flex items-center gap-2"
                        >
                            <button
                                @click.stop="editItem(department)"
                                class="flex-1 btn btn-secondary text-sm"
                            >
                                <Icon name="ri:edit-line" class="mr-1" />
                                Изменить
                            </button>
                            <button
                                @click.stop="deleteItem(department)"
                                class="btn bg-red-50 text-red-700 hover:bg-red-100 border border-red-200 text-sm"
                            >
                                <Icon name="ri:delete-bin-line" />
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </template>

        <!-- Positions List -->
        <template v-else>
            <div v-if="positions.length === 0" class="card">
                <div class="p-16 text-center">
                    <div
                        class="h-20 w-20 rounded-full bg-purple-100 flex items-center justify-center mx-auto mb-4"
                    >
                        <Icon
                            name="ri:briefcase-line"
                            class="text-4xl text-purple-600"
                        />
                    </div>
                    <h3 class="text-lg font-semibold text-gray-900 mb-2">
                        Должности не найдены
                    </h3>
                    <p class="text-sm text-gray-500 mb-4">
                        Создайте первую должность
                    </p>
                    <button @click="openCreateModal" class="btn btn-primary">
                        <Icon name="ri:add-line" class="mr-2" />
                        Добавить должность
                    </button>
                </div>
            </div>

            <div v-else class="card">
                <div class="overflow-x-auto">
                    <table class="w-full">
                        <thead>
                            <tr class="border-b border-gray-200">
                                <th
                                    class="px-6 py-4 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                >
                                    Название
                                </th>
                                <th
                                    class="px-6 py-4 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                >
                                    Отдел
                                </th>
                                <th
                                    class="px-6 py-4 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                >
                                    Сотрудников
                                </th>
                                <th
                                    class="px-6 py-4 text-right text-xs font-medium text-gray-500 uppercase tracking-wider"
                                >
                                    Действия
                                </th>
                            </tr>
                        </thead>
                        <tbody class="divide-y divide-gray-200">
                            <tr
                                v-for="position in positions"
                                :key="position.id"
                                class="hover:bg-gray-50 transition-colors"
                            >
                                <td class="px-6 py-4">
                                    <div class="flex items-center gap-3">
                                        <div
                                            class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center flex-shrink-0"
                                        >
                                            <Icon
                                                name="ri:briefcase-line"
                                                class="text-purple-600"
                                            />
                                        </div>
                                        <span
                                            class="font-medium text-gray-900"
                                            >{{ position.name }}</span
                                        >
                                    </div>
                                </td>
                                <td class="px-6 py-4">
                                    <span
                                        v-if="position.department_name"
                                        class="inline-flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-purple-50 text-purple-700 text-sm font-medium"
                                    >
                                        <Icon
                                            name="ri:building-line"
                                            class="text-xs"
                                        />
                                        {{ position.department_name }}
                                    </span>
                                    <span
                                        v-else
                                        class="text-sm text-gray-400 italic"
                                        >Без отдела</span
                                    >
                                </td>
                                <td class="px-6 py-4">
                                    <span class="text-sm text-gray-900">
                                        {{ position.employee_count || 0 }}
                                    </span>
                                </td>
                                <td class="px-6 py-4">
                                    <div
                                        class="flex items-center justify-end gap-2"
                                    >
                                        <button
                                            @click="editItem(position)"
                                            class="btn btn-secondary text-sm"
                                        >
                                            <Icon
                                                name="ri:edit-line"
                                                class="mr-1"
                                            />
                                            Изменить
                                        </button>
                                        <button
                                            @click="deleteItem(position)"
                                            class="btn bg-red-50 text-red-700 hover:bg-red-100 border border-red-200 text-sm"
                                        >
                                            <Icon name="ri:delete-bin-line" />
                                        </button>
                                    </div>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </template>

        <!-- Create/Edit Modal -->
        <Teleport to="body">
            <Transition name="modal">
                <div
                    v-if="showCreateModal || showEditModal"
                    class="fixed inset-0 z-50 overflow-y-auto"
                >
                    <div
                        class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm"
                        @click="closeModals"
                    />

                    <div
                        class="relative min-h-screen flex items-center justify-center p-4"
                    >
                        <div
                            class="relative bg-white rounded-2xl shadow-2xl w-full max-w-lg"
                        >
                            <div class="p-6 border-b border-gray-200">
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center gap-3">
                                        <div
                                            class="h-10 w-10 rounded-lg bg-purple-100 flex items-center justify-center"
                                        >
                                            <Icon
                                                :name="
                                                    activeTab === 'departments'
                                                        ? 'ri:building-line'
                                                        : 'ri:briefcase-line'
                                                "
                                                class="text-purple-600 text-xl"
                                            />
                                        </div>
                                        <h2
                                            class="text-xl font-semibold text-gray-900"
                                        >
                                            {{ modalTitle }}
                                        </h2>
                                    </div>
                                    <button
                                        @click="closeModals"
                                        class="p-2 rounded-lg hover:bg-gray-100"
                                    >
                                        <Icon
                                            name="ri:close-line"
                                            class="text-xl"
                                        />
                                    </button>
                                </div>
                            </div>

                            <form
                                @submit.prevent="handleSubmit"
                                class="p-6 space-y-4"
                            >
                                <div>
                                    <label
                                        class="block text-sm font-medium text-gray-700 mb-1"
                                    >
                                        Название
                                        {{
                                            activeTab === "departments"
                                                ? "отдела"
                                                : "должности"
                                        }}
                                        <span class="text-red-500">*</span>
                                    </label>
                                    <input
                                        v-model="form.name"
                                        type="text"
                                        required
                                        class="input-field"
                                        :placeholder="
                                            activeTab === 'departments'
                                                ? 'Например: IT отдел'
                                                : 'Например: Разработчик'
                                        "
                                    />
                                </div>

                                <div v-if="activeTab === 'departments'">
                                    <label
                                        class="block text-sm font-medium text-gray-700 mb-1"
                                    >
                                        Описание
                                    </label>
                                    <textarea
                                        v-model="form.description"
                                        rows="3"
                                        class="input-field"
                                        placeholder="Краткое описание отдела..."
                                    />
                                </div>

                                <div v-else>
                                    <label
                                        class="block text-sm font-medium text-gray-700 mb-1"
                                    >
                                        Отдел
                                    </label>
                                    <select
                                        v-model="form.department_id"
                                        class="input-field"
                                    >
                                        <option :value="null">
                                            Без отдела
                                        </option>
                                        <option
                                            v-for="dept in departments"
                                            :key="dept.id"
                                            :value="dept.id"
                                        >
                                            {{ dept.name }}
                                        </option>
                                    </select>
                                </div>

                                <div
                                    v-if="errorMessage"
                                    class="p-4 rounded-lg bg-red-50 text-red-800 text-sm flex items-start gap-2"
                                >
                                    <Icon
                                        name="ri:error-warning-line"
                                        class="text-lg flex-shrink-0 mt-0.5"
                                    />
                                    <span>{{ errorMessage }}</span>
                                </div>

                                <div class="flex gap-3 pt-4">
                                    <button
                                        type="button"
                                        @click="closeModals"
                                        class="flex-1 btn btn-secondary"
                                    >
                                        Отмена
                                    </button>
                                    <button
                                        type="submit"
                                        :disabled="saving"
                                        class="flex-1 btn btn-primary"
                                    >
                                        <Icon
                                            v-if="saving"
                                            name="ri:loader-4-line"
                                            class="mr-2 animate-spin"
                                        />
                                        <Icon
                                            v-else
                                            name="ri:save-line"
                                            class="mr-2"
                                        />
                                        {{
                                            saving
                                                ? "Сохранение..."
                                                : "Сохранить"
                                        }}
                                    </button>
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </Transition>
        </Teleport>

        <!-- Delete Confirmation Modal -->
        <ConfirmModal
            :show="showDeleteModal"
            :loading="deleting"
            :title="deleteModalTitle"
            :message="deleteModalMessage"
            confirm-text="Да, удалить"
            cancel-text="Отмена"
            loading-text="Удаление..."
            variant="danger"
            icon="ri:delete-bin-line"
            @confirm="confirmDelete"
            @cancel="showDeleteModal = false"
        />
    </div>
</template>

<script setup lang="ts">
import type { Department, Position } from "~/composables/useEmployees";

definePageMeta({
    middleware: "auth",
    pageTransition: {
        name: "slide-up",
        mode: "out-in",
    },
});

useHead({
    title: "Структура организации",
});

const toast = useToast();
const {
    fetchDepartments,
    fetchPositions,
    createDepartment,
    createPosition,
    updateDepartment,
    updatePosition,
    deleteDepartment,
    deletePosition,
} = useEmployees();

const activeTab = ref<"departments" | "positions">("departments");
const loading = ref(false);
const departments = ref<Department[]>([]);
const positions = ref<Position[]>([]);
const showCreateModal = ref(false);
const showEditModal = ref(false);
const showDeleteModal = ref(false);
const saving = ref(false);
const deleting = ref(false);
const itemToDelete = ref<Department | Position | null>(null);
const editingItem = ref<Department | Position | null>(null);

const form = ref({
    name: "",
    description: "",
    department_id: null as number | null,
});

const errorMessage = ref("");

// Computed
const totalEmployees = computed(() =>
    departments.value.reduce(
        (sum, dept) => sum + (dept.employee_count || 0),
        0,
    ),
);

const avgDepartmentSize = computed(() =>
    departments.value.length > 0
        ? Math.round(totalEmployees.value / departments.value.length)
        : 0,
);

const positionsWithDepartment = computed(
    () => positions.value.filter((p) => p.department_id).length,
);

const positionsWithoutDepartment = computed(
    () => positions.value.filter((p) => !p.department_id).length,
);

const modalTitle = computed(() => {
    if (activeTab.value === "departments") {
        return showEditModal.value ? "Редактировать отдел" : "Добавить отдел";
    }
    return showEditModal.value
        ? "Редактировать должность"
        : "Добавить должность";
});

const deleteModalTitle = computed(() => {
    if (!itemToDelete.value) return "";
    if (activeTab.value === "departments") {
        const dept = itemToDelete.value as Department;
        return dept.employee_count > 0
            ? "Невозможно удалить отдел"
            : "Удалить отдел?";
    }
    const pos = itemToDelete.value as Position;
    return pos.employee_count && pos.employee_count > 0
        ? "Невозможно удалить должность"
        : "Удалить должность?";
});

const deleteModalMessage = computed(() => {
    if (!itemToDelete.value) return "";

    if (activeTab.value === "departments") {
        const dept = itemToDelete.value as Department;
        if (dept.employee_count > 0) {
            return `В отделе "${dept.name}" числится ${dept.employee_count} сотрудников. Сначала переместите или удалите всех сотрудников из этого отдела.`;
        }
        const posCount = getPositionsCountForDepartment(dept.id);
        if (posCount > 0) {
            return `В отделе "${dept.name}" есть ${posCount} должностей. Сначала удалите или переместите должности.`;
        }
        return `Вы уверены, что хотите удалить отдел "${dept.name}"?`;
    }

    const pos = itemToDelete.value as Position;
    if (pos.employee_count && pos.employee_count > 0) {
        return `Должность "${pos.name}" назначена ${pos.employee_count} сотрудникам. Сначала измените должность у всех сотрудников.`;
    }
    return `Вы уверены, что хотите удалить должность "${pos.name}"?`;
});

// Methods
const getPositionsCountForDepartment = (departmentId: number) => {
    return positions.value.filter((p) => p.department_id === departmentId)
        .length;
};

const loadDepartments = async () => {
    try {
        const data = await fetchDepartments();
        departments.value = data || [];
    } catch (err) {
        toast.error("Ошибка загрузки отделов");
    }
};

const loadPositions = async () => {
    try {
        const data = await fetchPositions();
        positions.value = data || [];
    } catch (err) {
        toast.error("Ошибка загрузки должностей");
    }
};

const loadData = async () => {
    loading.value = true;
    try {
        await Promise.all([loadDepartments(), loadPositions()]);
    } finally {
        loading.value = false;
    }
};

const selectDepartment = (department: Department) => {
    navigateTo(`/employees?department_id=${department.id}`);
};

const openCreateModal = () => {
    showCreateModal.value = true;
};

const editItem = (item: Department | Position) => {
    editingItem.value = item;
    form.value.name = item.name;

    if (activeTab.value === "departments") {
        const dept = item as Department;
        form.value.description = dept.description || "";
    } else {
        const pos = item as Position;
        form.value.department_id = pos.department_id;
    }

    showEditModal.value = true;
};

const deleteItem = (item: Department | Position) => {
    itemToDelete.value = item;
    showDeleteModal.value = true;
};

const closeModals = () => {
    showCreateModal.value = false;
    showEditModal.value = false;
    editingItem.value = null;
    form.value.name = "";
    form.value.description = "";
    form.value.department_id = null;
    errorMessage.value = "";
};

const handleSubmit = async () => {
    saving.value = true;
    errorMessage.value = "";

    try {
        if (activeTab.value === "departments") {
            const data = {
                name: form.value.name,
                description: form.value.description || undefined,
            };

            if (showEditModal.value && editingItem.value) {
                await updateDepartment(editingItem.value.id, data);
                toast.success("Отдел успешно обновлен");
            } else {
                await createDepartment(data);
                toast.success("Отдел успешно создан");
            }
            await loadDepartments();
        } else {
            const data = {
                name: form.value.name,
                department_id: form.value.department_id || undefined,
            };

            if (showEditModal.value && editingItem.value) {
                await updatePosition(editingItem.value.id, data);
                toast.success("Должность успешно обновлена");
            } else {
                await createPosition(data);
                toast.success("Должность успешно создана");
            }
            await loadPositions();
        }

        closeModals();
    } catch (err: any) {
        errorMessage.value = err.message || "Ошибка при сохранении";
        toast.error(errorMessage.value);
    } finally {
        saving.value = false;
    }
};

const confirmDelete = async () => {
    if (!itemToDelete.value) return;

    // Проверка на наличие сотрудников
    if (
        "employee_count" in itemToDelete.value &&
        itemToDelete.value.employee_count > 0
    ) {
        toast.error(
            activeTab.value === "departments"
                ? "Невозможно удалить отдел с сотрудниками"
                : "Невозможно удалить должность, назначенную сотрудникам",
        );
        return;
    }

    // Дополнительная проверка для отдела - есть ли должности
    if (activeTab.value === "departments") {
        const dept = itemToDelete.value as Department;
        const posCount = getPositionsCountForDepartment(dept.id);
        if (posCount > 0) {
            toast.error("Невозможно удалить отдел с должностями");
            return;
        }
    }

    deleting.value = true;
    try {
        if (activeTab.value === "departments") {
            await deleteDepartment(itemToDelete.value.id);
            toast.success("Отдел успешно удален");
            await loadDepartments();
        } else {
            await deletePosition(itemToDelete.value.id);
            toast.success("Должность успешно удалена");
            await loadPositions();
        }

        showDeleteModal.value = false;
        itemToDelete.value = null;
    } catch (err: any) {
        toast.error(err.message || "Ошибка при удалении");
    } finally {
        deleting.value = false;
    }
};

onMounted(() => {
    loadData();
});
</script>

<style scoped>
.modal-enter-active,
.modal-leave-active {
    transition: opacity 0.2s ease;
}

.modal-enter-from,
.modal-leave-to {
    opacity: 0;
}

.modal-enter-active .relative,
.modal-leave-active .relative {
    transition: all 0.3s ease;
}

.modal-enter-from .relative {
    transform: scale(0.95) translateY(-20px);
}

.modal-leave-to .relative {
    transform: scale(0.95) translateY(-20px);
}
</style>
