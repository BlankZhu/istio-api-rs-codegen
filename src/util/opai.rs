use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

use log::debug;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct OpaiInfo {
    pub istio_version: String,
    pub api_group: String,
    pub api_version: String,
    pub resource: String,
}

impl OpaiInfo {
    pub fn new() -> Self {
        OpaiInfo {
            istio_version: "".to_string(),
            api_group: "".to_string(),
            api_version: "".to_string(),
            resource: "".to_string(),
        }
    }
}

impl Display for OpaiInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.istio_version, self.api_group, self.api_version, self.resource
        )
    }
}

pub struct Opai {
    openapi_path: PathBuf,
}

impl Opai {
    pub fn new(openapi_path: PathBuf) -> Self {
        return Opai { openapi_path };
    }

    pub fn parse_opai_infos(&self) -> Result<Vec<OpaiInfo>, OpaiError> {
        if !self.openapi_path.exists() || !self.openapi_path.is_dir() {
            let err_msg = format!("dir `{}` not found", self.openapi_path.display());
            let err = OpaiError::OpenApiDirNotFoundError { detail: err_msg };
            return Err(err);
        }

        self.parse_istio_versions()
    }

    fn parse_istio_versions(&self) -> Result<Vec<OpaiInfo>, OpaiError> {
        let rd = fs::read_dir(self.openapi_path.as_path()).map_err(|e| {
            OpaiError::NoIstioVersionFoundError {
                path: self.openapi_path.display().to_string(),
                detail: format!("{}", e),
            }
        })?;

        let mut ret = Vec::new();
        for entry in rd {
            let entry = entry.map_err(|e| OpaiError::NoIstioVersionFoundError {
                path: self.openapi_path.display().to_string(),
                detail: format!("{}", e),
            })?;

            if !entry.path().is_dir() {
                continue;
            }

            let mut info = OpaiInfo::new();
            info.istio_version = entry.file_name().to_string_lossy().into();
            let mut infos = self.parse_api_groups(entry.path().as_path(), info)?;
            ret.append(&mut infos);
        }
        ret.sort_by(|l, r| l.istio_version.cmp(&r.istio_version));
        Ok(ret)
    }

    fn parse_api_groups(
        &self,
        istio_version_path: &Path,
        base_info: OpaiInfo,
    ) -> Result<Vec<OpaiInfo>, OpaiError> {
        let rd = fs::read_dir(istio_version_path).map_err(|e| OpaiError::NoApiGroupFoundError {
            path: istio_version_path.display().to_string(),
            detail: format!("{}", e),
        })?;

        let mut ret = Vec::new();
        for entry in rd {
            let entry = entry.map_err(|e| OpaiError::NoApiGroupFoundError {
                path: istio_version_path.display().to_string(),
                detail: format!("{}", e),
            })?;

            if !entry.path().is_dir() {
                continue;
            }

            let mut info = base_info.clone();
            info.api_group = entry.file_name().to_string_lossy().into();
            let mut infos = self.parse_api_versions(entry.path().as_path(), info)?;
            ret.append(&mut infos);
        }
        ret.sort_by(|l, r| l.api_group.cmp(&r.api_group));
        Ok(ret)
    }

    fn parse_api_versions(
        &self,
        api_group_path: &Path,
        base_info: OpaiInfo,
    ) -> Result<Vec<OpaiInfo>, OpaiError> {
        let rd = fs::read_dir(api_group_path).map_err(|e| OpaiError::NoApiVersionFoundError {
            path: api_group_path.display().to_string(),
            detail: format!("{}", e),
        })?;

        let mut ret = Vec::new();
        for entry in rd {
            let entry = entry.map_err(|e| OpaiError::NoApiVersionFoundError {
                path: api_group_path.display().to_string(),
                detail: format!("{}", e),
            })?;

            if !entry.path().is_dir() {
                continue;
            }

            let mut info = base_info.clone();
            info.api_version = entry.file_name().to_string_lossy().into();
            let mut infos = self.parse_resources(entry.path().as_path(), info)?;
            ret.append(&mut infos);
        }
        ret.sort_by(|l, r| l.api_version.cmp(&r.api_group));
        Ok(ret)
    }

    fn parse_resources(
        &self,
        api_version_path: &Path,
        base_info: OpaiInfo,
    ) -> Result<Vec<OpaiInfo>, OpaiError> {
        let rd = fs::read_dir(api_version_path).map_err(|e| OpaiError::NoResourceFoundError {
            path: api_version_path.display().to_string(),
            detail: format!("{}", e),
        })?;

        let mut ret = Vec::new();
        for entry in rd {
            let entry = entry.map_err(|e| OpaiError::NoResourceFoundError {
                path: api_version_path.display().to_string(),
                detail: format!("{}", e),
            })?;

            if !entry.path().is_file() {
                continue;
            }

            let mut info = base_info.clone();
            info.resource = self
                .extract_resource_name(entry.file_name().to_string_lossy().to_string().as_str());
            debug!("{:?} added to OpenAPI JSONs list", info);
            ret.push(info);
        }

        if ret.is_empty() {
            let err = OpaiError::NoApiGroupFoundError {
                path: api_version_path.display().to_string(),
                detail: "no resource found, check the generation process!".to_string(),
            };
            return Err(err);
        }
        ret.sort_by(|l, r| l.resource.cmp(&r.resource));
        Ok(ret)
    }

    fn extract_resource_name(&self, filename: &str) -> String {
        if let Some(index) = filename.find(|c: char| c == '.') {
            return filename[0..index].to_string();
        }
        filename.to_string()
    }
}

#[derive(Error, Debug)]
pub enum OpaiError {
    #[error("no resource found at `{path:?}`: {detail:?}")]
    NoResourceFoundError { path: String, detail: String },
    #[error("no API version found at `{path:?}`: {detail:?}")]
    NoApiVersionFoundError { path: String, detail: String },
    #[error("no API group found at `{path:?}`: {detail:?}")]
    NoApiGroupFoundError { path: String, detail: String },
    #[error("no Istio version found at `{path:?}`: {detail:?}")]
    NoIstioVersionFoundError { path: String, detail: String },
    #[error("OpenAPI JSONs directory not found: `{detail:?}`")]
    OpenApiDirNotFoundError { detail: String },
}
