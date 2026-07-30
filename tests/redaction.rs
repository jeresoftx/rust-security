use rust_security::redaction::redact_token;

#[test]
fn redacts_a_synthetic_token_value() {
    assert_eq!(
        redact_token("event=login token=demo123 status=ok"),
        "event=login token=[REDACTED] status=ok"
    );
}

#[test]
fn leaves_messages_without_a_token_unchanged() {
    assert_eq!(
        redact_token("event=health status=ok"),
        "event=health status=ok"
    );
}
