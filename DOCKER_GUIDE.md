# Запуск в Docker

## Требования

- Docker Desktop или Docker Engine с Compose v2
- свободные порты `3000`, `8000`, `5432`, `6379`

## Подготовка

Скопируйте примеры окружения:

```powershell
Copy-Item backend/.env.example backend/.env
Copy-Item frontend/.env.example frontend/.env
```

В `backend/.env` при необходимости измените `JWT_SECRET` и параметры
подключения к Active Directory и MikroTik. Для Docker-сети `DATABASE_URL`
переопределяется в `docker-compose.yml` и указывает на сервис `postgres`.

## Production-режим

```powershell
docker compose up --build -d
docker compose logs -f
```

Остановка:

```powershell
docker compose down
```

Остановка с удалением данных PostgreSQL и Redis:

```powershell
docker compose down -v
```

## Режим разработки

```powershell
docker compose -f docker-compose.yml -f docker-compose.dev.yml up
```

В этом режиме исходники монтируются в контейнеры, бэкенд работает через
`cargo watch`, а Nuxt — в dev-режиме с hot reload.

## Демо-данные

```powershell
docker compose cp backend/seeds postgres:/tmp/seeds
docker compose exec postgres sh -c 'for f in /tmp/seeds/*.sql; do psql -U admin -d itadmin -v ON_ERROR_STOP=1 -f "$f" || exit 1; done'
```

Скрипты идемпотентные, их можно запускать повторно.

## Адреса API

В браузере фронтенд обращается к `http://localhost:8000/api`. Внутри
Docker-сети SSR использует `http://backend:8000/api`. Эти адреса разделены
переменными `NUXT_PUBLIC_API_BASE` и `NUXT_API_BASE_SERVER`, поэтому менять
код при развёртывании не нужно.

## Полезные команды

```powershell
# статус контейнеров
docker compose ps

# логи только бэкенда
docker compose logs -f backend

# подключиться к PostgreSQL
docker compose exec postgres psql -U admin -d itadmin

# пересобрать один сервис
docker compose up --build -d backend
```
