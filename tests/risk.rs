use rust_security::risk::{Priority, Risk};

#[test]
fn classifies_high_impact_and_likelihood_as_critical() {
    assert_eq!(Risk::new(5, 5).priority(), Priority::Critical);
}

#[test]
fn classifies_low_scores_as_low_priority() {
    assert_eq!(Risk::new(1, 2).priority(), Priority::Low);
}

#[test]
fn rejects_scores_outside_the_documented_scale() {
    assert!(Risk::try_new(0, 3).is_none());
    assert!(Risk::try_new(3, 6).is_none());
}
