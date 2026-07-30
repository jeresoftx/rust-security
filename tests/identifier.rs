use rust_security::identifier::is_safe_identifier;

#[test]
fn accepts_identifiers_with_the_documented_allowlist() {
    assert!(is_safe_identifier("team_42-admin"));
}

#[test]
fn rejects_empty_long_or_ambiguous_identifiers() {
    assert!(!is_safe_identifier(""));
    assert!(!is_safe_identifier("name with spaces"));
    assert!(!is_safe_identifier(&"a".repeat(33)));
}
