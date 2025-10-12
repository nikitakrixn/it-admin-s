<template>
    <div class="max-w-6xl mx-auto space-y-6">
        <div class="flex items-center gap-4 mb-2">
            <NuxtLink
                to="/employees"
                class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
            >
                <Icon name="ri:arrow-left-line" class="text-xl" />
            </NuxtLink>
            <div>
                <h1 class="text-2xl font-bold text-gray-900">
                    Добавить сотрудника
                </h1>
                <p class="text-sm text-gray-600">
                    Заполните информацию о новом сотруднике
                </p>
            </div>
        </div>

        <form @submit.prevent="handleSubmit" class="card">
            <div class="p-6 space-y-8">
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
                                >Отчество</label
                            >
                            <input
                                v-model="form.middle_name"
                                type="text"
                                class="input-field"
                            />
                        </div>
                    </div>
                </div>

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
                                >Отдел</label
                            >
                            <select
                                v-model="form.department_id"
                                class="input-field"
                            >
                                <option :value="undefined">Не выбрано</option>
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
                                >Должность</label
                            >
                            <select
                                v-model="form.position_id"
                                class="input-field"
                            >
                                <option :value="undefined">Не выбрано</option>
                                <option
                                    v-for="pos in positions"
                                    :key="pos.id"
                                    :value="pos.id"
                                >
                                    {{ pos.name }}
                                </option>
                            </select>
                        </div>
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                                >Дата приёма</label
                            >
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
                                <option value="terminated">Уволен</option>
                            </select>
                        </div>
                    </div>
                </div>

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
                                >Email</label
                            >
                            <input
                                v-model="form.email"
                                type="email"
                                class="input-field"
                            />
                        </div>
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                                >Телефон</label
                            >
                            <input
                                v-model="form.phone"
                                type="tel"
                                class="input-field"
                            />
                        </div>
                        <div>
                            <label
                                class="block text-sm font-medium text-gray-700 mb-2"
                                >AD Username</label
                            >
                            <input
                                v-model="form.ad_username"
                                type="text"
                                class="input-field"
                            />
                        </div>
                    </div>
                </div>

                <div class="border-t border-gray-200 pt-8">
                    <div class="flex items-center gap-2 mb-3">
                        <Icon
                            name="ri:file-text-line"
                            class="text-gray-400 text-xl"
                        />
                        <label class="block text-sm font-medium text-gray-700"
                            >Примечания</label
                        >
                    </div>
                    <textarea
                        v-model="form.notes"
                        rows="4"
                        class="input-field"
                    ></textarea>
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
            </div>

            <div
                class="px-6 py-4 bg-gray-50 border-t border-gray-200 flex items-center justify-between"
            >
                <p class="text-sm text-gray-500">
                    <span class="text-red-500">*</span> Обязательные поля
                </p>
                <div class="flex gap-3">
                    <NuxtLink to="/employees" class="btn btn-secondary">
                        <Icon name="ri:close-line" class="mr-2" />
                        Отмена
                    </NuxtLink>
                    <button
                        type="submit"
                        :disabled="loading"
                        class="btn btn-primary"
                    >
                        <Icon
                            v-if="loading"
                            name="ri:loader-4-line"
                            class="mr-2 animate-spin"
                        />
                        <Icon v-else name="ri:save-line" class="mr-2" />
                        {{ loading ? "Сохранение..." : "Сохранить" }}
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
    title: "Добавить сотрудника",
});

const { createEmployee, fetchDepartments, fetchPositions } = useEmployees();

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
    middle_name: "",
    position_id: undefined as number | undefined,
    department_id: undefined as number | undefined,
    email: "",
    phone: "",
    ad_username: "",
    hire_date: "",
    status: "active",
    notes: "",
});

const loading = ref(false);
const errorMessage = ref("");

const handleSubmit = async () => {
    loading.value = true;
    errorMessage.value = "";

    try {
        const data: any = {
            first_name: form.value.first_name,
            last_name: form.value.last_name,
            status: form.value.status,
        };

        if (form.value.middle_name) data.middle_name = form.value.middle_name;
        if (form.value.position_id) data.position_id = form.value.position_id;
        if (form.value.department_id)
            data.department_id = form.value.department_id;
        if (form.value.email) data.email = form.value.email;
        if (form.value.phone) data.phone = form.value.phone;
        if (form.value.ad_username) data.ad_username = form.value.ad_username;
        if (form.value.hire_date) data.hire_date = form.value.hire_date;
        if (form.value.notes) data.notes = form.value.notes;

        await createEmployee(data);

        const toast = useToast();
        toast.success("Сотрудник успешно создан");

        await navigateTo("/employees");
    } catch (err: any) {
        errorMessage.value = err.message || "Ошибка при создании сотрудника";
    } finally {
        loading.value = false;
    }
};
</script>
