# TODO - IT-Admin System

## 🎯 Приоритет 1: Критичный функционал

### Backend API

#### Computers Management
- [ ] GET /api/computers - список компьютеров с пагинацией и фильтрами
- [ ] GET /api/computers/:id - детальная информация о компьютере
- [ ] POST /api/computers - добавление нового компьютера
- [ ] PUT /api/computers/:id - обновление информации
- [ ] DELETE /api/computers/:id - удаление компьютера
- [ ] GET /api/computers/:id/components - все компоненты компьютера
- [ ] POST /api/computers/:id/components - добавление компонента
- [ ] PUT /api/computers/:id/components/:component_id - обновление компонента
- [ ] DELETE /api/computers/:id/components/:component_id - удаление компонента

#### Computer Monitoring
- [ ] GET /api/monitoring/computers/:id/latest - последние данные мониторинга
- [ ] GET /api/monitoring/computers/:id/history - история мониторинга
- [ ] POST /api/monitoring/computers/:id - добавление данных мониторинга
- [ ] GET /api/monitoring/dashboard - дашборд с общей статистикой

#### Equipment Management
- [ ] GET /api/equipment - список оборудования
- [ ] GET /api/equipment/:id - детальная информация
- [ ] POST /api/equipment - добавление оборудования
- [ ] PUT /api/equipment/:id - обновление
- [ ] DELETE /api/equipment/:id - удаление

#### Requests (Заявки)
- [ ] GET /api/requests - список заявок с фильтрами
- [ ] GET /api/requests/:id - детальная информация о заявке
- [ ] POST /api/requests - создание новой заявки
- [ ] PUT /api/requests/:id - обновление заявки
- [ ] DELETE /api/requests/:id - удаление заявки
- [ ] POST /api/requests/:id/comments - добавление комментария
- [ ] GET /api/requests/:id/comments - получение комментариев
- [ ] POST /api/requests/:id/attachments - загрузка файлов
- [ ] GET /api/requests/:id/attachments - список файлов

### Repositories & Services

- [ ] ComputerRepository - CRUD операции для компьютеров
- [ ] ComponentRepository - управление компонентами
- [ ] MonitoringRepository - работа с данными мониторинга
- [ ] EquipmentRepository - управление оборудованием
- [ ] RequestRepository - работа с заявками
- [ ] MonitoringService - бизнес-логика мониторинга
- [ ] NotificationService - отправка уведомлений

### Authentication & Authorization

- [ ] Реализовать API токены для клиентских приложений
- [ ] Добавить refresh tokens
- [ ] Реализовать 2FA аутентификацию
- [ ] Добавить блокировку аккаунта после неудачных попыток
- [ ] Реализовать принудительную смену пароля
- [ ] Добавить роли: technician, viewer
- [ ] Permission-based access control для детальных прав

## 🎯 Приоритет 2: Важный функционал

### Software Management

- [ ] GET /api/computers/:id/software - установленное ПО на компьютере
- [ ] POST /api/computers/:id/software - добавление ПО
- [ ] DELETE /api/computers/:id/software/:software_id - удаление ПО
- [ ] GET /api/software/history - история изменений ПО
- [ ] GET /api/software/licenses - управление лицензиями
- [ ] POST /api/software/licenses - добавление лицензии

### Consumables Management

- [ ] GET /api/consumables - список расходников
- [ ] POST /api/consumables - добавление расходника
- [ ] PUT /api/consumables/:id - обновление
- [ ] DELETE /api/consumables/:id - удаление
- [ ] POST /api/consumables/:id/movements - движение расходников
- [ ] GET /api/consumables/:id/movements - история движений
- [ ] GET /api/consumables/low-stock - расходники с низким остатком

### Network Management

- [ ] GET /api/network/devices - сетевые устройства
- [ ] GET /api/network/ip-addresses - управление IP адресами
- [ ] GET /api/network/subnets - подсети
- [ ] POST /api/network/scan - сканирование сети

### Active Directory Integration

- [ ] Реализовать LDAP клиент
- [ ] Синхронизация пользователей из AD
- [ ] Синхронизация групп из AD
- [ ] Автоматическое создание учетных записей
- [ ] Логирование синхронизации (ad_sync_history)
- [ ] Настройка расписания синхронизации

### Notifications System

