// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract CPITrace {
    mapping(bytes32 => bool) public replayGuard;

    event CPIReplayDetected(bytes32 indexed txHash);

    function trace(bytes32 txHash) external {
        require(!replayGuard[txHash], "Replay detected");
        replayGuard[txHash] = true;
    }
}
