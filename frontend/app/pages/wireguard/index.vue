<template>
    <div class="space-y-6">
        <!-- Header -->
        <div class="mb-8">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <div
                        class="h-16 w-16 rounded-2xl gradient-primary flex items-center justify-center shadow-lg"
                    >
                        <Icon name="ri:shield-user-line" class="text-3xl text-white" />
                    </div>
                    <div>
                        <h1 class="text-3xl font-bold text-gray-900">
                            WireGuard VPN
                        </h1>
                        <p class="mt-1 text-gray-600">
                            Управление VPN подключениями сотрудников
                        </p>
                    </div>
                </div>
                <div class="flex items-center gap-2">
                    <button
                        v-if="isAdmin"
                        @click="importFromMikrotik"
                        :disabled="syncing"
                        class="btn btn-secondary"
                        title="Импортировать пиры из MikroTik"
                    >
                        <Icon
                            name="ri:download-cloud-line"
                            class="mr-2"
                            :class="{ 'animate-spin': syncing }"
                        />
                        Импорт
                    </button>
                    <button
                        v-if="isAdmin"
                        @click="syncAll"
                        :disabled="syncing"
                        class="btn btn-secondary"
                        title="Синхронизировать статистику"
                    >
                        <Icon
                            name="ri:refresh-line"
                            class="mr-2"
                            :class="{ 'animate-spin': syncing }"
                        />
                        Синхронизировать
                    </button>
                    <button
                        v-if="isAdmin"
                        @click="showCreateModal = true"
                        class="btn btn-primary"
                    >
                        <Icon name="ri:add-line" class="mr-2" />
                        Добавить пир
                    </button>
                </div>
            </div>
        </div>

        <!-- Stats Cards -->
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
            <div class="card">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Всего пиров</p>
                            <p class="text-2xl font-bold text-gray-900">{{ stats.total }}</p>
                        </div>
                        <div class="h-12 w-12 rounded-xl bg-blue-100 flex items-center justify-center">
                            <Icon name="ri:shield-user-line" class="text-2xl text-blue-600" />
                        </div>
                    </div>
                </div>
            </div>

            <div 
                class="card cursor-pointer hover:shadow-md transition-shadow"
                :class="{ 'ring-2 ring-green-500': filters.status === 'active' }"
                @click="toggleStatusFilter('active')"
            >
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Активных</p>
                            <p class="text-2xl font-bold text-green-600">{{ stats.active }}</p>
                        </div>
                        <div class="h-12 w-12 rounded-xl bg-green-100 flex items-center justify-center">
                            <Icon name="ri:checkbox-circle-line" class="text-2xl text-green-600" />
                        </div>
                    </div>
                </div>
            </div>

            <div 
                class="card cursor-pointer hover:shadow-md transition-shadow"
                :class="{ 'ring-2 ring-gray-500': filters.status === 'disabled' }"
                @click="toggleStatusFilter('disabled')"
            >
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Отключенных</p>
                            <p class="text-2xl font-bold text-gray-600">{{ stats.disabled }}</p>
                        </div>
                        <div class="h-12 w-12 rounded-xl bg-gray-100 flex items-center justify-center">
                            <Icon name="ri:close-circle-line" class="text-2xl text-gray-600" />
                        </div>
                    </div>
                </div>
            </div>

            <div class="card bg-gradient-to-br from-purple-50 to-white">
                <div class="p-5">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm text-gray-600 mb-1">Онлайн сейчас</p>
                            <p class="text-2xl font-bold text-purple-600">{{ stats.online }}</p>
                            <p class="text-xs text-gray-500 mt-1">активны &lt; 5 мин</p>
                        </div>
                        <div class="h-12 w-12 rounded-xl bg-purple-100 flex items-center justify-center relative">
                            <Icon name="ri:signal-wifi-line" class="text-2xl text-purple-600" />
                            <span v-if="stats.online > 0" class="absolute -top-1 -right-1 w-3 h-3 bg-green-500 rounded-full animate-pulse"></span>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Filters & Table -->
        <div class="card">
            <div class="p-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-white">
                <div class="grid grid-cols-1 lg:grid-cols-12 gap-3">
                    <div class="lg:col-span-5 relative">
                        <Icon
                            name="ri:search-line"
                            class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400"
                        />
                        <input
                            v-model="searchQuery"
                            type="text"
                            placeholder="Поиск по имени или сотруднику..."
                            class="w-full h-10 pl-10 pr-4 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-all"
                        />
                    </div>
                    <div class="lg:col-span-3">
                        <select
                            v-model="filters.interface_id"
                            @change="loadPeers"
                            class="w-full h-10 px-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                        >
                            <option :value="undefined">Все интерфейсы</option>
                            <option
                                v-for="iface in interfaces"
                                :key="iface.id"
                                :value="iface.id"
                            >
                                {{ iface.name }}
                            </option>
                        </select>
                    </div>
                    <div class="lg:col-span-3">
                        <select
                            v-model="filters.status"
                            @change="loadPeers"
                            class="w-full h-10 px-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500"
                        >
                            <option :value="undefined">Все статусы</option>
                            <option value="active">Активен</option>
                            <option value="disabled">Отключен</option>
                            <option value="expired">Истек</option>
                            <option value="revoked">Отозван</option>
                        </select>
                    </div>
                    <div class="lg:col-span-1">
                        <button
                            v-if="filters.interface_id || filters.status || searchQuery"
                            @click="resetFilters"
                            class="w-full h-10 px-3 rounded-lg border border-gray-300 bg-white hover:bg-gray-50 text-gray-700 transition-colors flex items-center justify-center"
                            title="Сбросить фильтры"
                        >
                            <Icon name="ri:close-line" />
                        </button>
                    </div>
                </div>
            </div>

            <div v-if="loading" class="flex flex-col items-center justify-center py-16">
                <div class="relative">
                    <div class="h-16 w-16 rounded-full border-4 border-blue-100"></div>
                    <div class="absolute top-0 left-0 h-16 w-16 rounded-full border-4 border-blue-600 border-t-transparent animate-spin"></div>
                </div>
                <p class="mt-4 text-sm text-gray-600">Загрузка пиров...</p>
            </div>
            <div v-else class="overflow-x-auto">
                <table class="min-w-full divide-y divide-gray-200">
                    <thead class="bg-gray-50">
                        <tr>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Имя
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Сотрудник
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                IP адрес
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Статус
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Последнее подключение
                            </th>
                            <th
                                class="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase"
                            >
                                Трафик
                            </th>
                            <th
                                v-if="isAdmin"
                                class="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase"
                            >
                                Действия
                            </th>
                        </tr>
                    </thead>
                    <tbody class="bg-white divide-y divide-gray-200">
                        <tr v-if="filteredPeers.length === 0">
                            <td :colspan="isAdmin ? 7 : 6" class="px-6 py-20 text-center">
                                <div class="flex flex-col items-center">
                                    <div class="relative mb-6">
                                        <div
                                            class="h-24 w-24 rounded-2xl bg-gradient-to-br from-blue-100 to-purple-100 flex items-center justify-center"
                                        >
                                            <Icon
                                                name="ri:shield-user-line"
                                                class="text-5xl text-blue-600"
                                            />
                                        </div>
                                        <div class="absolute -bottom-2 -right-2 h-8 w-8 rounded-full bg-yellow-400 flex items-center justify-center shadow-lg">
                                            <Icon name="ri:search-line" class="text-lg text-yellow-900" />
                                        </div>
                                    </div>
                                    <h3 class="text-xl font-bold text-gray-900 mb-2">
                                        {{ searchQuery || filters.status || filters.interface_id ? 'Ничего не найдено' : 'Пиры отсутствуют' }}
                                    </h3>
                                    <p class="text-sm text-gray-500 mb-6 max-w-md">
                                        {{ searchQuery || filters.status || filters.interface_id 
                                            ? 'Попробуйте изменить параметры поиска или фильтры' 
                                            : 'Создайте первый WireGuard пир для начала работы с VPN' }}
                                    </p>
                                    <div class="flex gap-3">
                                        <button
                                            v-if="searchQuery || filters.status || filters.interface_id"
                                            @click="resetFilters"
                                            class="btn btn-secondary"
                                        >
                                            <Icon name="ri:close-line" class="mr-2" />
                                            Сбросить фильтры
                                        </button>
                                        <button
                                            v-if="isAdmin"
                                            @click="showCreateModal = true"
                                            class="btn btn-primary"
                                        >
                                            <Icon name="ri:add-line" class="mr-2" />
                                            Добавить пир
                                        </button>
                                    </div>
                                </div>
                            </td>
                        </tr>
                        <tr
                            v-for="peer in filteredPeers"
                            :key="peer.id"
                            class="hover:bg-gray-50 transition-colors"
                        >
                            <td class="px-4 py-4">
                                <div class="flex items-center gap-3">
                                    <div class="relative">
                                        <div
                                            class="h-10 w-10 rounded-lg bg-gradient-to-br from-blue-500 to-blue-700 flex items-center justify-center text-white font-semibold text-lg shadow-sm"
                                        >
                                            <Icon name="ri:shield-user-line" class="text-xl" />
                                        </div>
                                        <span 
                                            v-if="isOnline(peer)"
                                            class="absolute -top-1 -right-1 w-3 h-3 bg-green-500 border-2 border-white rounded-full"
                                            title="Онлайн"
                                        ></span>
                                    </div>
                                    <div>
                                        <div class="flex items-center gap-2">
                                            <span class="font-medium text-gray-900">
                                                {{ peer.name }}
                                            </span>
                                        </div>
                                        <div class="flex items-center gap-2 text-sm text-gray-500">
                                            <Icon name="ri:router-line" class="text-xs" />
                                            {{ peer.interface_name }}
                                        </div>
                                    </div>
                                </div>
                            </td>
                            <td class="px-4 py-4">
                                <div v-if="peer.employee_name" class="flex items-center gap-2">
                                    <div class="h-8 w-8 rounded-full bg-gradient-to-br from-gray-400 to-gray-600 flex items-center justify-center text-white text-xs font-semibold">
                                        {{ peer.employee_name.split(' ').map(n => n[0]).join('').slice(0, 2) }}
                                    </div>
                                    <span class="text-sm text-gray-900">{{ peer.employee_name }}</span>
                                </div>
                                <span v-else class="text-sm text-gray-400 italic">Не назначен</span>
                            </td>
                            <td class="px-4 py-4">
                                <div class="flex items-center gap-2">
                                    <Icon name="ri:ip-line" class="text-gray-400 text-sm" />
                                    <code class="text-sm bg-gray-100 px-2 py-1 rounded font-mono">
                                        {{ peer.client_address }}
                                    </code>
                                </div>
                            </td>
                            <td class="px-4 py-4">
                                <span
                                    :class="{
                                        'bg-green-100 text-green-800': peer.status === 'active',
                                        'bg-gray-100 text-gray-800': peer.status === 'disabled',
                                        'bg-red-100 text-red-800': peer.status === 'expired' || peer.status === 'revoked',
                                    }"
                                    class="badge"
                                >
                                    {{ getStatusLabel(peer.status) }}
                                </span>
                            </td>
                            <td class="px-4 py-4">
                                <div class="flex items-center gap-2 text-sm">
                                    <Icon 
                                        :name="isOnline(peer) ? 'ri:time-line' : 'ri:time-line'" 
                                        :class="isOnline(peer) ? 'text-green-600' : 'text-gray-400'"
                                    />
                                    <span :class="isOnline(peer) ? 'text-green-600 font-medium' : 'text-gray-500'">
                                        {{ formatLastHandshake(peer.last_handshake) }}
                                    </span>
                                </div>
                            </td>
                            <td class="px-4 py-4">
                                <div class="space-y-1">
                                    <div class="flex items-center gap-2 text-sm text-gray-600">
                                        <Icon name="ri:download-line" class="text-xs text-blue-500" />
                                        <span class="font-mono">{{ formatBytes(peer.rx_bytes) }}</span>
                                    </div>
                                    <div class="flex items-center gap-2 text-sm text-gray-600">
                                        <Icon name="ri:upload-line" class="text-xs text-green-500" />
                                        <span class="font-mono">{{ formatBytes(peer.tx_bytes) }}</span>
                                    </div>
                                </div>
                            </td>
                            <td v-if="isAdmin" class="px-4 py-4">
                                <div class="flex items-center justify-end gap-1">
                                    <button
                                        @click="viewPeerConfig(peer)"
                                        class="p-2 text-gray-600 hover:text-purple-600 hover:bg-purple-50 rounded-lg transition-colors group"
                                        title="Просмотр конфигурации"
                                    >
                                        <Icon name="ri:file-text-line" class="group-hover:scale-110 transition-transform" />
                                    </button>
                                    <button
                                        @click="downloadPeerConfig(peer.id)"
                                        class="p-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-colors group"
                                        title="Скачать конфиг"
                                    >
                                        <Icon name="ri:download-line" class="group-hover:scale-110 transition-transform" />
                                    </button>
                                    <button
                                        @click="editPeer(peer)"
                                        class="p-2 text-gray-600 hover:text-amber-600 hover:bg-amber-50 rounded-lg transition-colors group"
                                        title="Редактировать"
                                    >
                                        <Icon name="ri:edit-line" class="group-hover:scale-110 transition-transform" />
                                    </button>
                                    <button
                                        @click="confirmDelete(peer)"
                                        class="p-2 text-gray-600 hover:text-red-600 hover:bg-red-50 rounded-lg transition-colors group"
                                        title="Удалить"
                                    >
                                        <Icon name="ri:delete-bin-line" class="group-hover:scale-110 transition-transform" />
                                    </button>
                                </div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>

            <!-- Pagination -->
            <div
                v-if="total > 0"
                class="px-6 py-4 border-t border-gray-200 flex items-center justify-between bg-gray-50"
            >
                <p class="text-sm text-gray-700">
                    Показано
                    <span class="font-medium">{{ Math.min((currentPage - 1) * perPage + 1, total) }}</span>
                    -
                    <span class="font-medium">{{ Math.min(currentPage * perPage, total) }}</span>
                    из
                    <span class="font-medium">{{ total }}</span>
                </p>
                <div class="flex gap-2">
                    <button
                        @click="currentPage--"
                        :disabled="currentPage === 1"
                        class="btn btn-secondary disabled:opacity-50"
                    >
                        <Icon name="ri:arrow-left-s-line" />
                    </button>
                    <span class="px-4 py-2 text-sm text-gray-700 flex items-center">
                        {{ currentPage }} / {{ totalPages }}
                    </span>
                    <button
                        @click="currentPage++"
                        :disabled="currentPage >= totalPages"
                        class="btn btn-secondary disabled:opacity-50"
                    >
                        <Icon name="ri:arrow-right-s-line" />
                    </button>
                </div>
            </div>
        </div>

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
                    <div class="flex min-h-full items-center justify-center p-4">
                        <div
                            class="relative bg-white rounded-2xl shadow-2xl w-full max-w-2xl"
                        >
                            <div
                                class="flex items-center justify-between p-6 border-b border-gray-200"
                            >
                                <h3 class="text-xl font-semibold text-gray-900">
                                    {{ showEditModal ? "Редактировать пир" : "Добавить WireGuard пир" }}
                                </h3>
                                <button
                                    @click="closeModals"
                                    class="p-2 rounded-lg hover:bg-gray-100"
                                >
                                    <Icon name="ri:close-line" />
                                </button>
                            </div>
                    <form @submit.prevent="submitForm" class="p-6">
                        <div class="space-y-6">
                            <!-- Основная информация -->
                            <div class="space-y-4">
                                <h4 class="text-sm font-semibold text-gray-700 flex items-center gap-2">
                                    <Icon name="ri:information-line" class="text-blue-600" />
                                    Основная информация
                                </h4>
                                <div>
                                    <label class="label">Название *</label>
                                    <input
                                        v-model="form.name"
                                        type="text"
                                        required
                                        class="input-field"
                                        placeholder="VPN для Иванова И.И."
                                    />
                                </div>

                                <div class="grid grid-cols-2 gap-4">
                                    <div>
                                        <label class="label">Интерфейс *</label>
                                        <select
                                            v-model="form.interface_id"
                                            required
                                            class="input-field"
                                        >
                                            <option :value="null">Выберите интерфейс</option>
                                            <option
                                                v-for="iface in interfaces"
                                                :key="iface.id"
                                                :value="iface.id"
                                            >
                                                {{ iface.name }}
                                            </option>
                                        </select>
                                    </div>
                                    <div>
                                        <label class="label">Сотрудник</label>
                                        <select
                                            v-model="form.employee_id"
                                            class="input-field"
                                        >
                                            <option :value="null">Не назначен</option>
                                            <option
                                                v-for="employee in employees"
                                                :key="employee.id"
                                                :value="employee.id"
                                            >
                                                {{ employee.last_name }} {{ employee.first_name }}
                                            </option>
                                        </select>
                                    </div>
                                </div>
                            </div>

                            <!-- Сетевые настройки -->
                            <div class="space-y-4 pt-4 border-t border-gray-200">
                                <h4 class="text-sm font-semibold text-gray-700 flex items-center gap-2">
                                    <Icon name="ri:global-line" class="text-green-600" />
                                    Сетевые настройки
                                </h4>
                                <div class="grid grid-cols-2 gap-4">
                                    <div>
                                        <label class="label">IP адрес клиента *</label>
                                        <input
                                            v-model="form.client_address"
                                            type="text"
                                            required
                                            class="input-field font-mono"
                                            placeholder="192.168.99.10/30"
                                        />
                                    </div>
                                    <div>
                                        <label class="label">DNS сервер</label>
                                        <input
                                            v-model="form.client_dns"
                                            type="text"
                                            class="input-field font-mono"
                                            placeholder="192.168.78.254"
                                        />
                                    </div>
                                </div>

                                <div>
                                    <label class="label">Allowed IPs</label>
                                    <input
                                        v-model="form.allowed_ips"
                                        type="text"
                                        class="input-field font-mono"
                                        placeholder="0.0.0.0/0,::/0"
                                    />
                                    <p class="mt-1 text-xs text-gray-500">Разрешенные IP-адреса для маршрутизации через VPN</p>
                                </div>
                            </div>

                            <!-- Настройки подключения -->
                            <div class="space-y-4 pt-4 border-t border-gray-200">
                                <h4 class="text-sm font-semibold text-gray-700 flex items-center gap-2">
                                    <Icon name="ri:settings-3-line" class="text-purple-600" />
                                    Настройки подключения
                                </h4>
                                <div class="grid grid-cols-2 gap-4">
                                    <div>
                                        <label class="label">Endpoint адрес</label>
                                        <input
                                            v-model="form.endpoint_address"
                                            type="text"
                                            class="input-field"
                                            placeholder="vpn.example.com"
                                        />
                                    </div>
                                    <div>
                                        <label class="label">Endpoint порт</label>
                                        <input
                                            v-model.number="form.endpoint_port"
                                            type="number"
                                            class="input-field"
                                            placeholder="51821"
                                        />
                                    </div>
                                </div>

                                <div>
                                    <label class="label">Persistent Keepalive (секунды)</label>
                                    <input
                                        v-model.number="form.persistent_keepalive"
                                        type="number"
                                        class="input-field"
                                        placeholder="25"
                                    />
                                    <p class="mt-1 text-xs text-gray-500">Интервал отправки keepalive пакетов (рекомендуется 25)</p>
                                </div>
                            </div>

                            <!-- Дополнительно -->
                            <div class="space-y-4 pt-4 border-t border-gray-200">
                                <h4 class="text-sm font-semibold text-gray-700 flex items-center gap-2">
                                    <Icon name="ri:file-text-line" class="text-amber-600" />
                                    Дополнительно
                                </h4>
                                <div>
                                    <label class="label">Заметки</label>
                                    <textarea
                                        v-model="form.notes"
                                        rows="3"
                                        class="input-field"
                                        placeholder="Дополнительная информация о конфигурации..."
                                    />
                                </div>
                            </div>
                        </div>

                        <div class="mt-6 flex items-center justify-end gap-3 pt-6 border-t border-gray-200">
                            <button
                                type="button"
                                @click="closeModals"
                                class="btn btn-secondary"
                            >
                                Отмена
                            </button>
                            <button
                                type="submit"
                                :disabled="submitting"
                                class="btn btn-primary"
                            >
                                <Icon
                                    v-if="submitting"
                                    name="ri:loader-4-line"
                                    class="mr-2 animate-spin"
                                />
                                {{ submitting ? "Сохранение..." : (showEditModal ? "Сохранить изменения" : "Создать пир") }}
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
            </Transition>
        </Teleport>

        <!-- Config View Modal -->
        <Teleport to="body">
            <Transition name="modal">
                <div v-if="showConfigModal && peerConfig" class="fixed inset-0 z-50 overflow-y-auto">
                    <div class="fixed inset-0 bg-gray-900/50 backdrop-blur-sm" @click="showConfigModal = false" />
                    <div class="flex min-h-full items-center justify-center p-4">
                        <div class="relative bg-white rounded-2xl shadow-2xl w-full max-w-3xl">
                            <div class="flex items-center justify-between p-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-purple-50">
                                <div class="flex items-center gap-3">
                                    <div class="h-12 w-12 rounded-xl bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white shadow-lg">
                                        <Icon name="ri:file-text-line" class="text-2xl" />
                                    </div>
                                    <div>
                                        <h3 class="text-xl font-semibold text-gray-900">
                                            Конфигурация WireGuard
                                        </h3>
                                        <p class="text-sm text-gray-600">{{ peerConfig.peer.name }}</p>
                                    </div>
                                </div>
                                <button @click="showConfigModal = false" class="p-2 rounded-lg hover:bg-gray-100 transition-colors">
                                    <Icon name="ri:close-line" class="text-xl" />
                                </button>
                            </div>
                            
                            <div class="p-6">
                                <div class="bg-gray-900 rounded-xl p-4 overflow-x-auto">
                                    <pre class="text-sm text-green-400 font-mono">{{ peerConfig.config }}</pre>
                                </div>
                                
                                <div class="mt-4 p-4 bg-blue-50 rounded-lg border border-blue-200">
                                    <div class="flex items-start gap-3">
                                        <Icon name="ri:information-line" class="text-blue-600 text-xl flex-shrink-0 mt-0.5" />
                                        <div class="text-sm text-blue-900">
                                            <p class="font-medium mb-1">Как использовать:</p>
                                            <ol class="list-decimal list-inside space-y-1 text-blue-800">
                                                <li>Скачайте конфигурационный файл</li>
                                                <li>Импортируйте его в WireGuard клиент</li>
                                                <li>Активируйте подключение</li>
                                            </ol>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div class="flex items-center justify-end gap-3 p-6 border-t border-gray-200 bg-gray-50">
                                <button @click="showConfigModal = false" class="btn btn-secondary">
                                    Закрыть
                                </button>
                                <button @click="copyConfigToClipboard" class="btn bg-purple-600 text-white hover:bg-purple-700">
                                    <Icon name="ri:file-copy-line" class="mr-2" />
                                    Копировать
                                </button>
                                <button @click="downloadConfigFromModal" class="btn btn-primary">
                                    <Icon name="ri:download-line" class="mr-2" />
                                    Скачать
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </Transition>
        </Teleport>

        <!-- Delete Confirmation -->
        <ConfirmModal
            :show="showDeleteModal"
            title="Удалить пир?"
            :message="`Вы уверены, что хотите удалить пир '${peerToDelete?.name}'? Это действие нельзя отменить.`"
            confirm-text="Удалить"
            @confirm="deletePeerConfirmed"
            @cancel="showDeleteModal = false"
        />
    </div>
