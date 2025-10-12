<template>
    <div class="flex items-center gap-3">
        <div
            class="h-10 w-10 rounded-full gradient-primary flex items-center justify-center text-white font-semibold"
        >
            {{ initials }}
        </div>
        <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-gray-900 truncate">
                {{ user?.email || "Пользователь" }}
            </p>
            <button
                @click="handleLogout"
                class="text-xs text-gray-500 hover:text-primary-600 transition-colors"
            >
                Выйти
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
const { user, logout } = useAuth();

const initials = computed(() => {
    if (!user.value?.email) return "U";
    return user.value.email.charAt(0).toUpperCase();
});

const handleLogout = () => {
    logout();
};
</script>
