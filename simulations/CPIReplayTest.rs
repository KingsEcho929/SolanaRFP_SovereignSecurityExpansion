use modules::CPITrace;

#[test]
fn test_cpi_replay_detection() {
    let mut trace = CPITrace::new();
    let tx_hash = "0xabc123";

    assert!(trace.trace(tx_hash)); // first trace allowed
    assert!(!trace.trace(tx_hash)); // replay blocked
    assert_eq!(trace.last_event(), "CPIReplayDetected(0xabc123)");
}
