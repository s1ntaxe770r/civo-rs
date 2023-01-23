use std::collections::HashMap;
use crate::client::{CivoClient, SimpleResponse};
use crate::errors::GenericError;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use crate::errors::HTTPError; 



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
pub struct K8scluster {
 #[serde(default)] 
  pub id: String, 
  #[serde(default)] 
  pub name: Option<String>, 
  #[serde(default)] 
  pub generated_name: Option<String>,
  #[serde(default)] 
  pub version: Option<String>,
  #[serde(default)] 
  pub status: Option<String>, 
  #[serde(default)] 
  pub ready: bool , 
  #[serde(default)] 
  pub num_target_node:  Option<String>, 
  #[serde(default)] 
  pub built_at:  Option<String>, 
  #[serde(default)] 
  pub kubeconfig: Option<String> , 
  #[serde(default)] 
  pub kubnernetes_version:  Option<String>, 
  #[serde(default)] 
  pub api_endpoint: Option<String>,  
  #[serde(default)] 
  pub master_ip: Option<String>, 
  #[serde(default)] 
  pub dns_entry: Option<String>, 
  #[serde(default)] 
  pub upgrade_to_available: Option<String>, 
  #[serde(default)] 
  pub legacy: Option<String>, 
  #[serde(default)] 
  pub network_id: Option<String>, 
  #[serde(default)] 
  pub namespace: Option<String>, 
  #[serde(default)] 
  pub tags: Option<String>, 
  #[serde(default)] 
  pub created_at: Option<String>, 
  #[serde(default)] 
  pub instances: Option<Vec<KubernetesInstance>>,
  #[serde(default)] 
  pub pools: Option<Vec<KubernetesPool>>,
  pub required_pools: Option<Vec<RequiredPools>>,
  #[serde(default)] 
  pub installed_applications: Vec<KubernetesInstalledApplication>,
  #[serde(default)] 
  pub firewall_id: Option<String>, 
  #[serde(default)] 
  pub cni_plugin: Option<String>, 
   #[serde(default)] 
  pub ccm_installed: Option<String>
}

#[derive(Deserialize,Serialize,Debug)]
pub struct KubernetesCluster  { 
  pub id: String,
    pub name: String,
    pub version: String,
    pub status: String,
    pub ready: bool,
    pub num_target_nodes: u32,
    pub target_nodes_size: String,
    pub built_at: String,
    pub kubernetes_version: String,
    pub api_endpoint: String,
    pub dns_entry: String,
   pub  created_at: String,
    pub master_ip: String,
    pub pools: Option<()>,
    pub required_pools: Vec<RequiredPools>,
    pub firewall_id: String,
    pub master_ipv6: String,
    pub network_id: String,
    pub namespace: String,
    pub size: String,
    pub count: u32,
    pub kubeconfig: Option<()>,
    pub instances: Option<()>,
    pub installed_applications: Option<()>,
    pub ccm_installed: String,
}

#[derive(Deserialize,Serialize,Debug)]
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

pub trait ClusterConfig {
    fn name(&self) -> &str;
    fn region(&self) -> &str;
    fn network_id(&self) -> &str;
    fn firewall_rule(&self) -> &str;
    fn pools(&self) -> &Vec<KubernetesPoolConfig>;
}


#[derive(Deserialize,Serialize)] 
pub struct KubernetesClusterConfig { 
    pub name: String ,
    pub region: String,
    #[serde(default)]
    pub num_target_nodes: i32, 
    #[serde(default)]
    pub target_node_size: String ,
    #[serde(default)]
    pub kubnernetes_version: String, 
    #[serde(default)]
    pub node_destroy: String, 
    pub network_id: String,
    #[serde(default)]
    pub tags: String, 
    pub pools: Vec<KubernetesPoolConfig>,
    #[serde(default)]
    pub applications: String, 
    #[serde(default)]
    pub instance_firewall: String, 
    pub firewall_rule: String, 
    #[serde(default)]
    pub cni_plugin: String, 
}


#[derive(Deserialize,Serialize)]
pub struct SimpleClusterConfig {
    pub name: String, 
    pub region: String, 
    pub network_id: String,
    pub firewall_rule: String, 
    pub pools: Vec<KubernetesPoolConfig>
}


#[derive(Deserialize,Serialize)] 
pub struct KubernetesPoolConfig {
    pub id: String,
    pub count: i32,
    pub size: String
} 
#[derive(Deserialize,Serialize,Debug)]
pub struct  KubernetesVersion {
    #[serde(default)]
    pub version:String ,
   #[serde(rename(serialize = "type", deserialize = "type"))]
   #[serde(default)]
   pub r#type: String,
   #[serde(default)]
   pub default: bool,
}


