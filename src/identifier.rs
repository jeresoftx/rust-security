//! Validación allowlist para identificadores sintéticos de ejemplos defensivos.

/// Indica si un identificador cumple el contrato ASCII de uno a 32 caracteres.
pub fn is_safe_identifier(value: &str) -> bool {
    (1..=32).contains(&value.len())
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
}
