mod config;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use omnigne_core::{AssetId, CycleQuote, find_cycles, quote_cycle};
use omnigne_risk::{RiskDecision, assess};
use omnigne_storage::CandidateJournal;
use serde::Serialize;
use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Parser)]
#[command(
    name = "omnigne",
    version,
    about = "Omnigne paper opportunity scanner; no transaction signing"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Replay the embedded synthetic fixture at its recorded observation time.
    Demo {
        #[arg(long)]
        config: Option<PathBuf>,
        #[arg(long, default_value = "data/opportunities.jsonl")]
        journal: PathBuf,
    },
    /// Scan one Stellar JSON snapshot in paper mode. No network requests are made.
    Scan {
        #[arg(long)]
        snapshot: PathBuf,
        #[arg(long)]
        config: Option<PathBuf>,
        #[arg(long, default_value = "data/opportunities.jsonl")]
        journal: PathBuf,
        /// Explicit replay clock; omit to evaluate age against the system clock.
        #[arg(long)]
        as_of: Option<u64>,
    },
}

#[derive(Serialize)]
struct CandidateRecord {
    schema_version: u32,
    mode: &'static str,
    source: &'static str,
    evaluated_at_unix: u64,
    quote: CycleQuote,
    decision: RiskDecision,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .json()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();
    let cli = Cli::parse();
    let (snapshot_json, config_path, journal_path, replay_clock, source) = match cli.command {
        Command::Demo { config, journal } => (
            include_str!("../../../fixtures/stellar-soroban.json").to_string(),
            config,
            journal,
            None,
            "synthetic_fixture",
        ),
        Command::Scan {
            snapshot,
            config,
            journal,
            as_of,
        } => (
            tokio::fs::read_to_string(snapshot)
                .await
                .context("read snapshot")?,
            config,
            journal,
            as_of,
            "snapshot_file",
        ),
    };
    let config_text = match config_path {
        Some(path) => tokio::fs::read_to_string(path)
            .await
            .context("read config")?,
        None => include_str!("../../../config/development.toml").to_string(),
    };
    let config = config::Config::parse(&config_text).context("validate config")?;
    let snapshot =
        omnigne_stellar::parse_snapshot(&snapshot_json).context("validate Stellar snapshot")?;
    let now = if source == "synthetic_fixture" {
        snapshot.observed_at_unix
    } else {
        replay_clock.unwrap_or(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
    };
    let start = AssetId {
        network: snapshot.network.clone(),
        address: config.scanner.start_asset_address.clone(),
    };
    let routes = find_cycles(&snapshot, &start, config.scanner.max_hops)?;
    let mut journal = CandidateJournal::open(&journal_path).context("open candidate journal")?;
    let mut candidates = 0;
    let mut accepted = 0;
    for route in &routes {
        for amount in &config.scanner.trade_sizes {
            let quote = quote_cycle(&snapshot, &start, route, *amount)?;
            let decision = assess(&quote, &snapshot, &config.risk, now);
            candidates += 1;
            accepted += usize::from(decision.accepted);
            journal.append(&CandidateRecord {
                schema_version: 1,
                mode: "paper",
                source,
                evaluated_at_unix: now,
                quote,
                decision,
            })?;
        }
    }
    tracing::info!(
        candidates,
        accepted,
        source,
        "paper scan complete; accepted means a quote passed estimates"
    );
    println!(
        "{}",
        serde_json::json!({ "mode": "paper", "source": source,
        "candidates": candidates, "accepted": accepted, "journal": journal_path })
    );
    Ok(())
}
