#[test]
fn test_governance_replay_shield() {
    let mut shield = GovernanceReplayShield::new();
    let proposal_id = "proposal_xyz";

    assert!(shield.execute(proposal_id));
    assert!(!shield.execute(proposal_id)); // replay attempt
    assert_eq!(shield.last_event(), "ReplayBlocked");
}