- [ ] GET /api/notifications - список уведомлений
- [ ] PUT /api/notifications/:id/read - отметить как прочитанное
- [ ] POST /api/notifications/mark-all-read - отметить все
- [ ] DELETE /api/notifications/:id - удалить уведомление
- [ ] WebSocket для real-time уведомлений

### Alert System

- [ ] GET /api/alerts/rules - правила оповещений
- [ ] POST /api/alerts/rules - создание правила
- [ ] PUT /api/alerts/rules/:id - обновление правила
- [ ] DELETE /api/alerts/rules/:id - удаление правила
- [ ] GET /api/alerts/history - история оповещений
- [ ] POST /api/alerts/:id/acknowledge - подтвердить оповещение

## 🎯 Приоритет 3: Дополнительный функционал

### Knowledge Base

- [ ] GET /api/knowledge-base - статьи базы знаний
- [ ] POST /api/knowledge-base - создание статьи
- [ ] PUT /api/knowledge-base/:id - обновление статьи
- [ ] DELETE /api/knowledge-base/:id - удаление статьи
- [ ] GET /api/knowledge-base/search - поиск по базе знаний

### Projects Management

- [ ] GET /api/projects - список проектов
- [ ] POST /api/projects - создание проекта
- [ ] PUT /api/projects/:id - обновление проекта
- [ ] DELETE /api/projects/:id - удаление проекта
- [ ] GET /api/projects/:id/tasks - задачи проекта

### Checklists

- [ ] GET /api/checklists - список чек-листов
- [ ] POST /api/checklists - создание чек-листа
- [ ] PUT /api/checklists/:id - обновление
- [ ] POST /api/checklists/:id/items - добавление пункта
- [ ] PUT /api/checklists/:id/items/:item_id - обновление пункта

### Documents Management

- [ ] GET /api/documents - список документов
- [ ] POST /api/documents - загрузка документа
- [ ] GET /api/documents/:id/download - скачивание
- [ ] DELETE /api/documents/:id - удаление документа

### Backup Tracking

- [ ] GET /api/backups - список бэкапов
- [ ] POST /api/backups - регистрация бэкапа
- [ ] GET /api/backups/schedule - расписание бэкапов
- [ ] POST /api/backups/verify - верификация бэкапа

### Security

- [ ] GET /api/security/incidents - инциденты безопасности
- [ ] POST /api/security/incidents - регистрация инцидента
- [ ] GET /api/security/antivirus - статус антивирусов
- [ ] GET /api/security/patches - управление патчами
- [ ] GET /api/security/certificates - SSL сертификаты

### VPN & Remote Access

- [ ] GET /api/vpn/connections - VPN подключения
- [ ] POST /api/vpn/connections - создание подключения
- [ ] GET /api/remote-access/sessions - сессии удаленного доступа
- [ ] GET /api/wireguard/peers - WireGuard пиры

### Contracts & Vendors

- [ ] GET /api/contracts - список контрактов
- [ ] POST /api/contracts - создание контракта
- [ ] GET /api/vendors - поставщики
- [ ] POST /api/vendors - добавление поставщика

### Work Time Tracking

- [ ] GET /api/work-time - учет рабочего времени
- [ ] POST /api/work-time/clock-in - начало работы
- [ ] POST /api/work-time/clock-out - окончание работы
- [ ] GET /api/work-time/reports - отчеты

### Inventory Audits

- [ ] GET /api/audits - список инвентаризаций
- [ ] POST /api/audits - создание инвентаризации
- [ ] GET /api/audits/:id/items - элементы инвентаризации
- [ ] POST /api/audits/:id/complete - завершение инвентаризации

## 🔧 Технические улучшения

### Testing

- [ ] Unit тесты для services
- [ ] Unit тесты для repositories
- [ ] Integration тесты для API endpoints
- [ ] E2E тесты
- [ ] Тестовые fixtures для БД
- [ ] CI/CD pipeline с автоматическим запуском тестов

### Performance

- [ ] Реализовать Redis кеширование
- [ ] Добавить query optimization
- [ ] Реализовать pagination cursor-based
- [ ] Добавить database indexes review
- [ ] Реализовать connection pooling tuning
- [ ] Добавить query result caching

### Security Enhancements

