<template>
    <div class="max-w-6xl mx-auto space-y-6">
        <div class="flex items-center gap-4">
            <NuxtLink
                :to="`/employees/${employeeId}`"
                class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
            >
                <Icon name="ri:arrow-left-line" class="text-xl" />
            </NuxtLink>
            <div>
                <h1 class="text-2xl font-bold text-gray-900">
                    Редактировать сотрудника
                </h1>
                <p class="text-sm text-gray-600">
                    Обновите информацию о сотруднике
                </p>
            </div>
        </div>

        <div v-if="loading" class="flex justify-center py-12">
            <Icon
                name="ri:loader-4-line"
                class="text-4xl text-primary-600 animate-spin"
            />
        </div>

        <form v-else @submit.prevent="handleSubmit" class="card">
            <div class="p-6 space-y-8">
                <!-- Личная информация -->
                <div>
                    <div class="flex items-center gap-3 mb-6">
                        <div
                            class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:user-line"
                                class="text-blue-600 text-xl"
                            />
                        </div>
                        <h3 class="text-lg font-semibold text-gray-900">
                            Личная информация
                        </h3>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Фамилия <span class="text-red-500">*</span>
                            </label>
                            <input
                                v-model="form.last_name"
                                type="text"
                                required
                                class="input-field"
                            />
                        </div>
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Имя <span class="text-red-500">*</span>
                            </label>
                            <input
                                v-model="form.first_name"
                                type="text"
                                required
                                class="input-field"
                            />
                        </div>
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Отчество
                            </label>
                            <input
                                v-model="form.middle_name"
                                type="text"
                                class="input-field"
                            />
                        </div>
                    </div>
                </div>

                <!-- Рабочая информация -->
                <div class="border-t border-gray-200 pt-8">
                    <div class="flex items-center gap-3 mb-6">
                        <div
                            class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:briefcase-line"
                                class="text-purple-600 text-xl"
                            />
                        </div>
                        <h3 class="text-lg font-semibold text-gray-900">
                            Рабочая информация
                        </h3>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Отдел
                            </label>
                            <select
                                v-model="form.department_id"
                                class="input-field"
                            >
                                <option :value="null">Не выбрано</option>
                                <option
                                    v-for="dept in departments"
                                    :key="dept.id"
                                    :value="dept.id"
                                >
                                    {{ dept.name }}
                                </option>
                            </select>
                        </div>
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Должность
                            </label>
                            <select
                                v-model="form.position_id"
                                class="input-field"
                            >
                                <option :value="null">Не выбрано</option>
                                <option
                                    v-for="pos in filteredPositions"
                                    :key="pos.id"
                                    :value="pos.id"
                                >
                                    {{ pos.name }}
                                    <template v-if="pos.department_name">
                                        ({{ pos.department_name }})
                                    </template>
                                </option>
                            </select>
                            <p
                                v-if="form.department_id"
                                class="mt-1 text-xs text-gray-500"
                            >
                                Показаны только должности выбранного отдела
                            </p>
                        </div>
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Дата приёма
                            </label>
                            <input
                                v-model="form.hire_date"
                                type="date"
                                class="input-field"
                            />
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
                                <option value="active">Активен</option>
                                <option value="inactive">Неактивен</option>
                                <option value="vacation">В отпуске</option>
                                <option value="sick_leave">
                                    На больничном
                                </option>
                            </select>
                        </div>

                        <div
                            v-if="form.status === 'inactive'"
                            class="md:col-span-2"
                        >
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Дата увольнения
                            </label>
                            <input
                                v-model="form.termination_date"
                                type="date"
                                class="input-field"
                                :min="form.hire_date || undefined"
                            />
                            <p class="mt-1 text-xs text-gray-500">
                                Укажите дату увольнения для неактивного
                                сотрудника
                            </p>
                        </div>
                    </div>
                </div>

                <!-- Контактная информация -->
                <div class="border-t border-gray-200 pt-8">
                    <div class="flex items-center gap-3 mb-6">
                        <div
                            class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:contacts-line"
                                class="text-green-600 text-xl"
                            />
                        </div>
                        <h3 class="text-lg font-semibold text-gray-900">
                            Контактная информация
                        </h3>
                    </div>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Email
                            </label>
                            <input
                                v-model="form.email"
                                type="email"
                                class="input-field"
                                placeholder="example@company.com"
                            />
                        </div>
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                Телефон
                            </label>
                            <input
                                v-model="form.phone"
                                type="tel"
                                class="input-field"
                                placeholder="+7 (999) 123-45-67"
                            />
                        </div>
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                            >
                                AD Username
                            </label>
                            <input
                                v-model="form.ad_username"
                                type="text"
                                class="input-field"
                                placeholder="username"
                            />
                        </div>
                    </div>
                </div>

                <!-- Примечания -->
                <div class="border-t border-gray-200 pt-8">
                    <div class="flex items-center gap-2 mb-3">
                        <Icon
                            name="ri:file-text-line"
                            class="text-gray-400 text-xl"
                        />
                        <label class="block text-sm font-medium text-gray-700">
                            Примечания
                        </label>
                    </div>
                    <textarea
                        v-model="form.notes"
                        rows="4"
                        class="input-field"
                        placeholder="Дополнительная информация о сотруднике..."
                    ></textarea>
                </div>

                <!-- Сообщение об ошибке -->
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
            </div>

            <!-- Footer -->
            <div
                class="px-6 py-4 bg-gray-50 border-t border-gray-200 flex items-center justify-between"
            >
                <p class="text-sm text-gray-500">
                    <span class="text-red-500">*</span> Обязательные поля
                </p>
                <div class="flex gap-3">
                    <NuxtLink
                        :to="`/employees/${employeeId}`"
                        class="btn btn-secondary"
                    >
                        <Icon name="ri:close-line" class="mr-2" />
                        Отмена
                    </NuxtLink>
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
                        <Icon v-else name="ri:save-line" class="mr-2" />
                        {{ saving ? "Сохранение..." : "Сохранить изменения" }}
                    </button>
                </div>
            </div>
        </form>
    </div>
