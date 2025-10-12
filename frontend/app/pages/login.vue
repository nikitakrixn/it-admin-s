<template>
    <div
        class="min-h-screen flex items-center justify-center bg-gradient-to-br from-primary-50 via-white to-purple-50 p-4"
    >
        <div class="w-full max-w-md">
            <div class="card">
                <div class="p-8">
                    <div class="text-center mb-8">
                        <div
                            class="inline-flex h-16 w-16 items-center justify-center rounded-2xl gradient-primary mb-4"
                        >
                            <Icon
                                name="ri:lock-line"
                                class="text-3xl text-white"
                            />
                        </div>
                        <h1 class="text-2xl font-bold text-gray-900">
                            Вход в систему
                        </h1>
                        <p class="mt-2 text-sm text-gray-600">
                            IT-Admin Management System
                        </p>
                    </div>

                    <form @submit.prevent="handleLogin" class="space-y-4">
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
                                class="input-field"
                                placeholder="••••••••"
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
                            <Icon
                                v-else
                                name="ri:login-box-line"
                                class="mr-2"
                            />
                            {{ loading ? "Вход..." : "Войти" }}
                        </button>

                        <div class="text-center">
                            <NuxtLink
                                to="/register"
                                class="text-sm text-primary-600 hover:text-primary-700 font-medium"
                            >
                                Нет аккаунта? Зарегистрироваться
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
    title: "Вход",
});

const { login } = useAuth();

const credentials = ref({
    email: "",
    password: "",
});

const loading = ref(false);
const error = ref("");

const handleLogin = async () => {
    loading.value = true;
    error.value = "";

    try {
        await login(credentials.value);
        await navigateTo("/");
    } catch (err: any) {
        error.value = err.message || "Ошибка входа";
    } finally {
        loading.value = false;
    }
};
</script>
