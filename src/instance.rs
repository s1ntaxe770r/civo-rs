use crate::client::CivoClient;
use crate::client::SimpleResponse;
use crate::errors::GenericError;
use crate::errors::HTTPError;
use crate::network::Subnet;
use crate::utils;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Instance {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub openstack_server_id: String,
    #[serde(default)]
    pub hostname: String,
    #[serde(default)]
    pub reverse_dns: String,
    #[serde(default)]
    pub size: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub network_id: String,
    #[serde(default)]
    pub private_ip: String,
    #[serde(default)]
    pub public_ip: String,
    #[serde(default)]
    pub ipv6: String,
    #[serde(default)]
    pub pseudo_ip: String,
    #[serde(default)]
    pub template_id: String,
    #[serde(default)]
    pub source_type: String,
    #[serde(default)]
    pub source_id: String,
    #[serde(default)]
    pub snapshot_id: String,
    #[serde(default)]
    pub initial_user: String,
    #[serde(default)]
    pub initial_password: String,
    #[serde(default)]
    pub ssh_key: String,
    #[serde(default)]
    pub ssh_key_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub firewall_id: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub civostatsd_token: String,
    #[serde(default)]
    pub civostatsd_stats: String,
    #[serde(default)]
    pub civostatsd_stats_per_minute: Vec<String>,
    #[serde(default)]
    pub civostatsd_stats_per_hour: Vec<String>,
    #[serde(default)]
    pub openstack_image_id: String,
    #[serde(default)]
    pub rescue_password: String,
    #[serde(default)]
    pub volume_backed: bool,
    #[serde(default)]
    pub cpu_cores: i32,
    #[serde(default)]
    pub ram_mb: i32,
    #[serde(default)]
    pub disk_gb: i32,
    #[serde(default)]
    pub script: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub reserved_ip_id: String,
    #[serde(default)]
    subnets: Vec<Subnet>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct InstanceConfig {
    pub count: i32,
    pub hostname: String,
    #[serde(default)]
    pub reverse_dns: String,
    pub size: String,
    pub region: String,
    pub public_ip: String,
    pub network_id: String,
    pub template_id: String,
    pub source_type: String,
    pub snapshot_id: String,
    #[serde(default)]
    pub subnets: Vec<String>,
    pub initial_user: String,
    #[serde(default)]
    pub ssh_key_id: Option<String>,
    pub script: String,
    #[serde(default)]
    pub tags: String,
    #[serde(default)]
    pub tag_list: String,
    pub firewall_id: String,
}
#[derive(Deserialize, Serialize)]
pub struct InstanceConsole {
    pub url: String,
}

pub struct PaginatedInstanceList {
    pub page: i32,
    pub per_page: i32,
    pub pages: i32,
    pub items: Vec<Instance>,
}

impl CivoClient {
    /// Creates a new instance configuration
    ///
    /// # Returns
    ///
    /// Returns a Result containing an `InstanceConfig` on success, or a `GenericError` on failure.
    /// This method will fail if the call to get the default network or the disk image fails.
    pub async fn new_instance_config(&self) -> Result<InstanceConfig, GenericError> {
        let default_network = match self.get_default_network().await {
            Ok(network) => network,
            Err(error) => return Err(GenericError::new(&error.to_string())),
        };

        let disk_img = match self.get_disk_by_name("ubuntu-focal").await {
            Ok(img) => img,
            Err(error) => return Err(GenericError::new(&error.to_string())),
        };

        let instance_config = InstanceConfig {
            count: 1,
            hostname: utils::random_name(),
            reverse_dns: "".to_string(),
            size: "g3.medium".to_string(),
            region: self.region.to_string(),
            public_ip: "true".to_string(),
            network_id: default_network.ID,
            template_id: disk_img.id,
            source_type: "".to_string(),
            snapshot_id: "".to_string(),
            initial_user: "civo".to_string(),
            script: "".to_string(),
            tags: "".to_string(),
            tag_list: "".to_string(),
            firewall_id: "".to_string(),
            subnets: vec!["".to_string()],
            ssh_key_id: None,
        };

        Ok(instance_config)
    }

    /// Creates a new instance
    ///
    /// # Arguments
    ///
    /// * `config`: an `InstanceConfig` containing the configuration for the new instance
    ///
    /// # Returns
    ///
    /// Returns a Result containing an `Instance` on success, or an `HTTPError` on failure.
    pub async fn create_instance(&self, config: InstanceConfig) -> Result<Instance, HTTPError> {
        let instance_endpoint = self.prepare_client_url("/v2/instances");
        let resp = self.send_post_request(instance_endpoint, &config).await;
        match resp {
            Ok(instance) => Ok(instance.json::<Instance>().await.unwrap()),
            Err(err) => Err(err),
        }
    }

    /// Deletes an existing instance 
    ///
    /// # Arguments
    ///
    /// * `instance_id`: a string slice containing the id of the instance to be deleted
    ///
    /// # Returns
    ///
    /// Returns a Result containing a `SimpleResponse` on success, or an `HTTPError` on failure.
    pub async fn delete_instance(&self,instance_id:&str) -> Result<SimpleResponse,HTTPError> {
        let instance_endpoint = self.prepare_client_url("/v2/instances/").join(&instance_id);
        let resp = self.send_delete_request(instance_endpoint.unwrap().as_str()).await;
        match resp {
            Ok(simplresp) => Ok(simplresp.json::<SimpleResponse>().await.unwrap()),
            Err(err) => Err(err),
        }
    }
}
