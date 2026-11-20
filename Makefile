.PHONY: check test demo stellar-build evm-test
check:
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets --locked -- -D warnings

test:
	cargo test --workspace --locked

demo:
	cargo run --locked -p omnigne-runner -- demo

stellar-build:
	stellar contract build --manifest-path contracts/stellar/Cargo.toml --locked

evm-test:
	forge test --root contracts/evm

