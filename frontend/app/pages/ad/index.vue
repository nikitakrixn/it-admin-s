<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl bg-gradient-to-br from-blue-500 to-indigo-700 flex items-center justify-center shadow-lg"
                    >
                        <Icon
                            name="ri:account-circle-line"
                            class="text-3xl text-white"
                        />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Управление Active Directory
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Создание и управление учетными записями пользователей
                        </p>
                    </div>
                </div>
                <div class="flex gap-2">
                    <NuxtLink to="/ad/users" class="btn btn-secondary">
                        <Icon name="ri:user-search-line" class="mr-2" />
                        Список пользователей
                    </NuxtLink>
                    <NuxtLink to="/ad/groups" class="btn btn-secondary">
                        <Icon name="ri:group-line" class="mr-2" />
                        Группы
                    </NuxtLink>
                    <NuxtLink to="/ad/linked-employees" class="btn btn-secondary">
                        <Icon name="ri:link" class="mr-2" />
                        Привязка
                    </NuxtLink>
                </div>
            </div>
        </div>

        <div class="grid grid-cols-1 gap-6 lg:grid-cols-2">
            <!-- Форма создания пользователя -->
            <div class="card">
                <div
                    class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <div class="flex items-center">
                        <div
                            class="h-10 w-10 rounded-full bg-primary-100 flex items-center justify-center mr-3"
                        >
                            <Icon
                                name="ri:user-add-line"
                                class="text-primary-600"
                            />
                        </div>
                        <div>
                            <h3 class="text-lg font-medium text-gray-900">
                                Создание пользователя AD
                            </h3>
                            <p class="text-sm text-gray-500">
                                Заполните форму для создания нового пользователя
                            </p>
                        </div>
                    </div>
                </div>

                <form @submit.prevent="handleSubmit" class="p-6 space-y-6">
                    <!-- Персональные данные -->
                    <div>
                        <h4
                            class="text-sm font-medium text-gray-700 mb-4 pb-2 border-b"
                        >
                            Персональные данные
                        </h4>
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="label">Имя *</label>
                                <input
                                    v-model="form.firstName"
                                    type="text"
                                    required
                                    class="input-field"
                                    placeholder="Иван"
                                />
                            </div>
                            <div>
                                <label class="label">Фамилия *</label>
                                <input
                                    v-model="form.lastName"
                                    type="text"
                                    required
                                    class="input-field"
                                    placeholder="Иванов"
                                />
                            </div>
                        </div>
                    </div>

                    <!-- Учетные данные -->
                    <div>
                        <h4
                            class="text-sm font-medium text-gray-700 mb-4 pb-2 border-b"
                        >
                            Учетные данные
                        </h4>
                        <div class="space-y-4">
                            <div>
                                <label class="label">Имя пользователя *</label>
                                <div class="relative">
                                    <Icon
                                        name="ri:user-3-line"
                                        class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                                    />
                                    <input
                                        v-model="form.username"
                                        type="text"
                                        required
                                        class="input-field pl-10"
                                        placeholder="ivanov"
                                    />
                                </div>
                            </div>

                            <div>
                                <label class="label">Пароль *</label>
                                <div class="relative">
                                    <Icon
                                        name="ri:lock-line"
                                        class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                                    />
                                    <input
                                        v-model="form.password"
                                        :type="showPassword ? 'text' : 'password'"
                                        required
                                        class="input-field pl-10 pr-10"
                                        placeholder="••••••••"
                                    />
                                    <button
                                        type="button"
                                        @click="showPassword = !showPassword"
                                        class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600"
                                    >
                                        <Icon
                                            :name="
                                                showPassword
                                                    ? 'ri:eye-close-line'
                                                    : 'ri:eye-line'
                                            "
                                        />
                                    </button>
                                </div>
                                <div class="flex justify-between mt-1">
                                    <p class="text-xs text-gray-500">
                                        Минимум 8 символов
                                    </p>
                                    <button
                                        type="button"
                                        @click="generatePassword"
                                        class="text-xs text-primary-600 hover:text-primary-800"
                                    >
                                        Сгенерировать
                                    </button>
                                </div>
                            </div>

                            <div>
                                <label class="label">Email</label>
                                <div class="relative">
                                    <Icon
                                        name="ri:mail-line"
                                        class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                                    />
                                    <input
                                        v-model="form.email"
                                        type="email"
                                        class="input-field pl-10"
                                        placeholder="ivanov@company.com"
                                    />
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Организация -->
                    <div>
                        <h4
                            class="text-sm font-medium text-gray-700 mb-4 pb-2 border-b"
                        >
                            Организация
                        </h4>
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="label">Отдел</label>
                                <input
                                    v-model="form.department"
                                    type="text"
                                    class="input-field"
                                    placeholder="ИТ отдел"
                                />
                            </div>
                            <div>
                                <label class="label">Должность</label>
                                <input
                                    v-model="form.position"
                                    type="text"
                                    class="input-field"
                                    placeholder="Системный администратор"
                                />
                            </div>
                        </div>
                    </div>

                    <div class="flex justify-end">
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
                            <Icon
                                v-else
                                name="ri:user-add-line"
                                class="mr-2"
                            />
                            {{ saving ? "Создание..." : "Создать пользователя" }}
                        </button>
                    </div>
                </form>
            </div>

            <!-- Правая колонка -->
            <div class="space-y-6">
                <!-- Информация о подключении -->
                <div class="card">
                    <div
                        class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <div class="flex items-center">
                            <div
                                class="h-10 w-10 rounded-full bg-indigo-100 flex items-center justify-center mr-3"
                            >
                                <Icon
                                    name="ri:server-line"
                                    class="text-indigo-600"
                                />
                            </div>
                            <div>
                                <h3 class="text-lg font-medium text-gray-900">
                                    Информация о подключении
                                </h3>
                                <p class="text-sm text-gray-500">
                                    Статус соединения с контроллером домена
                                </p>
                            </div>
                        </div>
                    </div>

                    <div class="p-6 space-y-4">
                        <div>
                            <dt class="text-sm font-medium text-gray-500">
                                Сервер
                            </dt>
                            <dd class="mt-1 flex items-center">
                                <span class="font-mono text-sm text-gray-900"
                                    >dc.company.local</span
                                >
                                <span
                                    class="ml-2 h-2 w-2 rounded-full bg-gray-400"
                                    title="Не настроено"
                                ></span>
                            </dd>
                        </div>

                        <div>
                            <dt class="text-sm font-medium text-gray-500">
                                Домен
                            </dt>
                            <dd class="mt-1 font-mono text-sm text-gray-900">
                                COMPANY
                            </dd>
                        </div>

                        <div>
                            <dt class="text-sm font-medium text-gray-500">
                                Статус соединения
                            </dt>
                            <dd class="mt-1">
                                <span
                                    class="badge bg-gray-100 text-gray-800"
                                >
                                    <Icon
                                        name="ri:error-warning-line"
                                        class="mr-1"
                                    />
                                    Не настроено
                                </span>
                                <p class="mt-1 text-xs text-gray-500">
                                    Интеграция с AD находится в разработке
                                </p>
                            </dd>
                        </div>
                    </div>
                </div>

                <!-- Статистика -->
                <div class="card">
                    <div
                        class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <div class="flex items-center">
                            <div
                                class="h-10 w-10 rounded-full bg-green-100 flex items-center justify-center mr-3"
                            >
                                <Icon
                                    name="ri:pie-chart-line"
                                    class="text-green-600"
                                />
                            </div>
                            <div>
                                <h3 class="text-lg font-medium text-gray-900">
                                    Статистика Active Directory
                                </h3>
                                <p class="text-sm text-gray-500">
                                    Демо данные
                                </p>
                            </div>
                        </div>
                    </div>

                    <div class="p-6">
                        <div class="grid grid-cols-2 gap-4">
                            <div class="bg-gray-50 rounded-lg p-4">
                                <div class="flex items-center">
                                    <div
                                        class="rounded-md bg-primary-500 p-3"
                                    >
                                        <Icon
                                            name="ri:user-line"
                                            class="text-white text-xl"
                                        />
                                    </div>
                                    <div class="ml-4">
                                        <div
                                            class="text-sm font-medium text-gray-500"
                                        >
                                            Пользователей
                                        </div>
                                        <div
                                            class="text-2xl font-bold text-gray-900"
                                        >
                                            0
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div class="bg-gray-50 rounded-lg p-4">
                                <div class="flex items-center">
                                    <div
                                        class="rounded-md bg-indigo-500 p-3"
                                    >
                                        <Icon
                                            name="ri:group-line"
                                            class="text-white text-xl"
                                        />
                                    </div>
                                    <div class="ml-4">
                                        <div
                                            class="text-sm font-medium text-gray-500"
                                        >
                                            Групп
                                        </div>
                                        <div
                                            class="text-2xl font-bold text-gray-900"
                                        >
                                            0
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
definePageMeta({
    middleware: "auth",
});

