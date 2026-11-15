use std::collections::HashMap;

use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::config::database::Pool;
use crate::models::computer::{Computer, NewComputer, UpdateComputer};
use crate::models::department::NewDepartment;
use crate::models::employee::{Employee, NewEmployee, UpdateEmployee};
use crate::models::import::ImportReport;
use crate::models::position::NewPosition;
use crate::models::schema::{computers, departments, employees, positions};
use crate::utils::db_helpers::{DbResult, get_connection};
use crate::utils::error::AppError;

/// Больше 10 МБ CSV в память не берём: это заведомо больше любой выгрузки
/// сотрудников или компьютеров, а сервер не должен падать от случайного файла.
const MAX_FILE_BYTES: usize = 10 * 1024 * 1024;

/// Ограничение на количество строк данных (без заголовка).
const MAX_ROWS: usize = 20_000;

pub struct ImportService {
    pool: Pool,
}

impl ImportService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // Сотрудники
    // ========================================================================

    pub async fn import_employees(
        &self,
        content: &str,
        dry_run: bool,
    ) -> Result<ImportReport, AppError> {
        let table = CsvTable::parse(content)?;
        let columns = EmployeeColumns::new(&table)?;

        let mut report = ImportReport::new("employees", dry_run, table.rows.len());
        let mut conn = get_connection(&self.pool).await?;

        let existing = employees::table
            .select((employees::id, employees::email, employees::ad_username))
            .load::<(i32, Option<String>, Option<String>)>(&mut conn)
            .await?;

        let mut by_email: HashMap<String, Option<i32>> = HashMap::new();
        let mut by_ad_username: HashMap<String, Option<i32>> = HashMap::new();
        for (id, email, ad_username) in existing {
            if let Some(email) = email {
                by_email.insert(email.to_lowercase(), Some(id));
            }
            if let Some(ad_username) = ad_username {
                by_ad_username.insert(ad_username.to_lowercase(), Some(id));
            }
        }

        let mut department_cache = load_named_ids(
            departments::table
                .select((departments::id, departments::name))
                .load::<(i32, String)>(&mut conn)
                .await?,
        );
        // Должность уникальна в пределах подразделения (UNIQUE(name, department_id)),
        // поэтому кэш ключуется парой: иначе одноимённые должности разных отделов
        // склеились бы в одну.
        let mut position_cache: HashMap<(String, Option<i32>), Option<i32>> = positions::table
            .select((positions::id, positions::name, positions::department_id))
            .load::<(i32, String, Option<i32>)>(&mut conn)
            .await?
            .into_iter()
            .map(|(id, name, department_id)| ((name.to_lowercase(), department_id), Some(id)))
            .collect();

        for (row_number, row) in &table.rows {
            let parsed = match parse_employee_row(&columns, row) {
                Ok(parsed) => parsed,
                Err(message) => {
                    report.push_error(*row_number, message);
                    continue;
                }
            };

            let email_key = parsed.email.as_ref().map(|value| value.to_lowercase());
            let ad_key = parsed
                .ad_username
                .as_ref()
                .map(|value| value.to_lowercase());

            let known = email_key
                .as_ref()
                .is_some_and(|key| by_email.contains_key(key))
                || ad_key
                    .as_ref()
                    .is_some_and(|key| by_ad_username.contains_key(key));

            let existing_id = email_key
                .as_ref()
                .and_then(|key| by_email.get(key))
                .copied()
                .flatten()
                .or_else(|| {
                    ad_key
                        .as_ref()
                        .and_then(|key| by_ad_username.get(key))
                        .copied()
                        .flatten()
                });

            // Справочники создаём только после того, как строка прошла проверку:
            // иначе битая строка оставит после себя пустое подразделение.
            let department_id = resolve_department(
                &mut conn,
                &mut department_cache,
                parsed.department.as_deref(),
                dry_run,
            )
            .await?;

            let position_id = resolve_position(
                &mut conn,
                &mut position_cache,
                parsed.position.as_deref(),
                department_id,
                dry_run,
            )
            .await?;

            let result = if let Some(id) = existing_id {
                if dry_run {
                    Ok(Some(id))
                } else {
                    let changes = UpdateEmployee {
                        first_name: Some(parsed.first_name),
                        last_name: Some(parsed.last_name),
                        middle_name: parsed.middle_name,
                        position_id,
                        department_id,
                        email: parsed.email,
                        phone: parsed.phone,
                        ad_username: parsed.ad_username,
                        hire_date: parsed.hire_date,
                        termination_date: None,
                        status: Some(parsed.status),
                        notes: parsed.notes,
                        location_id: None,
                        photo_url: None,
                        telegram_username: None,
                        is_vip: None,
                    };

                    self.update_employee(id, changes)
                        .await
                        .map(|employee| Some(employee.id))
                }
            } else {
                let new_employee = NewEmployee {
                    first_name: parsed.first_name,
                    last_name: parsed.last_name,
                    middle_name: parsed.middle_name,
                    position_id,
                    department_id,
                    email: parsed.email.clone(),
                    phone: parsed.phone,
                    ad_username: parsed.ad_username.clone(),
                    hire_date: parsed.hire_date,
                    status: parsed.status,
                    notes: parsed.notes,
                    location_id: None,
                    photo_url: None,
                    telegram_username: None,
                    is_vip: None,
                };

                if dry_run {
                    Ok(None)
                } else {
                    self.insert_employee(new_employee)
                        .await
                        .map(|employee| Some(employee.id))
                }
            };

            match result {
                Ok(stored_id) => {
                    if known {
                        report.updated += 1;
                    } else {
                        report.created += 1;
                    }

                    if let Some(key) = email_key {
                        by_email.insert(key, stored_id);
                    }
                    if let Some(key) = ad_key {
                        by_ad_username.insert(key, stored_id);
                    }
                }
                Err(error) => report.push_error(*row_number, friendly_db_error(&error)),
            }
        }

        Ok(report)
    }

    // ========================================================================
    // Компьютеры
    // ========================================================================

    pub async fn import_computers(
        &self,
        content: &str,
        dry_run: bool,
    ) -> Result<ImportReport, AppError> {
        let table = CsvTable::parse(content)?;
        let columns = ComputerColumns::new(&table)?;

        let mut report = ImportReport::new("computers", dry_run, table.rows.len());
        let mut conn = get_connection(&self.pool).await?;

        let employee_ids: HashMap<String, i32> = employees::table
            .filter(employees::email.is_not_null())
            .select((employees::id, employees::email))
            .load::<(i32, Option<String>)>(&mut conn)
            .await?
            .into_iter()
            .filter_map(|(id, email)| email.map(|email| (email.to_lowercase(), id)))
            .collect();

        let existing = computers::table
            .select((
                computers::id,
                computers::inventory_number,
                computers::hostname,
            ))
            .load::<(i32, Option<String>, Option<String>)>(&mut conn)
            .await?;

        let mut by_inventory: HashMap<String, Option<i32>> = HashMap::new();
        let mut by_hostname: HashMap<String, Option<i32>> = HashMap::new();
        for (id, inventory_number, hostname) in existing {
            if let Some(inventory_number) = inventory_number {
                by_inventory.insert(inventory_number, Some(id));
            }
            if let Some(hostname) = hostname {
                by_hostname.insert(hostname.to_lowercase(), Some(id));
            }
        }

        for (row_number, row) in &table.rows {
            let parsed = match parse_computer_row(&columns, row) {
                Ok(parsed) => parsed,
                Err(message) => {
                    report.push_error(*row_number, message);
                    continue;
                }
            };

            let existing_id = parsed
                .inventory_number
                .as_ref()
                .and_then(|value| by_inventory.get(value).copied().flatten())
                .or_else(|| {
                    parsed
                        .hostname
                        .as_ref()
                        .and_then(|value| by_hostname.get(&value.to_lowercase()).copied().flatten())
                });

            let known = parsed
                .inventory_number
                .as_ref()
                .is_some_and(|value| by_inventory.contains_key(value))
                || parsed
                    .hostname
                    .as_ref()
                    .is_some_and(|value| by_hostname.contains_key(&value.to_lowercase()));

            let employee_id = match parsed.employee_email.as_ref() {
                Some(email) => match employee_ids.get(email) {
                    Some(id) => Some(*id),
                    None => {
                        report.push_error(
                            *row_number,
                            format!(
                                "сотрудник с email «{}» не найден — сначала импортируйте сотрудников",
                                email
                            ),
                        );
                        continue;
                    }
                },
                None => None,
            };

            let result = if let Some(id) = existing_id {
                if dry_run {
                    Ok(Some(id))
                } else {
                    let changes = UpdateComputer {
                        inventory_number: parsed.inventory_number.clone(),
                        hostname: parsed.hostname.clone(),
                        computer_type: Some(parsed.computer_type),
                        manufacturer: parsed.manufacturer,
                        model: parsed.model,
                        serial_number: parsed.serial_number,
                        os: parsed.os,
                        os_version: parsed.os_version,
                        os_build: None,
                        os_architecture: None,
                        domain_joined: None,
                        last_boot_time: None,
                        employee_id,
                        location: parsed.location,
                        purchase_date: parsed.purchase_date,
                        warranty_end_date: parsed.warranty_end_date,
                        status: Some(parsed.status),
                        notes: parsed.notes,
                        last_seen_online: None,
                        is_online: None,
                        last_user_logged: None,
                        bitlocker_enabled: None,
                        antivirus_status: None,
                        windows_activation_status: None,
                        location_id: None,
                    };

                    self.update_computer(id, changes)
                        .await
                        .map(|computer| Some(computer.id))
                }
            } else {
                let new_computer = NewComputer {
                    inventory_number: parsed.inventory_number.clone(),
                    hostname: parsed.hostname.clone(),
                    computer_type: Some(parsed.computer_type),
                    manufacturer: parsed.manufacturer,
                    model: parsed.model,
                    serial_number: parsed.serial_number,
                    os: parsed.os,
                    os_version: parsed.os_version,
                    os_build: None,
                    os_architecture: None,
                    domain_joined: None,
                    last_boot_time: None,
                    employee_id,
                    location: parsed.location,
                    purchase_date: parsed.purchase_date,
                    warranty_end_date: parsed.warranty_end_date,
                    status: Some(parsed.status),
                    notes: parsed.notes,
                    last_seen_online: None,
                    is_online: None,
                    last_user_logged: None,
                    bitlocker_enabled: None,
                    antivirus_status: None,
                    windows_activation_status: None,
                    location_id: None,
                };

                if dry_run {
                    Ok(None)
                } else {
                    self.insert_computer(new_computer)
                        .await
                        .map(|computer| Some(computer.id))
                }
            };

            match result {
                Ok(stored_id) => {
                    if known {
                        report.updated += 1;
                    } else {
                        report.created += 1;
                    }

                    if let Some(key) = parsed.inventory_number {
                        by_inventory.insert(key, stored_id);
                    }
                    if let Some(key) = parsed.hostname {
                        by_hostname.insert(key.to_lowercase(), stored_id);
                    }
                }
                Err(error) => report.push_error(*row_number, friendly_db_error(&error)),
            }
        }

        Ok(report)
    }

    // ========================================================================
    // Запись в БД
    // ========================================================================

    async fn insert_employee(&self, new_employee: NewEmployee) -> DbResult<Employee> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(employees::table)
            .values(&new_employee)
            .get_result::<Employee>(&mut conn)
            .await
            .map_err(Into::into)
    }

    async fn update_employee(&self, id: i32, changes: UpdateEmployee) -> DbResult<Employee> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(employees::table.find(id))
            .set(&changes)
            .get_result::<Employee>(&mut conn)
            .await
            .map_err(Into::into)
    }

    async fn insert_computer(&self, new_computer: NewComputer) -> DbResult<Computer> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::insert_into(computers::table)
            .values(&new_computer)
            .get_result::<Computer>(&mut conn)
            .await
            .map_err(Into::into)
    }

    async fn update_computer(&self, id: i32, changes: UpdateComputer) -> DbResult<Computer> {
        let mut conn = get_connection(&self.pool).await?;

        diesel::update(computers::table.find(id))
            .set(&changes)
            .get_result::<Computer>(&mut conn)
            .await
            .map_err(Into::into)
    }
}