</template>

<script setup lang="ts">
import type { WireguardPeer } from "~/composables/useWireguard";

definePageMeta({
    middleware: "auth",
});

useHead({
    title: "WireGuard VPN",
});

const { isAdmin } = useAuth();
const {
    fetchPeers,
    fetchInterfaces,
    syncInterfaces,
    importPeersFromMikrotik,
    createPeer,
    updatePeer,
    deletePeer,
    downloadConfig,
    syncAllPeers,
    formatBytes,
    formatLastHandshake,
    getStatusColor,
    getStatusLabel,
} = useWireguard();

const toast = useToast();

// State
const loading = ref(true);
const syncing = ref(false);
const submitting = ref(false);
const peers = ref<WireguardPeer[]>([]);
const interfaces = ref<any[]>([]);
const employees = ref<any[]>([]);
const total = ref(0);
const currentPage = ref(1);
const perPage = ref(20);
const searchQuery = ref("");
const filters = ref({
    interface_id: undefined as number | undefined,
    status: undefined as string | undefined,
});

const showCreateModal = ref(false);
const showEditModal = ref(false);
const showDeleteModal = ref(false);
const showConfigModal = ref(false);
const peerToDelete = ref<WireguardPeer | null>(null);
const peerToEdit = ref<WireguardPeer | null>(null);
const peerConfig = ref<{ peer: WireguardPeer; config: string } | null>(null);

