# Market fixtures

`stellar-soroban.json` is a deliberately synthetic three-pool constant-product market. Its token ids and venue names are not real Stellar deployments. The imbalance produces one profitable quoted direction and one losing direction for the configured trade sizes.

A snapshot includes:

- A unique snapshot id, network, execution domain, ledger and observation timestamp.
- An asset registry with network-qualified identities and decimals.
- Unique pool ids, venue labels, asset pairs, integer reserves and fees in basis points.

All pools must share the snapshot's execution domain. All assets must belong to its network. Duplicate pool ids, missing assets, zero reserves, invalid fees and cross-domain combinations are rejected.

Use `demo` for deterministic synthetic playback. `scan --snapshot PATH` uses the system clock for freshness checks unless an explicit `--as-of` replay time is supplied. Add captured real market fixtures only after verifying their source, state consistency and venue math.

