use std::fmt;

#[derive(Debug)]
pub enum MiniGrepError {
    FileParseError,
    QueryParseError,
}

use MiniGrepError::{FileParseError, QueryParseError};

impl fmt::Display for MiniGrepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileParseError => write!(f, "Failed to get the file path"),
            QueryParseError => write!(f, "Failed to get the query"),
        }
    }
}

impl std::error::Error for MiniGrepError {}