const form = ref({
    interface_id: null as number | null,
    employee_id: null as number | null,
    name: "",
    client_address: "",
    client_dns: "",
    endpoint_address: "",
    endpoint_port: null as number | null,
    allowed_ips: "0.0.0.0/0,::/0",
    persistent_keepalive: 25,
    notes: "",
});

// Computed
const totalPages = computed(() => Math.ceil(total.value / perPage.value));

const filteredPeers = computed(() => {
    let result = peers.value;

    if (searchQuery.value) {
        const query = searchQuery.value.toLowerCase();
        result = result.filter(
            (p) =>
                p.name.toLowerCase().includes(query) ||
                p.employee_name?.toLowerCase().includes(query),
        );
    }

    return result;
});

const isOnline = (peer: WireguardPeer) => {
    if (!peer.last_handshake) return false;
    const now = new Date();
    const fiveMinutesAgo = new Date(now.getTime() - 5 * 60 * 1000);
    const handshakeDate = new Date(peer.last_handshake + 'Z');
    return handshakeDate > fiveMinutesAgo;
};

const stats = computed(() => {
    return {
        total: peers.value.length,
        active: peers.value.filter((p) => p.status === "active").length,
        disabled: peers.value.filter((p) => p.status === "disabled").length,
        online: peers.value.filter(isOnline).length,
    };
});

