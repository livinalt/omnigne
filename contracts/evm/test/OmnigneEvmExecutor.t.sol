// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.28;

import {OmnigneEvmExecutor} from "../src/OmnigneEvmExecutor.sol";

contract OmnigneEvmExecutorTest {
    function testExecutionRemainsDisabled() public {
        OmnigneEvmExecutor executor = new OmnigneEvmExecutor();
        (bool success, bytes memory result) =
            address(executor).call(abi.encodeCall(OmnigneEvmExecutor.execute, (bytes(""))));
        require(!success, "scaffold must reject execution");
        require(
            keccak256(result) == keccak256(abi.encodeWithSelector(OmnigneEvmExecutor.ExecutionNotImplemented.selector)),
            "unexpected failure"
        );
    }
}

