# IT-Admin

Веб-панель для администрирования IT-инфраструктуры небольшой компании:
сотрудники и подразделения, компьютеры и комплектующие, оргтехника и расходники,
заявки с SLA и комментариями, каталог ПО и лицензии, сеть с IP-адресами и
сканированием, Active Directory, мониторинг и аналитика.

Проект сделан как портфолио: реальный REST API на Rust, PostgreSQL с миграциями,
JWT-авторизация, демонстрационные данные, unit-тесты и CI.

## Возможности

- **Главная и аналитика** — сводка по инфраструктуре, журнал активности, сроки
  лицензий, проблемные позиции, графики за 7/30/90 дней и экспорт CSV.
- **Сотрудники** — карточки, подразделения, должности, статусы, привязка техники
  и учётных записей Active Directory.
- **Компьютеры** — инвентарь, гарантии, онлайн-статус, установленное ПО,
  процессоры, память, диски, видеокарты, мониторы, периферия и антивирус.
- **Оргтехника и расходники** — принтеры, МФУ, сканеры, ИБП, телефоны; остатки,
  минимальные запасы и история движений по складу.
- **Заявки** — приоритеты, статусы, категории, SLA, комментарии, история
  изменений и канбан-представление.
- **ПО и лицензии** — каталог программ, установки на компьютерах, лицензионные
  пулы и назначения.
- **Сеть** — сегменты, коммутаторы, точки доступа, IP-адреса и реальное
  TCP-сканирование диапазона с сохранением результатов.
- **Active Directory** — группы, учётные записи, членство и привязка к
  сотрудникам.
- **Мониторинг** — последние метрики компьютеров и история замеров.
- **Инвентаризация** — сессии сверки техники с учётными данными: снимок
  компьютеров на момент старта, отметки «найдено / отсутствует / повреждено»,
  фактические места и состояния, итоги по завершении.
- **Импорт из CSV** — загрузка сотрудников и компьютеров из выгрузок Excel и 1С
  с предварительной проверкой файла и отчётом об ошибках по строкам.
- **Пользователи** — роли, блокировка учётных записей и смена пароля.

## Стек

| Слой           | Технологии                                               |
| -------------- | -------------------------------------------------------- |
| Бэкенд         | Rust 2024, poem + poem-openapi, Diesel Async, PostgreSQL |
| Авторизация    | JWT, Argon2, httpOnly-cookie на стороне Nuxt             |
| Фронтенд       | Nuxt 4, Vue 3, TypeScript, Tailwind CSS, Nuxt Icon       |
| Инфраструктура | Docker Compose, Caddy с TLS, GitHub Actions              |
| Качество       | rustfmt, Clippy, ESLint, Prettier, vue-tsc               |

## Структура

```text
backend/
  migrations/        SQL-миграции схемы
  seeds/             идемпотентные демо-данные
  src/
    models/          модели и OpenAPI-схемы
    repositories/    доступ к PostgreSQL
    routes/          REST-эндпоинты
    services/        бизнес-логика и интеграции
    middleware/      авторизация и rate limit
    utils/           ошибки, пароли, трекер изменений
frontend/
  app/
    pages/           страницы приложения
    components/      переиспользуемые компоненты
    composables/     клиенты API и состояние
    middleware/      защита маршрутов
    plugins/         настройка $fetch
  server/            BFF: вход, выход, прокси API
scripts/             резервное копирование и восстановление
Caddyfile            TLS и reverse proxy
```

## Быстрый старт

Production-развёртывание с TLS описано в [`DOCKER_GUIDE.md`](DOCKER_GUIDE.md):
нужен домен, `.env` с секретами и `docker compose up --build -d`.

Для локальной разработки нужны Rust (stable), Node.js 24+ и PostgreSQL 16+.

### 1. База данных

Создайте базу и укажите её в `backend/.env`:

```env
DATABASE_URL=postgres://postgres:12345678@localhost:5433/it-admin-s
```

Порт и учётные данные замените на свои, если PostgreSQL настроен иначе.

### 2. Бэкенд

```powershell
cd backend
Copy-Item .env.example .env
cargo run
```

Сервер слушает `http://localhost:8000`, миграции применяются при старте.
При первом запуске на пустой базе создаётся администратор из
`BOOTSTRAP_ADMIN_EMAIL` и `BOOTSTRAP_ADMIN_PASSWORD`.

### 3. Фронтенд

```powershell
cd frontend
npm ci
Copy-Item .env.example .env
npm run dev
```

`npm run dev` поднимает сразу бэкенд через `cargo watch` (нужен установленный
`cargo-watch`) и Nuxt. Если бэкенд уже запущен отдельно, используйте
`npm run frontend`.

Приложение — <http://localhost:3000>. Браузер не обращается к API напрямую:
Nuxt-сервер проксирует запросы через `/api/backend` и подставляет токен из
httpOnly-cookie.

### 4. Демо-данные

