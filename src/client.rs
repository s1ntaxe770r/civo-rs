
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::errors::HTTPError;
#[derive(Debug)]
pub struct CivoClient {
    pub api_key: String,
    pub base_url: Url,
    pub http_client: reqwest::Client,
    pub region: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleResponse {
    pub id: String,
    pub result: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub details: String,
}

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
