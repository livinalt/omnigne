# Starter validation

Validated on 20 November 2026 with Rust 1.100.0, Stellar CLI 30.0.1, Soroban SDK 29.0.1, Foundry 1.10.0 and Solidity 0.8.28.

| Check | Result |
| --- | --- |
| `cargo test --workspace` | 14 tests passed |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Passed |
| `cargo fmt --all -- --check` | Passed |
| `cargo build --locked --release -p omnigne-runner` and release demo | Passed; six candidates and three accepted paper estimates |
| Stellar contract workspace format check | Passed |
| `stellar contract build --manifest-path contracts/stellar/Cargo.toml --locked` | Both version-only contract scaffolds built to Wasm |
| Individual Stellar `--package` build commands in the local guide | Executor and settler builds passed |
| `forge test --root contracts/evm` | Disabled-execution regression test passed |
| Synthetic demo | Six candidate records; three accepted estimates and three rejected |
| Snapshot replay using system clock | All six stale candidates rejected |
| GitHub workflow and issue-form YAML | Syntax parsed successfully |
| Docker build | Not run; Docker is not installed in the validation environment |

Tests cover cycle detection, quote rounding and price impact, overflow rejection, pool reuse, mixed execution domains, foreign-network assets, fee-adjusted decisions, stale and future observations, input limits, journal append behavior, and the CLI's disabled live mode.

These checks establish starter behavior only. No live RPC indexing, on-chain swap simulation, real trade, liquidation, user order fulfillment, mainnet deployment or GitHub Actions run was performed. The synthetic fixture has no relationship to current market profitability.
