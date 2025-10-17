use serde_json::{Map, Value};

/// Helper для отслеживания изменений полей
pub struct ChangeTracker {
    changes: Map<String, Value>,
}

impl ChangeTracker {
    pub fn new() -> Self {
        Self {
            changes: Map::new(),
        }
    }

    /// Добавить изменение строкового поля
    pub fn track_string(&mut self, field: &str, old: &str, new: &str) {
        if old != new {
            self.changes.insert(
                field.to_string(),
                serde_json::json!({"old": old, "new": new}),
            );
        }
    }

    /// Добавить изменение опционального строкового поля
    pub fn track_option_string(&mut self, field: &str, old: &Option<String>, new: &Option<String>) {
        if old != new {
            self.changes.insert(
                field.to_string(),
                serde_json::json!({"old": old, "new": new}),
            );
        }
    }

    /// Добавить изменение опционального числового поля
    pub fn track_option_i32(&mut self, field: &str, old: &Option<i32>, new: &Option<i32>) {
        if old != new {
            self.changes.insert(
                field.to_string(),
                serde_json::json!({"old": old, "new": new}),
            );
        }
    }

    /// Добавить изменение опционального bool поля
    pub fn track_option_bool(&mut self, field: &str, old: &Option<bool>, new: &Option<bool>) {
        if old != new {
            self.changes.insert(
                field.to_string(),
                serde_json::json!({"old": old, "new": new}),
            );
        }
    }

    /// Проверить есть ли изменения
    pub fn has_changes(&self) -> bool {
        !self.changes.is_empty()
    }

    /// Получить изменения как JSON Value
    pub fn into_json(self) -> Value {
        Value::Object(self.changes)
    }
}

impl Default for ChangeTracker {
    fn default() -> Self {
        Self::new()
    }
}
