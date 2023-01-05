use crate::client::CivoClient;
use crate::errors::GenericError;
use crate::errors::HTTPError;
use reqwest::Error;
use crate::{disk_image, network::Subnet};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize,Debug)]
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
#[derive(Deserialize, Serialize,Debug)]
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
    pub tags: Vec<String>,
    pub tag_list: Vec<String>,
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
            hostname: "".to_string(),
            reverse_dns: "".to_string(),
            size: "g3.medium".to_string(),
            region: self.region.to_string(),
            public_ip: "true".to_string(),
            network_id: default_network.id,
            template_id: disk_img.id,
            source_type: "".to_string(),
            snapshot_id: "".to_string(),
            initial_user: "civo".to_string(),
            script: "".to_string(),
            tags: vec!["".to_string()],
            tag_list: vec!["".to_string()],
            firewall_id: "".to_string(),
            subnets: vec!["".to_string()] ,
            ssh_key_id: None,
        };
        

        Ok(instance_config)
    }
    
    pub async fn create_instance(&self,config:InstanceConfig) -> Result<Instance,HTTPError> {
        let instance_endpoint = self.prepare_client_url("/v2/instances");
        let resp = self.send_post_request(instance_endpoint, &config).await;
        match resp {
            Ok(instance) => Ok(instance.json::<Instance>().await.unwrap()),
            Err(err) => Err(err)
        }

    }


}
