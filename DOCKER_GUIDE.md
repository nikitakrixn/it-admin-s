# Docker Deployment Guide

## Использование

### Production

```bash
# Создание env файлов
cp backend/.env.example backend/.env
cp frontend/.env.example frontend/.env

# Запуск продакшн мода
docker-compose up -d

# Проверка лога
docker-compose logs -f

# Остановка
docker-compose down
```

### Development

```bash
# Запуск dev мода с hot reload
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up

# В фоне
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up -d

# Остановка
docker-compose -f docker-compose.yml -f docker-compose.dev.yml down
```

### Миграции базы данных

```bash
# Запуск миграции дизель вручнеую
docker-compose exec backend diesel migration run

# Откат последней миграции
docker-compose exec backend diesel migration revert
```

## Troubleshooting

### Backend не запускается
```bash
# Проверьте логи
docker-compose logs backend

# Проверьте подключение к БД
docker-compose exec backend diesel migration run

# Проверьте переменные окружения
docker-compose exec backend env | grep DATABASE_URL
```

### Frontend не может подключиться к backend
```bash
# Проверьте, что backend работает
docker-compose ps

# Проверьте healthcheck
docker-compose exec backend wget -O- http://localhost:8000/api/health

```

### Проблемы с миграциями
```bash
# Сброс БД
docker-compose down -v
docker-compose up -d postgres
docker-compose exec backend diesel migration run
```

## Дополнительные команды

```bash
# Пересобрать образы
docker-compose build --no-cache

# Посмотреть использование ресурсов
docker stats

# Очистить неиспользуемые образы
docker system prune -a

# Экспорт/импорт БД
docker-compose exec postgres pg_dump -U admin itadmin > backup.sql
docker-compose exec -T postgres psql -U admin itadmin < backup.sql
```
