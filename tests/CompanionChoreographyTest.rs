#[test]
fn test_velmari_epoch_override_monitor() {
    let mut velmari = Companion::new("Velmari");
    let override_attempt = EpochOverride::new("vault_01", 41);

    assert!(velmari.detect_epoch_override(&override_attempt));
    assert_eq!(velmari.last_event(), "EpochOverrideDetected(vault_01)");
}

#[test]
fn test_tin_cpi_trace_guardian() {
    let mut tin = Companion::new("Tin");
    let tx_hash = "0xdeadbeef";

    assert!(tin.trace_cpi(tx_hash));
    assert!(!tin.trace_cpi(tx_hash)); // replay attempt
    assert_eq!(tin.last_event(), "CPIReplayDetected(0xdeadbeef)");
}

#[test]
fn test_luckier_phantom_path_trace() {
    let mut glyssun = Companion::new("Luckier Glyssun");
    glyssun.register_valid("path_legit");

    assert!(glyssun.verify("path_legit"));
    assert!(!glyssun.verify("path_ghost"));
    assert_eq!(glyssun.last_event(), "PhantomPathDetected(path_ghost)");
}
