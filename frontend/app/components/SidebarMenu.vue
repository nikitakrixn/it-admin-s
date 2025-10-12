<template>
    <div class="space-y-1">
        <SidebarMenuItem to="/" icon="ri:dashboard-line" label="Главная" />
        <SidebarMenuItem
            to="/analytics"
            icon="ri:bar-chart-line"
            label="Аналитика"
        />
        <SidebarMenuItem
            to="/employees"
            icon="ri:team-line"
            label="Сотрудники"
        />
        <SidebarMenuItem
            to="/computers"
            icon="ri:computer-line"
            label="Компьютеры"
        />
        <SidebarMenuItem to="/software" icon="ri:apps-line" label="Программы" />
        <SidebarMenuItem
            to="/equipment"
            icon="ri:printer-line"
            label="Оборудование"
        />
        <SidebarMenuItem
            to="/consumables"
            icon="ri:inbox-line"
            label="Расходники"
        />
        <SidebarMenuItem
            to="/licenses"
            icon="ri:shield-check-line"
            label="Лицензии"
        />
        <SidebarMenuItem
            to="/requests"
            icon="ri:file-list-3-line"
            label="Заявки"
        />
        <SidebarMenuItem
            to="/network/scan"
            icon="ri:radar-line"
            label="Сканер сети"
        />
        <SidebarMenuItem to="/logs" icon="ri:history-line" label="Журнал" />
    </div>
</template>
