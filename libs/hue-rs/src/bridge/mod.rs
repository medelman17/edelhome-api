use buildstructor::buildstructor;
use serde::de::DeserializeOwned;
use std::net::IpAddr;
use url::Url;

use request::BridgeRequestMethod as RequestMethod;
use error::BridgeResult as Result;

#[derive(Clone, Debug)]
pub struct Bridge {
    pub user: String,
    pub ip: IpAddr,
    pub(self) client: reqwest::Client,
}

#[buildstructor]
impl Bridge {
    #[builder]
    fn new(user: String, ip: IpAddr) -> Bridge {
        let client = reqwest::Client::new();
        Bridge { user, ip, client }
    }

    fn get_url(&self) -> Url {
        Url::parse(format!("http://{}/api/{}", self.ip, self.user).as_str()).unwrap()
    }

    pub(crate) async fn request<S, T>(
        &self,
        method: RequestMethod,
        suffix: S,
        body: Option<serde_json::Value>,
    ) -> Result<T>
        where
            S: AsRef<str>,
            T: DeserializeOwned,
    {
        let url = self.get_url();
        let url = url.join(suffix.as_ref()).unwrap();

        let mut request = match method {
            RequestMethod::Get => self.client.get(url),
            RequestMethod::Post => self.client.post(url),
            RequestMethod::Put => self.client.put(url),
            RequestMethod::Delete => self.client.delete(url),
        };

        if body.is_some() {
            request = request.json(&body.unwrap());
        }

        match request.send().await {
            Ok(response) => {
                match response.json::<T>().await {
                    Ok(response) => return Ok(response),
                    Err(e) => {
                        panic!("Error parsing response: {}", e)
                    }
                };
            }
            Err(e) => {
                panic!("Error sending request: {}", e)
            }
        };
    }
}









mod request;
mod error;
mod response;
mod discover;
pub use error::*;
pub use discover::discover_bridges;
