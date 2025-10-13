<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl gradient-primary flex items-center justify-center shadow-lg"
                    >
                        <Icon name="ri:team-line" class="text-3xl text-white" />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Сотрудники
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Управление персоналом организации
                        </p>
                    </div>
                </div>
                <div class="flex items-center gap-2">
                    <button
                        v-if="isAdmin && selectedIds.length > 0"
                        @click="openBulkDeleteModal"
                        class="btn bg-red-50 text-red-700 hover:bg-red-100 border border-red-200"
                    >
                        <Icon name="ri:delete-bin-line" class="mr-2" />
                        Удалить ({{ selectedIds.length }})
                    </button>
                    <NuxtLink
                        to="/employees/departments"
                        class="btn btn-secondary"
                    >
                        <Icon name="ri:building-line" class="mr-2" />
                        Отделы и Должности
                    </NuxtLink>
                    <NuxtLink v-if="isAdmin" to="/employees/create" class="btn btn-primary">
                        <Icon name="ri:add-line" class="mr-2" />
                        Добавить
                    </NuxtLink>
                </div>
            </div>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">
                                Всего сотрудников
                            </p>
                            <p class="text-2xl font-bold text-gray-900">
                                {{ data?.total || 0 }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:user-line"
                                class="text-2xl text-blue-600"
                            />
                        </div>
                    </div>
                </div>
            </div>

            <div
                class="card cursor-pointer hover:shadow-md transition-shadow"
                @click="
                    filters.status = 'active';
                    loadEmployees();
                "
            >
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Активные</p>
                            <p class="text-2xl font-bold text-green-600">
                                {{ activeCount }}
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

            <div
                class="card cursor-pointer hover:shadow-md transition-shadow"
                @click="
                    filters.status = 'inactive';
                    loadEmployees();
                "
            >
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Неактивные</p>
                            <p class="text-2xl font-bold text-gray-600">
                                {{ inactiveCount }}
                            </p>
                        </div>
                        <div
                            class="h-12 w-12 rounded-xl bg-gray-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:close-circle-line"
                                class="text-2xl text-gray-600"
                            />
                        </div>
                    </div>
                </div>
            </div>

            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Отделов</p>
                            <p class="text-2xl font-bold text-purple-600">
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
                            placeholder="Поиск по имени, email, телефону..."
                            class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                        />
                    </div>
                    <select
                        v-model="filters.status"
                        @change="loadEmployees"
                        class="input-field w-48"
                    >
                        <option value="">Все статусы</option>
                        <option value="active">Активные</option>
                        <option value="inactive">Неактивные</option>
                        <option value="terminated">Уволенные</option>
                    </select>
                    <select
                        v-model="filters.department_id"
                        @change="loadEmployees"
                        class="input-field w-48"
                    >
                        <option :value="undefined">Все отделы</option>
                        <option
                            v-for="dept in departments"
                            :key="dept.id"
                            :value="dept.id"
                        >
                            {{ dept.name }}
                        </option>
                    </select>
                    <button
                        v-if="filters.status || filters.department_id"
                        @click="resetFilters"
                        class="btn btn-secondary"
                    >
                        <Icon name="ri:refresh-line" />
                    </button>
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
                            <th v-if="isAdmin" class="px-4 py-3 text-left w-12">
                                <input
                                    type="checkbox"
                                    v-model="selectAll"
                                    @change="toggleSelectAll"
                                    class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                                />
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                ФИО
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Должность
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Отдел
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Контакты
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
                        <tr v-if="!employees.length">
                            <td :colspan="isAdmin ? 7 : 6" class="px-6 py-16 text-center">
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
                                        Сотрудники не найдены
                                    </h3>
                                    <p class="text-sm text-gray-500 mb-4">
                                        {{
                                            filters.status ||
                                            filters.department_id
                                                ? "Попробуйте изменить фильтры"
                                                : "Добавьте первого сотрудника"
                                        }}
                                    </p>
                                    <NuxtLink
                                        to="/employees/create"
                                        class="btn btn-primary"
                                    >
                                        <Icon name="ri:add-line" class="mr-2" />
                                        Добавить сотрудника
                                    </NuxtLink>
                                </div>
                            </td>
                        </tr>
                        <tr
                            v-for="employee in employees"
                            :key="employee.id"
                            class="hover:bg-gray-50 transition-colors"
                        >
                            <td v-if="isAdmin" class="px-4 py-4">
                                <input
                                    type="checkbox"
                                    :checked="selectedIds.includes(employee.id)"
                                    @change="toggleSelect(employee.id)"
                                    class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                                />
                            </td>
                            <td class="px-4 py-4">
                                <div class="flex items-center gap-3">
                                    <div
                                        class="h-10 w-10 rounded-full gradient-primary flex items-center justify-center text-white font-semibold text-sm shadow-sm"
                                    >
                                        {{ employee.first_name.charAt(0)
                                        }}{{ employee.last_name.charAt(0) }}
                                    </div>
                                    <div>
                                        <div
                                            class="text-sm font-medium text-gray-900"
                                        >
                                            {{ employee.full_name }}
                                        </div>
                                        <div
                                            v-if="employee.ad_username"
                                            class="text-xs text-gray-500 flex items-center gap-1"
                                        >
                                            <Icon
                                                name="ri:windows-fill"
                                                class="text-xs"
                                            />
                                            {{ employee.ad_username }}
                                        </div>
                                    </div>
                                </div>
                            </td>
                            <td class="px-4 py-4 text-sm text-gray-900">
                                {{ employee.position_name || "-" }}
                            </td>
                            <td class="px-4 py-4 text-sm text-gray-900">
                                {{ employee.department_name || "-" }}
                            </td>
                            <td class="px-4 py-4">
                                <div
                                    v-if="employee.email"
                                    class="text-sm text-gray-900 flex items-center gap-1 mb-1"
                                >
                                    <Icon
                                        name="ri:mail-line"
                                        class="text-gray-400"
                                    />
                                    <a
                                        :href="`mailto:${employee.email}`"
                                        class="text-primary-600 hover:underline"
                                    >
                                        {{ employee.email }}
                                    </a>
                                </div>
                                <div
                                    v-if="employee.phone"
                                    class="text-xs text-gray-500 flex items-center gap-1"
                                >
                                    <Icon
                                        name="ri:phone-line"
                                        class="text-gray-400"
                                    />
                                    {{ employee.phone }}
                                </div>
                                <span
                                    v-if="!employee.email && !employee.phone"
                                    class="text-sm text-gray-400"
                                    >-</span
                                >
                            </td>
                            <td class="px-4 py-4">
                                <span
                                    :class="{
                                        'bg-green-100 text-green-800':
                                            employee.status === 'active',
                                        'bg-gray-100 text-gray-800':
                                            employee.status === 'inactive',
                                        'bg-red-100 text-red-800':
                                            employee.status === 'terminated',
                                    }"
                                    class="badge"
                                >
                                    {{ statusLabel(employee.status) }}
                                </span>
                            </td>
                            <td class="px-4 py-4 text-right">
                                <div
                                    class="flex items-center justify-end gap-1"
                                >
                                    <NuxtLink
                                        :to="`/employees/${employee.id}`"
                                        class="p-2 text-gray-600 hover:text-primary-600 hover:bg-primary-50 rounded-lg transition-colors"
                                        title="Просмотр"
                                    >
                                        <Icon name="ri:eye-line" />
                                    </NuxtLink>
                                    <NuxtLink
                                        v-if="isAdmin"
                                        :to="`/employees/${employee.id}/edit`"
                                        class="p-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors"
                                        title="Редактировать"
                                    >
                                        <Icon name="ri:edit-line" />
                                    </NuxtLink>
                                    <button
                                        v-if="isAdmin"
                                        @click="openDeleteModal(employee)"
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

        <ConfirmModal
            :show="showDeleteModal"
            :loading="deleting"
            title="Удалить сотрудника?"
            :message="
                employeeToDelete
                    ? `Вы уверены, что хотите удалить сотрудника ${employeeToDelete.full_name}? Это действие нельзя будет отменить.`
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

        <ConfirmModal
            :show="showBulkDeleteModal"
            :loading="bulkDeleting"
            title="Массовое удаление сотрудников"
            :message="`Вы уверены, что хотите удалить ${selectedIds.length} ${selectedIds.length === 1 ? 'сотрудника' : 'сотрудников'}? Это действие нельзя будет отменить.`"
            confirm-text="Да, удалить всех"
            cancel-text="Отмена"
            loading-text="Удаление..."
            variant="danger"
            icon="ri:delete-bin-line"
            @confirm="confirmBulkDelete"
            @cancel="closeBulkDeleteModal"
        />
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
    title: "Сотрудники",
});

