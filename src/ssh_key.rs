// use crate::client::CivoClient;
// use reqwest::{Error,};
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ssh_key {
//     pub id: String,
//     pub name: String,
//     pub fingerprint: String,
//     pub public_key: String,
//     // String for now because i don't know the rust equivalent of time.Time
//     pub created_at: String,
// }

// type SshKeys = Vec<ssh_key>;

// impl CivoClient {
//     pub async fn list_ssh_keys(&self) -> Result<Vec<SshKeys>,Error> {
//         let ssh_endpoint = self.prepare_client_url("/v2/sshkeys");
//         let req = self
//             .http_client
//             .get(ssh_endpoint)
//             .bearer_auth(&self.api_key)
//             .header("Accept", "Application/json")
//             .header("Content-Type", "application/json")
//             .query(&[("region", &self.region)])
//             .send()
//             .await?;
//         if !req.status().is_success() {
//             Err(Error::from(reqwest::StatusCodeError::new(response.status())));
//         }
//             let keys = req.json::<SshKeys>().await?;
//         Ok(())
//     }
// }
