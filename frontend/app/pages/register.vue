<template>
    <div
        class="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary-50 via-white to-purple-50 p-4"
    >
        <div class="w-full max-w-md">
            <div class="card">
                <div class="p-8">
                    <div class="text-center mb-8">
                        <div
                            class="inline-flex h-16 w-16 items-center justify-center rounded-2xl gradient-secondary mb-4"
                        >
                            <Icon
                                name="ri:user-add-line"
                                class="text-3xl text-white"
                            />
                        </div>
                        <h1 class="text-2xl font-bold text-gray-900">
                            Регистрация
                        </h1>
                        <p class="mt-2 text-sm text-gray-600">
                            Создайте новый аккаунт
                        </p>
                    </div>

                    <form @submit.prevent="handleRegister" class="space-y-4">
                        <div
                            v-if="error"
                            class="p-4 rounded-lg bg-red-50 text-red-800 text-sm flex items-start gap-2"
                        >
                            <Icon
                                name="ri:error-warning-line"
                                class="text-lg flex-shrink-0 mt-0.5"
                            />
                            <span>{{ error }}</span>
                        </div>

                        <div>
                            <label
                                for="name"
                                class="block text-sm font-medium text-gray-700 mb-1"
                                >Имя</label
                            >
                            <input
                                id="name"
                                v-model="credentials.name"
                                type="text"
                                required
                                class="input-field"
                                placeholder="Ваше имя"
                            />
                        </div>

                        <div>
                            <label
                                for="email"
                                class="block text-sm font-medium text-gray-700 mb-1"
                                >Email</label
                            >
                            <input
                                id="email"
                                v-model="credentials.email"
                                type="email"
                                required
                                class="input-field"
                                placeholder="your@email.com"
                            />
                        </div>

                        <div>
                            <label
                                for="password"
                                class="block text-sm font-medium text-gray-700 mb-1"
                                >Пароль</label
                            >
                            <input
                                id="password"
                                v-model="credentials.password"
                                type="password"
                                required
                                minlength="8"
                                class="input-field"
                                placeholder="Минимум 8 символов"
                            />
                        </div>

                        <button
                            type="submit"
                            :disabled="loading"
                            class="w-full btn btn-primary"
                        >
                            <Icon
                                v-if="loading"
                                name="ri:loader-4-line"
                                class="mr-2 animate-spin"
                            />
                            <Icon v-else name="ri:user-add-line" class="mr-2" />
                            {{
                                loading
                                    ? "Регистрация..."
                                    : "Зарегистрироваться"
                            }}
                        </button>

                        <div class="text-center">
                            <NuxtLink
                                to="/login"
                                class="text-sm text-primary-600 hover:text-primary-700 font-medium"
                            >
                                Уже есть аккаунт? Войти
                            </NuxtLink>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
definePageMeta({
    middleware: "guest",
    layout: false,
});

useHead({
    title: "Регистрация",
});

const { register } = useAuth();

const credentials = ref({
    name: "",
    email: "",
    password: "",
});

const loading = ref(false);
const error = ref("");

const handleRegister = async () => {
    if (credentials.value.password.length < 8) {
        error.value = "Пароль должен содержать минимум 8 символов";
        return;
    }

    loading.value = true;
    error.value = "";

    try {
        await register(credentials.value);
        await navigateTo("/");
    } catch (err: any) {
        error.value = err.message || "Ошибка регистрации";
    } finally {
        loading.value = false;
    }
};
</script>