// ============================================================================
// Справочники
// ============================================================================

fn load_named_ids(rows: Vec<(i32, String)>) -> HashMap<String, Option<i32>> {
    rows.into_iter()
        .map(|(id, name)| (name.to_lowercase(), Some(id)))
        .collect()
}

async fn resolve_department(
    conn: &mut AsyncPgConnection,
    cache: &mut HashMap<String, Option<i32>>,
    name: Option<&str>,
    dry_run: bool,
) -> Result<Option<i32>, AppError> {
    let Some(name) = name else {
        return Ok(None);
    };
    let key = name.to_lowercase();

    if let Some(id) = cache.get(&key) {
        return Ok(*id);
    }

    if dry_run {
        cache.insert(key, None);
        return Ok(None);
    }

    let id = diesel::insert_into(departments::table)
        .values(&NewDepartment {
            name: name.to_string(),
            description: None,
        })
        .returning(departments::id)
        .get_result::<i32>(&mut *conn)
        .await?;

    cache.insert(key, Some(id));
    Ok(Some(id))
}

async fn resolve_position(
    conn: &mut AsyncPgConnection,
    cache: &mut HashMap<(String, Option<i32>), Option<i32>>,
    name: Option<&str>,
    department_id: Option<i32>,
    dry_run: bool,
) -> Result<Option<i32>, AppError> {
    let Some(name) = name else {
        return Ok(None);
    };
    let key = (name.to_lowercase(), department_id);

    if let Some(id) = cache.get(&key) {
        return Ok(*id);
    }

    if dry_run {
        cache.insert(key, None);
        return Ok(None);
    }

    let id = diesel::insert_into(positions::table)
        .values(&NewPosition {
            name: name.to_string(),
            department_id,
        })
        .returning(positions::id)
        .get_result::<i32>(&mut *conn)
        .await?;

    cache.insert(key, Some(id));
    Ok(Some(id))
}

