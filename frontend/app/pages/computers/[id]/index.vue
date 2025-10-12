<template>
    <div class="space-y-6">
        <div class="mb-8">
            <div class="flex items-center gap-4 mb-6">
                <NuxtLink
                    to="/computers"
                    class="p-2 rounded-lg hover:bg-gray-100 transition-colors"
                >
                    <Icon name="ri:arrow-left-line" class="text-xl" />
                </NuxtLink>
                <div class="flex-1 flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <div
                            :class="[
                                'h-16 w-16 rounded-2xl flex items-center justify-center shadow-lg',
                                getTypeGradient(computer.computer_type),
                            ]"
                        >
                            <Icon
                                :name="getTypeIcon(computer.computer_type)"
                                class="text-3xl text-white"
                            />
                        </div>
                        <div>
                            <div class="flex items-center gap-3">
                                <h1 class="text-3xl font-bold text-gray-900">
                                    {{ computer.hostname }}
                                </h1>
                                <span
                                    :class="[
                                        'badge',
                                        getStatusColor(computer.status),
                                    ]"
                                >
                                    {{ getStatusLabel(computer.status) }}
                                </span>
                            </div>
                            <p class="mt-1 text-gray-600">
                                {{ computer.inventory_number }}
                            </p>
                        </div>
                    </div>

                    <div class="flex items-center gap-2">
                        <NuxtLink
                            :to="`/computers/${computer.id}/edit`"
                            class="btn btn-secondary"
                        >
                            <Icon name="ri:edit-line" class="mr-2" />
                            Редактировать
                        </NuxtLink>
                        <button
                            @click="showDeleteModal = true"
                            class="btn bg-red-50 text-red-600 hover:bg-red-100"
                        >
                            <Icon name="ri:delete-bin-line" class="mr-2" />
                            Удалить
                        </button>
                    </div>
                </div>
            </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <!-- Основная информация -->
            <div class="lg:col-span-2 space-y-6">
                <!-- Общая информация -->
                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <h2 class="text-lg font-semibold text-gray-900">
                            Общая информация
                        </h2>
                    </div>
                    <div class="p-6">
                        <div class="grid grid-cols-2 gap-6">
                            <div>
                                <p class="text-xs text-gray-600 mb-1">
                                    Производитель
                                </p>
                                <p class="text-sm font-medium text-gray-900">
                                    {{ computer.manufacturer }}
                                </p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">Модель</p>
                                <p class="text-sm font-medium text-gray-900">
                                    {{ computer.model }}
                                </p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">
                                    Серийный номер
                                </p>
                                <p
                                    class="text-sm font-medium text-gray-900 font-mono"
                                >
                                    {{ computer.serial_number }}
                                </p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">Тип</p>
                                <span class="badge bg-blue-100 text-blue-800">
                                    {{ getTypeLabel(computer.computer_type) }}
                                </span>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">
                                    Местоположение
                                </p>
                                <p class="text-sm font-medium text-gray-900">
                                    {{ computer.location || "-" }}
                                </p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">Домен</p>
                                <span
                                    :class="[
                                        'badge',
                                        computer.domain_joined
                                            ? 'bg-green-100 text-green-800'
                                            : 'bg-gray-100 text-gray-800',
                                    ]"
                                >
                                    {{
                                        computer.domain_joined
                                            ? "Присоединен"
                                            : "Не присоединен"
                                    }}
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Операционная система -->
                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <div class="flex items-center gap-2">
                            <Icon
                                :name="getOSIcon(computer.os)"
                                class="text-gray-600"
                            />
                            <h2 class="text-lg font-semibold text-gray-900">
                                Операционная система
                            </h2>
                        </div>
                    </div>
                    <div class="p-6">
                        <div class="grid grid-cols-2 gap-6">
                            <div>
                                <p class="text-xs text-gray-600 mb-1">ОС</p>
                                <p class="text-sm font-medium text-gray-900">
                                    {{ computer.os }}
                                </p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">Версия</p>
                                <p class="text-sm font-medium text-gray-900">
                                    {{ computer.os_version }}
                                </p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">Сборка</p>
                                <p class="text-sm font-medium text-gray-900">
                                    {{ computer.os_build || "-" }}
                                </p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">
                                    Архитектура
                                </p>
                                <p class="text-sm font-medium text-gray-900">
                                    {{ computer.os_architecture || "-" }}
                                </p>
                            </div>
                            <div>
                                <p class="text-xs text-gray-600 mb-1">
                                    Последняя загрузка
                                </p>
                                <p class="text-sm font-medium text-gray-900">
                                    {{ computer.last_boot_time || "-" }}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Компоненты -->
                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <div class="flex items-center justify-between">
                            <h2 class="text-lg font-semibold text-gray-900">
                                Компоненты
                            </h2>
                            <NuxtLink
                                :to="`/computers/${computer.id}/components`"
                                class="btn btn-secondary btn-sm"
                            >
                                <Icon name="ri:list-check" class="mr-2" />
                                Управление
                            </NuxtLink>
                        </div>
                    </div>
                    <div class="p-6">
                        <div class="grid grid-cols-2 gap-4">
                            <!-- Процессор -->
                            <div
                                class="p-4 rounded-lg bg-blue-50 border border-blue-100 group hover:shadow-md transition-all cursor-pointer relative"
                            >
                                <NuxtLink
                                    :to="`/computers/${computer.id}/components/1`"
                                    class="absolute inset-0"
                                />
                                <div class="flex items-center gap-3 mb-3">
                                    <div
                                        class="h-10 w-10 rounded-lg bg-blue-500 flex items-center justify-center flex-shrink-0"
                                    >
                                        <Icon
                                            name="ri:cpu-line"
                                            class="text-xl text-white"
                                        />
                                    </div>
                                    <div class="min-w-0 flex-1">
                                        <p
                                            class="text-sm font-semibold text-gray-900 group-hover:text-blue-600 transition-colors"
                                        >
                                            Процессор
                                        </p>
                                        <p
                                            class="text-xs text-gray-600 truncate"
                                        >
                                            {{ computer.processor.name }}
                                        </p>
                                    </div>
                                    <button
                                        @click.prevent="
                                            editComponent(1, 'processor')
                                        "
                                        class="relative z-10 p-1.5 rounded-lg hover:bg-blue-100 transition-colors opacity-0 group-hover:opacity-100"
                                        title="Редактировать"
                                    >
                                        <Icon
                                            name="ri:edit-line"
                                            class="text-blue-600"
                                        />
                                    </button>
                                </div>
                                <div class="grid grid-cols-3 gap-2 text-xs">
                                    <div>
                                        <p class="text-gray-600">Ядра</p>
                                        <p class="font-medium text-gray-900">
                                            {{ computer.processor.cores }}
                                        </p>
                                    </div>
                                    <div>
                                        <p class="text-gray-600">Потоки</p>
                                        <p class="font-medium text-gray-900">
                                            {{ computer.processor.threads }}
                                        </p>
                                    </div>
                                    <div>
                                        <p class="text-gray-600">Частота</p>
                                        <p class="font-medium text-gray-900">
                                            {{ computer.processor.frequency }}
                                        </p>
                                    </div>
                                </div>
                            </div>

                            <!-- Оперативная память -->
                            <div
                                class="p-4 rounded-lg bg-purple-50 border border-purple-100 group hover:shadow-md transition-all cursor-pointer relative"
                            >
                                <NuxtLink
                                    :to="`/computers/${computer.id}/components`"
                                    class="absolute inset-0"
                                />
                                <div class="flex items-center gap-3 mb-3">
                                    <div
                                        class="h-10 w-10 rounded-lg bg-purple-500 flex items-center justify-center flex-shrink-0"
                                    >
                                        <Icon
                                            name="ri:database-2-line"
                                            class="text-xl text-white"
                                        />
                                    </div>
                                    <div class="flex-1">
                                        <p
                                            class="text-sm font-semibold text-gray-900 group-hover:text-purple-600 transition-colors"
                                        >
                                            Оперативная память
                                        </p>
                                        <p class="text-xs text-gray-600">
                                            Всего {{ computer.ram.total }} ГБ
                                        </p>
                                    </div>
                                    <button
                                        @click.prevent="
                                            router.push(
                                                `/computers/${computer.id}/components`,
                                            )
                                        "
                                        class="relative z-10 p-1.5 rounded-lg hover:bg-purple-100 transition-colors opacity-0 group-hover:opacity-100"
                                        title="Управление"
                                    >
                                        <Icon
                                            name="ri:settings-line"
                                            class="text-purple-600"
                                        />
                                    </button>
                                </div>
                                <div class="space-y-1">
                                    <div
                                        v-for="(module, index) in computer.ram
                                            .modules"
                                        :key="index"
                                        class="flex items-center justify-between text-xs"
                                    >
                                        <span class="text-gray-600"
                                            >Слот {{ index + 1 }}</span
                                        >
                                        <span class="font-medium text-gray-900"
                                            >{{ module.capacity }} ГБ
                                            {{ module.type }}</span
                                        >
                                    </div>
                                </div>
                            </div>

                            <!-- Накопители -->
                            <NuxtLink
                                :to="`/computers/${computer.id}/components`"
                                class="block p-4 rounded-lg bg-green-50 border border-green-100 hover:shadow-md transition-all"
                            >
                                <div class="flex items-center gap-3 mb-3">
                                    <div
                                        class="h-10 w-10 rounded-lg bg-green-500 flex items-center justify-center flex-shrink-0"
                                    >
                                        <Icon
                                            name="ri:hard-drive-line"
                                            class="text-xl text-white"
                                        />
                                    </div>
                                    <div>
                                        <p
                                            class="text-sm font-semibold text-gray-900 hover:text-green-600 transition-colors"
                                        >
                                            Накопители
                                        </p>
                                        <p class="text-xs text-gray-600">
                                            Всего
                                            {{ computer.storage.total }} ГБ
                                        </p>
                                    </div>
                                </div>
                                <div class="space-y-1">
                                    <div
                                        v-for="(drive, index) in computer
                                            .storage.drives"
                                        :key="index"
                                        class="text-xs"
                                    >
                                        <div
                                            class="flex items-center justify-between"
                                        >
                                            <span
                                                class="badge bg-green-100 text-green-800"
                                                >{{ drive.type }}</span
                                            >
                                            <span
                                                class="font-medium text-gray-900"
                                                >{{ drive.capacity }} ГБ</span
                                            >
                                        </div>
                                    </div>
                                </div>
                            </NuxtLink>

                            <!-- Видеокарта -->
                            <NuxtLink
                                :to="`/computers/${computer.id}/components/5`"
                                class="block p-4 rounded-lg bg-amber-50 border border-amber-100 hover:shadow-md transition-all"
                            >
                                <div class="flex items-center gap-3 mb-3">
                                    <div
                                        class="h-10 w-10 rounded-lg bg-amber-500 flex items-center justify-center flex-shrink-0"
                                    >
                                        <Icon
                                            name="ri:gamepad-line"
                                            class="text-xl text-white"
                                        />
                                    </div>
                                    <div class="min-w-0">
                                        <p
                                            class="text-sm font-semibold text-gray-900 hover:text-amber-600 transition-colors"
                                        >
                                            Видеокарта
                                        </p>
                                        <p
                                            class="text-xs text-gray-600 truncate"
                                        >
                                            {{ computer.gpu.name }}
                                        </p>
                                    </div>
                                </div>
                                <div class="grid grid-cols-2 gap-2 text-xs">
                                    <div>
                                        <p class="text-gray-600">Память</p>
                                        <p class="font-medium text-gray-900">
                                            {{ computer.gpu.memory }} ГБ
                                        </p>
                                    </div>
                                    <div>
                                        <p class="text-gray-600">Драйвер</p>
                                        <p class="font-medium text-gray-900">
                                            {{ computer.gpu.driver }}
                                        </p>
                                    </div>
                                </div>
                            </NuxtLink>

                            <!-- Материнская плата -->
                            <NuxtLink
                                :to="`/computers/${computer.id}/components/6`"
                                class="block p-4 rounded-lg bg-indigo-50 border border-indigo-100 hover:shadow-md transition-all"
                            >
                                <div class="flex items-center gap-3 mb-3">
                                    <div
                                        class="h-10 w-10 rounded-lg bg-indigo-500 flex items-center justify-center flex-shrink-0"
                                    >
                                        <Icon
                                            name="ri:circuit-board-line"
                                            class="text-xl text-white"
                                        />
                                    </div>
                                    <div class="min-w-0">
                                        <p
                                            class="text-sm font-semibold text-gray-900 hover:text-indigo-600 transition-colors"
                                        >
                                            Материнская плата
                                        </p>
                                        <p
                                            class="text-xs text-gray-600 truncate"
                                        >
                                            {{ computer.motherboard.model }}
                                        </p>
                                    </div>
                                </div>
                                <div class="grid grid-cols-2 gap-2 text-xs">
                                    <div>
                                        <p class="text-gray-600">Чипсет</p>
                                        <p class="font-medium text-gray-900">
                                            {{ computer.motherboard.chipset }}
                                        </p>
                                    </div>
                                    <div>
                                        <p class="text-gray-600">BIOS</p>
                                        <p class="font-medium text-gray-900">
                                            {{ computer.motherboard.bios }}
                                        </p>
                                    </div>
                                </div>
                            </NuxtLink>

                            <!-- Сетевые адаптеры -->
                            <NuxtLink
                                :to="`/computers/${computer.id}/components`"
                                class="block p-4 rounded-lg bg-cyan-50 border border-cyan-100 hover:shadow-md transition-all"
                            >
                                <div class="flex items-center gap-3 mb-3">
                                    <div
                                        class="h-10 w-10 rounded-lg bg-cyan-500 flex items-center justify-center flex-shrink-0"
                                    >
                                        <Icon
                                            name="ri:wifi-line"
                                            class="text-xl text-white"
                                        />
                                    </div>
                                    <div>
                                        <p
                                            class="text-sm font-semibold text-gray-900 hover:text-cyan-600 transition-colors"
                                        >
                                            Сетевые адаптеры
                                        </p>
                                        <p class="text-xs text-gray-600">
                                            {{
                                                computer.network.adapters.length
                                            }}
                                            шт.
                                        </p>
                                    </div>
                                </div>
                                <div class="space-y-1">
                                    <div
                                        v-for="(adapter, index) in computer
                                            .network.adapters"
                                        :key="index"
                                        class="text-xs"
                                    >
                                        <div
                                            class="flex items-center justify-between"
                                        >
                                            <span
                                                class="text-gray-600 truncate"
                                                >{{ adapter.name }}</span
                                            >
                                            <span
                                                class="badge bg-cyan-100 text-cyan-800"
                                                >{{ adapter.type }}</span
                                            >
                                        </div>
                                    </div>
                                </div>
                            </NuxtLink>

                            <!-- Мониторы -->
                            <NuxtLink
                                :to="`/computers/${computer.id}/components`"
                                class="block p-4 rounded-lg bg-pink-50 border border-pink-100 hover:shadow-md transition-all"
                            >
                                <div class="flex items-center gap-3 mb-3">
                                    <div
                                        class="h-10 w-10 rounded-lg bg-pink-500 flex items-center justify-center flex-shrink-0"
                                    >
                                        <Icon
                                            name="ri:monitor-line"
                                            class="text-xl text-white"
                                        />
                                    </div>
                                    <div>
                                        <p
                                            class="text-sm font-semibold text-gray-900 hover:text-pink-600 transition-colors"
                                        >
                                            Мониторы
                                        </p>
                                        <p class="text-xs text-gray-600">
                                            {{ computer.monitors.length }} шт.
                                        </p>
                                    </div>
                                </div>
                                <div class="space-y-1">
                                    <div
                                        v-for="(
                                            monitor, index
                                        ) in computer.monitors"
                                        :key="index"
                                        class="text-xs"
                                    >
                                        <div
                                            class="flex items-center justify-between"
                                        >
                                            <span class="text-gray-600">{{
                                                monitor.model
                                            }}</span>
                                            <span
                                                class="font-medium text-gray-900"
                                                >{{ monitor.size }}"
                                                {{ monitor.resolution }}</span
                                            >
                                        </div>
                                    </div>
                                </div>
                            </NuxtLink>

                            <!-- Периферия -->
                            <NuxtLink
                                :to="`/computers/${computer.id}/components`"
                                class="block p-4 rounded-lg bg-teal-50 border border-teal-100 hover:shadow-md transition-all"
                            >
                                <div class="flex items-center gap-3 mb-3">
                                    <div
                                        class="h-10 w-10 rounded-lg bg-teal-500 flex items-center justify-center flex-shrink-0"
                                    >
                                        <Icon
                                            name="ri:keyboard-line"
                                            class="text-xl text-white"
                                        />
                                    </div>
                                    <div>
                                        <p
                                            class="text-sm font-semibold text-gray-900 hover:text-teal-600 transition-colors"
                                        >
                                            Периферия
                                        </p>
                                        <p class="text-xs text-gray-600">
                                            {{
                                                computer.peripherals.length
                                            }}
                                            устройств
                                        </p>
                                    </div>
                                </div>
                                <div class="space-y-1">
                                    <div
                                        v-for="(
                                            device, index
                                        ) in computer.peripherals"
                                        :key="index"
                                        class="text-xs"
                                    >
                                        <div
                                            class="flex items-center justify-between"
                                        >
                                            <span class="text-gray-600">{{
                                                device.name
                                            }}</span>
                                            <span
                                                class="badge bg-teal-100 text-teal-800"
                                                >{{ device.type }}</span
                                            >
                                        </div>
                                    </div>
                                </div>
                            </NuxtLink>
                        </div>
                    </div>
                </div>

                <!-- Установленное ПО -->
                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <div class="flex items-center justify-between">
                            <div class="flex items-center gap-2">
                                <Icon
                                    name="ri:apps-line"
                                    class="text-gray-600"
                                />
                                <h2 class="text-lg font-semibold text-gray-900">
                                    Установленное ПО
                                </h2>
                                <span
                                    class="badge bg-primary-100 text-primary-800"
                                    >{{ computer.software.length }}</span
                                >
                            </div>
                            <NuxtLink
                                :to="`/computers/${computer.id}/software`"
                                class="btn btn-secondary btn-sm"
                            >
                                <Icon name="ri:list-check" class="mr-2" />
                                Управление
                            </NuxtLink>
                        </div>
                    </div>
                    <div class="p-6">
                        <div class="space-y-2">
                            <NuxtLink
                                v-for="software in computer.software"
                                :key="software.id"
                                :to="`/software/${software.id}`"
                                class="flex items-center justify-between p-3 rounded-lg hover:bg-gray-50 transition-colors"
                            >
                                <div class="flex items-center gap-3">
                                    <Icon
                                        name="ri:apps-2-line"
                                        class="text-gray-400"
                                    />
                                    <div>
                                        <p
                                            class="text-sm font-medium text-gray-900 hover:text-primary-600 transition-colors"
                                        >
                                            {{ software.name }}
                                        </p>
                                        <p class="text-xs text-gray-500">
                                            {{ software.manufacturer }}
                                        </p>
                                    </div>
                                </div>
                                <div class="text-right">
                                    <p
                                        class="text-sm font-medium text-gray-900"
                                    >
                                        {{ software.version }}
                                    </p>
                                    <span
                                        v-if="software.license_type"
                                        class="text-xs text-gray-500"
                                        >{{ software.license_type }}</span
                                    >
                                </div>
                            </NuxtLink>
                        </div>
                    </div>
                </div>

                <!-- Журнал активности -->
                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <div class="flex items-center gap-2">
                            <Icon
                                name="ri:history-line"
                                class="text-gray-600"
                            />
                            <h2 class="text-lg font-semibold text-gray-900">
                                Журнал активности
                            </h2>
                        </div>
                    </div>
                    <div class="p-6">
                        <div class="space-y-4">
                            <div
                                v-for="(event, index) in activityLog"
                                :key="event.id"
                                class="flex gap-4"
                            >
                                <div class="flex flex-col items-center">
                                    <div
                                        :class="[
                                            'h-8 w-8 rounded-full flex items-center justify-center flex-shrink-0',
                                            getActivityColor(event.type),
                                        ]"
                                    >
                                        <Icon
                                            :name="getActivityIcon(event.type)"
                                            class="text-sm text-white"
                                        />
                                    </div>
                                    <div
                                        v-if="index < activityLog.length - 1"
                                        class="w-0.5 flex-1 bg-gray-200 mt-2"
                                    />
                                </div>
                                <div class="flex-1 pb-4">
                                    <div
                                        class="flex items-start justify-between gap-4"
                                    >
                                        <div>
                                            <p class="text-sm text-gray-900">
                                                <span class="font-semibold">{{
                                                    event.user
                                                }}</span>
                                                {{ event.action }}
                                            </p>
                                            <p
                                                v-if="event.details"
                                                class="text-sm text-gray-600 mt-1"
                                            >
                                                {{ event.details }}
                                            </p>
                                        </div>
                                        <button
                                            v-if="event.hasDetails"
                                            @click="showActivityDetails(event)"
                                            class="text-xs text-primary-600 hover:text-primary-700 flex-shrink-0"
                                        >
                                            Подробнее
                                        </button>
                                    </div>
                                    <p class="text-xs text-gray-500 mt-1">
                                        {{ event.created_at }}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Боковая панель -->
            <div class="space-y-6">
                <!-- Статус и назначение -->
                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <h3 class="text-sm font-semibold text-gray-900">
                            Статус
                        </h3>
                    </div>
                    <div class="p-6 space-y-4">
                        <div>
                            <p class="text-xs text-gray-600 mb-2">
                                Текущий статус
                            </p>
                            <select
                                v-model="computer.status"
                                @change="updateStatus"
                                class="input-field"
                            >
                                <option value="active">Активный</option>
                                <option value="inactive">Неактивный</option>
                                <option value="repair">В ремонте</option>
                                <option value="storage">На складе</option>
                                <option value="decommissioned">Списан</option>
                            </select>
                        </div>

                        <div>
                            <p class="text-xs text-gray-600 mb-2">
                                Назначен сотруднику
                            </p>
                            <div
                                v-if="computer.employee"
                                class="flex items-center gap-2 p-3 rounded-lg bg-blue-50 border border-blue-100"
                            >
                                <div
                                    class="h-8 w-8 rounded-full bg-blue-500 flex items-center justify-center"
                                >
                                    <Icon
                                        name="ri:user-line"
                                        class="text-white"
                                    />
                                </div>
                                <div class="flex-1">
                                    <p
                                        class="text-sm font-medium text-gray-900"
                                    >
                                        {{ computer.employee.name }}
                                    </p>
                                    <p class="text-xs text-gray-600">
                                        {{ computer.employee.position }}
                                    </p>
                                </div>
                            </div>
                            <button
                                v-else
                                class="w-full btn btn-secondary btn-sm"
                            >
                                <Icon name="ri:user-add-line" class="mr-2" />
                                Назначить
                            </button>
                        </div>
                    </div>
                </div>

                <!-- Гарантия и покупка -->
                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <h3 class="text-sm font-semibold text-gray-900">
                            Гарантия и покупка
                        </h3>
                    </div>
                    <div class="p-6 space-y-4">
                        <div>
                            <p class="text-xs text-gray-600 mb-1">
                                Дата покупки
                            </p>
                            <p class="text-sm font-medium text-gray-900">
                                {{ computer.purchase_date || "-" }}
                            </p>
                        </div>
                        <div>
                            <p class="text-xs text-gray-600 mb-1">
                                Гарантия до
                            </p>
                            <div class="flex items-center gap-2">
                                <p class="text-sm font-medium text-gray-900">
                                    {{ computer.warranty_end_date || "-" }}
                                </p>
                                <span
                                    v-if="computer.warranty_active"
                                    class="badge bg-green-100 text-green-800 text-xs"
                                >
                                    Активна
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Мониторинг -->
                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <div class="flex items-center gap-2">
                            <Icon name="ri:pulse-line" class="text-gray-600" />
                            <h3 class="text-sm font-semibold text-gray-900">
                                Мониторинг
                            </h3>
                        </div>
                    </div>
                    <div class="p-6 space-y-4">
                        <div>
                            <div class="flex items-center justify-between mb-2">
                                <span class="text-xs text-gray-600">CPU</span>
                                <span class="text-xs font-medium text-gray-900"
                                    >{{ computer.monitoring.cpu }}%</span
                                >
                            </div>
                            <div class="w-full bg-gray-200 rounded-full h-2">
                                <div
                                    class="bg-blue-500 h-2 rounded-full"
                                    :style="{
                                        width: `${computer.monitoring.cpu}%`,
                                    }"
                                />
                            </div>
                        </div>

                        <div>
                            <div class="flex items-center justify-between mb-2">
                                <span class="text-xs text-gray-600">RAM</span>
                                <span class="text-xs font-medium text-gray-900"
                                    >{{ computer.monitoring.ram }}%</span
                                >
                            </div>
                            <div class="w-full bg-gray-200 rounded-full h-2">
                                <div
                                    class="bg-purple-500 h-2 rounded-full"
                                    :style="{
                                        width: `${computer.monitoring.ram}%`,
                                    }"
                                />
                            </div>
                        </div>

                        <div>
                            <div class="flex items-center justify-between mb-2">
                                <span class="text-xs text-gray-600"
                                    >Диск C:</span
                                >
                                <span class="text-xs font-medium text-gray-900"
                                    >{{ computer.monitoring.disk }}%</span
                                >
                            </div>
                            <div class="w-full bg-gray-200 rounded-full h-2">
                                <div
                                    class="bg-green-500 h-2 rounded-full"
                                    :style="{
                                        width: `${computer.monitoring.disk}%`,
                                    }"
                                />
                            </div>
                        </div>

                        <div class="pt-3 border-t border-gray-200">
                            <p class="text-xs text-gray-600 mb-1">
                                Время работы
                            </p>
                            <p class="text-sm font-medium text-gray-900">
                                {{ computer.monitoring.uptime }}
                            </p>
                        </div>
                    </div>
                </div>

                <!-- Временные метки -->
                <div class="card">
                    <div
                        class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white"
                    >
                        <h3 class="text-sm font-semibold text-gray-900">
                            Временные метки
                        </h3>
                    </div>
                    <div class="p-6 space-y-3">
                        <div class="flex items-center gap-2 text-sm">
                            <Icon name="ri:time-line" class="text-gray-600" />
                            <div>
                                <p class="text-xs text-gray-600">Создан</p>
                                <p class="text-gray-900">
                                    {{ computer.created_at }}
                                </p>
                            </div>
                        </div>
                        <div class="flex items-center gap-2 text-sm">
                            <Icon
                                name="ri:refresh-line"
                                class="text-gray-600"
                            />
                            <div>
                                <p class="text-xs text-gray-600">Обновлен</p>
                                <p class="text-gray-900">
                                    {{ computer.updated_at }}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Модальное окно удаления -->
        <Teleport to="body">
            <Transition name="modal">
                <div
                    v-if="showDeleteModal"
                    class="fixed inset-0 z-[60] overflow-y-auto"
                >
                    <div
                        class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm"
                        @click="showDeleteModal = false"
                    />

                    <div
                        class="relative min-h-screen flex items-center justify-center p-4"
                    >
                        <div
                            class="relative bg-white rounded-2xl shadow-2xl max-w-md w-full p-6"
                        >
                            <div class="text-center">
                                <div
                                    class="h-16 w-16 rounded-full bg-red-100 flex items-center justify-center mx-auto mb-4"
                                >
                                    <Icon
                                        name="ri:delete-bin-line"
                                        class="text-3xl text-red-600"
                                    />
                                </div>
                                <h3
                                    class="text-lg font-semibold text-gray-900 mb-2"
                                >
                                    Удалить компьютер?
                                </h3>
                                <p class="text-sm text-gray-600 mb-6">
                                    Это действие нельзя отменить. Компьютер
                                    будет удален навсегда.
                                </p>
                                <div class="flex gap-3">
                                    <button
                                        @click="showDeleteModal = false"
                                        class="flex-1 btn btn-secondary"
                                    >
                                        Отмена
                                    </button>
                                    <button
                                        @click="deleteComputer"
                                        class="flex-1 btn bg-red-600 text-white hover:bg-red-700"
                                    >
                                        Удалить
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </Transition>
        </Teleport>

        <!-- Модальное окно деталей активности -->
        <ActivityLogDetailModal
            :show="showActivityModal"
            :activity="selectedActivity"
            @close="showActivityModal = false"
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

