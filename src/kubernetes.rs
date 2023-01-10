use std::collections::HashMap;

use crate::client::CivoClient;
use crate::client::SimpleResponse;
use crate::errors::GenericError;
use crate::errors::HTTPError;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct KubernetesInstance {
    pub id: String ,
    #[serde(default)]
    pub hostname:  String ,
    #[serde(default)]
    pub size: String ,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub source_type: String, 
    #[serde(default)]
    pub source_id: String , 
    #[serde(default)]
    pub initial_user: String,
    #[serde(default)]
    pub initial_password: String, 
    #[serde(default)]
    pub status:  String, 
    #[serde(default)]
    pub firewall_id:  String, 
    #[serde(default)]
    pub public_ip: String, 
    pub cpu_cores: i32, 
    pub ram_mb: i32 ,
    #[serde(default)] 
    pub tags: String, 
    pub created_at: String , 
    #[serde(default)]
    pub civo_statsd_token: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct  KubernetesPool   {
    pub id: String,
    pub count:i32,
    pub size: String,
    pub instance_names: Vec<String>,
    pub instances: Vec<KubernetesInstance>
}

type ApplcationConfiguration:HashMap<String,String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct KubernetesInstalledApplication {
    pub application: String, 
    pub name: String , 
    pub version: String, 
    pub dependencies: String, 
    pub maintainer: String, 
    pub description: String, 
    pub post_install: String, 
    pub installed: String, 
    pub url: String,
    pub category: String, 
    pub update_at: String, 
    pub image_url: String, 
    pub plan: String, 
    pub configuration: HashMap<String,ApplcationConfiguration> 
}




impl  CivoClient {
    
}
