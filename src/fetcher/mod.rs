use crate::constant;
use crate::r#type;
use crate::utility;
use log::{error, info};
use std::fs;
use std::path;
use std::time;

#[derive(Debug)]
pub struct Fetcher {
    client: reqwest::Client,
}

impl Fetcher {
    pub fn new() -> Self {
        return Fetcher {
            client: reqwest::Client::builder()
                .timeout(time::Duration::from_secs(10))
                .build()
                .unwrap(),
        };
    }

    pub async fn fetch(&self, istio_version: &str) -> r#type::Result<()> {
        if let Some(crd_yaml) = self.get_crd_yaml(istio_version).await {
            if let Err(e) = self.save_to_tmp_dir(istio_version, crd_yaml) {
                return Err(e);
            }
        }

        Ok(())
    }

    async fn get_crd_yaml(&self, istio_version: &str) -> Option<String> {
        let url = [
            constant::ISTIO_CRD_ALL_URL_PREFIX,
            istio_version,
            constant::ISTIO_CRD_ALL_URL_SUFFIX,
        ]
        .join("");

        info!("fetching all-in-one yaml from {}", url.as_str());
        let resp = self.client.get(url).send().await;
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
            }
            Err(e) => {
                error!("failed to fetch yaml: {}", e);
                return None;
            }
        }
    }

    fn save_to_tmp_dir(&self, istio_version: &str, content: String) -> r#type::Result<()> {
        let tmp_dir = path::Path::new(constant::ISTIO_CRD_TEMP_DIRECTORY);
        let save_dir = tmp_dir.join(utility::istio_version_to_directory_name(istio_version));
        let save_dir = save_dir.as_path();

        if !save_dir.exists() {
            if let Err(e) = fs::create_dir_all(save_dir) {
                return Err(Box::new(e));
            }
        }

        let save_path = save_dir.join(constant::ISTIO_CRD_FILENAME);
        let save_path = save_path.as_path();
        if let Err(e) = fs::write(save_path, content) {
            return Err(Box::new(e));
        };

        Ok(())
    }
}
