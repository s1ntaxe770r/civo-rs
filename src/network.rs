use crate::{client::CivoClient, errors::GenericError};
use serde::{Deserialize,Serialize};
#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Network {
    #[serde(rename = "id")]

    pub ID: String,
    #[serde(rename = "name")]
    pub Name: String,
    #[serde(rename = "default")]
    pub Default: bool,
    #[serde(rename = "cidr")]
    pub CIDR: String,
    #[serde(rename = "cidr_v6")]
    #[serde(default)]
    pub CIDRV6: String,
    #[serde(rename = "label")]
    pub Label: String,
    #[serde(rename = "status")]
    pub Status: String,
    #[serde(rename = "ipv4_enabled")]
    pub IPv4Enabled: bool,
    #[serde(rename = "ipv6_enabled")]
    #[serde(default)]
    pub IPv6Enabled: bool,
    #[serde(rename = "nameservers_v4")]
    pub NameserversV4: Vec<String>,
    #[serde(rename = "nameservers_v6")]
    #[serde(default)]
    pub NameserversV6: Vec<String>,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct Subnet {
    pub id: String,
    #[serde(default)]
    pub name:String,
    pub network_id:String,
    #[serde(default)]
    pub subnet_size: String,
    #[serde(default)]
    pub status: String,
}

impl  CivoClient {
    /// get_default_network finds the default private network for an account 
    pub async fn get_default_network(&self) -> Result<Network,GenericError> {
        let network_endpoint = self.prepare_client_url("v2/networks");
        let req = self
            .http_client
            .get(network_endpoint)
            .bearer_auth(&self.api_key)
            .header("Accept", "Application/json")
            .header("Content-Type", "application/json")
            .query(&[("region", &self.region)])
            .send()
            .await.unwrap();
            if req.status().as_u16() >= 300 {
                return Err(GenericError::new(
                    req.text().await.unwrap().as_str(),
                ));
            }
    
        let networks = &req.json::<Vec<Network>>().await.unwrap();
        match networks.iter().find(|n| n.Default) {
            Some(default_network) => Ok(default_network.clone()),
            None => Err(GenericError { message: "Unable to find default network".to_string() }),
        }
    }
}