// ============================================================================
// Разбор CSV
// ============================================================================

struct CsvTable {
    headers: Vec<String>,
    /// Номер строки в файле и значения полей.
    rows: Vec<(usize, Vec<String>)>,
}

impl CsvTable {
    fn parse(content: &str) -> Result<Self, AppError> {
        if content.len() > MAX_FILE_BYTES {
            return Err(AppError::ValidationError(
                "Файл больше 10 МБ — разделите его на части".to_string(),
            ));
        }

        let content = content.strip_prefix('\u{feff}').unwrap_or(content);
        if content.trim().is_empty() {
            return Err(AppError::ValidationError("Файл пуст".to_string()));
        }

        let delimiter = detect_delimiter(content);
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .flexible(true)
            .trim(csv::Trim::All)
            .from_reader(content.as_bytes());

        let headers = reader
            .headers()
            .map_err(|error| {
                AppError::ValidationError(format!("Не удалось прочитать заголовок CSV: {}", error))
            })?
            .iter()
            .map(normalize)
            .collect::<Vec<_>>();

        if headers.iter().all(String::is_empty) {
            return Err(AppError::ValidationError(
                "В первой строке нет названий колонок".to_string(),
            ));
        }

        let mut rows = Vec::new();
        for (index, record) in reader.records().enumerate() {
            let record = record.map_err(|error| {
                AppError::ValidationError(format!("Ошибка чтения строки {}: {}", index + 2, error))
            })?;

            let fields = record.iter().map(str::to_string).collect::<Vec<_>>();
            if fields.iter().all(|value| value.trim().is_empty()) {
                continue;
            }

            rows.push((index + 2, fields));
        }

        if rows.is_empty() {
            return Err(AppError::ValidationError(
                "В файле нет строк с данными".to_string(),
            ));
        }
        if rows.len() > MAX_ROWS {
            return Err(AppError::ValidationError(format!(
                "В файле {} строк — максимум {}. Разделите импорт на части",
                rows.len(),
                MAX_ROWS
            )));
        }

        Ok(Self { headers, rows })
    }