const { isAdmin } = useAuth();
const {
    fetchEmployees,
    fetchDepartments,
    deleteEmployee,
    bulkDeleteEmployees,
} = useEmployees();

const searchQuery = ref("");
const currentPage = ref(1);
const perPage = ref(20);
const filters = ref({
    status: "",
    department_id: undefined as number | undefined,
});

const selectedIds = ref<number[]>([]);
const selectAll = ref(false);

const { data: departmentsData } = await useAsyncData("departments", () =>
    fetchDepartments(),
);
const departments = computed(() => departmentsData.value || []);

const { data, pending, error, refresh } = await useAsyncData(
    "employees",
    () =>
        fetchEmployees({
            page: currentPage.value,
            per_page: perPage.value,
            status: filters.value.status || undefined,
            department_id: filters.value.department_id,
        }),
    {
        watch: [currentPage],
    },
);

const employees = computed(() => data.value?.employees || []);
const totalPages = computed(() =>
    data.value ? Math.ceil(data.value.total / data.value.per_page) : 0,
);

const activeCount = computed(
    () => employees.value.filter((e) => e.status === "active").length,
);
const inactiveCount = computed(
    () => employees.value.filter((e) => e.status === "inactive").length,
);

const loadEmployees = () => {
    currentPage.value = 1;
    refresh();
};

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

