use log::{info, error};
use std::time;
use crate::constant;

#[derive(Debug)]
pub struct Fetcher {
    client: Option<reqwest::Client>,
}

impl Fetcher {
    pub fn new() -> Self {
        return Fetcher { client: None };
    }

    pub fn initialize(&mut self) {
        let client = reqwest::Client::builder()
            .timeout(time::Duration::from_secs(10))
            .build();
        match client {
            Ok(c) => self.client = Some(c),
            Err(e) => panic!("failed to intialize reqwest client: {}", e),
        }
    }

    pub async fn fetch(&self, istio_version: &str) -> Option<String> {
        let url = [
            constant::ISTIO_CRD_ALL_URL_PREFIX,
            istio_version,
            constant::ISTIO_CRD_ALL_URL_SUFFIX,
        ]
        .join("");

        info!("fetching all-in-one yaml from {}", url.as_str());

        let resp = self.client.as_ref()?.get(url).send().await;
        match resp {
            Ok(resp) => {
                let text_body = resp.text().await;
                match text_body {
                    Ok(text) => return Some(text),
                    Err(e) => {
                        error!("failed to read body: {}", e);
                        return None;
                    }
                }
            },
            Err(e) => {
                error!("failed to fetch yaml: {}", e);
                return None;
            }
        }
    }

    // fn start_fetching(&self) {
    //     constant::ISTIO_VERSIONS.iter().for_each(|item| {
    //         let url = constant::ISTIO_CRD_ALL_URL_PREFIX.to_string() + &item.to_string() + &constant::ISTIO_CRD_ALL_URL_SUFFIX.to_string();
    //         println!("fetching url: {}", url)
    //     })
    // }
}
