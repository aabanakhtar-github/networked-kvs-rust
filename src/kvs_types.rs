use std::fmt::Display;

#[derive(Debug)]
pub enum DocType {
    JSON(String),
    Raw(String),
}

#[derive(Debug)]
pub enum KVSError {
    InvalidJSON(String),
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

impl Display for KVSError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            KVSError::InvalidJSON(v) | KVSError::InvalidKey(v) => {
                write!(fmt, "{}", v)
            }
        }
    }
}