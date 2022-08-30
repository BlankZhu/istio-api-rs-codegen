use crate::{constant, r#type, utility};
use log::{debug, error};
use std::fs;
use std::path;
use yaml_rust::{self, Yaml, YamlEmitter};

#[derive(Debug)]
pub struct Resolver {}

impl Resolver {
    pub fn new() -> Self {
        return Resolver {};
    }

    pub fn resolve(&self, istio_version: &str) -> r#type::Result<()> {
        let tmp_dir = path::Path::new(constant::ISTIO_CRD_TEMP_DIRECTORY);
        let versioned_tmp_dir =
            tmp_dir.join(utility::istio_version_to_directory_name(istio_version));
        let crd_yaml_path = versioned_tmp_dir.join(constant::ISTIO_CRD_FILENAME);

        let yamls = match self.split_yamls(crd_yaml_path.as_path()) {
            Ok(ys) => ys,
            Err(e) => {
                error!("failed to split yamls: {}", e);
                return Err(e);
            }
        };

        for yaml in yamls {
            let kind = match self.extract_kind(&yaml) {
                Some(k) => k,
                None => {
                    error!(
                        "failed to extract kind from yaml: {}",
                        crd_yaml_path.display()
                    );
                    continue;
                }
            };

            let api_versions = match self.extract_api_version(&yaml) {
                Some(av) => av,
                None => {
                    error!(
                        "failed to extract api version from yaml: {}",
                        crd_yaml_path.display()
                    );
                    continue;
                }
            };

            let istio_api_group = match self.extract_istio_api_group(&yaml) {
                Some(ag) => ag,
                None => {
                    error!(
                        "failed to extract api version from yaml: {}",
                        crd_yaml_path.display()
                    );
                    continue;
                }
            };

            for api_version in api_versions {
                debug!(
                    "saving yaml for {}/{}/{}/{}",
                    istio_version, istio_api_group, api_version, kind
                );
                if let Err(e) = self.save_to_resolved_path(
                    &yaml,
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

    fn split_yamls(&self, yaml_file_path: &path::Path) -> r#type::Result<Vec<Yaml>> {
        let content = match fs::read_to_string(yaml_file_path) {
            Ok(c) => c,
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        match yaml_rust::YamlLoader::load_from_str(&content.as_str()) {
            Ok(v) => return Ok(v),
            Err(e) => return Err(Box::new(e)),
        };
    }

    fn extract_api_version(&self, doc: &Yaml) -> Option<Vec<String>> {
        let mut ret = Vec::new();
        let versions = &doc["spec"]["versions"];
        if !versions.is_badvalue() {
            let vs = versions.clone();
            vs.into_iter().for_each(|obj| {
                let version_name = &obj["name"];
                if let Some(version_name) = version_name.as_str() {
                    ret.push(version_name.to_string())
                }
            })
        }
        return Some(ret);
    }

    fn extract_istio_api_group(&self, doc: &Yaml) -> Option<String> {
        let spec_group = &doc["spec"]["group"];
        if !spec_group.is_badvalue() {
            if let Some(group) = spec_group.as_str() {
                let ret = match group.strip_suffix(".istio.io") {
                    Some(g) => g.to_string(),
                    None => group.to_string(),
                };
                return Some(ret);
            }
        }
        return None;
    }

    fn extract_kind(&self, doc: &Yaml) -> Option<String> {
        let kind = &doc["spec"]["names"]["kind"];
        if !kind.is_badvalue() {
            if let Some(k) = kind.clone().into_string() {
                return Some(k);
            }
        }
        return None;
    }

    fn save_to_resolved_path(
        &self,
        yaml: &Yaml,
        istio_version: &str,
        istio_api_group: &str,
        api_version: &str,
        kind: &str,
    ) -> r#type::Result<()> {
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

        let mut output_str = String::new();
        let mut emitter = YamlEmitter::new(&mut output_str);
        if let Err(e) = emitter.dump(yaml) {
            return Err(Box::new(e));
        }
        if let Err(e) = fs::write(kind_path, output_str) {
            return Err(Box::new(e));
        };

        Ok(())
    }
}
