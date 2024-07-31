use thiserror::Error;

pub mod types;

#[derive(Error, Debug)]
pub enum ParseError {
    Serde(#[from] serde_json::Error),
    Io(#[from] std::io::Error),
    UnexpectedFormat(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Error, Debug)]
pub enum WriteError {
    Serde(#[from] serde_json::Error),
    Io(#[from] std::io::Error),
}

impl std::fmt::Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
