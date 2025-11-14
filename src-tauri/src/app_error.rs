use std::ffi::OsString;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum AppError {
    IoError(String),
    DocumentNotFound(String),
    InvalidPath(String),
    ParseError(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

impl From<OsString> for AppError {
    fn from(value: OsString) -> Self {
        match value.into_string() {
            Ok(s) => AppError::InvalidPath(s),
            Err(_) => AppError::InvalidPath("Invalid UTF-8 in path".to_string()),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::IoError(msg) => write!(f, "IO error: {}", msg),
            AppError::DocumentNotFound(path) => write!(f, "Document not found: {}", path),
            AppError::InvalidPath(path) => write!(f, "Invalid path provided: {}", path),
            AppError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}
