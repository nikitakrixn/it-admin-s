#!/usr/bin/env bash
#
# Восстановление базы данных из резервной копии.
#
# Запуск на сервере из корня проекта:
#   ./scripts/restore.sh backups/it-admin-20260101-030000.sql.gz
#
# Скрипт перезаписывает текущие данные: pg_dump сделан с --clean --if-exists.

set -euo pipefail

if [ $# -ne 1 ]; then
  echo "Использование: $0 <файл.sql.gz>" >&2
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COMPOSE_FILE="${COMPOSE_FILE:-$ROOT_DIR/docker-compose.yml}"
file="$1"

if [ ! -f "$file" ]; then
  echo "Файл не найден: $file" >&2
  exit 1
fi

echo "Будет восстановлена база из $file."
echo "Текущие данные будут перезаписаны."
read -r -p "Продолжить? [y/N] " answer

case "$answer" in
  [yYдД]*) ;;
  *)
    echo "Отменено"
    exit 1
    ;;
esac

gzip -dc "$file" | docker compose -f "$COMPOSE_FILE" exec -T postgres \
  sh -c 'psql -v ON_ERROR_STOP=1 -U "$POSTGRES_USER" -d "$POSTGRES_DB"'

echo "Восстановление завершено"
