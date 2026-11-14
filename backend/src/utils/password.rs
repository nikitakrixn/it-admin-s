use argon2::{
    Argon2, PasswordHasher,
    password_hash::{self, PasswordVerifier},
};

use crate::utils::error::AppError;

/// Хеширует пароль методом Argon2id.
///
/// Соль генерируется автоматически внутри `hash_password`, результат —
/// строка в стандартном PHC-формате (`$argon2id$v=19$...`).
pub fn hash_password(password: &str) -> Result<String, AppError> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|e| AppError::InternalError(format!("Password hashing failed: {e}")))
}

/// Минимальная длина пароля для новых учётных записей и смены пароля.
pub const MIN_PASSWORD_LENGTH: usize = 12;

/// Проверяет пароль на соответствие политике безопасности.
///
/// Требования: не короче 12 символов, есть хотя бы одна буква и одна цифра.
/// Политика применяется только при установке пароля, вход не ограничивается,
/// чтобы старые учётные записи можно было перевести на новые правила постепенно.
pub fn validate_strength(password: &str) -> Result<(), AppError> {
    if password.chars().count() < MIN_PASSWORD_LENGTH {
        return Err(AppError::ValidationError(format!(
            "Пароль должен содержать минимум {MIN_PASSWORD_LENGTH} символов"
        )));
    }

    if !password.chars().any(|c| c.is_alphabetic()) {
        return Err(AppError::ValidationError(
            "Пароль должен содержать хотя бы одну букву".to_string(),
        ));
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(AppError::ValidationError(
            "Пароль должен содержать хотя бы одну цифру".to_string(),
        ));
    }

    Ok(())
}

/// Проверяет пароль против сохранённого хеша.
///
/// Возвращает `Ok(false)`, если пароль не подошёл, и ошибку,
/// если сохранённое значение вообще не является корректным хешем.
pub fn verify_password(password: &str, stored_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = password_hash::phc::PasswordHash::new(stored_hash)
        .map_err(|e| AppError::InternalError(format!("Invalid password hash: {e}")))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify_roundtrip() {
        let password = "correct horse battery staple";
        let hash = hash_password(password).expect("hashing should succeed");

        assert!(verify_password(password, &hash).expect("verification should run"));
        assert!(!verify_password("wrong password", &hash).expect("verification should run"));
    }

    #[test]
    fn hash_contains_phc_format() {
        let hash = hash_password("s3cure-p@ss").expect("hashing should succeed");

        assert!(
            hash.starts_with("$argon2id$"),
            "unexpected hash format: {hash}"
        );
        assert!(hash.contains("v=19"), "unexpected Argon2 version: {hash}");
    }

    #[test]
    fn malformed_hash_is_rejected() {
        let err = verify_password("any", "not-a-hash").expect_err("malformed hash must fail");
        assert!(matches!(err, AppError::InternalError(_)));
    }

    #[test]
    fn different_passwords_produce_different_hashes() {
        // Одинаковые пароли — разные соли — разные хеши.
        let first = hash_password("same-password").expect("hashing should succeed");
        let second = hash_password("same-password").expect("hashing should succeed");

        assert_ne!(first, second);
    }

    #[test]
    fn accepts_password_matching_policy() {
        assert!(validate_strength("Str0ngPassword!").is_ok());
    }

    #[test]
    fn rejects_short_password() {
        let err = validate_strength("Short1").expect_err("short password must fail");
        assert!(matches!(err, AppError::ValidationError(_)));
    }

    #[test]
    fn rejects_password_without_digit() {
        let err = validate_strength("NoDigitsHereAtAll").expect_err("password without digit");
        assert!(matches!(err, AppError::ValidationError(_)));
    }

    #[test]
    fn rejects_password_without_letter() {
        let err = validate_strength("123456789012").expect_err("password without letter");
        assert!(matches!(err, AppError::ValidationError(_)));
    }
}
