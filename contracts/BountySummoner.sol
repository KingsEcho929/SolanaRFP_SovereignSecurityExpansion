// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract BountySummoner {
    event BountyDeclared(address indexed summoner, string codexLink, uint256 timestamp);

    function declareBounty(string memory codexLink) external {
        emit BountyDeclared(msg.sender, codexLink, block.timestamp);
    }
}