// Methods
const loadPeers = async () => {
    loading.value = true;
    try {
        const data = await fetchPeers({
            page: currentPage.value,
            per_page: perPage.value,
            interface_id: filters.value.interface_id,
            status: filters.value.status,
        });
        peers.value = data.peers;
        total.value = data.total;
    } catch (error: any) {
        toast.error("Ошибка загрузки пиров: " + error.message);
    } finally {
        loading.value = false;
    }
};

const loadInterfaces = async () => {
    try {
        interfaces.value = await fetchInterfaces();
        
        // Если интерфейсов нет, попробуем синхронизировать с MikroTik
        if (interfaces.value.length === 0 && isAdmin.value) {
            toast.info("Синхронизация интерфейсов с MikroTik...");
            try {
                interfaces.value = await syncInterfaces();
                if (interfaces.value.length > 0) {
                    toast.success(`Синхронизировано ${interfaces.value.length} интерфейсов`);
                }
            } catch (syncError: any) {
                console.error("Failed to sync interfaces:", syncError);
                toast.warning("Не удалось синхронизировать интерфейсы с MikroTik");
            }
        }
    } catch (error: any) {
        console.error("Failed to load interfaces:", error);
    }
};

const loadEmployees = async () => {
    try {
        const { $api } = useNuxtApp();
        const data = await $api<any>("/employees", {
            params: { page: 1, per_page: 1000 },
        });
        employees.value = data.employees || [];
    } catch (error: any) {
        console.error("Failed to load employees:", error);
    }
};