impl  CivoClient {
     /// Lists all Kubernetes clusters.
    ///
    /// # Examples
    /// ```rust
    /// let client = new_civo_client("API_KEY", "REGION");
    /// let clusters = client.list_kubernetes_clusters().await;
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` containing a `PaginatedKubernetesClusters` struct on success, or an `Error` on failure.
    pub  async fn list_kubernetes_clusters(&self) -> Result<PaginatedKubernetesClusters,Error> {
        let cluster_endpoint = self.prepare_client_url("/v2/kubernetes/clusters");
        let resp = self.send_get_request(&cluster_endpoint.as_str()).await;
        match resp {
            Ok(clusters) => return Ok(clusters.json::<PaginatedKubernetesClusters>().await?),
            Err(error) => return Err(error),
        }
    }
     /// Retrieves a single Kubernetes cluster by ID.
    ///
    /// # Examples
    /// ```
    /// let client = new_civo_client("API_KEY", "REGION");
    /// let cluster = client.get_kubernetes_cluster("CLUSTER_ID").await;
    /// ```
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the cluster to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `KubernetesCluster` struct on success, or an `Error` on failure.
    pub async fn get_kubernetes_cluster(&self, id: &str) -> Result<KubernetesCluster, Error> {
        let cluster_plus_id = format!("/v2/kubernetes/cluster/{}", id);
        let cluster_endopoint = self.prepare_client_url(&cluster_plus_id);
        let resp = self.send_get_request(cluster_endopoint.as_str()).await;
        match resp {
            Ok(cluster) => return Ok(cluster.json::<KubernetesCluster>().await?),
            Err(error) => return Err(error),
        }
    }
     /// Creates a new Kubernetes cluster with a specified configuration.
    ///
    /// # Examples
    /// ```
    /// let client = new_civo_client("API_KEY", "REGION");
    /// let cluster_config = KubernetesClusterConfig { ... };
    /// let cluster = client.new_kubernetes_cluster(cluster_config).await;
    /// ```
    ///
    /// # Arguments
    ///
    /// * `kc` - The `KubernetesClusterConfig` struct containing the configuration for the new cluster.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `KubernetesCluster` struct on success, or an `HTTPError` on failure.
    pub async fn new_kubernetes_cluster(&self, mut kc: KubernetesClusterConfig) ->  Result<KubernetesCluster,HTTPError> {
        kc.region = self.region.clone();
        let cluster_enpoint = self.prepare_client_url("/v2/kubernetes/clusters");
        let resp = self.send_post_request(cluster_enpoint,kc).await;
        match resp {
            Ok(cluster) => return Ok(cluster.json::<KubernetesCluster>().await.unwrap()),
            Err(error) => return Err(error),

        }      
    }
    /// Creates a new simple Kubernetes cluster with a specified configuration.
    /// This function provides the bare minimum options to get a cluster running on civo and is sufficient for most use cases.
    ///
    /// # Examples
    /// ```
    /// let client = new_civo_client("API_KEY", "REGION");
    /// let cluster_config = SimpleClusterConfig { ... };
    /// let cluster = client.new_simple_kubernetes_cluster(cluster_config).await;
    /// ```
    ///
    /// # Arguments
    ///
    /// * `kc` - The `SimpleClusterConfig` struct containing the configuration for the new cluster.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `KubernetesCluster` struct on success, or an `HTTPError` on failure.
    pub async fn new_simple_kubernetes_cluster(&self, mut kc: SimpleClusterConfig) ->  Result<KubernetesCluster,HTTPError> {
        kc.region = self.region.clone();
        let cluster_enpoint = self.prepare_client_url("/v2/kubernetes/clusters");
        let resp = self.send_post_request(cluster_enpoint,kc).await;
        match resp {
            Ok(cluster) => return Ok(cluster.json::<KubernetesCluster>().await.unwrap()),
            Err(error) => return Err(error),

        }      
    }
    /// Deletes a Kubernetes cluster by ID.
    ///
    /// # Examples
    /// ```
    /// let client = new_civo_client("API_KEY", "REGION");
    /// let response = client.delete_kubernetes_cluster("CLUSTER_ID").await;
    /// ```
    ///
    /// # Arguments
    ///
    /// * `cluster_id` - The ID of the cluster to delete.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `SimpleResponse` struct on success, or an `HTTPError` on failure.
    pub async fn delete_kubernetes_cluster(&self,cluster_id:&str) -> Result<SimpleResponse,HTTPError> {
        let cluster_endpoint = self.prepare_client_url("/v2/kubernetes/cluster").join(&cluster_id);
        let resp = self.send_delete_request(cluster_endpoint.unwrap().as_str()).await;
        match resp {
            Ok(simplresp) => Ok(simplresp.json::<SimpleResponse>().await.unwrap()),
            Err(err) => Err(err),
        }
    }
    /// Retrieves a list of available Kubernetes versions.
    ///
    /// # Examples
    /// ```
    /// let client = new_civo_client("API_KEY", "REGION");
    /// let versions = client.get_kubernetes_versions().await;
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `KubernetesVersion` structs on success, or an `Error` on failure.
    pub async fn get_kubernetes_versions(&self) -> Result<Vec<KubernetesVersion>, Error> {
        let versions_endopoint = self.prepare_client_url("/v2/kubernetes/versions");
        let resp = self.send_get_request(versions_endopoint.as_str()).await;
        match resp {
            Ok(versions) => return Ok(versions.json::<Vec<KubernetesVersion>>().await?),
            Err(error) => return Err(error),
        }
    }

}
