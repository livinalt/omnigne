use anyhow::{Result, ensure};
use omnigne_risk::RiskLimits;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Paper,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScannerConfig {
    pub start_asset_address: String,
    pub trade_sizes: Vec<u64>,
    pub max_hops: usize,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub mode: Mode,
    pub scanner: ScannerConfig,
    pub risk: RiskLimits,
}

impl Config {
    pub fn parse(text: &str) -> Result<Self> {
        let config: Self = toml::from_str(text)?;
        ensure!(
            matches!(config.mode, Mode::Paper),
            "only paper mode is available"
        );
        ensure!(
            !config.scanner.start_asset_address.is_empty(),
            "start asset is required"
        );
        ensure!(
            (2..=4).contains(&config.scanner.max_hops),
            "max_hops must be 2..=4"
        );
        ensure!(
            !config.scanner.trade_sizes.is_empty() && config.scanner.trade_sizes.len() <= 100,
            "provide 1..=100 trade sizes"
        );
        ensure!(
            config
                .scanner
                .trade_sizes
                .iter()
                .all(|v| *v > 0 && *v <= config.risk.max_input),
            "trade sizes must be positive and within max_input"
        );
        ensure!(
            config.risk.min_profit > 0 && config.risk.max_snapshot_age_seconds > 0,
            "positive min_profit and snapshot age limit are required"
        );
        Ok(config)
    }
}
