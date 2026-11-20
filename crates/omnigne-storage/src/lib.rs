//! Single-writer, append-only JSONL journal for paper candidate records.
//! This is not yet a pending-transaction or settlement recovery database.
use serde::Serialize;
use std::{
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub struct CandidateJournal {
    file: File,
}

impl CandidateJournal {
    pub fn open(path: &Path) -> io::Result<Self> {
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }
        Ok(Self {
            file: OpenOptions::new().create(true).append(true).open(path)?,
        })
    }

    pub fn append(&mut self, record: &impl Serialize) -> io::Result<()> {
        let mut bytes = serde_json::to_vec(record).map_err(io::Error::other)?;
        bytes.push(b'\n');
        self.file.write_all(&bytes)?;
        self.file.sync_data()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reopening_preserves_existing_complete_records() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("journal.jsonl");
        CandidateJournal::open(&path)
            .unwrap()
            .append(&vec![1, 2])
            .unwrap();
        CandidateJournal::open(&path)
            .unwrap()
            .append(&vec![3, 4])
            .unwrap();
        let text = std::fs::read_to_string(path).unwrap();
        let values: Vec<Vec<u32>> = text
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        assert_eq!(values, vec![vec![1, 2], vec![3, 4]]);
    }
}
