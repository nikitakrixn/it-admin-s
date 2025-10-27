# 🗺️ IT-Admin & Corporate Portal - Roadmap

## 📋 Общая концепция

**Единая корпоративная платформа** с тремя модулями:
1. **IT-Admin** - управление IT-инфраструктурой (для сисадминов)
2. **СЭД** - система электронного документооборота (для всех)
3. **Портал** - заявки, задачи, почта (для всех)

---

## 🏗️ Фаза 0: Фундамент (Foundation)

**Цель:** Создать базовую архитектуру для модульной системы

### Milestone 0.1: RBAC система ✅ Готово к старту

**Branch:** `feature/rbac-foundation`

#### Backend
- [ ] Миграция: Расширенная ролевая модель
  - [ ] Enum `user_role` (admin, it_admin, manager, employee, guest)
  - [ ] Таблица `permissions` (модуль, ресурс, действие)
  - [ ] Таблица `role_permissions` (связь ролей и прав)
  - [ ] Таблица `user_permissions` (переопределение прав)
  - [ ] Таблица `system_modules` (модули системы)
  - [ ] Таблица `module_access` (доступ к модулям по ролям)
  
- [ ] Middleware: `RbacChecker`
  - [ ] `check_permission()` - проверка конкретного права
  - [ ] `check_module_access()` - проверка доступа к модулю
  - [ ] `get_user_permissions()` - получение всех прав пользователя
  
- [ ] Security Schemes для Poem
  - [ ] `ModuleAuth` - проверка доступа к модулю
  - [ ] `PermissionAuth` - проверка конкретного права
  
- [ ] API endpoints: `/api/permissions`
  - [ ] GET `/api/permissions/my` - мои права
  - [ ] GET `/api/permissions/modules` - доступные модули
  - [ ] GET `/api/permissions/check` - проверка права

#### Frontend
- [ ] Composable: `usePermissions`
  - [ ] `hasPermission(module, resource, action)` - проверка права
  - [ ] `hasModuleAccess(module)` - проверка доступа к модулю
  - [ ] `canAccessITAdmin` - computed для IT-Admin
  - [ ] `canAccessSED` - computed для СЭД
  - [ ] `canAccessPortal` - computed для Портала
  
- [ ] Компоненты
  - [ ] `ModuleSection.vue` - секция модуля в меню
  - [ ] `PermissionGuard.vue` - обертка для проверки прав
  
- [ ] Обновить `layouts/default.vue`
  - [ ] Модульная навигация
  - [ ] Динамическое меню по правам

#### Тесты
- [ ] Unit тесты для `RbacChecker`
- [ ] Integration тесты для API permissions
- [ ] E2E тесты для навигации по ролям

**Коммиты:**
```
feat(rbac): add role-based access control foundation
feat(rbac): add permissions middleware
feat(rbac): add module access control
feat(frontend): add permissions composable
feat(frontend): add modular navigation
test(rbac): add unit and integration tests
docs(rbac): add RBAC documentation
```

---

## 🏗️ Фаза 1: IT-Admin модуль (текущий функционал)

**Цель:** Организовать существующий функционал в модуль IT-Admin

### Milestone 1.1: Реорганизация структуры

**Branch:** `feature/it-admin-module`

#### Backend
- [ ] Переместить в `src/modules/it_admin/`
  - [ ] routes/ (computers, equipment, network, software, etc.)
  - [ ] services/
  - [ ] repositories/
  
- [ ] Добавить проверки прав для всех endpoints
  - [ ] Только `admin` и `it_admin` имеют доступ

#### Frontend
- [ ] Переместить в `pages/it-admin/`
  - [ ] computers/
  - [ ] equipment/
  - [ ] network/
  - [ ] software/
  - [ ] licenses/
  
- [ ] Добавить middleware для проверки доступа
- [ ] Обновить навигацию

**Коммиты:**
```
refactor(backend): reorganize IT-Admin module structure
refactor(frontend): reorganize IT-Admin pages
feat(it-admin): add permission checks to all endpoints
docs(it-admin): update module documentation
```

### Milestone 1.2: Доработка существующего функционала

См. текущий TODO.md для деталей

---

## 🏗️ Фаза 2: СЭД модуль (Документооборот)

**Цель:** Создать полноценную систему электронного документооборота с ЭЦП

### Milestone 2.1: База данных и модели

**Branch:** `feature/sed-database`

#### Backend
- [ ] Миграция: Основные таблицы СЭД
  - [ ] `document_types` - типы документов
  - [ ] `documents` - документы
  - [ ] `document_signatures` - подписи
  - [ ] `document_history` - история изменений
  - [ ] `document_comments` - комментарии
  - [ ] `document_access` - права доступа
  
