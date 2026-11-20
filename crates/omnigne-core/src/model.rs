use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("invalid snapshot: {0}")]
    InvalidSnapshot(String),
    #[error("invalid route: {0}")]
    InvalidRoute(String),
    #[error("quote arithmetic overflow")]
    ArithmeticOverflow,
    #[error("quote rounds to zero")]
    ZeroOutput,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssetId {
    pub network: String,
    pub address: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionDomain {
    StellarClassic,
    StellarSoroban,
    Evm,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Asset {
    pub id: AssetId,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConstantProductPool {
    pub id: String,
    pub venue: String,
    pub domain: ExecutionDomain,
    pub asset_a: AssetId,
    pub asset_b: AssetId,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_bps: u16,
}

impl ConstantProductPool {
    pub fn other(&self, input: &AssetId) -> Option<&AssetId> {
        if *input == self.asset_a {
            Some(&self.asset_b)
        } else if *input == self.asset_b {
            Some(&self.asset_a)
        } else {
            None
        }
    }

    /// Exact-input x*y=k quote, rounding the received amount down.
    /// This model is not valid for stable-swap or concentrated-liquidity pools.
    pub fn quote(&self, input: &AssetId, amount: u64) -> Result<u64, EngineError> {
        if amount == 0 || self.fee_bps >= 10_000 || self.reserve_a == 0 || self.reserve_b == 0 {
            return Err(EngineError::InvalidRoute(
                "invalid amount, fee or reserves".into(),
            ));
        }
        let (reserve_in, reserve_out) = if *input == self.asset_a {
            (self.reserve_a, self.reserve_b)
        } else if *input == self.asset_b {
            (self.reserve_b, self.reserve_a)
        } else {
            return Err(EngineError::InvalidRoute(
                "input asset is absent from pool".into(),
            ));
        };
        let adjusted = u128::from(amount) * u128::from(10_000 - self.fee_bps);
        let numerator = adjusted
            .checked_mul(u128::from(reserve_out))
            .ok_or(EngineError::ArithmeticOverflow)?;
        let denominator = u128::from(reserve_in)
            .checked_mul(10_000)
            .and_then(|v| v.checked_add(adjusted))
            .ok_or(EngineError::ArithmeticOverflow)?;
        let output =
            u64::try_from(numerator / denominator).map_err(|_| EngineError::ArithmeticOverflow)?;
        if output == 0 {
            return Err(EngineError::ZeroOutput);
        }
        Ok(output)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketSnapshot {
    pub id: String,
    pub network: String,
    pub domain: ExecutionDomain,
    pub ledger: u64,
    pub observed_at_unix: u64,
    pub assets: Vec<Asset>,
    pub pools: Vec<ConstantProductPool>,
}

impl MarketSnapshot {
    pub fn validate(&self) -> Result<(), EngineError> {
        let invalid = |message: &str| EngineError::InvalidSnapshot(message.into());
        if self.id.is_empty() || self.assets.is_empty() || self.pools.is_empty() {
            return Err(invalid("snapshot id, assets and pools are required"));
        }
        let compatible = match self.domain {
            ExecutionDomain::StellarClassic | ExecutionDomain::StellarSoroban => {
                matches!(self.network.as_str(), "stellar:testnet" | "stellar:mainnet")
            }
            ExecutionDomain::Evm => self
                .network
                .strip_prefix("eip155:")
                .and_then(|v| v.parse::<u64>().ok())
                .is_some_and(|id| id > 0),
        };
        if !compatible {
            return Err(invalid("network and execution domain disagree"));
        }
        let mut assets = HashSet::new();
        for asset in &self.assets {
            if asset.id.network != self.network
                || asset.id.address.is_empty()
                || asset.symbol.is_empty()
                || asset.decimals > 18
                || !assets.insert(asset.id.clone())
            {
                return Err(invalid("invalid, duplicate or foreign-network asset"));
            }
        }
        let mut pool_ids = HashSet::new();
        for pool in &self.pools {
            if pool.id.is_empty()
                || pool.venue.is_empty()
                || !pool_ids.insert(&pool.id)
                || pool.domain != self.domain
                || pool.asset_a == pool.asset_b
                || !assets.contains(&pool.asset_a)
                || !assets.contains(&pool.asset_b)
                || pool.reserve_a == 0
                || pool.reserve_b == 0
                || pool.fee_bps >= 10_000
            {
                return Err(invalid(
                    "invalid pool, duplicate id or mixed execution domain",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hop {
    pub pool_id: String,
    pub input: AssetId,
    pub output: AssetId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CycleQuote {
    pub snapshot_id: String,
    pub ledger: u64,
    pub domain: ExecutionDomain,
    pub settlement_asset: AssetId,
    pub hops: Vec<Hop>,
    pub amount_in: u64,
    pub amount_out: u64,
}
