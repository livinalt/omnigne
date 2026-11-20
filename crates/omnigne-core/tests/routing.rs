use omnigne_core::{EngineError, ExecutionDomain, MarketSnapshot, find_cycles, quote_cycle};

fn snapshot() -> MarketSnapshot {
    serde_json::from_str(include_str!("../../../fixtures/stellar-soroban.json")).unwrap()
}

#[test]
fn triangle_has_two_directions_and_one_profitable_quote() {
    let market = snapshot();
    let start = &market.assets[0].id;
    let routes = find_cycles(&market, start, 3).unwrap();
    assert_eq!(routes.len(), 2);
    let quotes: Vec<_> = routes
        .iter()
        .map(|r| quote_cycle(&market, start, r, 1_000_000).unwrap())
        .collect();
    assert_eq!(
        quotes.iter().filter(|q| q.amount_out > q.amount_in).count(),
        1
    );
}

#[test]
fn quote_rounds_down_and_models_price_impact() {
    let mut pool = snapshot().pools[0].clone();
    pool.reserve_a = 1_000;
    pool.reserve_b = 1_000;
    pool.fee_bps = 0;
    assert_eq!(pool.quote(&pool.asset_a, 100).unwrap(), 90);
    assert_eq!(pool.quote(&pool.asset_a, 1_000).unwrap(), 500);
    pool.fee_bps = 30;
    assert!(pool.quote(&pool.asset_a, 1_000).unwrap() < 500);
}

#[test]
fn mixed_classic_and_soroban_pools_are_rejected() {
    let mut market = snapshot();
    market.pools[0].domain = ExecutionDomain::StellarClassic;
    assert!(market.validate().is_err());
}

#[test]
fn foreign_network_assets_and_duplicate_pool_ids_are_rejected() {
    let mut market = snapshot();
    market.assets[0].id.network = "stellar:mainnet".into();
    assert!(market.validate().is_err());
    let mut market = snapshot();
    market.pools[1].id = market.pools[0].id.clone();
    assert!(market.validate().is_err());
}

#[test]
fn repeated_pool_and_disconnected_routes_are_rejected() {
    let market = snapshot();
    let start = &market.assets[0].id;
    let mut route = find_cycles(&market, start, 3).unwrap().remove(0);
    route.push(route[0].clone());
    assert!(quote_cycle(&market, start, &route, 1_000_000).is_err());
    route.remove(0);
    assert!(quote_cycle(&market, start, &route, 1_000_000).is_err());
}

#[test]
fn zero_input_and_overflow_fail_closed() {
    let mut pool = snapshot().pools[0].clone();
    assert!(pool.quote(&pool.asset_a, 0).is_err());
    pool.reserve_b = u64::MAX;
    assert!(matches!(
        pool.quote(&pool.asset_a, u64::MAX),
        Err(EngineError::ArithmeticOverflow)
    ));
}

#[test]
fn search_respects_hop_limits_and_never_reuses_a_pool() {
    let market = snapshot();
    let start = &market.assets[0].id;
    assert!(find_cycles(&market, start, 2).unwrap().is_empty());
    assert!(find_cycles(&market, start, 0).is_err());
    for route in find_cycles(&market, start, 4).unwrap() {
        let unique: std::collections::HashSet<_> = route.iter().map(|h| &h.pool_id).collect();
        assert_eq!(unique.len(), route.len());
    }
}