- [ ] Модели Diesel
  - [ ] DocumentType
  - [ ] Document
  - [ ] DocumentSignature
  - [ ] DocumentHistory
  
- [ ] Репозитории
  - [ ] DocumentRepository
  - [ ] SignatureRepository

**Коммиты:**
```
feat(sed): add database schema for documents
feat(sed): add Diesel models for SED module
feat(sed): add document repositories
test(sed): add repository tests
```

### Milestone 2.2: Хранилище файлов (S3/MinIO)

**Branch:** `feature/sed-storage`

#### Backend
- [ ] Интеграция с MinIO/S3
  - [ ] StorageService
  - [ ] Загрузка файлов
  - [ ] Скачивание файлов
  - [ ] Удаление файлов
  - [ ] Генерация временных ссылок
  
- [ ] Конфигурация
  - [ ] S3_ENDPOINT
  - [ ] S3_BUCKET
  - [ ] S3_ACCESS_KEY
  - [ ] S3_SECRET_KEY

#### Docker
- [ ] Добавить MinIO в docker-compose.yml
- [ ] Настроить volumes для хранения

**Коммиты:**
```
feat(storage): add MinIO/S3 integration
feat(storage): add file upload/download service
chore(docker): add MinIO to docker-compose
docs(storage): add storage configuration guide
```

### Milestone 2.3: API для документов

**Branch:** `feature/sed-api`

#### Backend
- [ ] DocumentService
  - [ ] create_document()
  - [ ] get_document()
  - [ ] update_document()
  - [ ] delete_document()
  - [ ] list_documents()
  - [ ] search_documents()
  
- [ ] API endpoints: `/api/documents`
  - [ ] POST `/api/documents/upload` - загрузка документа
  - [ ] GET `/api/documents` - список документов
  - [ ] GET `/api/documents/:id` - получение документа
  - [ ] PUT `/api/documents/:id` - обновление
  - [ ] DELETE `/api/documents/:id` - удаление
  - [ ] GET `/api/documents/:id/download` - скачивание
  - [ ] GET `/api/documents/:id/history` - история
  - [ ] POST `/api/documents/:id/comments` - комментарий

**Коммиты:**
```
feat(sed): add document service
feat(sed): add documents API endpoints
feat(sed): add document search functionality
test(sed): add API integration tests
```

### Milestone 2.4: ЭЦП (Электронная подпись)

**Branch:** `feature/sed-signatures`

#### Backend
- [ ] Миграция: Таблицы для подписей
  - [ ] `document_signatures` (расширенная)
  - [ ] `user_certificates` - сертификаты пользователей
  
- [ ] SignatureService
  - [ ] create_signature()
  - [ ] verify_signature()
  - [ ] add_signature_stamp_to_pdf()
  - [ ] verify_certificate()
  
- [ ] Интеграция с КриптоПро (опционально)
  - [ ] CryptoProClient
  
- [ ] API endpoints: `/api/signatures`
  - [ ] POST `/api/documents/:id/sign` - подписать документ
  - [ ] POST `/api/documents/:id/verify` - проверить подписи
  - [ ] GET `/api/signatures/my-certificates` - мои сертификаты
  - [ ] POST `/api/signatures/upload-certificate` - загрузить сертификат

#### Frontend
- [ ] Composable: `useSignature`
  - [ ] getCertificates()
  - [ ] signDocument()
  - [ ] verifySignature()
  
- [ ] Компоненты
  - [ ] SignatureDialog.vue - диалог подписания
  - [ ] SignatureStamp.vue - штамп подписи
  - [ ] CertificateSelector.vue - выбор сертификата

**Коммиты:**
```
feat(sed): add signature database schema
feat(sed): add signature service
feat(sed): add signature API endpoints
feat(frontend): add signature composable
feat(frontend): add signature UI components
test(sed): add signature tests
docs(sed): add signature integration guide
```

### Milestone 2.5: Маршруты согласования

**Branch:** `feature/sed-approvals`

#### Backend
- [ ] Миграция: Таблицы согласования
  - [ ] `approval_routes` - маршруты
  - [ ] `approval_steps` - шаги маршрута
  - [ ] `document_approvals` - согласования
  
- [ ] ApprovalService
  - [ ] create_route()
  - [ ] start_approval()
  - [ ] approve_step()
  - [ ] reject_step()
  - [ ] delegate_step()
  
- [ ] API endpoints: `/api/approvals`
  - [ ] POST `/api/documents/:id/route` - создать маршрут
  - [ ] POST `/api/approvals/:id/approve` - согласовать
  - [ ] POST `/api/approvals/:id/reject` - отклонить
  - [ ] POST `/api/approvals/:id/delegate` - делегировать
  - [ ] GET `/api/approvals/pending` - ожидающие согласования