    fn column(&self, aliases: &[&str]) -> Option<usize> {
        self.headers.iter().position(|header| {
            !header.is_empty() && aliases.iter().any(|alias| normalize(alias) == *header)
        })
    }

    fn require_column(&self, aliases: &[&str], title: &str) -> Result<usize, AppError> {
        self.column(aliases).ok_or_else(|| {
            AppError::ValidationError(format!("В файле нет обязательной колонки «{}»", title))
        })
    }
}

fn detect_delimiter(content: &str) -> u8 {
    let first_line = content.lines().next().unwrap_or_default();
    let mut best = (b',', 0usize);

    for candidate in *b",;\t" {
        let count = first_line.bytes().filter(|byte| *byte == candidate).count();
        if count > best.1 {
            best = (candidate, count);
        }
    }

    best.0
}

/// Приводит заголовок к сравнимому виду: нижний регистр, без пробелов,
/// подчёркиваний, дефисов и точек. Так «Инв. номер» и `inv_number`
/// превращаются в `инвномер` и `invnumber` и не мешают друг другу.
fn normalize(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .replace('ё', "е")
        .chars()
        .filter(|ch| !matches!(ch, ' ' | '\t' | '_' | '-' | '.' | '(' | ')' | '"' | '\''))
        .collect()
}

