// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

/// @notice Inert scaffold. No swaps, approvals, loans or settlement are implemented.
contract OmnigneEvmExecutor {
    error ExecutionNotImplemented();

    function scaffoldVersion() external pure returns (uint256) {
        return 1;
    }

    function execute(bytes calldata) external pure {
        revert ExecutionNotImplemented();
    }
}

