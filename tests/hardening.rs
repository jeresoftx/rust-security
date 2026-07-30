use rust_security::hardening::{Configuration, Finding, evaluate};

#[test]
fn secure_synthetic_configuration_has_no_findings() {
    let config = Configuration::new(true, false, true);
    assert!(evaluate(config).is_empty());
}

#[test]
fn insecure_synthetic_configuration_names_missing_controls() {
    let findings = evaluate(Configuration::new(false, true, false));
    assert_eq!(
        findings,
        vec![
            Finding::TlsRequired,
            Finding::DebugDisabled,
            Finding::AdminRestricted
        ]
    );
}
