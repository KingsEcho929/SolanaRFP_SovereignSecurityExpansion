// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract InflowDistributor {
    address public treasury;

    constructor(address _treasury) {
        treasury = _treasury;
    }

    function distribute(address recipient, uint256 amount) external {
        payable(recipient).transfer(amount);
    }

    receive() external payable {}
}
