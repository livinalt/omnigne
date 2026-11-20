# Stellar contracts

See the [local development guide](../../docs/LOCAL_DEVELOPMENT.md) for tool installation and building the contracts individually.

This separate Cargo workspace keeps Soroban's Wasm toolchain and dependencies out of the off-chain engine build. It contains two **scaffolds**, each exposing only `scaffold_version`.

| Directory | Intended responsibility | Current behavior |
| --- | --- | --- |
| `executor/` | Authorized atomic swaps, financing repayment, and profit invariants | Version query only |
| `settler/` | User authorization, nonce and cancellation tracking, output constraints | Version query only |

From the repository root:

```bash
rustup target add wasm32v1-none
stellar contract build --manifest-path contracts/stellar/Cargo.toml --locked
```

Use Stellar CLI 30.0.1, the version validated with this scaffold. Install it using the official Stellar CLI instructions. The SDK requires the CLI's contract build pipeline, including spec processing; a direct Wasm Cargo build is insufficient.

No deployment scripts or funded addresses are included. Implement and verify contract behavior before deployment. SDEX operations belong in the Classic transaction builder, not in this contract invocation tree.

