use serde::{Deserialize, Serialize};
use crate::{network::Subnet};
#[derive(Deserialize, Serialize)]
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
