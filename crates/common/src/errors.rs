use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmartFileeError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Protected path access refused: {path} ({reason})")]
    ProtectedPath { path: PathBuf, reason: String },

    #[error("File or directory not found: {0}")]
    NotFound(PathBuf),

    #[error("Destination collision: {0}")]
    Collision(PathBuf),

    #[error("Operation cancelled by user")]
    Cancelled,

    #[error("Invalid parameter: {0}")]
    InvalidInput(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("General error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, SmartFileeError>;