const route = useRoute();
const router = useRouter();
const toast = useToast();

const showDeleteModal = ref(false);
const showActivityModal = ref(false);
const selectedActivity = ref<any>(null);

// Мок данные компьютера
const computer = ref({
    id: route.params.id,
    hostname: "PC-ADMIN-001",
    inventory_number: "INV-2024-001",
    computer_type: "desktop",
    manufacturer: "HP",
    model: "EliteDesk 800 G6",
    serial_number: "SN123456789",
    os: "Windows 11 Pro",
    os_version: "23H2",
    os_build: "22631.3007",
    os_architecture: "x64",
    domain_joined: true,
    location: "Офис 301, 3 этаж",
    last_boot_time: "10.12.2025 08:30",
    status: "active",
    purchase_date: "15.01.2024",
    warranty_end_date: "15.01.2027",
    warranty_active: true,
    created_at: "15.01.2024 10:00",
    updated_at: "10.12.2025 14:30",
    employee: {
        name: "Иванов Иван Иванович",
        position: "Системный администратор",
    },
    processor: {
        name: "Intel Core i7-10700",
        cores: 8,
        threads: 16,
        frequency: "2.90 GHz",
    },
    ram: {
        total: 16,
        modules: [
            { capacity: 8, type: "DDR4", frequency: "3200 MHz" },
            { capacity: 8, type: "DDR4", frequency: "3200 MHz" },
        ],
    },
    storage: {
        total: 512,
        drives: [{ type: "NVMe", model: "Samsung 970 EVO", capacity: 512 }],
    },
    gpu: {
        name: "NVIDIA GeForce GTX 1650",
        memory: 4,
        driver: "531.68",
    },
    motherboard: {
        manufacturer: "HP",
        model: "HP 8704",
        chipset: "Intel Q470",
        bios: "S09 Ver. 02.06.00",
    },
    network: {
        adapters: [
            { name: "Intel I219-LM", type: "Ethernet" },
            { name: "Intel Wi-Fi 6 AX201", type: "WiFi" },
        ],
    },
    monitors: [{ model: "Dell P2419H", size: 24, resolution: "1920x1080" }],
    peripherals: [
        { name: "Logitech K120", type: "Клавиатура" },
        { name: "Logitech M185", type: "Мышь" },
        { name: "Logitech C920", type: "Веб-камера" },
    ],
    monitoring: {
        cpu: 35,
        ram: 62,
        disk: 45,
        uptime: "2 дня 14 часов",
    },
    software: [
        {
            id: 1,
            name: "Microsoft Office 365",
            manufacturer: "Microsoft",
            version: "16.0.16827",
            license_type: "Subscription",
        },
        {
            id: 2,
            name: "Google Chrome",
            manufacturer: "Google",
            version: "120.0.6099",
            license_type: "Free",
        },
        {
            id: 3,
            name: "Adobe Acrobat Reader",
            manufacturer: "Adobe",
            version: "23.008.20470",
            license_type: "Free",
        },
        {
            id: 4,
            name: "Visual Studio Code",
            manufacturer: "Microsoft",
            version: "1.85.1",
            license_type: "Free",
        },
    ],
});

