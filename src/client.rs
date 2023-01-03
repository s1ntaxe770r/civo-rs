use reqwest::{Client,Response,Error};
use serde::{Deserialize, Serialize};
use url::Url;
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
    pub code: String,
    pub reason: String,
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
    pub async fn send_get_request(&self,endpoint:&str) -> Result<Response,Error> {
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
}