const syncAll = async () => {
    syncing.value = true;
    try {
        await syncAllPeers();
        toast.success("Синхронизация завершена");
        await loadPeers();
    } catch (error: any) {
        toast.error("Ошибка синхронизации: " + error.message);
    } finally {
        syncing.value = false;
    }
};

const importFromMikrotik = async () => {
    syncing.value = true;
    try {
        const data = await importPeersFromMikrotik();
        toast.success(`Импортировано ${data.peers.length} пиров`);
        await loadPeers();
    } catch (error: any) {
        toast.error("Ошибка импорта: " + error.message);
    } finally {
        syncing.value = false;
    }
};

const viewPeerConfig = async (peer: WireguardPeer) => {
    try {
        const data = await downloadConfig(peer.id);
        peerConfig.value = { peer, config: data.config };
        showConfigModal.value = true;
    } catch (error: any) {
        toast.error("Ошибка загрузки конфигурации: " + error.message);
    }
};

const downloadPeerConfig = async (id: number) => {
    try {
        const data = await downloadConfig(id);
        const blob = new Blob([data.config], { type: "text/plain" });
        const url = URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.href = url;
        a.download = `wireguard-peer-${id}.conf`;
        a.click();
        URL.revokeObjectURL(url);
        toast.success("Конфигурация скачана");
    } catch (error: any) {
        toast.error("Ошибка скачивания: " + error.message);
    }
};