- [ ] Реализовать rate limiting (middleware)
- [ ] Добавить request validation middleware
- [ ] Реализовать CSRF protection
- [ ] Добавить security headers middleware
- [ ] Реализовать audit logging для всех операций
- [ ] Добавить IP whitelist/blacklist
- [ ] Реализовать session management

### Monitoring & Observability

- [ ] Интеграция с Prometheus для метрик
- [ ] Добавить structured logging (JSON)
- [ ] Реализовать distributed tracing (Jaeger/Zipkin)
- [ ] Добавить health check endpoints (liveness/readiness)
- [ ] Реализовать metrics endpoint (/metrics)
- [ ] Добавить error tracking (Sentry)

### Documentation

- [ ] API documentation (расширить Swagger)
- [ ] Deployment guide
- [ ] Development setup guide
- [ ] Architecture documentation
- [ ] Database schema documentation
- [ ] API usage examples
- [ ] Troubleshooting guide

### DevOps

- [ ] Kubernetes deployment manifests
- [ ] Helm charts
- [ ] CI/CD pipeline (GitHub Actions)
- [ ] Automated database backups
- [ ] Monitoring alerts setup
- [ ] Log aggregation (ELK/Loki)
- [ ] Infrastructure as Code (Terraform)

## 🔌 Интеграции

### MikroTik Integration

- [ ] Реализовать MikroTik API клиент
- [ ] Синхронизация DHCP leases
- [ ] Управление firewall rules
- [ ] Мониторинг bandwidth
- [ ] Управление VPN подключениями

### Email Integration

- [ ] Реализовать SMTP клиент
- [ ] Email уведомления о заявках
- [ ] Email уведомления о критичных событиях
- [ ] Email отчеты
- [ ] Email templates

### Telegram Bot Integration

- [ ] Реализовать Telegram bot
- [ ] Уведомления в Telegram
- [ ] Создание заявок через бота
- [ ] Статус заявок через бота
- [ ] Команды для быстрого доступа к информации

### External Monitoring Tools

- [ ] Интеграция с Zabbix
- [ ] Интеграция с Nagios
- [ ] Интеграция с Grafana
- [ ] Webhook для внешних систем

## 📱 Client Applications

### Windows Agent

- [ ] Разработать Windows агент для сбора данных
- [ ] Автоматическая инвентаризация железа
- [ ] Сбор установленного ПО
- [ ] Мониторинг состояния (CPU, RAM, диски)
- [ ] Отправка данных на сервер
- [ ] Автоматическое обновление агента

### Web Dashboard

- [ ] Дашборд с общей статистикой
- [ ] Real-time мониторинг компьютеров
- [ ] Графики и charts
- [ ] Фильтры и поиск
- [ ] Экспорт данных (CSV, Excel, PDF)

## 🐛 Bug Fixes & Improvements

### Known Issues

- [ ] Проверить все миграции на корректность
- [ ] Добавить валидацию для всех input данных
- [ ] Улучшить error messages
- [ ] Добавить transaction support для критичных операций

### Code Quality

- [ ] Добавить rustfmt configuration
- [ ] Добавить clippy lints
- [ ] Рефакторинг дублирующегося кода
- [ ] Улучшить naming conventions
- [ ] Добавить code comments для сложной логики
- [ ] Документация для публичных API

## 📊 Reporting

- [ ] Отчет по компьютерам
- [ ] Отчет по установленному ПО
- [ ] Отчет по лицензиям
- [ ] Отчет по заявкам
- [ ] Отчет по расходникам
- [ ] Отчет по инвентаризации
- [ ] Scheduled reports (автоматическая генерация)

## 🎨 Frontend (Nuxt)

### Core Pages

- [ ] Dashboard page
- [ ] Computers list & detail pages
- [ ] Employees list & detail pages
- [ ] Departments management
- [ ] Positions management
- [ ] Software catalog
- [ ] Equipment management
- [ ] Requests (заявки) management
- [ ] Activity log viewer
- [ ] User profile page
- [ ] Settings page

### Components

- [ ] DataTable component с сортировкой и фильтрами
- [ ] Form components (input, select, date picker)
- [ ] Modal dialogs
- [ ] Notification toasts
- [ ] Charts components (для дашборда)
- [ ] File upload component
- [ ] Search component
- [ ] Pagination component

### State Management

- [ ] Pinia stores для всех entities
- [ ] Auth store
- [ ] User preferences store
- [ ] Notifications store
- [ ] WebSocket integration

