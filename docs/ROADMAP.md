# Implementation roadmap

## Included in this starter

- [x] Seven-crate Rust workspace with a pinned toolchain and lockfile.
- [x] Constant-product quotes with integer rounding and overflow handling.
- [x] Bounded cycle discovery with execution-domain isolation.
- [x] Paper profitability, input-cap and freshness checks.
- [x] Candidate journal, synthetic fixture and headless demo.
- [x] Soroban and Solidity contract scaffolds.
- [x] CI, configuration, contribution guide and GitHub instructions.

## Milestone 1 Live data without trading

- [ ] Select two actual Stellar venues with overlapping liquidity in one domain.
- [ ] Verify asset issuers, contract identities and deployed versions.
- [ ] Implement indexers with consistent ledger watermarks and durable cursors.
- [ ] Add recorded-state fixtures for the real venue math.
- [ ] Measure opportunities, exact output, fee estimates and state age.

Acceptance: restarts and missed updates recover a consistent snapshot; quotes match protocol reference calculations; no signing is enabled.

## Milestone 2 Stellar executor

- [ ] Define an allowlisted route schema and authorization policy.
- [ ] Implement swap adapters and exact balance accounting.
- [ ] Add ledger/time limits, input caps and profit checks after all obligations.
- [ ] Add RPC simulation, authorization assembly and resource budgeting.
- [ ] Implement inventory reservations, sequence coordination and pending-state recovery.
- [ ] Verify unauthorized, stale, unprofitable and partial-failure cases.

Acceptance: controlled tests establish balance invariants; every submitted transaction can be reconciled after a crash; readiness review precedes a limited live pilot.

## Milestone 3 Blend and Stellar intents

- [ ] Index version-specific liquidation auctions and borrower state.
- [ ] Quote debt repayment and executable collateral unwind.
- [ ] Evaluate Blend v2 financing with explicit repayment accounting.
- [ ] Confirm an intent source and order format.
- [ ] Add authorization, replay protection, cancellation and fill tracking.

Acceptance: independently verify liquidation economics and signed user terms, including failure and recovery cases.

## Milestone 4 EVM networks

- [ ] Add Alloy and one network with measured opportunity depth.
- [ ] Implement nonce handling, fee valuation and reorganization recovery.
- [ ] Integrate one financing provider and a small DEX set.
- [ ] Add pinned-state fork tests for callbacks and repayment.
- [ ] Add a second network and one external intent protocol after stabilization.

Acceptance: each deployed network has its own capability, address and protocol-version matrix.

## Milestone 5 Cross chain fulfillment

- [ ] Specify source proofs, destination delivery, finality and settlement transport.
- [ ] Implement deadlines, refunds, outstanding claims and inventory rebalancing.
- [ ] Test duplicate messages, delayed settlement and interrupted delivery.

Acceptance: every intermediate state has a tested recovery path, and fees cover the measured exposure.

