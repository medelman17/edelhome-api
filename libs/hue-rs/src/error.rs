use crate::bridge;

#[derive(thiserror::Error, Debug)]
pub enum HueError {
    #[error("An error occurred while performing an HTTP request")]
    Request(#[from] reqwest::Error),
    #[error("An error occurred while parsing the response")]
    SerdeJson(#[from] serde_json::Error),
    #[error("The bridge reported an error {}: {} ", .0.kind, .0.description)]
    BridgeError(#[from] bridge::BridgeError),
}
