use std::collections::HashSet;

use crate::{AssetId, CycleQuote, EngineError, Hop, MarketSnapshot};

/// Enumerates simple cycles of 2..=4 hops anchored at a settlement asset.
/// Distinct pools can connect the same pair. A pool cannot be used twice.
pub fn find_cycles(
    snapshot: &MarketSnapshot,
    start: &AssetId,
    max_hops: usize,
) -> Result<Vec<Vec<Hop>>, EngineError> {
    snapshot.validate()?;
    if !(2..=4).contains(&max_hops) || !snapshot.assets.iter().any(|a| a.id == *start) {
        return Err(EngineError::InvalidRoute(
            "unknown start asset or invalid hop limit".into(),
        ));
    }
    let mut routes = Vec::new();
    walk(
        snapshot,
        start,
        start,
        max_hops,
        &mut Vec::new(),
        &mut vec![start.clone()],
        &mut routes,
    );
    Ok(routes)
}

fn walk(
    snapshot: &MarketSnapshot,
    current: &AssetId,
    start: &AssetId,
    max_hops: usize,
    route: &mut Vec<Hop>,
    visited: &mut Vec<AssetId>,
    routes: &mut Vec<Vec<Hop>>,
) {
    if route.len() >= max_hops {
        return;
    }
    for pool in &snapshot.pools {
        if route.iter().any(|hop| hop.pool_id == pool.id) {
            continue;
        }
        let Some(next) = pool.other(current) else {
            continue;
        };
        route.push(Hop {
            pool_id: pool.id.clone(),
            input: current.clone(),
            output: next.clone(),
        });
        if next == start {
            if route.len() >= 2 {
                routes.push(route.clone());
            }
        } else if !visited.contains(next) {
            visited.push(next.clone());
            walk(snapshot, next, start, max_hops, route, visited, routes);
            visited.pop();
        }
        route.pop();
    }
}

pub fn quote_cycle(
    snapshot: &MarketSnapshot,
    start: &AssetId,
    route: &[Hop],
    amount_in: u64,
) -> Result<CycleQuote, EngineError> {
    snapshot.validate()?;
    if !(2..=4).contains(&route.len()) || amount_in == 0 {
        return Err(EngineError::InvalidRoute(
            "expected a positive input and 2..=4 hops".into(),
        ));
    }
    let mut amount_out = amount_in;
    let mut current = start.clone();
    let mut used = HashSet::new();
    for hop in route {
        let pool = snapshot
            .pools
            .iter()
            .find(|p| p.id == hop.pool_id)
            .ok_or_else(|| EngineError::InvalidRoute("unknown pool".into()))?;
        if !used.insert(&hop.pool_id)
            || hop.input != current
            || pool.other(&current) != Some(&hop.output)
        {
            return Err(EngineError::InvalidRoute(
                "disconnected or repeated-pool route".into(),
            ));
        }
        amount_out = pool.quote(&current, amount_out)?;
        current = hop.output.clone();
    }
    if current != *start {
        return Err(EngineError::InvalidRoute(
            "route does not return to settlement asset".into(),
        ));
    }
    Ok(CycleQuote {
        snapshot_id: snapshot.id.clone(),
        ledger: snapshot.ledger,
        domain: snapshot.domain,
        settlement_asset: start.clone(),
        hops: route.to_vec(),
        amount_in,
        amount_out,
    })
}
