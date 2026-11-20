# EVM contracts

See the [local development guide](../../docs/LOCAL_DEVELOPMENT.md) for Foundry installation, build output and expected test results.

Foundry project for the future EVM executor. The current contract exposes a version query and explicitly reverts from `execute`. It has no flash-loan, DEX, intent or withdrawal functionality.

```bash
cd contracts/evm
forge test
```

The test verifies that execution stays disabled. Next work: define the authorized route schema, lender callback boundaries, venue allowlists, repayment accounting, and protected balance invariants. Add pinned mainnet-fork integration tests when real network adapters are introduced.

