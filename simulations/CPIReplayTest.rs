#[test]
fn test_cpi_replay_detection() {
    let mut trace = CPITrace::new();
    let tx_hash = "0xabc123";

    assert!(trace.trace(tx_hash));
    assert!(!trace.trace(tx_hash)); // replay attempt
    assert_eq!(trace.last_event(), "CPIReplayDetected");
}
