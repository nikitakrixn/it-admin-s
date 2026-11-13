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

## Стек

| Слой           | Технологии                                               |
| -------------- | -------------------------------------------------------- |
| Бэкенд         | Rust 2024, poem + poem-openapi, Diesel Async, PostgreSQL |
| Авторизация    | JWT, Argon2                                              |
| Фронтенд       | Nuxt 4, Vue 3, TypeScript, Tailwind CSS, Nuxt Icon       |
| Инфраструктура | Docker Compose, GitHub Actions                           |
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
    middleware/      JWT-авторизация
    utils/           ошибки, пароли, трекер изменений
frontend/
  app/
    pages/           страницы приложения
    components/      переиспользуемые компоненты
    composables/     клиенты API и состояние
    plugins/         настройка $fetch
```

## Быстрый старт в Docker

```powershell
Copy-Item backend/.env.example backend/.env
Copy-Item frontend/.env.example frontend/.env
docker compose up --build
```

После запуска:

- приложение — <http://localhost:3000>
- API — <http://localhost:8000/api>
- Swagger UI — <http://localhost:8000/docs>

Миграции применяются автоматически при старте бэкенда. Для загрузки демо-данных
в контейнерную базу:

```powershell
docker compose cp backend/seeds postgres:/tmp/seeds
docker compose exec postgres sh -c 'for f in /tmp/seeds/*.sql; do psql -U admin -d itadmin -v ON_ERROR_STOP=1 -f "$f" || exit 1; done'
```

## Локальный запуск

Нужны Rust (stable), Node.js 24+ и PostgreSQL 16+.

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

### 4. Демо-данные

```powershell
$env:PGPASSWORD='12345678'
$psql = 'C:\Program Files\PostgreSQL\17\bin\psql.exe'
Get-ChildItem backend\seeds\*.sql | Sort-Object Name | ForEach-Object {
  & $psql -h localhost -p 5433 -U postgres -d it-admin-s -v ON_ERROR_STOP=1 -f $_.FullName
}
```

Подробнее о наборе данных — в [`backend/seeds/README.md`](backend/seeds/README.md).

## Учётная запись

Публичной регистрации администратора нет: новый пользователь создаётся через
`POST /api/auth/register` с ролью `user`. Чтобы выдать права администратора:

```powershell
$body = @{ email='admin@example.com'; password='TestPass123!'; name='Администратор' } | ConvertTo-Json
Invoke-RestMethod -Method Post -Uri 'http://localhost:8000/api/auth/register' `
  -ContentType 'application/json' -Body $body

$env:PGPASSWORD='12345678'
& 'C:\Program Files\PostgreSQL\17\bin\psql.exe' -h localhost -p 5433 -U postgres -d it-admin-s `
  -c "UPDATE users SET role = 'admin' WHERE email = 'admin@example.com';"
```

После этого можно войти в интерфейс и пользоваться разделами администрирования.

## API

- Swagger UI: `http://localhost:8000/docs`
- OpenAPI-спецификация: `http://localhost:8000/api-spec`
- Проверка здоровья: `http://localhost:8000/health`

Основные группы эндпоинтов: `/api/auth`, `/api/employees`, `/api/computers`,
`/api/equipment`, `/api/consumables`, `/api/requests`, `/api/software`,
`/api/licenses`, `/api/network`, `/api/ad`, `/api/monitoring`,
`/api/dashboard`, `/api/activity-log`.

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

Бэкенд (`backend/.env`):

| Переменная                     | Назначение                      |
| ------------------------------ | ------------------------------- |
| `DATABASE_URL`                 | строка подключения к PostgreSQL |
| `APP_HOST`, `APP_PORT`         | адрес и порт API                |
| `JWT_SECRET`, `JWT_EXPIRATION` | подпись и срок жизни токена     |
| `CORS_ALLOWED_ORIGINS`         | разрешённые источники           |
| `AD_*`                         | подключение к Active Directory  |
| `MIKROTIK_*`                   | интеграция с MikroTik           |
| `REDIS_URL`                    | кэш и сессии                    |

Фронтенд (`frontend/.env`):

| Переменная             | Назначение                           |
| ---------------------- | ------------------------------------ |
| `NUXT_PUBLIC_API_BASE` | адрес API для браузера               |
| `NUXT_API_BASE_SERVER` | адрес API для SSR и Docker-сети      |
| `NUXT_PUBLIC_APP_NAME` | название приложения                  |
| `NUXT_PUBLIC_WG_*`     | параметры по умолчанию для WireGuard |
