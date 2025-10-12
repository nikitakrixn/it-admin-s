<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl gradient-primary flex items-center justify-center shadow-lg"
                    >
                        <Icon
                            name="ri:dashboard-3-line"
                            class="text-3xl text-white"
                        />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            Панель управления
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Обзор основных показателей и статистики системы
                        </p>
                    </div>
                </div>
                <div class="flex items-center gap-2">
                    <button @click="refreshData" class="btn btn-secondary">
                        <Icon
                            name="ri:refresh-line"
                            class="mr-2"
                            :class="{ 'animate-spin': refreshing }"
                        />
                        Обновить
                    </button>
                    <button @click="exportData" class="btn btn-primary">
                        <Icon name="ri:download-line" class="mr-2" />
                        Экспорт
                    </button>
                </div>
            </div>
        </div>

        <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
            <NuxtLink to="/employees" class="card card-hover group">
                <div class="relative overflow-hidden">
                    <div class="p-6">
                        <div class="flex justify-between items-start">
                            <div class="flex-1">
                                <p
                                    class="text-sm font-medium text-gray-600 mb-2"
                                >
                                    Сотрудники
                                </p>
                                <div class="flex items-baseline gap-2 mb-3">
                                    <p class="text-3xl font-bold text-gray-900">
                                        {{ stats.employees }}
                                    </p>
                                    <span
                                        class="text-sm font-medium text-green-600 flex items-center"
                                    >
                                        <Icon
                                            name="ri:arrow-up-line"
                                            class="mr-1"
                                        />
                                        +12%
                                    </span>
                                </div>
                                <div
                                    class="flex items-center text-sm text-gray-500"
                                >
                                    <Icon
                                        name="ri:user-add-line"
                                        class="mr-1"
                                    />
                                    <span>+5 за неделю</span>
                                </div>
                            </div>
                            <div
                                class="flex h-14 w-14 items-center justify-center rounded-xl bg-blue-100 text-blue-600 group-hover:scale-110 transition-transform"
                            >
                                <Icon name="ri:team-line" class="text-2xl" />
                            </div>
                        </div>
                        <div class="mt-4 pt-4 border-t border-gray-100">
                            <div
                                class="flex items-center justify-between text-sm"
                            >
                                <span class="text-gray-600">Активных</span>
                                <span class="font-semibold text-gray-900">{{
                                    stats.employees - 3
                                }}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </NuxtLink>

            <NuxtLink to="/computers" class="card card-hover group">
                <div class="relative overflow-hidden">
                    <div class="p-6">
                        <div class="flex justify-between items-start">
                            <div class="flex-1">
                                <p
                                    class="text-sm font-medium text-gray-600 mb-2"
                                >
                                    Компьютеры
                                </p>
                                <div class="flex items-baseline gap-2 mb-3">
                                    <p class="text-3xl font-bold text-gray-900">
                                        {{ stats.computers }}
                                    </p>
                                    <span
                                        class="text-sm font-medium text-green-600 flex items-center"
                                    >
                                        <Icon
                                            name="ri:arrow-up-line"
                                            class="mr-1"
                                        />
                                        +5%
                                    </span>
                                </div>
                                <div
                                    class="flex items-center text-sm text-gray-500"
                                >
                                    <Icon
                                        name="ri:checkbox-circle-line"
                                        class="mr-1"
                                    />
                                    <span
                                        >{{ stats.computers - 8 }} онлайн</span
                                    >
                                </div>
                            </div>
                            <div
                                class="flex h-14 w-14 items-center justify-center rounded-xl bg-purple-100 text-purple-600 group-hover:scale-110 transition-transform"
                            >
                                <Icon
                                    name="ri:computer-line"
                                    class="text-2xl"
                                />
                            </div>
                        </div>
                        <div class="mt-4 pt-4 border-t border-gray-100">
                            <div
                                class="flex items-center justify-between text-sm"
                            >
                                <span class="text-gray-600"
                                    >Требуют внимания</span
                                >
                                <span class="font-semibold text-amber-600"
                                    >8</span
                                >
                            </div>
                        </div>
                    </div>
                </div>
            </NuxtLink>

            <NuxtLink to="/equipment" class="card card-hover group">
                <div class="relative overflow-hidden">
                    <div class="p-6">
                        <div class="flex justify-between items-start">
                            <div class="flex-1">
                                <p
                                    class="text-sm font-medium text-gray-600 mb-2"
                                >
                                    Оргтехника
                                </p>
                                <div class="flex items-baseline gap-2 mb-3">
                                    <p class="text-3xl font-bold text-gray-900">
                                        {{ stats.equipment }}
                                    </p>
                                    <span
                                        class="text-sm font-medium text-green-600 flex items-center"
                                    >
                                        <Icon
                                            name="ri:arrow-up-line"
                                            class="mr-1"
                                        />
                                        +8%
                                    </span>
                                </div>
                                <div
                                    class="flex items-center text-sm text-gray-500"
                                >
                                    <Icon name="ri:printer-line" class="mr-1" />
                                    <span>45 принтеров</span>
                                </div>
                            </div>
                            <div
                                class="flex h-14 w-14 items-center justify-center rounded-xl bg-green-100 text-green-600 group-hover:scale-110 transition-transform"
                            >
                                <Icon name="ri:printer-line" class="text-2xl" />
                            </div>
                        </div>
                        <div class="mt-4 pt-4 border-t border-gray-100">
                            <div
                                class="flex items-center justify-between text-sm"
                            >
                                <span class="text-gray-600">Принтеров</span>
                                <span class="font-semibold text-gray-900"
                                    >45</span
                                >
                            </div>
                        </div>
                    </div>
                </div>
            </NuxtLink>

            <NuxtLink to="/requests" class="card card-hover group">
                <div class="relative overflow-hidden">
                    <div class="p-6">
                        <div class="flex justify-between items-start">
                            <div class="flex-1">
                                <p
                                    class="text-sm font-medium text-gray-600 mb-2"
                                >
                                    Активные заявки
                                </p>
                                <div class="flex items-baseline gap-2 mb-3">
                                    <p class="text-3xl font-bold text-gray-900">
                                        {{ stats.requests }}
                                    </p>
                                    <span
                                        class="text-sm font-medium text-red-600 flex items-center"
                                    >
                                        <Icon
                                            name="ri:arrow-down-line"
                                            class="mr-1"
                                        />
                                        -3%
                                    </span>
                                </div>
                                <div
                                    class="flex items-center text-sm text-gray-500"
                                >
                                    <Icon name="ri:time-line" class="mr-1" />
                                    <span>5 в работе</span>
                                </div>
                            </div>
                            <div
                                class="flex h-14 w-14 items-center justify-center rounded-xl bg-amber-100 text-amber-600 group-hover:scale-110 transition-transform"
                            >
                                <Icon
                                    name="ri:file-list-3-line"
                                    class="text-2xl"
                                />
                            </div>
                        </div>
                        <div class="mt-4 pt-4 border-t border-gray-100">
                            <div
                                class="flex items-center justify-between text-sm"
                            >
                                <span class="text-gray-600">Новых сегодня</span>
                                <span class="font-semibold text-gray-900"
                                    >8</span
                                >
                            </div>
                        </div>
                    </div>
                </div>
            </NuxtLink>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <NuxtLink to="/requests/create" class="card card-hover group">
                <div class="p-5 flex items-center gap-4">
                    <div
                        class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-primary-100 text-primary-600 group-hover:bg-primary-200 transition-colors"
                    >
                        <Icon name="ri:add-circle-line" class="text-2xl" />
                    </div>
                    <div>
                        <p class="text-sm font-semibold text-gray-900">
                            Создать заявку
                        </p>
                        <p class="text-xs text-gray-500 mt-0.5">
                            Новая заявка в систему
                        </p>
                    </div>
                </div>
            </NuxtLink>

            <NuxtLink to="/employees/create" class="card card-hover group">
                <div class="p-5 flex items-center gap-4">
                    <div
                        class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-blue-100 text-blue-600 group-hover:bg-blue-200 transition-colors"
                    >
                        <Icon name="ri:user-add-line" class="text-2xl" />
                    </div>
                    <div>
                        <p class="text-sm font-semibold text-gray-900">
                            Добавить сотрудника
                        </p>
                        <p class="text-xs text-gray-500 mt-0.5">
                            Новый профиль сотрудника
                        </p>
                    </div>
                </div>
            </NuxtLink>

            <NuxtLink to="/network/scan" class="card card-hover group">
                <div class="p-5 flex items-center gap-4">
                    <div
                        class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-indigo-100 text-indigo-600 group-hover:bg-indigo-200 transition-colors"
                    >
                        <Icon name="ri:radar-line" class="text-2xl" />
                    </div>
                    <div>
                        <p class="text-sm font-semibold text-gray-900">
                            Сканировать сеть
                        </p>
                        <p class="text-xs text-gray-500 mt-0.5">
                            Поиск устройств в сети
                        </p>
                    </div>
                </div>
            </NuxtLink>

            <NuxtLink to="/analytics" class="card card-hover group">
                <div class="p-5 flex items-center gap-4">
                    <div
                        class="flex h-12 w-12 shrink-0 items-center justify-center rounded-xl bg-purple-100 text-purple-600 group-hover:bg-purple-200 transition-colors"
                    >
                        <Icon name="ri:line-chart-line" class="text-2xl" />
                    </div>
                    <div>
                        <p class="text-sm font-semibold text-gray-900">
                            Аналитика
                        </p>
                        <p class="text-xs text-gray-500 mt-0.5">
                            Отчеты и статистика
                        </p>
                    </div>
                </div>
            </NuxtLink>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <div class="lg:col-span-2 card">
                <div
                    class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <div class="flex items-center justify-between">
                        <div class="flex items-center gap-3">
                            <div
                                class="h-10 w-10 rounded-lg bg-primary-100 flex items-center justify-center"
                            >
                                <Icon
                                    name="ri:history-line"
                                    class="text-primary-600 text-xl"
                                />
                            </div>
                            <h2 class="text-lg font-semibold text-gray-900">
                                Последние действия
                            </h2>
                        </div>
                        <NuxtLink
                            to="/logs"
                            class="text-sm text-primary-600 hover:text-primary-700 font-medium"
                        >
                            Все действия →
                        </NuxtLink>
                    </div>
                </div>
                <div class="p-6">
                    <ActivityTimeline :activities="recentActivities" />
                </div>
            </div>

            <div class="card">
                <div
                    class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="h-10 w-10 rounded-lg bg-green-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:server-line"
                                class="text-green-600 text-xl"
                            />
                        </div>
                        <h2 class="text-lg font-semibold text-gray-900">
                            Статус системы
                        </h2>
                    </div>
                </div>
                <div class="p-6 space-y-4">
                    <SystemStatus label="API Сервер" status="online" />
                    <SystemStatus label="База данных" status="online" />
                    <SystemStatus label="Active Directory" status="online" />
                    <SystemStatus
                        label="Резервное копирование"
                        status="warning"
                        message="2 часа назад"
                    />
                </div>
            </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div class="card">
                <div
                    class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="h-10 w-10 rounded-lg bg-amber-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:calendar-check-line"
                                class="text-amber-600 text-xl"
                            />
                        </div>
                        <h2 class="text-lg font-semibold text-gray-900">
                            Предстоящие события
                        </h2>
                    </div>
                </div>
                <div class="p-6 space-y-3">
                    <div
                        class="flex items-start gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors cursor-pointer"
                    >
                        <div
                            class="flex-shrink-0 w-12 h-12 rounded-lg bg-red-100 flex flex-col items-center justify-center"
                        >
                            <span class="text-xs text-red-600 font-medium"
                                >ДЕК</span
                            >
                            <span class="text-lg font-bold text-red-600"
                                >15</span
                            >
                        </div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-gray-900">
                                Обновление антивируса
                            </p>
                            <p class="text-xs text-gray-500 mt-1">
                                Все рабочие станции
                            </p>
                        </div>
                        <span class="badge bg-red-100 text-red-800 text-xs"
                            >Срочно</span
                        >
                    </div>

                    <div
                        class="flex items-start gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors cursor-pointer"
                    >
                        <div
                            class="flex-shrink-0 w-12 h-12 rounded-lg bg-blue-100 flex flex-col items-center justify-center"
                        >
                            <span class="text-xs text-blue-600 font-medium"
                                >ДЕК</span
                            >
                            <span class="text-lg font-bold text-blue-600"
                                >18</span
                            >
                        </div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-gray-900">
                                Резервное копирование
                            </p>
                            <p class="text-xs text-gray-500 mt-1">
                                Еженедельная проверка
                            </p>
                        </div>
                        <span class="badge bg-blue-100 text-blue-800 text-xs"
                            >Плановое</span
                        >
                    </div>

                    <div
                        class="flex items-start gap-3 p-3 rounded-lg hover:bg-gray-50 transition-colors cursor-pointer"
                    >
                        <div
                            class="flex-shrink-0 w-12 h-12 rounded-lg bg-green-100 flex flex-col items-center justify-center"
                        >
                            <span class="text-xs text-green-600 font-medium"
                                >ДЕК</span
                            >
                            <span class="text-lg font-bold text-green-600"
                                >20</span
                            >
                        </div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-gray-900">
                                Инвентаризация
                            </p>
                            <p class="text-xs text-gray-500 mt-1">
                                Оборудование офиса
                            </p>
                        </div>
                        <span class="badge bg-green-100 text-green-800 text-xs"
                            >Запланировано</span
                        >
                    </div>
                </div>
            </div>

            <div class="card">
                <div
                    class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                >
                    <div class="flex items-center gap-3">
                        <div
                            class="h-10 w-10 rounded-lg bg-red-100 flex items-center justify-center"
                        >
                            <Icon
                                name="ri:alert-line"
                                class="text-red-600 text-xl"
                            />
                        </div>
                        <h2 class="text-lg font-semibold text-gray-900">
                            Требуют внимания
                        </h2>
                    </div>
                </div>
                <div class="p-6 space-y-3">
                    <div
                        class="flex items-start gap-3 p-3 rounded-lg bg-red-50 border border-red-100"
                    >
                        <div class="flex-shrink-0">
                            <Icon
                                name="ri:error-warning-line"
                                class="text-xl text-red-600"
                            />
                        </div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-gray-900">
                                Истекает лицензия
                            </p>
                            <p class="text-xs text-gray-600 mt-1">
                                Office 365 - через 5 дней
                            </p>
                        </div>
                    </div>

                    <div
                        class="flex items-start gap-3 p-3 rounded-lg bg-amber-50 border border-amber-100"
                    >
                        <div class="flex-shrink-0">
                            <Icon
                                name="ri:alert-line"
                                class="text-xl text-amber-600"
                            />
                        </div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-gray-900">
                                Низкое место на диске
                            </p>
                            <p class="text-xs text-gray-600 mt-1">
                                Сервер DB-01 - осталось 15%
                            </p>
                        </div>
                    </div>

                    <div
                        class="flex items-start gap-3 p-3 rounded-lg bg-blue-50 border border-blue-100"
                    >
                        <div class="flex-shrink-0">
                            <Icon
                                name="ri:information-line"
                                class="text-xl text-blue-600"
                            />
                        </div>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium text-gray-900">
                                Доступно обновление
                            </p>
                            <p class="text-xs text-gray-600 mt-1">
                                Windows Server 2022
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <div class="card">
            <div
                class="px-6 py-5 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
            >
                <div class="flex items-center gap-3">
                    <div
                        class="h-10 w-10 rounded-lg bg-blue-100 flex items-center justify-center"
                    >
                        <Icon
                            name="ri:information-line"
                            class="text-blue-600 text-xl"
                        />
                    </div>
                    <h2 class="text-lg font-semibold text-gray-900">
                        Информация о системе
                    </h2>
                </div>
            </div>

            <div
                class="p-6 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-5"
            >
                <div
                    class="p-5 bg-gradient-to-br from-green-50 to-emerald-50 rounded-xl border border-green-100"
                >
                    <div class="flex items-center gap-3 mb-3">
                        <div
                            class="w-10 h-10 rounded-lg bg-green-100 flex items-center justify-center text-green-600"
                        >
                            <Icon name="ri:server-line" class="text-xl" />
                        </div>
                        <div>
                            <p class="text-xs text-gray-600">Статус сервера</p>
                            <p class="text-sm font-semibold text-gray-900">
                                Активен
                            </p>
                        </div>
                    </div>
                    <div class="flex items-center text-xs text-gray-500">
                        <div
                            class="w-2 h-2 rounded-full bg-green-500 animate-pulse mr-2"
                        ></div>
                        Uptime: 99.9%
                    </div>
                </div>

                <div
                    class="p-5 bg-gradient-to-br from-blue-50 to-cyan-50 rounded-xl border border-blue-100"
                >
                    <div class="flex items-center gap-3 mb-3">
                        <div
                            class="w-10 h-10 rounded-lg bg-blue-100 flex items-center justify-center text-blue-600"
                        >
                            <Icon name="ri:database-2-line" class="text-xl" />
                        </div>
                        <div>
                            <p class="text-xs text-gray-600">База данных</p>
                            <p class="text-sm font-semibold text-gray-900">
                                Подключена
                            </p>
                        </div>
                    </div>
                    <div class="flex items-center text-xs text-gray-500">
                        <Icon name="ri:check-line" class="mr-1" />
                        PostgreSQL 15.2
                    </div>
                </div>

                <div
                    class="p-5 bg-gradient-to-br from-purple-50 to-violet-50 rounded-xl border border-purple-100"
                >
                    <div class="flex items-center gap-3 mb-3">
                        <div
                            class="w-10 h-10 rounded-lg bg-purple-100 flex items-center justify-center text-purple-600"
                        >
                            <Icon
                                name="ri:user-settings-line"
                                class="text-xl"
                            />
                        </div>
                        <div>
                            <p class="text-xs text-gray-600">
                                Active Directory
                            </p>
                            <p class="text-sm font-semibold text-gray-900">
                                Синхронизирован
                            </p>
                        </div>
                    </div>
                    <div class="flex items-center text-xs text-gray-500">
                        <Icon name="ri:time-line" class="mr-1" />
                        Обновлено 5 мин назад
                    </div>
                </div>

                <div
                    class="p-5 bg-gradient-to-br from-amber-50 to-orange-50 rounded-xl border border-amber-100"
                >
                    <div class="flex items-center gap-3 mb-3">
                        <div
                            class="w-10 h-10 rounded-lg bg-amber-100 flex items-center justify-center text-amber-600"
                        >
                            <Icon
                                name="ri:notification-3-line"
                                class="text-xl"
                            />
                        </div>
                        <div>
                            <p class="text-xs text-gray-600">Уведомления</p>
                            <p class="text-sm font-semibold text-gray-900">
                                Включены
                            </p>
                        </div>
                    </div>
                    <div class="flex items-center text-xs text-gray-500">
                        <Icon name="ri:mail-line" class="mr-1" />
                        Email + Push
                    </div>
                </div>
            </div>
        </div>
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
    title: "Панель управления",
});