fn cell(row: &[String], index: Option<usize>) -> Option<&str> {
    index
        .and_then(|index| row.get(index))
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
}

fn parse_date(value: &str, column: &str) -> Result<NaiveDate, String> {
    for format in ["%Y-%m-%d", "%d.%m.%Y", "%d/%m/%Y", "%Y/%m/%d"] {
        if let Ok(date) = NaiveDate::parse_from_str(value, format) {
            return Ok(date);
        }
    }

    Err(format!(
        "не удалось разобрать дату «{}» в колонке «{}» (нужно ГГГГ-ММ-ДД или ДД.ММ.ГГГГ)",
        value, column
    ))
}

// ============================================================================
// Сотрудники: колонки и строка
// ============================================================================

struct EmployeeColumns {
    first_name: usize,
    last_name: usize,
    middle_name: Option<usize>,
    email: Option<usize>,
    phone: Option<usize>,
    ad_username: Option<usize>,
    position: Option<usize>,
    department: Option<usize>,
    hire_date: Option<usize>,
    status: Option<usize>,
    notes: Option<usize>,
}

impl EmployeeColumns {
    fn new(table: &CsvTable) -> Result<Self, AppError> {
        Ok(Self {
            first_name: table.require_column(&["имя", "firstname", "имяработника"], "Имя")?,
            last_name: table.require_column(
                &["фамилия", "lastname", "surname", "фамилияработника"],
                "Фамилия",
            )?,
            middle_name: table.column(&["отчество", "middlename", "patronymic"]),
            email: table.column(&["email", "почта", "электроннаяпочта", "mail"]),
            phone: table.column(&["телефон", "phone", "мобильный", "телефонмобильный"]),
            ad_username: table.column(&[
                "adusername",
                "логин",
                "логинад",
                "ad",
                "учетнаязапись",
                "учетка",
            ]),
            position: table.column(&["должность", "position", "jobtitle", "профессия"]),
            department: table.column(&["подразделение", "отдел", "department", "division", "цех"]),
            hire_date: table.column(&["датаприема", "hiredate", "дата", "дататрудоустройства"]),
            status: table.column(&["статус", "status"]),
            notes: table.column(&["заметки", "примечание", "notes", "комментарий", "comment"]),
        })
    }
}

#[derive(Debug)]
struct ParsedEmployee {
    first_name: String,
    last_name: String,
    middle_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    ad_username: Option<String>,
    position: Option<String>,
    department: Option<String>,
    hire_date: Option<NaiveDate>,
    status: String,
    notes: Option<String>,
}

fn parse_employee_row(columns: &EmployeeColumns, row: &[String]) -> Result<ParsedEmployee, String> {
    let first_name = cell(row, Some(columns.first_name))
        .ok_or_else(|| "не указано имя".to_string())?
        .to_string();
    let last_name = cell(row, Some(columns.last_name))
        .ok_or_else(|| "не указана фамилия".to_string())?
        .to_string();

    let email = cell(row, columns.email).map(|value| value.to_lowercase());
    if let Some(email) = email.as_deref()
        && (!email.contains('@') || email.contains(char::is_whitespace))
    {
        return Err(format!("некорректный email «{}»", email));
    }

    let ad_username = cell(row, columns.ad_username).map(str::to_string);
    if email.is_none() && ad_username.is_none() {
        return Err(
            "нужен email или логин AD: без них повторный импорт создаст дубликат".to_string(),
        );
    }

    let status = match cell(row, columns.status) {
        Some(value) => normalize_employee_status(value)?,
        None => "active".to_string(),
    };

    let hire_date = match cell(row, columns.hire_date) {
        Some(value) => Some(parse_date(value, "Дата приёма")?),
        None => None,
    };

    Ok(ParsedEmployee {
        first_name,
        last_name,
        middle_name: cell(row, columns.middle_name).map(str::to_string),
        email,
        phone: cell(row, columns.phone).map(str::to_string),
        ad_username,
        position: cell(row, columns.position).map(str::to_string),
        department: cell(row, columns.department).map(str::to_string),
        hire_date,
        status,
        notes: cell(row, columns.notes).map(str::to_string),
    })
}

