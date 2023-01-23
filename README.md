# civo-rs
Unofficial rust client for the civo api

## Why ? 
Because Civo's go client wasn't fast enough( Joking obviously ) 😅. This is purely for educational purposes as i am trying to get better at rust ,this also means that a large part of this code is bad and more expreienced rust devs would be disgusted by it. See something that isn't right? Open a PR as i mentioned earlier i am doing this to improve and would appreciate any feedback and or help i can get. 

> Fun fact: this originally started out as a client to interact with Civo objectstores.

## project goals 

Right now there aren't many i would like to mimic as much civo's go client as possible, however i am focused on some of the core functionalities right now 

- [x] managing ssh keys
- [x] creating compute instances 
- [x] creating kubernetes clusters
- [x] managing object stores 
- [ ] support for civo databases

## How do i use this thing
for more advanced use cases checkout the rust docs [here]()
```rust 
use civo_rs::{client::new_civo_client, instance};
use civo_rs::kubernetes::{SimpleClusterConfig};
use civo_rs::utils::random_name;


async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = String::from("SOME-REALLY-REAL-API-KEY");
    let region = String::from("LON1");
    let new_cc = new_civo_client(api_key, region);
 let skc = SimpleClusterConfig {
        name: random_name(),
        region: "LON1".to_string(),
        network_id: "f89250e9-da9a-401b-blah-blah-blah".to_string(),
        pools: vec![KubernetesPoolConfig {
            id: random_name(),
            count: 1,
            size: "g4s.kube.medium".to_string(),
        }],
        firewall_rule: "443".to_string(),
    };
    let cluster = new_cc.new_simple_kubernetes_cluster(skc).await;
    println!("{:?}", cluster);
  Ok(())
}
```

```rust
use civo_rs::{client::new_civo_client,};

async fn main() -> Result<(), Box<dyn std::error::Error>> {
 let api_key = String::from("SOME-REALLY-REAL-API-KEY");
 let region = String::from("LON1");
 let new_cc = new_civo_client(api_key, region);
 let default_network = new_cc.get_default_network().await.unwrap();
  match default_network {
        Ok(network) => println!("Default network: {}", network.Name),
        Err(error) => println!("Error: {}", error.message),
    }

    let mut config  =  new_cc.new_instance_config().await.unwrap();
    config.hostname = "civo-rs".to_string();

    let instance  =  new_cc.create_instance(config).await;
    match instance {
        Ok(instance) => println!("{:?}",serde_json::to_string(&instance)),
        Err(error) => println!("Error: {}",&error),
    }
    println!("{}",serde_json::to_string(&config).unwrap());
    let deleted_instance = new_cc.delete_instance("efd7c5ca-516a-494d-b8ac-8eabbc215fef");
    println!("{}",deleted_instance.await.unwrap().result);
    Ok(())
}
```




