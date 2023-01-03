use crate::{client::CivoClient, errors::GenericError};
use reqwest::Error;
use serde::{Deserialize,Serialize};
#[derive(Deserialize,Serialize,Clone)]
pub struct Network{
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub cidr: String,
    #[serde(default)]
    pub cidr_v6: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub ipv4_enabled:  bool,
    #[serde(default)]
    pub ipv6_enabled: bool,
    #[serde(default)]
    pub nameservers_v4: Vec<String>,
    #[serde(default)]
    pub nameservers_v6:  Vec<String>
}


#[derive(Deserialize,Serialize)]
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
    // get_default_network finds the default private network for an account 
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
        match networks.iter().find(|n| n.default) {
            Some(default_network) => Ok(default_network.clone()),
            None => Err(GenericError { message: "Unable to find default network".to_string() }),
        }
    }
}