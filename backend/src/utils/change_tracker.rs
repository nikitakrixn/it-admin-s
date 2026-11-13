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
    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_only_changed_string_fields() {
        let mut tracker = ChangeTracker::new();

        tracker.track_string("name", "old", "new");
        tracker.track_string("status", "active", "active");

        let changes = tracker.into_json();

        assert_eq!(changes["name"]["old"], "old");
        assert_eq!(changes["name"]["new"], "new");
        assert!(changes.get("status").is_none());
    }

    #[test]
    fn tracks_optional_values() {
        let mut tracker = ChangeTracker::new();

        tracker.track_option_string("assignee", &None, &Some("a.smirnov".to_string()));
        tracker.track_option_i32("computer_id", &Some(3), &Some(3));

        let changes = tracker.into_json();

        assert_eq!(changes["assignee"]["new"], "a.smirnov");
        assert!(changes.get("computer_id").is_none());
    }

    #[test]
    fn reports_empty_changes() {
        let mut tracker = ChangeTracker::new();
        assert!(!tracker.has_changes());

        tracker.track_option_bool("is_online", &Some(false), &Some(true));

        assert!(tracker.has_changes());
        assert_eq!(tracker.into_json()["is_online"]["new"], true);
    }
}