**Коммиты:**
```
feat(sed): add approval routes schema
feat(sed): add approval service
feat(sed): add approval API endpoints
feat(frontend): add approval workflow UI
test(sed): add approval workflow tests
```

### Milestone 2.6: Шаблоны документов

**Branch:** `feature/sed-templates`

#### Backend
- [ ] Миграция: `document_templates`
- [ ] TemplateService
- [ ] API endpoints: `/api/templates`

#### Frontend
- [ ] Страницы для шаблонов
- [ ] Редактор шаблонов

**Коммиты:**
```
feat(sed): add document templates
feat(frontend): add template editor
```

### Milestone 2.7: Frontend для СЭД

**Branch:** `feature/sed-frontend`

#### Frontend
- [ ] Страницы
  - [ ] `/documents` - список документов
  - [ ] `/documents/upload` - загрузка
  - [ ] `/documents/[id]` - просмотр документа
  - [ ] `/documents/[id]/sign` - подписание
  - [ ] `/documents/templates` - шаблоны
  - [ ] `/documents/archive` - архив
  
- [ ] Компоненты
  - [ ] DocumentList.vue
  - [ ] DocumentViewer.vue (PDF viewer)
  - [ ] DocumentUpload.vue
  - [ ] DocumentCard.vue
  - [ ] ApprovalTimeline.vue
  
- [ ] Composables
  - [ ] useDocuments.ts
  - [ ] useApprovals.ts

**Коммиты:**
```
feat(frontend): add documents pages
feat(frontend): add document viewer
feat(frontend): add document upload
feat(frontend): add approval timeline
```

---

## 🏗️ Фаза 3: Портал (Заявки + Задачи)

**Цель:** Расширить систему заявок и добавить задачи

### Milestone 3.1: Расширение заявок

**Branch:** `feature/portal-requests`

#### Backend
- [ ] Миграция: Расширение requests
  - [ ] `request_types` - типы заявок
  - [ ] Расширить `requests` (SLA, рейтинг, и т.д.)
  
- [ ] RequestService (расширить)
  - [ ] SLA tracking
  - [ ] Auto-assignment
  - [ ] Satisfaction rating
  
- [ ] API endpoints (расширить)
  - [ ] GET `/api/requests/types` - типы заявок
  - [ ] POST `/api/requests/:id/rate` - оценить заявку
  - [ ] GET `/api/requests/sla-report` - отчет по SLA

**Коммиты:**
```
feat(portal): extend requests system
feat(portal): add SLA tracking
feat(portal): add request types
feat(portal): add satisfaction rating
```

### Milestone 3.2: Система задач

**Branch:** `feature/portal-tasks`

#### Backend
- [ ] Миграция: Таблицы задач
  - [ ] `tasks` - задачи
  - [ ] `task_comments` - комментарии
  - [ ] `task_attachments` - вложения
  
- [ ] TaskService
  - [ ] create_task()
  - [ ] assign_task()
  - [ ] update_task_status()
  - [ ] add_comment()
  
- [ ] API endpoints: `/api/tasks`
  - [ ] GET `/api/tasks` - список задач
  - [ ] POST `/api/tasks` - создать задачу
  - [ ] GET `/api/tasks/:id` - детали задачи
  - [ ] PUT `/api/tasks/:id` - обновить
  - [ ] POST `/api/tasks/:id/comments` - комментарий
  - [ ] GET `/api/tasks/my` - мои задачи
  - [ ] GET `/api/tasks/assigned-by-me` - назначенные мной

#### Frontend
- [ ] Страницы
  - [ ] `/tasks` - список задач
  - [ ] `/tasks/create` - создать задачу
  - [ ] `/tasks/[id]` - детали задачи
  - [ ] `/tasks/my` - мои задачи
  
- [ ] Компоненты
  - [ ] TaskList.vue
  - [ ] TaskCard.vue
  - [ ] TaskBoard.vue (Kanban)
  - [ ] TaskForm.vue
  
- [ ] Composables
  - [ ] useTasks.ts

**Коммиты:**
```
feat(portal): add tasks database schema
feat(portal): add task service
feat(portal): add tasks API endpoints
feat(frontend): add tasks pages
feat(frontend): add Kanban board
test(portal): add tasks tests
```

---

## 🏗️ Фаза 4: Почтовый модуль (Stalwart)

**Цель:** Интегрировать корпоративную почту

### Milestone 4.1: Stalwart интеграция

**Branch:** `feature/mail-stalwart`

#### Backend
- [ ] Миграция: Таблицы почты
  - [ ] `mail_accounts` - почтовые аккаунты
  - [ ] `mail_folders` - папки
  - [ ] `mail_messages` - метаданные писем
  
- [ ] StalwartClient
  - [ ] create_account()
  - [ ] get_messages()
  - [ ] send_message()
  - [ ] sync_folders()
  