const activityLog = ref([
    {
        id: 1,
        type: "created",
        user: "Администратор",
        action: "создал компьютер",
        details: null,
        created_at: "15.01.2024 10:00",
        hasDetails: false,
    },
    {
        id: 2,
        type: "assigned",
        user: "Администратор",
        action: "назначил компьютер сотруднику",
        details: "Иванов И.И. - Системный администратор",
        created_at: "15.01.2024 10:15",
        hasDetails: false,
    },
    {
        id: 3,
        type: "component_added",
        user: "Система",
        action: "добавил компонент",
        details: "RAM: 8 ГБ DDR4 3200 MHz",
        created_at: "20.03.2024 14:30",
        hasDetails: true,
    },
    {
        id: 4,
        type: "software_installed",
        user: "Система",
        action: "установил ПО",
        details: "Microsoft Office 365 v16.0.16827",
        created_at: "25.05.2024 09:20",
        hasDetails: false,
    },
    {
        id: 5,
        type: "status_change",
        user: "Техник",
        action: "изменил статус",
        details: "Активный → В ремонте",
        created_at: "10.08.2024 16:45",
        hasDetails: false,
    },
    {
        id: 6,
        type: "status_change",
        user: "Техник",
        action: "изменил статус",
        details: "В ремонте → Активный",
        created_at: "12.08.2024 11:00",
        hasDetails: false,
    },
    {
        id: 7,
        type: "updated",
        user: "Администратор",
        action: "обновил информацию",
        details: "Изменено местоположение: Офис 301, 3 этаж",
        created_at: "10.12.2025 14:30",
        hasDetails: false,
    },
]);

