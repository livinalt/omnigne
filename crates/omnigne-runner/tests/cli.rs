use std::process::Command;

#[test]
fn demo_produces_accepted_and_rejected_records_without_keys() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("results.jsonl");
    let output = Command::new(env!("CARGO_BIN_EXE_omnigne"))
        .args(["demo", "--journal"])
        .arg(&path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(summary["mode"], "paper");
    assert_eq!(summary["candidates"], 6);
    assert_eq!(summary["accepted"], 3);
    let records = std::fs::read_to_string(path).unwrap();
    assert_eq!(records.lines().count(), 6);
    for line in records.lines() {
        let record: serde_json::Value = serde_json::from_str(line).unwrap();
        assert_eq!(record["mode"], "paper");
        assert_eq!(record["source"], "synthetic_fixture");
    }
}

#[test]
fn unknown_live_flag_is_rejected() {
    let output = Command::new(env!("CARGO_BIN_EXE_omnigne"))
        .args(["demo", "--live"])
        .output()
        .unwrap();
    assert!(!output.status.success());
}

#[test]
fn live_mode_in_config_is_rejected_before_journal_creation() {
    let dir = tempfile::tempdir().unwrap();
    let config = dir.path().join("bad.toml");
    let journal = dir.path().join("absent.jsonl");
    std::fs::write(
        &config,
        include_str!("../../../config/development.toml").replace("\"paper\"", "\"live\""),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_omnigne"))
        .args(["demo", "--config"])
        .arg(config)
        .arg("--journal")
        .arg(&journal)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!journal.exists());
}
