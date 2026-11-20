# Contributing

Use a short-lived branch for each change. Keep chain-specific transaction and RPC behavior in its adapter crate, quote math in the core, and policy checks in the risk layer.

Start with the [local development guide](docs/LOCAL_DEVELOPMENT.md) for tool installation, package-specific commands, contract builds and troubleshooting.

Before opening a pull request:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

Run the separate contract checks when contract sources change. Add meaningful regression cases for numerical correctness, authorization, state boundaries and recovery behavior. Use fixtures with explicit network and deployment identities.

Live integrations must document what is implemented, how simulation is performed, how balances and fees are reconciled, and what happens after timeouts or restarts. Keep incomplete execution paths disabled. Do not replace a required check with a successful mock response.

Choose the project's license before accepting contributions intended for licensed open-source distribution.

