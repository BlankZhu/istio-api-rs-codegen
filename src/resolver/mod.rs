use crate::{constant, utility};
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;
use log::warn;
use log::{debug, error};
use serde::Deserialize;
use std::fs;
use std::path;

#[derive(Debug)]
pub struct Resolver {}

impl Resolver {
    pub fn new() -> Self {
        return Resolver {};
    }

    pub fn resolve(&self, istio_version: &str) -> utility::Result<()> {
        let tmp_dir = path::Path::new(constant::ISTIO_CRD_TEMP_DIRECTORY);
        let versioned_tmp_dir =
            tmp_dir.join(utility::istio_version_to_directory_name(istio_version));
        let crd_yaml_path = versioned_tmp_dir.join(constant::ISTIO_CRD_FILENAME);

        let crds = match self.split_yamls(crd_yaml_path.as_path()) {
            Ok(crds) => crds,
            Err(e) => {
                error!("failed to split yamls: {}", e);
                return Err(e);
            }
        };

        for mut crd in crds {
            // prune spec.versions[].subresources if spec.versions[].subresources.status has no keys
            // 'cause kopium will genereate CRD Spec without Status field
            self.prune_empty_status(&mut crd);

            let kind = self.extract_kind(&crd);
            let api_versions = self.extract_api_version(&crd);
            let istio_api_group = self.extract_istio_api_group(&crd);

            for api_version in api_versions {
                debug!(
                    "saving yaml for {}/{}/{}/{}",
                    istio_version, istio_api_group, api_version, kind
                );
                if let Err(e) = self.save_to_resolved_path(
                    &crd,
                    istio_version,
                    istio_api_group.as_str(),
                    api_version.as_str(),
                    kind.as_str(),
                ) {
                    error!(
                        "failed to save yaml for {}/{}/{}/{} : {}",
                        istio_version, istio_api_group, api_version, kind, e
                    );
                    continue;
                }
            }
        }

        Ok(())
    }

    fn split_yamls(
        &self,
        yaml_file_path: &path::Path,
    ) -> utility::Result<Vec<CustomResourceDefinition>> {
        let content = match fs::read_to_string(yaml_file_path) {
            Ok(c) => c,
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        let docs = serde_yaml::Deserializer::from_str(content.as_str());
        let mut ret = Vec::new();

        for doc in docs {
            match serde_yaml::Value::deserialize(doc) {
                Ok(raw_value) => {
                    match serde_yaml::from_value::<CustomResourceDefinition>(raw_value) {
                        Ok(crd) => ret.push(crd),
                        Err(e) => warn!("failed to convert yaml doc to CRD: {}", e),
                    }
                }
                Err(e) => warn!("failed to deserialize yaml doc: {}", e),
            }
        }

        Ok(ret)
    }

    fn extract_api_version(&self, crd: &CustomResourceDefinition) -> Vec<String> {
        let mut ret = Vec::new();
        for version in &crd.spec.versions {
            ret.push(version.name.clone());
        }
        ret
    }

    fn extract_istio_api_group(&self, crd: &CustomResourceDefinition) -> String {
        match crd.spec.group.strip_suffix(".istio.io") {
            Some(group) => group.to_string(),
            None => crd.spec.group.clone(),
        }
    }

    fn extract_kind(&self, crd: &CustomResourceDefinition) -> String {
        crd.spec.names.kind.clone()
    }

    fn save_to_resolved_path(
        &self,
        crd: &CustomResourceDefinition,
        istio_version: &str,
        istio_api_group: &str,
        api_version: &str,
        kind: &str,
    ) -> utility::Result<()> {
        let tmp_dir_path = path::Path::new(constant::ISTIO_CRD_TEMP_DIRECTORY);
        let versioned_dir_path =
            tmp_dir_path.join(utility::istio_version_to_directory_name(istio_version));
        let api_group_dir_path = versioned_dir_path.join(istio_api_group);
        let api_version_dir_path = api_group_dir_path.join(api_version);
        let filename = kind.to_string() + ".yaml";
        let kind_path = api_version_dir_path.join(filename);

        if !api_version_dir_path.exists() {
            if let Err(e) = fs::create_dir_all(api_version_dir_path) {
                return Err(Box::new(e));
            }
        }

        let output_str = serde_yaml::to_string(crd)?;
        if let Err(e) = fs::write(kind_path, output_str) {
            return Err(Box::new(e));
        };

        Ok(())
    }

    fn prune_empty_status(&self, crd: &mut CustomResourceDefinition) {
        crd.spec.versions.iter_mut().for_each(|version| {
            if let Some(subresources) = &version.subresources {
                if let Some(status) = &subresources.status {
                    if let Some(obj) = status.0.as_object() {
                        if obj.len() == 0 && subresources.scale.is_none() {
                            version.subresources = None;
                        }
                    }
                }
            }
        })
    }
}