</template>

<script setup lang="ts">
import type { UpdateEmployeeData } from "~/composables/useEmployees";

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
const { fetchEmployee, updateEmployee, fetchDepartments, fetchPositions } =
    useEmployees();

const employeeId = computed(() => Number(route.params.id));

useHead({
    title: "Редактировать сотрудника",
});

const loading = ref(true);
const saving = ref(false);
const errorMessage = ref("");

// Загружаем справочники
const { data: departmentsData } = await useAsyncData("departments", () =>
    fetchDepartments(),
);
const { data: positionsData } = await useAsyncData("positions", () =>
    fetchPositions(),
);

const departments = computed(() => departmentsData.value || []);
const positions = computed(() => positionsData.value || []);

const form = ref({
    first_name: "",
    last_name: "",
    middle_name: null as string | null,
    position_id: null as number | null,
    department_id: null as number | null,
    email: null as string | null,
    phone: null as string | null,
    ad_username: null as string | null,
    hire_date: null as string | null,
    termination_date: null as string | null,
    status: "active",
    notes: null as string | null,
});

const filteredPositions = computed(() => {
    if (!form.value.department_id) {
        return positions.value;
    }
    return positions.value.filter(
        (pos) =>
            pos.department_id === form.value.department_id ||
            pos.department_id === null,
    );
});

watch(
    () => form.value.department_id,
    (newDeptId) => {
        if (form.value.position_id) {
            const position = positions.value.find(
                (p) => p.id === form.value.position_id,
            );
            if (
                position &&
                position.department_id &&
                position.department_id !== newDeptId
            ) {
                form.value.position_id = null;
                toast.info(
                    "Должность сброшена, так как она не относится к выбранному отделу",
                );
            }
        }
    },
);

watch(
    () => form.value.status,
    (newStatus) => {
        if (newStatus !== "inactive") {
            form.value.termination_date = null;
        }
    },
);

const loadEmployee = async () => {
    loading.value = true;
    try {
        const employee = await fetchEmployee(employeeId.value);

        form.value = {
            first_name: employee.first_name,
            last_name: employee.last_name,
            middle_name: employee.middle_name || null,
            position_id: employee.position_id || null,
            department_id: employee.department_id || null,
            email: employee.email || null,
            phone: employee.phone || null,
            ad_username: employee.ad_username || null,
            hire_date: employee.hire_date || null,
            termination_date: employee.termination_date || null,
            status: employee.status,
            notes: employee.notes || null,
        };
    } catch (err: any) {
        toast.error("Ошибка загрузки данных сотрудника");
        router.push("/employees");
    } finally {
        loading.value = false;
    }
};

const handleSubmit = async () => {
    saving.value = true;
    errorMessage.value = "";

    try {
        const data: UpdateEmployeeData = {
            first_name: form.value.first_name,
            last_name: form.value.last_name,
            middle_name: form.value.middle_name || undefined,
            position_id: form.value.position_id || undefined,
            department_id: form.value.department_id || undefined,
            email: form.value.email || undefined,
            phone: form.value.phone || undefined,
            ad_username: form.value.ad_username || undefined,
            hire_date: form.value.hire_date || undefined,
            termination_date: form.value.termination_date || undefined,
            status: form.value.status,
            notes: form.value.notes || undefined,
        };

        await updateEmployee(employeeId.value, data);
        toast.success("Сотрудник успешно обновлен");
        router.push(`/employees/${employeeId.value}`);
    } catch (err: any) {
        errorMessage.value =
            err.data?.message ||
            err.message ||
            "Ошибка при обновлении сотрудника";
        toast.error(errorMessage.value);
    } finally {
        saving.value = false;
    }
};

onMounted(() => {
    loadEmployee();
});
</script>
