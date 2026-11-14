# Запуск в Docker

## Требования

- Docker Engine с Compose v2
- домен, A-запись которого указывает на сервер
- открытые порты `80` и `443` (Caddy выпускает TLS-сертификат Let's Encrypt)

PostgreSQL, API и Nuxt наружу не публикуются: снаружи доступен только Caddy,
который проксирует запросы на Nuxt-сервер. Браузер общается с API через
встроенный BFF, а JWT хранится в httpOnly-cookie.

## Production

1. Скопируйте пример окружения и заполните его:

   ```bash
   cp .env.example .env
   ```

   Обязательно замените:

   - `DOMAIN` и `ACME_EMAIL` — домен и почта для Let's Encrypt;
   - `APP_ORIGIN` — тот же адрес с `https://`;
   - `POSTGRES_PASSWORD` и `DATABASE_URL` — пароль должен совпадать в обоих
     местах, а в URL допускаются только буквы, цифры, `-` и `_`;
   - `JWT_SECRET` — не короче 32 символов (`openssl rand -base64 48`);
   - `BOOTSTRAP_ADMIN_EMAIL` и `BOOTSTRAP_ADMIN_PASSWORD` — первый
     администратор, пароль не короче 12 символов с буквой и цифрой.

2. Запустите сервисы:

   ```bash
   docker compose up --build -d
   docker compose logs -f
   ```

3. Откройте `https://<DOMAIN>` и войдите под учётной записью из
   `BOOTSTRAP_ADMIN_*`. Смените пароль в разделе «Пароль» в левом нижнем углу.

Начальный администратор создаётся только при пустой таблице `users`. Если
пользователи уже есть, переменные `BOOTSTRAP_ADMIN_*` игнорируются. В
production без них и с пустой базой приложение не запустится — это защита от
случайного развёртывания без администратора.

Дальнейшие пользователи создаются в разделе «Пользователи» (доступен только
администраторам).

### Обновление

```bash
git pull
docker compose up --build -d
docker image prune -f
```

Миграции применяются автоматически при старте бэкенда.

### Остановка

```bash
docker compose down          # остановить, данные сохранить
docker compose down -v       # остановить и удалить тома (база, сертификаты)
```

## Режим разработки

```bash
docker compose -f docker-compose.dev.yml up
```

Исходники монтируются в контейнеры, бэкенд работает через `cargo watch`, Nuxt —
в dev-режиме с hot reload. Доступны:

- приложение — <http://localhost:3000>
- API — <http://localhost:8000/api>
- Swagger UI — <http://localhost:8000/docs>

Демо-администратор в dev-режиме: `admin@example.com` / `AdminPassword123!`.

## Демо-данные

```bash
docker compose exec -T postgres sh -c 'psql -U "$POSTGRES_USER" -d "$POSTGRES_DB"' < backend/seeds/01_employees.sql
```

Файлы в `backend/seeds/` идемпотентные, их можно применять повторно и в любом
порядке. Набор данных описан в [`backend/seeds/README.md`](backend/seeds/README.md).

## Резервное копирование

Скрипты рассчитаны на Linux-сервер с bash.

```bash
./scripts/backup.sh                       # архив в ./backups
./scripts/restore.sh backups/it-admin-20260101-030000.sql.gz
```

`backup.sh` делает `pg_dump`, сжимает результат, проверяет, что архив не пустой,
и удаляет копии старше 14 дней. Каталог и срок хранения меняются переменными
`BACKUP_DIR` и `RETENTION_DAYS`.

Пример ежедневного запуска в 3:00 через cron:

```cron
0 3 * * * cd /opt/it-admin && BACKUP_DIR=/var/backups/it-admin ./scripts/backup.sh >> /var/log/it-admin-backup.log 2>&1
```

Копии нужно хранить вне сервера: отправляйте каталог `backups/` в объектное
хранилище или на отдельный диск. Восстановление перезаписывает текущую базу,
поэтому перед запуском скрипт спрашивает подтверждение.

## Полезные команды

```bash
# статус контейнеров
docker compose ps

# логи
docker compose logs -f backend
docker compose logs -f caddy

# подключиться к PostgreSQL
docker compose exec postgres sh -c 'psql -U "$POSTGRES_USER" -d "$POSTGRES_DB"'

# пересобрать один сервис
docker compose up --build -d backend
```

## Если что-то не запускается

- **`JWT_SECRET должен содержать минимум 32 символа`** — значение в `.env`
  слишком короткое или осталось из примера.
- **`В базе нет пользователей. Задайте BOOTSTRAP_ADMIN_EMAIL...`** — на пустой
  базе не заданы данные первого администратора.
- **Caddy не выпускает сертификат** — проверьте, что A-запись домена указывает
  на сервер, порты 80/443 открыты, а `DOMAIN` в `.env` совпадает с адресом, по
  которому открывается панель.
- **Не удаётся войти после смены домена** — cookie привязана к домену; очистите
  cookie для старого адреса.
