
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::errors::HTTPError;
/// Represents a client for interacting with the Civo Kubernetes API.
#[derive(Debug)]
pub struct CivoClient {
    /// API key for authentication
    pub api_key: String,
    /// Base URL for the API
    pub base_url: Url,
    /// HTTP client for making requests
    pub http_client: reqwest::Client,
    /// Region for the API requests
    pub region: String,
}

/// Represents a simple response from the Civo API
#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleResponse {
    /// ID of the resource
    pub id: String,
    /// Result of the operation
    pub result: String,
    /// Code of the response
    #[serde(default)]
    pub code: String,
    /// Reason for the response
    #[serde(default)]
    pub reason: String,
    /// Additional details about the response
    #[serde(default)]
    pub details: String,
}

/// Creates a new Civo client for interacting with the API
///
/// # Parameters
///
/// * `apikey`: API key for authentication
/// * `region`: Region for the API requests
///
/// # Returns
///
/// A new instance of the Civo client
pub fn new_civo_client(apikey: String, region: String) -> CivoClient {
    let http_c = Client::new();
    let u = Url::parse("https://api.civo.com/v2").unwrap();
    let c = CivoClient {
        base_url: u,
        api_key: apikey,
        http_client: http_c,
        region: region,
    };
    c
}

impl CivoClient {
    pub async fn send_get_request(&self, endpoint: &str) -> Result<Response, Error> {
        let req = self
            .http_client
            .get(endpoint)
            .bearer_auth(&self.api_key)
            .header("Accept", "Application/json")
            .header("Content-Type", "application/json")
            .query(&[("region", &self.region)])
            .send()
            .await;
        match req {
            Ok(response) => return Ok(response),
            Err(error) => return Err(error),
        }
    }
    pub async fn send_post_request<T>(&self, endpoint: Url, data: T) -> Result<Response, HTTPError>
    where
        T: Serialize,
    {
        let res = self
            .http_client
            .post(endpoint)
            .bearer_auth(&self.api_key)
            .header("Accept", "Application/json")
            .header("Content-Type", "application/json")
            .query(&[("region", &self.region)])
            .json(&data)
            .send()
            .await;
        match res {
            Ok(resp) => {
                if !resp.status().is_success() {
                    let err = HTTPError::new(
                        resp.status().as_u16(),
                        &resp.text().await.unwrap().to_string(),
                    );
                    return Err(err);
                } else {
                    return Ok(resp);
                }
            }
            Err(err) => Err(HTTPError::new(0, &err.to_string())),
        }
    }

    pub async fn send_delete_request(&self, endpoint: &str) -> Result<Response, HTTPError> {
        let res = self
            .http_client
            .delete(endpoint)
            .bearer_auth(&self.api_key)
            .header("Accept", "Application/json")
            .header("Content-Type", "application/json")
            .query(&[("region", &self.region)])
            .send()
            .await;
        match res {
            Ok(resp) => {
                if !resp.status().is_success() {
                    let err = HTTPError::new(
                        resp.status().as_u16(),
                        &resp.text().await.unwrap().to_string(),
                    );
                    return Err(err);
                } else {
                    return Ok(resp);
                }
            }
            Err(err) => Err(HTTPError::new(0, &err.to_string())),
        }
    }
}

