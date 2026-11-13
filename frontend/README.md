# IT-Admin — фронтенд

Клиентская часть панели администрирования IT-инфраструктуры на Nuxt 4 и Vue 3.
Общее описание проекта, запуск и настройка — в [корневом README](../README.md).

## Запуск

```powershell
npm ci
Copy-Item .env.example .env
npm run frontend
```

`npm run dev` дополнительно поднимает бэкенд через `cargo watch`.

## Скрипты

| Команда                | Действие                                   |
| ---------------------- | ------------------------------------------ |
| `npm run frontend`     | dev-сервер Nuxt на `http://localhost:3000` |
| `npm run dev`          | бэкенд и фронтенд одновременно             |
| `npm run build`        | production-сборка                          |
| `npm run preview`      | предпросмотр production-сборки             |
| `npm run typecheck`    | проверка типов через `vue-tsc`             |
| `npm run lint`         | ESLint                                     |
| `npm run lint:fix`     | ESLint с автоисправлением                  |
| `npm run format`       | Prettier                                   |
| `npm run format:check` | проверка форматирования                    |

## Структура

```text
app/
  pages/          маршруты приложения
  components/     компоненты по разделам
  composables/    клиенты API и состояние
  plugins/        настройка $fetch и авторизации
  middleware/     защита маршрутов
```

Адрес API задаётся через `NUXT_PUBLIC_API_BASE` (браузер) и
`NUXT_API_BASE_SERVER` (SSR, Docker).
