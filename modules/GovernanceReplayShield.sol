// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract GovernanceReplayShield {
    mapping(bytes32 => bool) public proposalExecuted;

    event ReplayBlocked(bytes32 indexed proposalId);

    function executeProposal(bytes32 proposalId) external {
        require(!proposalExecuted[proposalId], "Replay blocked");
        proposalExecuted[proposalId] = true;
    }
}
