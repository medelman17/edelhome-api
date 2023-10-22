use crate::bridge::error::{BridgeError, BridgeResult};
use serde::de::DeserializeOwned;
use serde::{Deserialize};
use serde_json::Value as JsonValue;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BridgeResponse<T> {
    Success(T),
    Error(BridgeError),
}

impl<T> BridgeResponse<T> {
    pub fn into_result(self) -> Result<T, BridgeError> {
        match self {
            BridgeResponse::Success(value) => Ok(value),
            BridgeResponse::Error(error) => Err(error),
        }
    }
}

impl<T: fmt::Display> fmt::Display for BridgeResponse<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BridgeResponse::Success(v) => write!(f, "Success: {}", v),
            BridgeResponse::Error(e) => write!(f, "Error: {:?}", e),
        }
    }
}

pub(crate) fn parse<T>(response: JsonValue) -> BridgeResult<T>
where
    T: DeserializeOwned,
{
    Ok(serde_json::from_value(response).unwrap())
}