fn normalize_employee_status(raw: &str) -> Result<String, String> {
    let status = match normalize(raw).as_str() {
        "active" | "активен" | "активный" | "работает" => "active",
        "inactive" | "неактивен" | "неактивный" => "inactive",
        "vacation" | "отпуск" => "vacation",
        "sickleave" | "больничный" => "sick_leave",
        // В базе и интерфейсе уволенный сотрудник — отдельный статус, а не
        // «неактивный»: иначе карточка теряет причину и дату увольнения.
        "terminated" | "уволен" | "уволена" => "terminated",
        _ => {
            return Err(format!(
                "неизвестный статус «{}» (допустимы active, inactive, vacation, sick_leave, terminated)",
                raw
            ));
        }
    };

    Ok(status.to_string())
}

// ============================================================================
// Компьютеры: колонки и строка
// ============================================================================

struct ComputerColumns {
    inventory_number: Option<usize>,
    hostname: Option<usize>,
    computer_type: Option<usize>,
    manufacturer: Option<usize>,
    model: Option<usize>,
    serial_number: Option<usize>,
    os: Option<usize>,
    os_version: Option<usize>,
    employee_email: Option<usize>,
    location: Option<usize>,
    purchase_date: Option<usize>,
    warranty_end_date: Option<usize>,
    status: Option<usize>,
    notes: Option<usize>,
}

impl ComputerColumns {
    fn new(table: &CsvTable) -> Result<Self, AppError> {
        let columns = Self {
            inventory_number: table.column(&[
                "инвентарныйномер",
                "inventorynumber",
                "invnumber",
                "инв",
                "inv",
            ]),
            hostname: table.column(&[
                "имякомпьютера",
                "hostname",
                "host",
                "имя",
                "компьютер",
                "пк",
            ]),
            computer_type: table.column(&["тип", "типкомпьютера", "computertype", "type"]),
            manufacturer: table.column(&[
                "производитель",
                "manufacturer",
                "brand",
                "vendor",
                "марка",
            ]),
            model: table.column(&["модель", "model"]),
            serial_number: table.column(&[
                "серийныйномер",
                "serialnumber",
                "serial",
                "sn",
                "sernumber",
            ]),
            os: table.column(&[
                "ос",
                "операционнаясистема",
                "os",
                "operatingsystem",
                "операционка",
            ]),
            os_version: table.column(&["версияос", "osversion", "версия", "version"]),
            employee_email: table.column(&[
                "emailсотрудника",
                "сотрудник",
                "employeeemail",
                "employee",
                "пользователь",
                "фио",
                "ответственный",
            ]),
            location: table.column(&["расположение", "location", "кабинет", "офис", "помещение"]),
            purchase_date: table.column(&[
                "датапокупки",
                "purchasedate",
                "датаприобретения",
                "датапостановки",
            ]),
            warranty_end_date: table.column(&[
                "гарантиядо",
                "warrantyenddate",
                "конецгарантии",
                "гарантия",
            ]),
            status: table.column(&["статус", "status", "состояние"]),
            notes: table.column(&["заметки", "примечание", "notes", "комментарий"]),
        };

        if columns.inventory_number.is_none() && columns.hostname.is_none() {
            return Err(AppError::ValidationError(
                "Нужна колонка «Инвентарный номер» или «Имя компьютера»: по ним импорт находит существующие записи".to_string(),
            ));
        }

        Ok(columns)
    }
}

#[derive(Debug)]
struct ParsedComputer {
    inventory_number: Option<String>,
    hostname: Option<String>,
    computer_type: String,
    manufacturer: Option<String>,
    model: Option<String>,
    serial_number: Option<String>,
    os: Option<String>,
    os_version: Option<String>,
    employee_email: Option<String>,
    location: Option<String>,
    purchase_date: Option<NaiveDate>,
    warranty_end_date: Option<NaiveDate>,
    status: String,
    notes: Option<String>,
}

