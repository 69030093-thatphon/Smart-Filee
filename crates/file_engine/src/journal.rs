use chrono::Local;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UndoBatchStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "undone")]
    Undone,
    #[serde(rename = "stale")]
    Stale,
}

impl UndoBatchStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Undone => "undone",
            Self::Stale => "stale",
        }
    }
}

impl std::str::FromStr for UndoBatchStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "undone" => Self::Undone,
            "stale" => Self::Stale,
            _ => Self::Active,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoMoveRecord {
    pub position: usize,
    pub src: PathBuf,
    pub dst: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoBatchRecord {
    pub id: Option<i64>,
    pub label: String,
    pub created_at: String,
    pub status: UndoBatchStatus,
    pub moves: Vec<UndoMoveRecord>,
}

impl UndoBatchRecord {
    pub fn new(label: impl Into<String>, moves: Vec<(PathBuf, PathBuf)>) -> Self {
        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let move_records = moves
            .into_iter()
            .enumerate()
            .map(|(position, (src, dst))| UndoMoveRecord { position, src, dst })
            .collect();

        Self {
            id: None,
            label: label.into(),
            created_at,
            status: UndoBatchStatus::Active,
            moves: move_records,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_record_lifecycle() {
        let moves = vec![
            (PathBuf::from("/a/1.txt"), PathBuf::from("/b/1.txt")),
            (PathBuf::from("/a/2.txt"), PathBuf::from("/b/2.txt")),
        ];
        let batch = UndoBatchRecord::new("Organize Downloads", moves);

        assert_eq!(batch.label, "Organize Downloads");
        assert_eq!(batch.status, UndoBatchStatus::Active);
        assert_eq!(batch.moves.len(), 2);
        assert_eq!(batch.moves[0].position, 0);

        let json = batch.to_json().unwrap();
        let deserialized = UndoBatchRecord::from_json(&json).unwrap();
        assert_eq!(batch, deserialized);
    }
}
