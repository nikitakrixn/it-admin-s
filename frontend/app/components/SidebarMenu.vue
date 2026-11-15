<template>
  <div class="space-y-1">
    <SidebarMenuItem to="/" icon="ri:dashboard-line" label="Главная" />
    <SidebarMenuItem to="/analytics" icon="ri:bar-chart-line" label="Аналитика" />
    <SidebarMenuItem to="/employees" icon="ri:team-line" label="Сотрудники" />
    <SidebarMenuItem to="/computers" icon="ri:computer-line" label="Компьютеры" />
    <SidebarMenuItem to="/inventory" icon="ri:task-line" label="Инвентаризация" />
    <SidebarMenuItem to="/monitoring" icon="ri:pulse-line" label="Мониторинг" />
    <SidebarMenuItem to="/software" icon="ri:apps-line" label="Программы" />
    <SidebarMenuItem to="/equipment" icon="ri:printer-line" label="Оборудование" />
    <SidebarMenuItem to="/consumables" icon="ri:inbox-line" label="Расходники" />
    <SidebarMenuItem to="/licenses" icon="ri:shield-check-line" label="Лицензии" />
    <SidebarMenuItem to="/requests" icon="ri:file-list-3-line" label="Заявки" />
    <SidebarMenuItem to="/network/segments" icon="ri:map-line" label="Сети и подсети" />
    <SidebarMenuItem to="/network/devices" icon="ri:router-line" label="Сетевое оборудование" />
    <SidebarMenuItem to="/network/ip-addresses" icon="ri:global-line" label="IP-адреса" />
    <SidebarMenuItem to="/network/scan" icon="ri:radar-line" label="Сканер сети" />
    <SidebarMenuItem to="/ad" icon="ri:account-circle-line" label="Active Directory" />
    <SidebarMenuItem to="/wireguard" icon="ri:shield-user-line" label="WireGuard VPN" />
    <SidebarMenuItem to="/logs" icon="ri:history-line" label="Журнал" />

    <template v-if="isAdmin">
      <p class="px-3 pt-4 pb-1 text-xs font-semibold uppercase text-gray-400">Администрирование</p>
      <SidebarMenuItem to="/admin/users" icon="ri:user-settings-line" label="Пользователи" />
      <SidebarMenuItem to="/import" icon="ri:upload-cloud-2-line" label="Импорт из CSV" />
    </template>
  </div>
</template>

<script setup lang="ts">
const { isAdmin } = useAuth()
</script>
