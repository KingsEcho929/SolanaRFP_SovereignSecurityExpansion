use modules::EpochGuard;

#[test]
fn test_epoch_drift_detection() {
    let mut guard = EpochGuard::new();
    let module = "vault_01";
    
    guard.lock_epoch(module, 42);
    let result = guard.lock_epoch(module, 41); // simulate drift

    assert!(!result); // drift rejected
    assert_eq!(guard.last_event(), "EpochLocked(42)");
}
