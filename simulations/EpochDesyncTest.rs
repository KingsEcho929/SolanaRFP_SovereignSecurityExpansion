#[test]
fn test_epoch_drift_detection() {
    let mut vault = Vault::new();
    vault.set_epoch(42);
    vault.override_epoch(41); // simulate drift

    assert!(vault.detect_epoch_drift());
    assert_eq!(vault.emit_drift_event(), "EpochDriftDetected");
}