fn parse_computer_row(columns: &ComputerColumns, row: &[String]) -> Result<ParsedComputer, String> {
    let inventory_number = cell(row, columns.inventory_number).map(str::to_string);
    let hostname = cell(row, columns.hostname).map(str::to_string);

    if inventory_number.is_none() && hostname.is_none() {
        return Err("нужен инвентарный номер или имя компьютера".to_string());
    }

    let computer_type = match cell(row, columns.computer_type) {
        Some(value) => normalize_computer_type(value)?,
        None => "desktop".to_string(),
    };

    let status = match cell(row, columns.status) {
        Some(value) => normalize_computer_status(value)?,
        None => "active".to_string(),
    };

    let employee_email = cell(row, columns.employee_email).map(|value| value.to_lowercase());
    if let Some(email) = employee_email.as_deref()
        && (!email.contains('@') || email.contains(char::is_whitespace))
    {
        return Err(format!(
            "в колонке ответственного ожидается email, получено «{}»",
            email
        ));
    }

    let purchase_date = match cell(row, columns.purchase_date) {
        Some(value) => Some(parse_date(value, "Дата покупки")?),
        None => None,
    };
    let warranty_end_date = match cell(row, columns.warranty_end_date) {
        Some(value) => Some(parse_date(value, "Гарантия до")?),
        None => None,
    };

    Ok(ParsedComputer {
        inventory_number,
        hostname,
        computer_type,
        manufacturer: cell(row, columns.manufacturer).map(str::to_string),
        model: cell(row, columns.model).map(str::to_string),
        serial_number: cell(row, columns.serial_number).map(str::to_string),
        os: cell(row, columns.os).map(str::to_string),
        os_version: cell(row, columns.os_version).map(str::to_string),
        employee_email,
        location: cell(row, columns.location).map(str::to_string),
        purchase_date,
        warranty_end_date,
        status,
        notes: cell(row, columns.notes).map(str::to_string),
    })
}

fn normalize_computer_type(raw: &str) -> Result<String, String> {
    let value = normalize(raw);

    let kind = match value.as_str() {
        "desktop" | "пк" | "компьютер" | "стационарный" => "desktop",
        "laptop" | "ноутбук" | "нетбук" => "laptop",
        "server" | "сервер" => "server",
        "workstation" | "рабочаястанция" | "графическаястанция" => {
            "workstation"
        }
        "thinclient" | "тонкийклиент" => "thin_client",
        _ => {
            return Err(format!(
                "неизвестный тип «{}» (допустимы desktop, laptop, server, workstation, thin_client)",
                raw
            ));
        }
    };

    Ok(kind.to_string())
}

fn normalize_computer_status(raw: &str) -> Result<String, String> {
    let value = normalize(raw);

    let status = match value.as_str() {
        "active" | "активен" | "вэксплуатации" | "работает" => "active",
        "inactive" | "неактивен" | "выключен" => "inactive",
        "repair" | "ремонт" | "времонте" => "repair",
        "storage" | "склад" | "наскладе" => "storage",
        "decommissioned" | "списан" | "списано" | "decommission" => "decommissioned",
        _ => {
            return Err(format!(
                "неизвестный статус «{}» (допустимы active, inactive, repair, storage, decommissioned)",
                raw
            ));
        }
    };

    Ok(status.to_string())
}

// ============================================================================
// Ошибки БД
// ============================================================================

/// Превращает текст ошибки Postgres в понятную фразу для отчёта.
fn friendly_db_error(error: &AppError) -> String {
    let text = error.to_string();

    if text.contains("duplicate key") {
        if text.contains("employees_email_key") {
            return "email уже занят другой записью".to_string();
        }
        if text.contains("employees_ad_username_key") {
            return "логин AD уже занят другой записью".to_string();
        }
        if text.contains("computers_inventory_number_key") {
            return "инвентарный номер уже занят другой записью".to_string();
        }
        if text.contains("computers_hostname_key") {
            return "имя компьютера уже занято другой записью".to_string();
        }
        if text.contains("computers_serial_number_key") {
            return "серийный номер уже занят другой записью".to_string();
        }

        return "нарушено ограничение уникальности".to_string();
    }

    text
}

