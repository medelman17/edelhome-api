use std::fmt;
use std::fmt::Error;
use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize, thiserror::Error)]
#[error("{description}")]
pub struct BridgeError {
    #[serde(rename = "type")]
    pub kind: BridgeErrorKind,
    pub address: String,
    pub description: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize, thiserror::Error)]
pub enum BridgeErrorKind {
    UnauthorizedUser = 1
}

impl fmt::Display for BridgeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BridgeErrorKind::UnauthorizedUser => write!(f, "1"),
        }
    }
}

pub type BridgeResult<T> = std::result::Result<T, Error>;
