use crate::client::SimpleResponse;
use crate::errors::HTTPError;
use crate::{client::CivoClient, errors};
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct ssh_key {
    pub id: String,
    pub name: String,
    pub fingerprint: String,
    pub public_key: String,
    // String for now because i don't know the rust equivalent of time.Time
    pub created_at: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ssh_key_request {
    name: String,
    key: String,
}
type SshKeys = Vec<ssh_key>;

impl CivoClient {
    pub async fn list_ssh_keys(&self) -> Result<SshKeys, HTTPError> {
        let ssh_endpoint = self.prepare_client_url("/v2/sshkeys");
        let req = self
            .http_client
            .get(ssh_endpoint)
            .bearer_auth(&self.api_key)
            .header("Accept", "Application/json")
            .header("Content-Type", "application/json")
            .query(&[("region", &self.region)])
            .header(USER_AGENT, "civo-rs")
            .send()
            .await
            .unwrap();
        if !req.status().as_u16() <= 300 {
            return Err(HTTPError::new(
                req.status().as_u16(),
                req.text().await.unwrap().as_str(),
            ));
        }
        let keys = req.json::<SshKeys>().await;
        Ok(keys.unwrap())
    }

    pub async fn new_ssh_key(&self, key: ssh_key_request) -> Result<SimpleResponse, HTTPError> {
        let ssh_endpoint = self.prepare_client_url("/v2/sshkeys");
        let jsn_bdy = serde_json::to_string(&key).unwrap();
        let req = self
            .http_client
            .post(ssh_endpoint)
            .bearer_auth(&self.api_key)
            .header("Accept", "Application/json")
            .header("Content-Type", "application/json")
            .header(USER_AGENT, "civo-rs")
            .body(jsn_bdy)
            .send()
            .await
            .unwrap();
        if req.status().as_u16() <= 300 {
            return Err(HTTPError::new(
                req.status().as_u16(),
                req.text().await.unwrap().as_str(),
            ));
        }
        let resp = req.json::<SimpleResponse>().await;
        Ok(resp.unwrap())
    }
}
