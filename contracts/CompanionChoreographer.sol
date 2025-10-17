// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract CompanionChoreographer {
    mapping(address => string) public assignedCompanion;

    function assignCompanion(address apprentice, string memory companionName) external {
        assignedCompanion[apprentice] = companionName;
    }
}
