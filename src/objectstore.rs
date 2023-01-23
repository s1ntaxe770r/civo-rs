use crate::{client::CivoClient, errors::{ HTTPError}};
use serde::{Deserialize, Serialize};
/// Represents an object store on Civo's object storage service.
#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectStore {
    /// The ID of the object store.
    pub id: String,
    /// The name of the object store.
    pub name: String,
    /// The maximum size of the object store.
    pub max_size: String,
    /// Information about the owner of the object store.
    pub owner_info: BucketOwner,
    /// The status of the object store.
    pub status: String,
}

/// Represents the owner of an object store on Civo's object storage service.
#[derive(Serialize, Deserialize, Debug)]
pub struct BucketOwner {
    /// The access key ID of the object store owner.
    pub access_key_id: String,
    /// The name of the object store owner.
    pub name: String,
    /// The credential ID of the object store owner.
    pub credential_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateObjectStoreRequest {
    pub name: String,
    pub max_size_gb: String,
    pub access_key_id: String,
    pub region: String,
}
/// Represents a paginated list of object stores on Civo's object storage service.
#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedObjectstores {
    /// The current page number.
    pub page: i32,
    /// The number of items per page.
    pub per_page: i32,
    /// The total number of pages.
    pub pages: i32,
    /// The items in the current page.
    pub items: Vec<ObjectStore>,
}

impl CivoClient {
    /// Retrieves a list of object stores on Civo's object storage service.
    ///
    /// # Examples
    /// ```
    /// let client = new_civo_client("API_KEY", "REGION");
    /// let object_stores = client.list_object_stores().await;
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` containing a `PaginatedObjectstores` struct on success, or an `Error` on failure.
    pub async fn list_object_stores(&self) -> Result<PaginatedObjectstores,reqwest::Error> {
        let endpoint = self.prepare_client_url("/v2/objectstores");
        let resp = self.send_get_request(endpoint.as_str()).await;
        match resp {
            Ok(object_stores) => return Ok(object_stores.json::<PaginatedObjectstores>().await?),
            Err(error) => return  Err(error)
        };
    }
    /// Creates a new object store on Civo's object storage service.
    ///
    /// # Examples
    /// ```
    /// let client = new_civo_client("API_KEY", "REGION");
    /// let object_store_request = CreateObjectStoreRequest {
    ///     name: "my-object-store".to_string(),
    ///     max_size_gb: "100".to_string(),
    ///     access_key_id: "ACCESS_KEY".to_string(),
    ///     region: "REGION".to_string()
    /// };
    /// let object_store = client.create_object_store(object_store_request).await;
    /// ```
    ///
    /// # Arguments
    ///
    /// * `obj_req` - The `CreateObjectStoreRequest` struct containing the configuration for the new object store
    pub async fn create_object_store(&self,obj_req:CreateObjectStoreRequest) -> Result<ObjectStore,HTTPError> {
        let endpoint = self.prepare_client_url("/v2/objectstores");
        let resp =  self.send_post_request(endpoint, obj_req).await;
        match resp {
            Ok(object_stores) => return Ok(object_stores.json::<ObjectStore>().await.unwrap()),
            Err(fail) => return  Err(fail)
        };
    }
}
