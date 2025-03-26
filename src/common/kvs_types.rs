use std::fmt::Display;
use thiserror::Error;

#[derive(Debug)]
pub enum DocType {
    JSON(String),
    Raw(String),
}

#[derive(Error, Debug)]
pub enum KVSError {
    #[error("Invalid JSON String!")]
    InvalidJSON(String),
    #[error("Invalid Key String!")]
    InvalidKey(String),
}

impl Display for DocType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let value = match self {
            DocType::JSON(v) => v,
            DocType::Raw(v) => v,
        };
        write!(fmt, "{}", value)
    }
}