const toast = useToast();
const refreshing = ref(false);

const stats = ref({
    employees: 156,
    computers: 142,
    requests: 23,
    equipment: 89,
});

const recentActivities = ref([
    {
        id: 1,
        type: "user",
        message: "Добавлен новый сотрудник: Иванов И.И.",
        time: "5 минут назад",
        icon: "ri:user-add-line",
        color: "blue",
    },
    {
        id: 2,
        type: "computer",
        message: "Зарегистрирован компьютер: PC-2024-001",
        time: "15 минут назад",
        icon: "ri:computer-line",
        color: "purple",
    },
    {
        id: 3,
        type: "request",
        message: "Закрыта заявка #1234",
        time: "1 час назад",
        icon: "ri:checkbox-circle-line",
        color: "green",
    },
    {
        id: 4,
        type: "equipment",
        message: "Добавлен принтер HP LaserJet Pro",
        time: "2 часа назад",
        icon: "ri:printer-line",
        color: "amber",
    },
]);

const refreshData = async () => {
    refreshing.value = true;
    toast.info("Обновление данных...");

    await new Promise((resolve) => setTimeout(resolve, 1000));

    stats.value.employees += Math.floor(Math.random() * 3);
    stats.value.requests = Math.floor(Math.random() * 30);

    refreshing.value = false;
    toast.success("Данные обновлены!");
};

const exportData = () => {
    toast.success(
        "Экспорт данных начат. Файл будет загружен через несколько секунд.",
    );
};
</script>