useHead({
    title: "Active Directory",
});

const showPassword = ref(false);
const saving = ref(false);

const form = ref({
    firstName: "",
    lastName: "",
    username: "",
    password: "",
    email: "",
    department: "",
    position: "",
});

const generatePassword = () => {
    const length = 12;
    const charset =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*";
    let password = "";

    // Гарантируем наличие разных типов символов
    password += "abcdefghijklmnopqrstuvwxyz"[Math.floor(Math.random() * 26)];
    password += "ABCDEFGHIJKLMNOPQRSTUVWXYZ"[Math.floor(Math.random() * 26)];
    password += "0123456789"[Math.floor(Math.random() * 10)];
    password += "!@#$%^&*"[Math.floor(Math.random() * 8)];

    for (let i = 4; i < length; i++) {
        password += charset[Math.floor(Math.random() * charset.length)];
    }

    // Перемешиваем
    password = password
        .split("")
        .sort(() => 0.5 - Math.random())
        .join("");

    form.value.password = password;
    showPassword.value = true;
};

const handleSubmit = async () => {
    saving.value = true;
    try {
        // TODO: Реализовать API для создания пользователя AD
        await new Promise((resolve) => setTimeout(resolve, 1000));
        alert(
            "Функция создания пользователя AD находится в разработке.\n\nДанные формы:\n" +
                JSON.stringify(form.value, null, 2),
        );
    } finally {
        saving.value = false;
    }
};
</script>
