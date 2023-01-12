use std::collections::HashMap;

use crate::client::CivoClient;
use crate::errors::GenericError;
use reqwest::Error;
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

type ApplcationConfiguration = HashMap<String,String>;

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


#[derive(Deserialize,Serialize,Debug)]
pub struct KubernetesCluster {
  pub id: String, 
  pub name: String, 
  pub generated_name: String,
  pub version: String,
  pub status: String, 
  pub ready: bool , 
  pub num_target_node:  String, 
  pub built_at:  String, 
  pub kubeconfig: String , 
  pub kubnernetes_version:  String, 
  pub api_endpoint: String,  
  pub master_ip: String, 
  pub dns_entry: String, 
  pub upgrade_to_available: String, 
  pub legacy: String, 
  pub network_id: String, 
  pub namespace: String, 
  pub tags: String, 
  pub created_at: String, 
  pub instances: Vec<KubernetesInstance>,
  pub pool: Vec<KubernetesPool>,
  pub installed_applications: Vec<KubernetesInstalledApplication>,
  pub firewall_id: String, 
  pub cni_plugin: String, 
  pub ccm_installed: String
}

#[derive(Deserialize,Serialize)]
pub struct RequiredPools { 
    pub id: String,
    pub size: String, 
    pub count: i32
}

#[derive(Deserialize,Serialize)]
pub struct PaginatedKubernetesClusters  {
    pub page: i32, 
    pub per_page: i32, 
    pub items: Vec<KubernetesCluster>

}

#[derive(Deserialize,Serialize)] 
pub struct KubernetesClusterConfig { 
    pub name: String ,
    pub region: String,
    pub num_target_nodes: i32, 
    pub target_node_size: String ,
    pub kubnernetes_version: String, 
    pub node_destroy: String, 
    pub network_id: String,
    pub tags: String, 
    pub pool: Vec<KubernetesPoolConfig>,
    pub applications: String, 
    pub instance_firewall: String, 
    pub firewall_rule: String, 
    pub cni_plugin: String, 
}


#[derive(Deserialize,Serialize)] 
pub struct KubernetesPoolConfig {
    id: String,
    count: i32,
    size: String
} 
#[derive(Deserialize,Serialize)]
pub struct  KubernetesVersion {
    pub version:String ,
   // #[serde(rename(serialize = "type", deserialize = "type"))]
   pub r#type: String,
   pub default: bool,
}


impl  CivoClient {
    pub  async fn list_kubernetes_clusters(&self) -> Result<PaginatedKubernetesClusters,Error> {
        let cluster_endpoint = self.prepare_client_url("/v2/kubernetes/clusters");
        let resp = self.send_get_request(&cluster_endpoint.as_str()).await;
        match resp {
            Ok(clusters) => return Ok(clusters.json::<PaginatedKubernetesClusters>().await?),
            Err(error) => return Err(error),
        }
    }

    pub async fn get_kubernetes_cluster(&self, id: &str) -> Result<KubernetesCluster, Error> {
        let cluster_plus_id = format!("/v2/kubernetes/cluster/{}", id);
        let disk_endpoint = self.prepare_client_url(&cluster_plus_id);
        let resp = self.send_get_request(disk_endpoint.as_str()).await;
        match resp {
            Ok(cluster) => return Ok(cluster.json::<KubernetesCluster>().await?),
            Err(error) => return Err(error),
        }
    }

    pub async fn new_kubernetes_cluster(&self, mut kc: KubernetesClusterConfig) ->  Result<KubernetesCluster,GenericError> {
        kc.region = self.region.clone();
        Ok(todo!())
    }

}
