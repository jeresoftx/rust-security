//! Redacción local de tokens sintéticos en ejemplos de telemetría.

/// Sustituye el valor continuo después de `token=` por un marcador seguro.
pub fn redact_token(message: &str) -> String {
    let Some(start) = message.find("token=") else {
        return message.to_owned();
    };
    let value_start = start + "token=".len();
    let value_end = message[value_start..]
        .find(char::is_whitespace)
        .map_or(message.len(), |offset| value_start + offset);

    format!(
        "{}token=[REDACTED]{}",
        &message[..start],
        &message[value_end..]
    )
}
