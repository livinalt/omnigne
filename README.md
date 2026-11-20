# Omnigne

A headless Rust foundation for DeFi arbitrage, liquidations and intent solving, starting on Stellar and designed for later EVM expansion.

**Current stage: working paper scanner and integration scaffolds.** The executable scans synthetic or file-based Stellar snapshots, quotes constant-product cycles, applies estimated cost and freshness checks, and writes candidate decisions to a JSONL journal. It does not sign or submit transactions.

## Quick start

Install [Rust through rustup](https://rustup.rs/), then run these commands from the extracted `omnigne` directory. The pinned toolchain installs automatically on the first Cargo command.

```bash
cargo run --locked -p omnigne-runner -- demo
```

The demo uses synthetic liquidity and its recorded observation time. It produces six candidates: three pass the configured estimates and three are rejected. Results append to `data/opportunities.jsonl`. These are paper quotes, not executed trades or evidence of market profitability.

```bash
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
```

No keys, RPC subscription, database server, frontend or paid service is needed for the demo.

For prerequisites, Windows/WSL notes, every crate and contract command, configuration, Docker and troubleshooting, follow the [complete local development guide](docs/LOCAL_DEVELOPMENT.md).

## Repository layout

| Path | Responsibility |
| --- | --- |
| `crates/omnigne-core/` | Asset identity, execution domains, integer constant-product quotes and bounded cycle discovery |
| `crates/omnigne-risk/` | Profit-after-estimated-cost checks, input limits and state freshness |
| `crates/omnigne-storage/` | Append-only journal for paper candidate records |
| `crates/omnigne-stellar/` | Stellar snapshot parsing; future live indexing, simulation and submission |
| `crates/omnigne-evm/` | EVM network types; future Alloy and protocol adapters |
| `crates/omnigne-intents/` | Unsigned order terms and shape validation; future authentication and protocol adapters |
| `crates/omnigne-runner/` | Headless CLI and paper scanning pipeline |
| `contracts/stellar/executor/` | Soroban executor scaffold |
| `contracts/stellar/settler/` | Soroban intent-settler scaffold |
| `contracts/evm/` | Foundry project with an executor that explicitly rejects execution |
| `config/` | Non-secret configuration |
| `fixtures/` | Synthetic market snapshots |
| `docs/` | Architecture, specification, roadmap and GitHub upload instructions |
| `infra/` | Operational integration notes |
| `.github/` | CI, issue forms and pull request template |

## Scan a snapshot

```bash
cargo run --locked -p omnigne-runner -- scan \
  --snapshot fixtures/stellar-soroban.json \
  --config config/development.toml \
  --as-of 1795183200 \
  --journal data/replay.jsonl
```

`scan` uses the current system time unless `--as-of` explicitly selects a replay clock. Old snapshots are rejected by the configured freshness limit. Only the `demo` command automatically uses the fixture's time. The scanner makes no network requests.

All amounts and fee estimates use the settlement asset's integer base units. Fixtures declare token decimals. The `demo:*` identifiers and pool labels are synthetic and must not be substituted for verified deployment identities. The constant-product model is not suitable for stable-swap or concentrated-liquidity pools.

`accepted: true` means an estimated paper quote passed the implemented checks. It does not mean an on-chain simulation passed, funds were reserved, or a transaction executed. Routes cannot reuse a pool. Search enumerates simple cycles of 2–4 hops; Bellman-Ford, optimal sizing and split routing are follow-up work.

## Contract workspaces

Soroban contracts are isolated from the off-chain Cargo workspace:

```bash
rustup target add wasm32v1-none
stellar contract build --manifest-path contracts/stellar/Cargo.toml --locked
```

Install [Stellar CLI](https://developers.stellar.org/docs/tools/cli/install-cli) for this command; version 30.0.1 was used for validation. This SDK requires the CLI build pipeline rather than a direct Wasm Cargo build. The contracts currently expose version queries only. The EVM contract deliberately reverts from `execute`:

```bash
forge test --root contracts/evm
```

Install [Foundry](https://getfoundry.sh/) for EVM development. Neither chain has deployment scripts or transaction-signing capability in this starter. See [the implementation roadmap](docs/ROADMAP.md) for the next steps.

## Upload to GitHub

Extract the archive and upload the **contents of `omnigne/`**, so `Cargo.toml` sits at the repository root. Do not upload the ZIP as your source repository. The [GitHub guide](docs/GITHUB_UPLOAD.md) includes commands for a new empty repository and instructions for GitHub Desktop.

## Project references

- [Complete local development guide](docs/LOCAL_DEVELOPMENT.md)
- [Architecture and crate boundaries](docs/ARCHITECTURE.md)
- [Technical specification](docs/SPECIFICATION.md)
- [Implementation status and next tasks](docs/ROADMAP.md)
- [Validation results](docs/VALIDATION.md)
- [Fixture format](fixtures/README.md)
- [Contribution guide](CONTRIBUTING.md)
- [Security reporting](SECURITY.md)

No project license has been selected. Choose one before inviting reuse under an open-source license.