useHead({
    title: `${computer.value.hostname} - Компьютеры`,
});

const updateStatus = () => {
    toast.success(
        `Статус изменен на "${getStatusLabel(computer.value.status)}"`,
    );
};

const deleteComputer = () => {
    toast.success("Компьютер удален");
    router.push("/computers");
};

const editComponent = (componentId: number, type: string) => {
    router.push(
        `/computers/${computer.value.id}/components/${componentId}/edit`,
    );
};

const showActivityDetails = (event: any) => {
    selectedActivity.value = event;
    showActivityModal.value = true;
};

const getActivityIcon = (type: string) => {
    const icons: Record<string, string> = {
        created: "ri:add-line",
        updated: "ri:edit-line",
        assigned: "ri:user-add-line",
        status_change: "ri:refresh-line",
        component_added: "ri:cpu-line",
        component_removed: "ri:delete-bin-line",
        software_installed: "ri:download-line",
        software_removed: "ri:uninstall-line",
        maintenance: "ri:tools-line",
    };
    return icons[type] || "ri:record-circle-line";
};

const getActivityColor = (type: string) => {
    const colors: Record<string, string> = {
        created: "bg-blue-500",
        updated: "bg-purple-500",
        assigned: "bg-green-500",
        status_change: "bg-amber-500",
        component_added: "bg-cyan-500",
        component_removed: "bg-red-500",
        software_installed: "bg-indigo-500",
        software_removed: "bg-pink-500",
        maintenance: "bg-orange-500",
    };
    return colors[type] || "bg-gray-500";
};

