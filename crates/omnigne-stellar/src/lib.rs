//! Stellar snapshot boundary. Live indexing, RPC simulation and submission are planned.
use omnigne_core::{EngineError, ExecutionDomain, MarketSnapshot};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StellarError {
    #[error("invalid snapshot JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Engine(#[from] EngineError),
    #[error("expected a Stellar execution domain")]
    WrongDomain,
}

pub fn parse_snapshot(json: &str) -> Result<MarketSnapshot, StellarError> {
    let snapshot: MarketSnapshot = serde_json::from_str(json)?;
    snapshot.validate()?;
    if !matches!(
        snapshot.domain,
        ExecutionDomain::StellarClassic | ExecutionDomain::StellarSoroban
    ) {
        return Err(StellarError::WrongDomain);
    }
    Ok(snapshot)
}
