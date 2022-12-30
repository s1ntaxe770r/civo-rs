use serde::{Deserialize,Serialize};
#[derive(Deserialize,Serialize)]
pub struct Network{
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub default: bool,
    #[serde(default)]
    pub cidr: String,
    #[serde(default)]
    pub cidr_v6: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub ipv4_enabled:  bool,
    #[serde(default)]
    pub ipv6_enabled: bool,
    #[serde(default)]
    pub nameservers_v4: Vec<String>,
    #[serde(default)]
    pub nameservers_v6:  Vec<String>
}


#[derive(Deserialize,Serialize)]
pub struct Subnet {
    pub id: String,
    #[serde(default)]
    pub name:String,
    pub network_id:String,
    #[serde(default)]
    pub subnet_size: String,
    #[serde(default)]
    pub status: String,
}