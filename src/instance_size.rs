use crate::client::CivoClient;
use crate::errors::HTTPError;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct InstanceSize {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub nice_name: String,
    pub cpu_cores: i32,
    pub ram_mb: i32,
    pub disk_gb: i32,
    pub transfer_tb: i32,
    pub description: String,
    pub selectable: bool,
}

impl CivoClient {
    pub async fn list_instance_sizes(&self) -> Result<Vec<InstanceSize>, HTTPError> {
        let instance_endpoint = self.prepare_client_url("/v2/sizes");
        let req = self
            .http_client
            .get(instance_endpoint)
            .bearer_auth(&self.api_key)
            .header("Accept", "Application/json")
            .header("Content-Type", "application/json")
            .query(&[("region", &self.region)])
            .header(USER_AGENT, "civo-rs")
            .send()
            .await
            .unwrap();
        if &req.status().as_u16() >= &300 {
            return Err(HTTPError::new(
                req.status().as_u16().clone(),
                req.text().await.unwrap().as_str(),
            ));
        }
        let resp = req.json::<Vec<InstanceSize>>().await;
        Ok(resp.unwrap())
    }
}