// ============================================================================
// Тесты
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_semicolon_and_comma() {
        assert_eq!(detect_delimiter("a;b;c\n1;2;3"), b';');
        assert_eq!(detect_delimiter("a,b,c\n1,2,3"), b',');
        assert_eq!(detect_delimiter("a\tb\tc\n1\t2\t3"), b'\t');
        assert_eq!(detect_delimiter("одна колонка\nзначение"), b',');
    }

    #[test]
    fn normalizes_russian_headers() {
        assert_eq!(normalize(" Инв. номер "), "инвномер");
        assert_eq!(normalize("Дата приёма"), "датаприема");
        assert_eq!(normalize("ad_username"), "adusername");
    }

    #[test]
    fn parses_bom_and_semicolon_file() {
        let content = "\u{feff}Фамилия;Имя;Email;Отдел\nИванов;Пётр;PETR@Example.com;ИТ\n";
        let table = CsvTable::parse(content).expect("файл должен разобраться");
        let columns = EmployeeColumns::new(&table).expect("колонки должны найтись");

        assert_eq!(table.rows.len(), 1);
        let row =
            parse_employee_row(&columns, &table.rows[0].1).expect("строка должна разобраться");

        assert_eq!(row.first_name, "Пётр");
        assert_eq!(row.last_name, "Иванов");
        assert_eq!(row.email.as_deref(), Some("petr@example.com"));
        assert_eq!(row.department.as_deref(), Some("ИТ"));
    }

    #[test]
    fn rejects_employee_without_identity() {
        let content = "Фамилия,Имя,Телефон\nИванов,Пётр,+79990000000\n";
        let table = CsvTable::parse(content).expect("файл должен разобраться");
        let columns = EmployeeColumns::new(&table).expect("колонки должны найтись");

        let error = parse_employee_row(&columns, &table.rows[0].1)
            .expect_err("без email и логина строка должна отклоняться");

        assert!(error.contains("email или логин AD"));
    }

    #[test]
    fn parses_dates_in_two_formats() {
        assert_eq!(
            parse_date("2026-11-14", "Дата").unwrap(),
            NaiveDate::from_ymd_opt(2026, 11, 14).unwrap()
        );
        assert_eq!(
            parse_date("14.11.2026", "Дата").unwrap(),
            NaiveDate::from_ymd_opt(2026, 11, 14).unwrap()
        );
        assert!(parse_date("14 ноября 2026", "Дата").is_err());
    }

    #[test]
    fn normalizes_types_and_statuses() {
        assert_eq!(normalize_computer_type("Ноутбук").unwrap(), "laptop");
        assert_eq!(
            normalize_computer_type("Тонкий клиент").unwrap(),
            "thin_client"
        );
        assert_eq!(normalize_computer_status("В ремонте").unwrap(), "repair");
        assert!(normalize_computer_type("планшет").is_err());

        assert_eq!(
            normalize_employee_status("Больничный").unwrap(),
            "sick_leave"
        );
        assert_eq!(normalize_employee_status("Уволен").unwrap(), "terminated");
        assert_eq!(
            normalize_employee_status("terminated").unwrap(),
            "terminated"
        );
        assert!(normalize_employee_status("декрет").is_err());
    }

    #[test]
    fn requires_key_column_for_computers() {
        let content = "Модель,Производитель\nLatitude, Dell\n";
        let table = CsvTable::parse(content).expect("файл должен разобраться");

        assert!(ComputerColumns::new(&table).is_err());
    }

    #[test]
    fn explains_duplicate_key_constraints() {
        let email = AppError::DatabaseError(
            "UniqueViolation: duplicate key value violates unique constraint \"employees_email_key\""
                .to_string(),
        );
        assert_eq!(friendly_db_error(&email), "email уже занят другой записью");

        let hostname = AppError::DatabaseError(
            "UniqueViolation: duplicate key value violates unique constraint \"computers_hostname_key\""
                .to_string(),
        );
        assert_eq!(
            friendly_db_error(&hostname),
            "имя компьютера уже занято другой записью"
        );

        let other = AppError::DatabaseError(
            "UniqueViolation: duplicate key value violates unique constraint \"some_other_key\""
                .to_string(),
        );
        assert_eq!(
            friendly_db_error(&other),
            "нарушено ограничение уникальности"
        );
    }

    #[test]
    fn parses_computer_row() {
        let content = "Инвентарный номер;Имя компьютера;Тип;Ответственный;Гарантия до\nPC-001;pc-001;Ноутбук;ivanov@example.com;31.12.2027\n";
        let table = CsvTable::parse(content).expect("файл должен разобраться");
        let columns = ComputerColumns::new(&table).expect("колонки должны найтись");

        let row =
            parse_computer_row(&columns, &table.rows[0].1).expect("строка должна разобраться");

        assert_eq!(row.inventory_number.as_deref(), Some("PC-001"));
        assert_eq!(row.hostname.as_deref(), Some("pc-001"));
        assert_eq!(row.computer_type, "laptop");
        assert_eq!(row.employee_email.as_deref(), Some("ivanov@example.com"));
        assert_eq!(
            row.warranty_end_date,
            Some(NaiveDate::from_ymd_opt(2027, 12, 31).unwrap())
        );
    }
}
