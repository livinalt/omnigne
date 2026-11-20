# Run Omnigne locally

Run the commands below from the repository root: the directory containing `Cargo.toml`, `README.md`, `crates/` and `contracts/`.

## What runs today

| Part | How to use it locally | Current scope |
| --- | --- | --- |
| Runner | `cargo run --locked -p omnigne-runner -- demo` | One paper scan, followed by process exit |
| Core, risk and storage | Workspace tests or individual package commands below | Libraries used by the runner |
| Stellar adapter | Used by the runner to validate a snapshot file | No live RPC/indexer connection |
| EVM adapter | Compile as a library | Network identity types; no live client |
| Intent module | Compile as a library | Unsigned terms and shape validation; no API server |
| Stellar executor and settler | Build the separate contract workspace | Version-only Wasm scaffolds |
| EVM executor | Build and test with Foundry | Version query and deliberately disabled execution |
| Docker | Build the root Dockerfile and run the image | The same one-shot paper runner |

There are no separate frontend, database, solver API, signing, alerting or monitoring services to start in this version. No keys, wallet funding, RPC credentials, local Stellar node or Anvil process are required for the implemented checks. See [the roadmap](ROADMAP.md) for those later integrations.

## 1 Install the tools

The project was validated on Linux. On Windows, use WSL with Ubuntu for one Bash environment across Rust, Stellar CLI and Foundry. The Rust runner can also be developed in native PowerShell with Rust's Windows build prerequisites installed. Native Windows and macOS were not tested for this release.

| Tool | Required for | Version used for validation |
| --- | --- | --- |
| Rust via rustup | All Rust crates and Stellar contract compilation | Rust 1.100.0, pinned in `rust-toolchain.toml` |
| Stellar CLI | Building Soroban Wasm artifacts | 30.0.1 |
| Foundry | EVM compilation and tests | 1.10.0; Solidity 0.8.28 is pinned in `foundry.toml` |
| Docker | Optional container run | Docker build was not tested in the validation environment |
| Git | Cloning and publishing the repository | Any compatible installation |

