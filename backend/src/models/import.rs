use poem_openapi::Object;
use serde::{Deserialize, Serialize};

/// Тело запроса импорта: содержимое файла целиком.
///
/// Файл передаётся строкой, а не multipart: так проще тестировать, не нужен
/// парсер форм, а ограничение размера проверяется до разбора CSV.
#[derive(Debug, Deserialize, Object)]
pub struct ImportCsvRequest {
    /// Содержимое CSV-файла в UTF-8.
    pub csv: String,

    /// true — только проверить файл и показать отчёт, ничего не записывая.
    pub dry_run: Option<bool>,
}

/// Ошибка конкретной строки файла.
#[derive(Debug, Serialize, Object)]
pub struct ImportRowError {
    /// Номер строки в файле, считая заголовок (первая строка данных — 2).
    pub row: usize,
    pub message: String,
}

/// Итог импорта. Даже при ошибках в отдельных строках остальные строки
/// импортируются — это удобнее для реальных выгрузок из 1С и Excel.
#[derive(Debug, Serialize, Object)]
pub struct ImportReport {
    /// `employees` или `computers`.
    pub entity: String,
    pub dry_run: bool,
    /// Сколько строк данных было в файле.
    pub total_rows: usize,
    /// Сколько записей создано (в dry-run — было бы создано).
    pub created: usize,
    /// Сколько записей обновлено (в dry-run — было бы обновлено).
    pub updated: usize,
    /// Сколько строк пропущено из-за ошибок.
    pub skipped: usize,
    pub errors: Vec<ImportRowError>,
}

impl ImportReport {
    pub fn new(entity: &str, dry_run: bool, total_rows: usize) -> Self {
        Self {
            entity: entity.to_string(),
            dry_run,
            total_rows,
            created: 0,
            updated: 0,
            skipped: 0,
            errors: Vec::new(),
        }
    }

    pub fn push_error(&mut self, row: usize, message: impl Into<String>) {
        self.skipped += 1;
        self.errors.push(ImportRowError {
            row,
            message: message.into(),
        });
    }
}
