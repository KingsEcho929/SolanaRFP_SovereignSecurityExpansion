// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract ValidatorDriftMonitor {
    mapping(address => bool) public validatorSync;

    event DriftDetected(address indexed validator);

    function reportDrift(address validator) external {
        validatorSync[validator] = false;
        emit DriftDetected(validator);
    }
}