Install Rust using the [official Rust installer](https://rust-lang.org/tools/install). For native Windows, install the Visual Studio C++ build tools when prompted. For WSL setup, follow [Microsoft's instructions](https://learn.microsoft.com/en-us/windows/wsl/install).

On Ubuntu or WSL Ubuntu, install the native compiler tools before building Rust packages:

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev curl git
```

After installing rustup, open a new terminal and run:

```bash
rustup toolchain install 1.100.0 --profile minimal --component rustfmt,clippy
rustup target add wasm32v1-none --toolchain 1.100.0
rustc --version
cargo --version
```

The initial build needs internet access to download the pinned toolchain and dependencies. Keep both `Cargo.lock` files committed.

### Stellar CLI

Use an installation method from the [official Stellar CLI guide](https://developers.stellar.org/docs/tools/cli/install-cli). To match the validated version, use the appropriate binary from the [30.0.1 release](https://github.com/stellar/stellar-cli/releases/tag/v30.0.1), extract it, and add its directory to your shell's PATH. A source-build alternative, after installing its native build prerequisites, is:

```bash
cargo install --locked stellar-cli --version 30.0.1
stellar --version
```

### Foundry

Follow the [official Foundry installation guide](https://www.getfoundry.sh/introduction/installation). Foundryup requires Bash on Windows through WSL or Git Bash; it does not run in PowerShell or Command Prompt. After installing foundryup and reopening the terminal:

```bash
foundryup --install 1.10.0
forge --version
```

Node.js, npm and a browser are not required for this headless starter.

## 2 Start the paper scanner

From the extracted `omnigne` directory:

```bash
cargo run --locked -p omnigne-runner -- demo --journal data/local-demo.jsonl
```

Expected summary fields are `mode: "paper"`, `source: "synthetic_fixture"`, `candidates: 6` and `accepted: 3`. The process exits after scanning. Each run appends six records to the journal; it does not replace previous records.

The fixture contains invented asset and pool identifiers. An accepted candidate is an estimated quote, not a submitted transaction or realized profit.

Inspect the latest records in Bash:

```bash
tail -n 6 data/local-demo.jsonl
```

Or in PowerShell:

```powershell
Get-Content data/local-demo.jsonl -Tail 6
```

See the available commands and flags:

```bash
cargo run --locked -p omnigne-runner -- --help
cargo run --locked -p omnigne-runner -- scan --help
```

## 3 Supply configuration and replay a snapshot

`config/development.toml` contains the paper-mode settings. Pass `--config` to load a file at runtime; otherwise the binary uses its compiled-in default configuration. Local configuration files named `config/local*.toml` are ignored by Git.

```bash
cargo run --locked -p omnigne-runner -- demo --config config/development.toml --journal data/configured-demo.jsonl
```

All trade sizes, fee estimates, profit thresholds and buffers use the settlement asset's integer base units. With seven decimals, `10000000` represents one whole token.

Replay the supplied snapshot with its recorded timestamp:

```bash
cargo run --locked -p omnigne-runner -- scan --snapshot fixtures/stellar-soroban.json --config config/development.toml --as-of 1795183200 --journal data/replay.jsonl
```

`demo` always uses its embedded fixture's observation time. `scan` uses the system clock unless `--as-of` selects an explicit replay time. Omitting `--as-of` for this historical fixture should reject all six candidates as stale. Replaying a timestamp is for paper analysis and does not establish live state freshness.

Optional logging in Bash:

```bash
RUST_LOG=debug cargo run --locked -p omnigne-runner -- demo
```

In PowerShell:

```powershell
$env:RUST_LOG = "debug"
cargo run --locked -p omnigne-runner -- demo
```

Structured logs go to stderr; the JSON summary goes to stdout. `.env.example` is a reference only: the runner does not automatically load a `.env` file. Set environment variables in your shell. There is no live mode or private-key setting to configure.

## 4 Check every Rust package

Build and check the whole off-chain workspace:

```bash
cargo build --workspace --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Run a package-specific command when working on one part:

| Package | Command | What it establishes |
| --- | --- | --- |
| Core | `cargo test --locked -p omnigne-core` | Existing quote and route regression tests |
| Risk | `cargo test --locked -p omnigne-risk` | Existing cost, size, age and state checks |
| Storage | `cargo test --locked -p omnigne-storage` | Existing append/reopen behavior test |
| Runner | `cargo test --locked -p omnigne-runner` | Existing CLI integration tests |
| Stellar | `cargo check --locked -p omnigne-stellar` | Snapshot-parser crate compiles; no standalone service |
| EVM | `cargo check --locked -p omnigne-evm` | Integration boundary compiles; no live EVM client |
| Intents | `cargo check --locked -p omnigne-intents` | Terms module compiles; no signature or settlement service |

To build an optimized runner:

```bash
cargo build --locked --release -p omnigne-runner
```

Run `./target/release/omnigne demo` on Linux/macOS or `./target/release/omnigne.exe demo` on native Windows. Library crates do not have their own `cargo run` entrypoints.

## 5 Build both Stellar contracts

The Soroban contracts have a separate Cargo workspace and lockfile. Root workspace tests do not build or test these contracts.

```bash
cargo fmt --manifest-path contracts/stellar/Cargo.toml --all -- --check
stellar contract build --manifest-path contracts/stellar/Cargo.toml --locked
```

Expected artifacts:

- `contracts/stellar/target/wasm32v1-none/release/omnigne_stellar_executor.wasm`
- `contracts/stellar/target/wasm32v1-none/release/omnigne_stellar_settler.wasm`

Each exports only `scaffold_version`. To build just one package:

```bash
stellar contract build --manifest-path contracts/stellar/Cargo.toml --package omnigne-stellar-executor --locked
stellar contract build --manifest-path contracts/stellar/Cargo.toml --package omnigne-stellar-settler --locked
```

Use `stellar contract build`, not a direct Wasm Cargo build: this SDK requires the CLI's spec-processing pipeline. The current workspace has no Soroban host tests, local-network deployment script or implemented swap/settlement behavior. Successful compilation checks the scaffolds, not an arbitrage execution flow.

## 6 Build and test the EVM contract

From the repository root:

```bash
forge build --root contracts/evm
forge fmt --root contracts/evm --check
forge test --root contracts/evm -vv
```

Foundry downloads Solidity 0.8.28 if it is not already cached. Expect one passing test, `testExecutionRemainsDisabled`. Build artifacts go under `contracts/evm/out/`.

No RPC URL, Anvil instance, funded wallet or fork is needed for this test. The test verifies that the scaffold rejects execution; flash loans, venue integrations and intent settlement are not implemented yet.

## 7 Optional Docker run

With Docker installed and its daemon running, from the repository root:

```bash
docker build -t omnigne:dev .
docker run --rm omnigne:dev
```

This runs the paper demo once and exits. No ports are exposed. The image builds only the Rust runner, not the contract projects. With `--rm`, the journal inside the container is removed with the container; the JSON summary remains visible in your terminal.

To retain the container's journal across runs, use a named volume:

```bash
docker volume create omnigne-data
docker run --rm --mount source=omnigne-data,target=/var/lib/omnigne omnigne:dev
```

Docker commands are provided for local use but have not been executed in this project's validation environment. The equivalent native runner has been tested.

## 8 Stop and restart

The current commands are one-shot jobs. They finish automatically; interrupt a build or scan with Ctrl+C if needed. Run the same command again to repeat it. Journals append on each scan, and generated data/build directories are ignored by Git.

There is no daemon, Compose stack or background task to stop in this version.

## Troubleshooting

| Symptom | Action |
| --- | --- |
| `cargo`, `stellar` or `forge` is not found | Install that tool in the terminal environment you are using, ensure its directory is on PATH, and reopen the terminal. Windows and WSL have separate installations. |
| Native Windows linker error involving `link.exe` | Install Rust's Visual Studio C++ build prerequisites or use a configured WSL environment. |
| SDK asks for Stellar CLI build support | Use `stellar contract build` and check the CLI version; do not bypass the SDK check with manual environment variables. |
| All snapshot candidates are stale | Use the explicit fixture replay timestamp for paper replay, or supply a genuinely current snapshot. |
| No journal appears where expected | Output paths are relative to your current working directory. Run from the repository root and inspect the summary's `journal` field. |
| Old records appear in output | Journals append. Use a different `--journal` filename for a separate run. |
| Config changes seem ignored | Pass `--config path/to/file.toml` to load that file at runtime. |
| `--live` or `mode = "live"` fails | Live execution is intentionally absent from the current implementation. |
| `cargo run` fails for a library crate | Run `omnigne-runner`; use `cargo test` or `cargo check` for library packages. |
| Cargo lockfile errors | Confirm you are using the matching root or Stellar workspace and pinned toolchain. Do not delete lockfiles to hide dependency changes. |
| Docker cannot connect to its daemon | Start Docker Desktop or your Docker service; native Cargo commands do not require Docker. |

See [validation results](VALIDATION.md) for exactly which checks have been run and [the roadmap](ROADMAP.md) for the parts still to implement.
