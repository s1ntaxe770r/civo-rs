use reqwest::Client;
use url::Url;

#[derive(Debug)]
pub struct CivoClient {
    pub api_key: String,
    pub base_url: Url,
    pub http_client: reqwest::Client,
    pub region: String,
}

fn new_civo_client(apikey: String, region: String) -> CivoClient {
    let http_c = Client::new();
    let u = Url::parse("https://api.civo.com/v2").unwrap();
    let c = CivoClient {
        base_url: u,
        api_key: apikey,
        http_client: http_c,
        region: region,
    };
    c
}
