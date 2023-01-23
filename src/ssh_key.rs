use crate::client::CivoClient;
use crate::errors::HTTPError;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};


/// Represents a SSH key on Civo's platform
#[derive(Serialize, Deserialize, Debug)]
pub struct SshKey {
    /// The unique identifier for the SSH key
    pub id: String,
    /// The name of the SSH key
    pub name: String,
    /// The fingerprint of the SSH key
    pub fingerprint: String,
    /// The public key content
    pub public_key: String,
    /// The timestamp of the creation of the key
    pub created_at: String,
}

/// Represents a request to create a new SSH key on Civo's platform
#[derive(Serialize, Deserialize, Debug)]
pub struct SshKeyRequest {
    /// The name of the SSH key
    pub name: String,
    /// The public key content
    pub public_key: String,
}

/// Represents the response when creating a new SSH key on Civo's platform
#[derive(Serialize, Deserialize, Debug)]
pub struct SshKeyResponse {
    /// The unique identifier of the created SSH key
    pub id: String,
    /// The result of the creation request
    pub result: String,
}

/// Alias for a vector of `SshKey` structs
type SshKeys = Vec<SshKey>;

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
        if !req.status().as_u16() >= 300 {
            return Err(HTTPError::new(
                req.status().as_u16(),
                req.text().await.unwrap().as_str(),
            ));
        }
        let keys = req.json::<SshKeys>().await;
        Ok(keys.unwrap())
    }

    pub async fn new_ssh_key(&self, key: SshKeyRequest) -> Result<SshKeyResponse, HTTPError> {
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
        if &req.status().as_u16() >= &300 {
            return Err(HTTPError::new(
                req.status().as_u16().clone(),
                req.text().await.unwrap().as_str(),
            ));
        }
        let resp = req.json::<SshKeyResponse>().await;
        Ok(resp.unwrap())
    }

    pub async fn update_ssh_key(&self, name: &str, ssh_key_id: &str) -> Result<SshKey, HTTPError> {
        let ssh_endpoint = self.prepare_client_url("/v2/sshkeys");
        // /v2/sshkeys/key_id
        ssh_endpoint.join(ssh_key_id).unwrap();
        let jsn_bdy = serde_json::to_string(name).unwrap();
        let req = self
            .http_client
            .put(ssh_endpoint)
            .bearer_auth(&self.api_key)
            .header("Accept", "Application/json")
            .header("Content-Type", "application/json")
            .header(USER_AGENT, "civo-rs")
            .body(jsn_bdy)
            .send()
            .await
            .unwrap();
        if &req.status().as_u16() >= &300 {
            return Err(HTTPError::new(
                req.status().as_u16().clone(),
                req.text().await.unwrap().as_str(),
            ));
        }
        let resp = req.json::<SshKey>().await;
        Ok(resp.unwrap())
        // dbg!(req.text().await.unwrap())
    }
}
