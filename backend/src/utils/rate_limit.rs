use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Простой ограничитель частоты по фиксированному окну.
///
/// Используется для защиты входа в систему от перебора. Состояние хранится
/// в памяти процесса: для одного экземпляра приложения этого достаточно,
/// при горизонтальном масштабировании нужен общий store (Redis).
pub struct FixedWindowLimiter {
    limit: u32,
    window: Duration,
    entries: Mutex<HashMap<String, Window>>,
}

struct Window {
    started_at: Instant,
    count: u32,
}

impl FixedWindowLimiter {
    pub fn new(limit: u32, window: Duration) -> Self {
        Self {
            limit,
            window,
            entries: Mutex::new(HashMap::new()),
        }
    }

    /// Учитывает попытку для ключа. При превышении лимита возвращает
    /// количество секунд до сброса окна.
    pub fn check(&self, key: &str) -> Result<(), u64> {
        let now = Instant::now();
        let mut entries = self
            .entries
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        // Страховка от неограниченного роста памяти при большом числе адресов.
        if entries.len() > 5_000 {
            entries.retain(|_, window| now.duration_since(window.started_at) < self.window);
        }

        let window = entries.entry(key.to_string()).or_insert(Window {
            started_at: now,
            count: 0,
        });

        if now.duration_since(window.started_at) >= self.window {
            window.started_at = now;
            window.count = 0;
        }

        if window.count >= self.limit {
            let elapsed = now.duration_since(window.started_at);
            let retry_after = self.window.saturating_sub(elapsed).as_secs().max(1);
            return Err(retry_after);
        }

        window.count += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_requests_up_to_limit() {
        let limiter = FixedWindowLimiter::new(3, Duration::from_secs(60));

        assert!(limiter.check("127.0.0.1").is_ok());
        assert!(limiter.check("127.0.0.1").is_ok());
        assert!(limiter.check("127.0.0.1").is_ok());
    }

    #[test]
    fn blocks_requests_over_limit() {
        let limiter = FixedWindowLimiter::new(2, Duration::from_secs(60));

        limiter.check("client").expect("first request");
        limiter.check("client").expect("second request");

        let retry_after = limiter
            .check("client")
            .expect_err("third request must fail");
        assert!(retry_after > 0);
    }

    #[test]
    fn counts_clients_separately() {
        let limiter = FixedWindowLimiter::new(1, Duration::from_secs(60));

        limiter.check("first").expect("first client");
        limiter.check("second").expect("second client");
    }

    #[test]
    fn resets_after_window() {
        let limiter = FixedWindowLimiter::new(1, Duration::from_millis(50));

        limiter.check("client").expect("first request");
        assert!(limiter.check("client").is_err());

        std::thread::sleep(Duration::from_millis(80));

        assert!(limiter.check("client").is_ok());
    }
}
