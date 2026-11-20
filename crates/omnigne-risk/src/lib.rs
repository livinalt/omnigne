//! Per-candidate paper checks. Portfolio reservations and circuit breakers follow later.
use omnigne_core::{CycleQuote, MarketSnapshot};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RiskLimits {
    pub min_profit: u64,
    pub estimated_network_fee: u64,
    pub uncertainty_buffer: u64,
    pub max_input: u64,
    pub max_snapshot_age_seconds: u64,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Rejection {
    InvalidLimits,
    SnapshotMismatch,
    FutureSnapshot,
    StaleSnapshot,
    InputLimit,
    InsufficientNetProfit,
}

#[derive(Debug, Serialize)]
pub struct RiskDecision {
    pub accepted: bool,
    pub estimated_net_profit: i128,
    pub reasons: Vec<Rejection>,
}

pub fn assess(
    quote: &CycleQuote,
    snapshot: &MarketSnapshot,
    limits: &RiskLimits,
    now: u64,
) -> RiskDecision {
    let mut reasons = Vec::new();
    if limits.min_profit == 0 || limits.max_input == 0 || limits.max_snapshot_age_seconds == 0 {
        reasons.push(Rejection::InvalidLimits);
    }
    if quote.snapshot_id != snapshot.id
        || quote.ledger != snapshot.ledger
        || quote.domain != snapshot.domain
        || quote.settlement_asset.network != snapshot.network
    {
        reasons.push(Rejection::SnapshotMismatch);
    }
    match now.checked_sub(snapshot.observed_at_unix) {
        None => reasons.push(Rejection::FutureSnapshot),
        Some(age) if age > limits.max_snapshot_age_seconds => {
            reasons.push(Rejection::StaleSnapshot)
        }
        Some(_) => {}
    }
    if quote.amount_in == 0 || quote.amount_in > limits.max_input {
        reasons.push(Rejection::InputLimit);
    }
    // Fee and uncertainty estimates are in the settlement asset's base units.
    let net = i128::from(quote.amount_out)
        - i128::from(quote.amount_in)
        - i128::from(limits.estimated_network_fee)
        - i128::from(limits.uncertainty_buffer);
    if net < i128::from(limits.min_profit) {
        reasons.push(Rejection::InsufficientNetProfit);
    }
    RiskDecision {
        accepted: reasons.is_empty(),
        estimated_net_profit: net,
        reasons,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use omnigne_core::{AssetId, ExecutionDomain};

    fn inputs() -> (CycleQuote, MarketSnapshot, RiskLimits) {
        let quote = CycleQuote {
            snapshot_id: "test".into(),
            ledger: 1,
            domain: ExecutionDomain::StellarSoroban,
            settlement_asset: AssetId {
                network: "stellar:testnet".into(),
                address: "demo:USDC".into(),
            },
            hops: vec![],
            amount_in: 100,
            amount_out: 120,
        };
        let snapshot = MarketSnapshot {
            id: "test".into(),
            network: "stellar:testnet".into(),
            domain: ExecutionDomain::StellarSoroban,
            ledger: 1,
            observed_at_unix: 100,
            assets: vec![],
            pools: vec![],
        };
        let limits = RiskLimits {
            min_profit: 10,
            estimated_network_fee: 5,
            uncertainty_buffer: 5,
            max_input: 100,
            max_snapshot_age_seconds: 15,
        };
        (quote, snapshot, limits)
    }

    #[test]
    fn accepts_exact_threshold_after_costs() {
        let (quote, snapshot, limits) = inputs();
        let result = assess(&quote, &snapshot, &limits, 101);
        assert!(result.accepted);
        assert_eq!(result.estimated_net_profit, 10);
    }

    #[test]
    fn rejects_gross_profit_that_cannot_cover_costs() {
        let (mut quote, snapshot, limits) = inputs();
        quote.amount_out = 105;
        let result = assess(&quote, &snapshot, &limits, 101);
        assert_eq!(result.estimated_net_profit, -5);
        assert!(result.reasons.contains(&Rejection::InsufficientNetProfit));
    }

    #[test]
    fn rejects_stale_future_mismatched_and_oversized_candidates() {
        let (mut quote, snapshot, limits) = inputs();
        assert!(
            assess(&quote, &snapshot, &limits, 116)
                .reasons
                .contains(&Rejection::StaleSnapshot)
        );
        assert!(
            assess(&quote, &snapshot, &limits, 99)
                .reasons
                .contains(&Rejection::FutureSnapshot)
        );
        quote.ledger = 2;
        quote.amount_in = 101;
        let reasons = assess(&quote, &snapshot, &limits, 101).reasons;
        assert!(reasons.contains(&Rejection::SnapshotMismatch));
        assert!(reasons.contains(&Rejection::InputLimit));
    }
}