const copyConfigToClipboard = async () => {
    if (!peerConfig.value) return;
    try {
        await navigator.clipboard.writeText(peerConfig.value.config);
        toast.success("Конфигурация скопирована в буфер обмена");
    } catch (error) {
        toast.error("Ошибка копирования");
    }
};

const downloadConfigFromModal = () => {
    if (!peerConfig.value) return;
    const blob = new Blob([peerConfig.value.config], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `wireguard-${peerConfig.value.peer.name.replace(/\s+/g, '-')}.conf`;
    a.click();
    URL.revokeObjectURL(url);
    toast.success("Конфигурация скачана");
};

const toggleStatusFilter = (status: string) => {
    if (filters.value.status === status) {
        filters.value.status = undefined;
    } else {
        filters.value.status = status;
    }
    currentPage.value = 1;
    loadPeers();
};

const editPeer = (peer: WireguardPeer) => {
    peerToEdit.value = peer;
    form.value = {
        interface_id: peer.interface_id,
        employee_id: peer.employee_id,
        name: peer.name,
        client_address: peer.client_address,
        client_dns: peer.client_dns || "",
        endpoint_address: peer.endpoint_address || "",
        endpoint_port: peer.endpoint_port,
        allowed_ips: peer.allowed_ips,
        persistent_keepalive: peer.persistent_keepalive || 25,
        notes: peer.notes || "",
    };
    showEditModal.value = true;
};

const confirmDelete = (peer: WireguardPeer) => {
    peerToDelete.value = peer;
    showDeleteModal.value = true;
};

const deletePeerConfirmed = async () => {
    if (!peerToDelete.value) return;

    try {
        await deletePeer(peerToDelete.value.id);
        toast.success("Пир удален");
        showDeleteModal.value = false;
        await loadPeers();
    } catch (error: any) {
        toast.error("Ошибка удаления: " + error.message);
    }
};

const resetFilters = () => {
    filters.value = {
        interface_id: undefined,
        status: undefined,
    };
    searchQuery.value = "";
    currentPage.value = 1;
    loadPeers();
};

const closeModals = () => {
    if (!submitting.value) {
        showCreateModal.value = false;
        showEditModal.value = false;
        peerToEdit.value = null;
        form.value = {
            interface_id: null,
            employee_id: null,
            name: "",
            client_address: "",
            client_dns: "",
            endpoint_address: "",
            endpoint_port: null,
            allowed_ips: "0.0.0.0/0,::/0",
            persistent_keepalive: 25,
            notes: "",
        };
    }
};

const submitForm = async () => {
    submitting.value = true;
    try {
        const data = {
            interface_id: form.value.interface_id!,
            employee_id: form.value.employee_id || undefined,
            name: form.value.name,
            client_address: form.value.client_address,
            client_dns: form.value.client_dns || undefined,
            endpoint_address: form.value.endpoint_address || undefined,
            endpoint_port: form.value.endpoint_port || undefined,
            allowed_ips: form.value.allowed_ips || undefined,
            persistent_keepalive: form.value.persistent_keepalive || undefined,
            notes: form.value.notes || undefined,
        };

        if (showEditModal.value && peerToEdit.value) {
            await updatePeer(peerToEdit.value.id, data);
            toast.success("Пир успешно обновлен");
        } else {
            await createPeer(data);
            toast.success("Пир успешно создан");
        }

        submitting.value = false;
        closeModals();
        await loadPeers();
    } catch (err: any) {
        submitting.value = false;
        if (err.statusCode === 401) {
            toast.error("Недостаточно прав. Требуется роль администратора.");
        } else {
            toast.error("Ошибка при сохранении: " + (err.message || "Неизвестная ошибка"));
        }
    }
};

// Watchers
watch([currentPage, () => filters.value.interface_id, () => filters.value.status], () => {
    loadPeers();
});

// Auto-sync interval
let syncInterval: NodeJS.Timeout | null = null;

// Lifecycle
onMounted(async () => {
    await loadPeers();
    await loadInterfaces();
    await loadEmployees();

    // Автоматическая синхронизация каждые 30 секунд (тихая, без логирования)
    if (isAdmin.value && peers.value.length > 0) {
        syncInterval = setInterval(async () => {
            try {
                await syncAllPeers();
                await loadPeers();
            } catch (error) {
                console.error("Auto-sync failed:", error);
            }
        }, 30000); // 30 секунд
    }
});

onUnmounted(() => {
    if (syncInterval) {
        clearInterval(syncInterval);
    }
});
</script>

<style scoped>
.modal-enter-active,
.modal-leave-active {
    transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
    opacity: 0;
}

.modal-enter-active .modal-content,
.modal-leave-active .modal-content {
    transition: transform 0.3s ease;
}

.modal-enter-from .modal-content,
.modal-leave-to .modal-content {
    transform: scale(0.95) translateY(-20px);
}

.badge {
    @apply px-2 py-1 text-xs font-medium rounded-full;
}

.label {
    @apply block text-sm font-medium text-gray-700 mb-1;
}

.input-field {
    @apply w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500;
}

/* Пульсация для онлайн индикатора */
@keyframes pulse-ring {
    0% {
        transform: scale(0.95);
        opacity: 1;
    }
    50% {
        transform: scale(1.05);
        opacity: 0.7;
    }
    100% {
        transform: scale(0.95);
        opacity: 1;
    }
}

.animate-pulse {
    animation: pulse-ring 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}
</style>
