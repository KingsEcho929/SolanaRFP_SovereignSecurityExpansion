// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract CodexVerifier {
    mapping(string => bool) public verifiedCodex;

    function verifyCodex(string memory codexLink) external {
        verifiedCodex[codexLink] = true;
    }
}
