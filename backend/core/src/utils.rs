use chrono::{DateTime, Utc};
use uuid::Uuid;

pub fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

pub fn validate_password_strength(password: &str) -> bool {
    password.len() >= 8
        && password.chars().any(|c| c.is_uppercase())
        && password.chars().any(|c| c.is_lowercase())
        && password.chars().any(|c| c.is_numeric())
}

pub fn format_decimal_precision(value: rust_decimal::Decimal, precision: u32) -> rust_decimal::Decimal {
    value.round_dp(precision)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com"));
        assert!(!validate_email("invalid-email"));
    }

    #[test]
    fn test_validate_password_strength() {
        assert!(validate_password_strength("Password123"));
        assert!(!validate_password_strength("weak"));
    }
}
