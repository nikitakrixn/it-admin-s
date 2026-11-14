#!/usr/bin/env bash
#
# Резервная копия базы данных PostgreSQL.
#
# Запуск на сервере из корня проекта:
#   ./scripts/backup.sh
#
# Переменные окружения:
#   BACKUP_DIR      каталог для архивов (по умолчанию ./backups)
#   RETENTION_DAYS  сколько дней хранить копии (по умолчанию 14)

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPOSE_FILE="${COMPOSE_FILE:-$ROOT_DIR/docker-compose.yml}"
BACKUP_DIR="${BACKUP_DIR:-$ROOT_DIR/backups}"
RETENTION_DAYS="${RETENTION_DAYS:-14}"

mkdir -p "$BACKUP_DIR"

timestamp="$(date +%Y%m%d-%H%M%S)"
file="$BACKUP_DIR/it-admin-$timestamp.sql.gz"

# Переменные POSTGRES_USER и POSTGRES_DB уже заданы внутри контейнера,
# поэтому их не нужно дублировать в окружении хоста.
if ! docker compose -f "$COMPOSE_FILE" exec -T postgres \
  sh -c 'pg_dump --clean --if-exists -U "$POSTGRES_USER" -d "$POSTGRES_DB"' \
  | gzip > "$file"; then
  echo "Ошибка: не удалось создать резервную копию" >&2
  rm -f "$file"
  exit 1
fi

# Пустой архив означает, что pg_dump ничего не отдал — такую копию хранить нельзя.
if [ ! -s "$file" ]; then
  echo "Ошибка: резервная копия пуста" >&2
  rm -f "$file"
  exit 1
fi

find "$BACKUP_DIR" -name 'it-admin-*.sql.gz' -mtime +"$RETENTION_DAYS" -delete

echo "Резервная копия создана: $file"
echo "Размер: $(du -h "$file" | cut -f1)"
