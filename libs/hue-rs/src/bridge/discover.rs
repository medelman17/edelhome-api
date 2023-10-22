use crate::bridge::error::BridgeResult as Result;
use std::net::IpAddr;
use url::Url;

pub async fn discover_bridges() -> Result<Vec<IpAddr>> {
    let url = Url::parse("https://discovery.meethue.com").unwrap();
    let request = reqwest::Client::new().get(url);
    #[derive(serde::Deserialize)]
    struct BridgeDiscoveryJson {
        #[serde(rename = "internalipaddress")]
        ip_address: String,
    }

    let bridges: Vec<BridgeDiscoveryJson> = request.send().await.unwrap().json().await.unwrap();

    Ok(bridges
        .iter()
        .map(|bridge| bridge.ip_address.parse().unwrap())
        .collect())
}
