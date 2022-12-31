use crate::client::CivoClient;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Region {
    pub code: String,
    pub name: String,
    // #[serde(rename(serialize = "type", deserialize = "type"))]
    pub r#type: String,
    pub out_of_capacity: bool,
    pub country: String,
    pub country_name: String,
    pub features: feature,
    pub default: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct feature {
    pub iaas: bool,
    pub kubernetes: bool,
    pub object_store: bool,
}

type Regions = Vec<Region>;
impl CivoClient {
    pub fn prepare_client_url(&self, path: &str) -> Url {
        let new = self.base_url.join(path).unwrap();
        new.clone()
    }

    fn add_region_query(&self) -> Vec<(&str, String)> {
        let query = vec![("region", self.region.to_string())];
        query
    }

    // Result<Vec<Region>, reqwest::Error>
    pub async fn list_regions(&self) -> Result<Vec<Region>, Error> {
        let region_url = self.prepare_client_url("/v2/regions");
        let req = self
            .http_client
            .get(region_url)
            .bearer_auth(&self.api_key)
            .header("Accept", "Application/json")
            .header("Content-Type", "application/json")
            .query(&[("region", &self.region)])
            .send()
            .await?;
        let regions = req.json::<Regions>().await?;
        // let reg = serde_json::from_str::<Regions>(req.text().await.unwrap().as_str());
        // Ok(reg.unwrap())
        Ok(regions)
    }
}
