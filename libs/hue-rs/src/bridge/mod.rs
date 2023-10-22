mod discover;
pub(crate) mod error;
mod request;
mod response;

use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use std::net::IpAddr;
use url::Url;

use crate::bridge;
use bridge::{error::BridgeResult as Result, request::BridgeRequestMethod as RequestMethod};

pub use bridge::discover::discover_bridges;
pub use bridge::error::BridgeError;

/// Bridge
#[derive(Clone, Debug)]
pub struct Bridge {
    /// Name of the user that is connected to the bridge.
    pub username: String,
    pub ip_address: IpAddr,
    pub(self) client: reqwest::Client,
}

impl Bridge {
    pub fn new<S>(ip_address: IpAddr, username: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            username: username.into(),
            ip_address,
            client: reqwest::Client::new(),
        }
    }

    pub fn get_api_url(&self) -> Url {
        let mut url =
            Url::parse(format!("http://{}/api/{}", self.ip_address, self.username).as_str())
                .unwrap();
        url
    }

    pub(crate) async fn request<S, T>(
        &self,
        path: S,
        method: RequestMethod,
        body: Option<JsonValue>,
    ) -> Result<T>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
    {
        let mut url = self.get_api_url();
        url.join(path.as_ref()).unwrap();
        let request = match method {
            RequestMethod::Get => reqwest::Client::new().get(url),
            RequestMethod::Post => reqwest::Client::new().post(url),
            RequestMethod::Put => reqwest::Client::new().put(url),
            RequestMethod::Delete => reqwest::Client::new().delete(url),
        };

        let response = match body {
            Some(v) => request.json(&v).send().await.unwrap(),
            None => request.send().await.unwrap(),
        };

        Ok(response.json::<T>().await.unwrap())
    }
}
