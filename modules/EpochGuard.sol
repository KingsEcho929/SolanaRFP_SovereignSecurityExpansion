// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract EpochGuard {
    mapping(address => uint256) public lastEpoch;

    event EpochLocked(address indexed module, uint256 epoch);

    function lockEpoch(address module, uint256 epoch) external {
        require(epoch > lastEpoch[module], "Epoch already locked");
        lastEpoch[module] = epoch;
        emit EpochLocked(module, epoch);
    }
}
