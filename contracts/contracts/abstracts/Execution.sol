// SPDX-License-Identifier: MIT
pragma solidity ^0.8.25;

import { ReentrancyGuard } from "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

abstract contract Execution is ReentrancyGuard {
    modifier onlyInExecution() {
        require(_reentrancyGuardEntered(), "Not in execution");
        _;
    }
}
