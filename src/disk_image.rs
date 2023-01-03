use crate::{
    client::{self, CivoClient},
    errors,
};
use reqwest::Error;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Clone)]
pub struct DiskImage {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    state: String,
    #[serde(default)]
    distribution: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    label: String,
}

impl CivoClient {
    pub async fn list_disk_images(&self) -> Result<Vec<DiskImage>, Error> {
        let disk_endpoint = self.prepare_client_url("/v2/disk_images");
        let req = self
            .http_client
            .get(disk_endpoint)
            .bearer_auth(&self.api_key)
            .header("Accept", "Application/json")
            .header("Content-Type", "application/json")
            .query(&[("region", &self.region)])
            .send()
            .await?;

        let resp = req.json::<Vec<DiskImage>>();
        match resp.await {
            Ok(disk_images) => return Ok(disk_images),
            Err(error) => return Err(error),
        }
    }

    pub async fn get_disk_image(&self, id: &str) -> Result<DiskImage, Error> {
        let disk_plus_id = format!("/v2/disk_image/{}", id);
        let disk_endpoint = self.prepare_client_url(&disk_plus_id);
        let resp = self.send_get_request(disk_endpoint.as_str()).await;
        match resp {
            Ok(disk_images) => return Ok(disk_images.json::<DiskImage>().await?),
            Err(error) => return Err(error),
        }
    }

    pub async fn get_disk_by_name(&self, name: &str) -> Result<DiskImage, errors::GenericError> {
        let images = self.list_disk_images().await;
        match images {
            Ok(images) => match images.iter().find(|i| i.name == name) {
                Some(image) => Ok(image.clone()),
                None => Err(errors::GenericError::new("disk image not found")),
            },
            Err(error) => Err(errors::GenericError::new(&error.to_string())),
        }
    }
}