const getTypeIcon = (type: string) => {
    const icons: Record<string, string> = {
        desktop: "ri:computer-line",
        laptop: "ri:macbook-line",
        server: "ri:server-line",
        workstation: "ri:dashboard-line",
        thin_client: "ri:tv-line",
    };
    return icons[type] || "ri:computer-line";
};

const getTypeGradient = (type: string) => {
    const gradients: Record<string, string> = {
        desktop: "bg-gradient-to-br from-blue-500 to-blue-700",
        laptop: "bg-gradient-to-br from-purple-500 to-purple-700",
        server: "bg-gradient-to-br from-red-500 to-red-700",
        workstation: "bg-gradient-to-br from-green-500 to-green-700",
        thin_client: "bg-gradient-to-br from-gray-500 to-gray-700",
    };
    return gradients[type] || "bg-gradient-to-br from-gray-500 to-gray-700";
};

const getTypeLabel = (type: string) => {
    const labels: Record<string, string> = {
        desktop: "Десктоп",
        laptop: "Ноутбук",
        server: "Сервер",
        workstation: "Рабочая станция",
        thin_client: "Тонкий клиент",
    };
    return labels[type] || type;
};

const getStatusColor = (status: string) => {
    const colors: Record<string, string> = {
        active: "bg-green-100 text-green-800",
        inactive: "bg-gray-100 text-gray-800",
        repair: "bg-amber-100 text-amber-800",
        storage: "bg-blue-100 text-blue-800",
        decommissioned: "bg-red-100 text-red-800",
    };
    return colors[status] || "bg-gray-100 text-gray-800";
};

const getStatusLabel = (status: string) => {
    const labels: Record<string, string> = {
        active: "Активный",
        inactive: "Неактивный",
        repair: "В ремонте",
        storage: "На складе",
        decommissioned: "Списан",
    };
    return labels[status] || status;
};

const getOSIcon = (os: string) => {
    if (os.includes("Windows")) return "ri:windows-line";
    if (os.includes("Linux")) return "ri:ubuntu-line";
    if (os.includes("macOS")) return "ri:apple-line";
    return "ri:computer-line";
};
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
</style>
