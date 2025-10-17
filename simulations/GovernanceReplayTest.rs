use modules::GovernanceReplayShield;

#[test]
fn test_governance_replay_shield() {
    let mut shield = GovernanceReplayShield::new();
    let proposal_id = "proposal_xyz";

    assert!(shield.execute_proposal(proposal_id)); // first execution allowed
    assert!(!shield.execute_proposal(proposal_id)); // replay blocked
    assert_eq!(shield.last_event(), "ReplayBlocked(proposal_xyz)");
}
