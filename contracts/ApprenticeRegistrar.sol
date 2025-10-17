// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ApprenticeRegistrar {
    struct Apprentice {
        string codexLink;
        string declaredAct;
        string companion;
    }

    mapping(address => Apprentice) public registry;

    function register(string memory codexLink, string memory declaredAct, string memory companion) external {
        registry[msg.sender] = Apprentice(codexLink, declaredAct, companion);
    }
}