```powershell
$env:PGPASSWORD='12345678'
$psql = 'C:\Program Files\PostgreSQL\17\bin\psql.exe'
Get-ChildItem backend\seeds\*.sql | Sort-Object Name | ForEach-Object {
  & $psql -h localhost -p 5433 -U postgres -d it-admin-s -v ON_ERROR_STOP=1 -f $_.FullName
}
```

Подробнее о наборе данных — в [`backend/seeds/README.md`](backend/seeds/README.md).
Демо-администратор: `qa-monitor@example.com` / `TestPass123!`. Эти данные
предназначены только для локальной разработки.

## Импорт из CSV

Раздел доступен администраторам. Файл читается как UTF-8, при ошибке
кодировки — как Windows-1251; разделитель `;`, `,` или табуляция определяется
автоматически. Заголовки понимаются на русском и английском, даты — `ГГГГ-ММ-ДД`
или `ДД.ММ.ГГГГ`.

Порядок работы: скачать шаблон, заполнить и нажать «Проверить файл» (dry-run —
ничего не пишется в базу), затем «Импортировать». Повторный импорт не создаёт
дубликатов: сотрудники сопоставляются по email, затем по логину AD; компьютеры —
по инвентарному номеру, затем по имени. Подразделения и должности создаются по
названию, если их ещё нет. Строки без идентификатора и строки с ошибками
попадают в отчёт, остальные импортируются.

## Пользователи и безопасность

- Публичной регистрации нет. Новых пользователей создаёт администратор в
  разделе «Пользователи»; роль назначается там же.
- Вход блокируется на 15 минут после 5 неудачных попыток. Дополнительно
  действует ограничение 20 попыток за 5 минут с одного IP.
- Пароль при создании и смене — не короче 12 символов, обязательно буква и
  цифра. Хеширование — Argon2id.
- JWT не попадает в JavaScript: Nuxt-сервер хранит его в httpOnly-cookie
  (`SameSite=Lax`, `Secure` в production) и проверяет заголовок `Origin` у
  изменяющих запросов.
- Все эндпоинты API требуют токен, кроме `/api/auth/login`, `/health` и
  документации (последняя выключена в production).
- В production наружу опубликован только Caddy с TLS; PostgreSQL, API и
  Nuxt-сервер доступны лишь внутри Docker-сети.

## API

- Swagger UI: `http://localhost:8000/docs` (только при `DOCS_ENABLED=true`)
- OpenAPI-спецификация: `http://localhost:8000/api-spec`
- Проверка здоровья: `http://localhost:8000/health`

Основные группы эндпоинтов: `/api/auth`, `/api/employees`, `/api/computers`,
`/api/equipment`, `/api/consumables`, `/api/requests`, `/api/software`,
`/api/licenses`, `/api/network`, `/api/ad`, `/api/monitoring`,
`/api/dashboard`, `/api/activity-log`, `/api/inventory`, `/api/import`.

Запросы к API выполняются с заголовком `Authorization: Bearer <JWT>`. Токен
выдаёт `POST /api/auth/login`.

## Проверки

Бэкенд:

```powershell
cd backend
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Фронтенд:

```powershell
cd frontend
npm run lint
npm run format:check
npm run typecheck
npm run build
```

CI-workflow [`.github/workflows/ci.yml`](.github/workflows/ci.yml) запускает эти
проверки на каждый push в `main` и на pull request.

## Переменные окружения

Production-конфигурация лежит в `.env` в корне проекта — полный список с
комментариями в [`.env.example`](.env.example).

Бэкенд (`backend/.env` при локальном запуске):

| Переменная                     | Назначение                                         |
| ------------------------------ | -------------------------------------------------- |
| `DATABASE_URL`                 | строка подключения к PostgreSQL                    |
| `APP_HOST`, `APP_PORT`         | адрес и порт API                                   |
| `JWT_SECRET`, `JWT_EXPIRATION` | подпись и срок жизни токена (секрет ≥ 32 символов) |
| `CORS_ALLOWED_ORIGINS`         | разрешённые источники                              |
| `BOOTSTRAP_ADMIN_EMAIL`        | email первого администратора                       |
| `BOOTSTRAP_ADMIN_PASSWORD`     | пароль первого администратора                      |
| `DOCS_ENABLED`                 | Swagger UI (по умолчанию выключен в production)    |
| `AD_*`                         | подключение к Active Directory                     |
| `MIKROTIK_*`                   | интеграция с MikroTik                              |

Фронтенд (`frontend/.env`):

| Переменная             | Назначение                           |
| ---------------------- | ------------------------------------ |
| `NUXT_API_BASE_SERVER` | адрес API для Nuxt-сервера           |
| `NUXT_PUBLIC_APP_NAME` | название приложения                  |
| `NUXT_PUBLIC_APP_ENV`  | `development` или `production`       |
| `NUXT_PUBLIC_WG_*`     | параметры по умолчанию для WireGuard |