- [ ] MailService
  - [ ] sync_account()
  - [ ] send_email()
  - [ ] get_inbox()
  
- [ ] API endpoints: `/api/mail`
  - [ ] GET `/api/mail/inbox` - входящие
  - [ ] GET `/api/mail/folders` - папки
  - [ ] POST `/api/mail/send` - отправить
  - [ ] POST `/api/mail/sync` - синхронизация

#### Docker
- [ ] Добавить Stalwart в docker-compose.yml
- [ ] Настроить DNS и домен

**Коммиты:**
```
feat(mail): add Stalwart integration
feat(mail): add mail database schema
feat(mail): add mail service
feat(mail): add mail API endpoints
chore(docker): add Stalwart to docker-compose
docs(mail): add mail configuration guide
```

### Milestone 4.2: Frontend для почты

**Branch:** `feature/mail-frontend`

#### Frontend
- [ ] Страницы
  - [ ] `/mail` - почтовый клиент
  - [ ] `/mail/compose` - написать письмо
  - [ ] `/mail/[id]` - просмотр письма
  
- [ ] Компоненты
  - [ ] MailList.vue
  - [ ] MailViewer.vue
  - [ ] MailComposer.vue
  - [ ] MailFolders.vue
  
- [ ] Composables
  - [ ] useMail.ts

**Коммиты:**
```
feat(frontend): add mail client pages
feat(frontend): add mail composer
feat(frontend): add mail viewer
```

---

## 🏗️ Фаза 5: Дополнительные модули

### Milestone 5.1: Календарь и события

**Branch:** `feature/portal-calendar`

- [ ] Календарь событий
- [ ] Встречи и напоминания
- [ ] Интеграция с задачами

### Milestone 5.2: Новости и объявления

**Branch:** `feature/portal-news`

- [ ] Корпоративные новости
- [ ] Объявления
- [ ] Комментарии

### Milestone 5.3: Чат (опционально)

**Branch:** `feature/portal-chat`

- [ ] WebSocket чат
- [ ] Личные сообщения
- [ ] Групповые чаты

---

## 🏗️ Фаза 6: Production готовность

### Milestone 6.1: Тестирование

- [ ] Unit тесты (покрытие >80%)
- [ ] Integration тесты
- [ ] E2E тесты
- [ ] Load тесты

### Milestone 6.2: Безопасность

- [ ] Security audit
- [ ] Rate limiting
- [ ] CSRF protection
- [ ] XSS protection
- [ ] SQL injection protection

### Milestone 6.3: Мониторинг

- [ ] Prometheus метрики
- [ ] Grafana дашборды
- [ ] Error tracking (Sentry)
- [ ] Logging (ELK/Loki)

### Milestone 6.4: Документация

- [ ] API документация (Swagger)
- [ ] User guide
- [ ] Admin guide
- [ ] Development guide
- [ ] Deployment guide

---

## 📊 Приоритеты

### Высокий приоритет (MVP)
1. ✅ RBAC система
2. ✅ IT-Admin модуль (реорганизация)
3. ✅ СЭД: Документы + загрузка
4. ✅ СЭД: ЭЦП базовая
5. ✅ Портал: Задачи

### Средний приоритет
6. СЭД: Маршруты согласования
7. СЭД: Шаблоны
8. Портал: Расширенные заявки
9. Почта: Базовая интеграция

### Низкий приоритет
10. Календарь
11. Новости
12. Чат

---

## 🎯 Текущий статус

**Текущая фаза:** Фаза 0 - Фундамент
**Текущий Milestone:** 0.1 - RBAC система
**Текущая ветка:** `main`
**Следующая ветка:** `feature/rbac-foundation`

---

## 📝 Git workflow

### Стратегия веток
```
main (production)
  ├── develop (integration)
  │   ├── feature/rbac-foundation
  │   ├── feature/it-admin-module
  │   ├── feature/sed-database
  │   └── ...
  └── hotfix/* (критичные исправления)
```

### Naming convention
- `feature/*` - новый функционал
- `fix/*` - исправление багов
- `refactor/*` - рефакторинг
- `docs/*` - документация
- `test/*` - тесты
- `chore/*` - инфраструктура

### Commit messages
```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types:**
- `feat` - новая функция
- `fix` - исправление бага
- `refactor` - рефакторинг
- `docs` - документация
- `test` - тесты
- `chore` - инфраструктура
- `perf` - оптимизация

**Examples:**
```
feat(rbac): add role-based access control
fix(auth): resolve token expiration issue
refactor(sed): reorganize document service
docs(api): update API documentation
test(tasks): add unit tests for task service
chore(docker): update docker-compose configuration
```

---

## 🚀 Начало работы

1. Создать ветку `develop` от `main`
2. Создать ветку `feature/rbac-foundation` от `develop`
3. Начать работу над Milestone 0.1