const statusLabel = (status: string) => {
    const labels: Record<string, string> = {
        active: "Активен",
        inactive: "Неактивен",
        vacation: "В отпуске",
        sick_leave: "На больничном",
        terminated: "Уволен",
    };
    return labels[status] || status;
};

const resetFilters = () => {
    filters.value.status = "";
    filters.value.department_id = undefined;
    loadEmployees();
};

const showDeleteModal = ref(false);
const employeeToDelete = ref<any>(null);
const deleting = ref(false);

const showBulkDeleteModal = ref(false);
const bulkDeleting = ref(false);

const toggleSelectAll = () => {
    if (selectAll.value) {
        selectedIds.value = employees.value.map((e) => e.id);
    } else {
        selectedIds.value = [];
    }
};

const toggleSelect = (id: number) => {
    const index = selectedIds.value.indexOf(id);
    if (index > -1) {
        selectedIds.value.splice(index, 1);
    } else {
        selectedIds.value.push(id);
    }
    selectAll.value = selectedIds.value.length === employees.value.length;
};

watch(employees, () => {
    selectedIds.value = selectedIds.value.filter((id) =>
        employees.value.some((e) => e.id === id),
    );
    selectAll.value =
        selectedIds.value.length > 0 &&
        selectedIds.value.length === employees.value.length;
});

const openDeleteModal = (employee: any) => {
    employeeToDelete.value = employee;
    showDeleteModal.value = true;
};

const closeDeleteModal = () => {
    if (!deleting.value) {
        showDeleteModal.value = false;
        employeeToDelete.value = null;
    }
};

const confirmDelete = async () => {
    if (!employeeToDelete.value) return;

    deleting.value = true;
    try {
        await deleteEmployee(employeeToDelete.value.id);
        await refresh();
        showDeleteModal.value = false;
        employeeToDelete.value = null;
    } catch (err: any) {
        if (err.statusCode === 401) {
            alert("Недостаточно прав. Требуется роль администратора для удаления сотрудников.");
        } else {
            alert("Ошибка при удалении сотрудника: " + (err.message || "Неизвестная ошибка"));
        }
    } finally {
        deleting.value = false;
    }
};

const openBulkDeleteModal = () => {
    if (selectedIds.value.length === 0) return;
    showBulkDeleteModal.value = true;
};

const closeBulkDeleteModal = () => {
    if (!bulkDeleting.value) {
        showBulkDeleteModal.value = false;
    }
};

const confirmBulkDelete = async () => {
    if (selectedIds.value.length === 0) return;

    bulkDeleting.value = true;
    try {
        const result = await bulkDeleteEmployees(selectedIds.value);
        if (result.failed_ids.length > 0) {
            alert(
                `Удалено: ${result.deleted_count}. Не удалось удалить: ${result.failed_ids.length}`,
            );
        }
        selectedIds.value = [];
        selectAll.value = false;
        await refresh();
        showBulkDeleteModal.value = false;
    } catch (err: any) {
        if (err.statusCode === 401) {
            alert("Недостаточно прав. Требуется роль администратора для удаления сотрудников.");
        } else {
            alert("Ошибка при массовом удалении сотрудников: " + (err.message || "Неизвестная ошибка"));
        }
    } finally {
        bulkDeleting.value = false;
    }
};
</script>
