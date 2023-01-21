use crate::{client::CivoClient, errors::{self, HTTPError}};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectStore {
    pub id: String,
    pub name: String,
    pub max_size: String,
    pub owner_info: BucketOwner,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BucketOwner {
    pub access_key_id: String,
    pub name: String,
    pub credential_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateObjectStoreRequest {
    pub name: String,
    pub max_size_gb: String,
    pub access_key_id: String,
    pub region: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedObjectstores {
    pub page: i32,
    pub per_page: i32,
    pub pages: i32,
    pub items: Vec<ObjectStore>,
}

impl CivoClient {
    pub async fn list_object_stores(&self) -> Result<PaginatedObjectstores,reqwest::Error> {
        let endpoint = self.prepare_client_url("/v2/objectstores");
        let resp = self.send_get_request(endpoint.as_str()).await;
        match resp {
            Ok(object_stores) => return Ok(object_stores.json::<PaginatedObjectstores>().await?),
            Err(error) => return  Err(error)
        };
    }

    pub async fn create_object_store(&self,obj_req:CreateObjectStoreRequest) -> Result<ObjectStore,HTTPError> {
        let endpoint = self.prepare_client_url("/v2/objectstores");
        let resp =  self.send_post_request(endpoint, obj_req).await;
        match resp {
            Ok(object_stores) => return Ok(object_stores.json::<ObjectStore>().await.unwrap()),
            Err(fail) => return  Err(fail)
        };
    }
}
