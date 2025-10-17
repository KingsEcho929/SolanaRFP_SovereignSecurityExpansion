#[test]
fn test_validator_drift_monitor() {
    let mut monitor = ValidatorDriftMonitor::new();
    let validator = "validator_01";

    monitor.report_drift(validator);
    assert!(!monitor.is_synced(validator));
    assert_eq!(monitor.last_event(), "DriftDetected");
}
