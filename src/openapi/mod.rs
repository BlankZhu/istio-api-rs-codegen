use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use log::{debug, info};
use thiserror::Error;

use crate::constant;

pub struct Opai {
    openapi_path: PathBuf,
    openapi_generator_cli_jar_path: PathBuf,
    output_path: PathBuf,
}

impl Opai {
    pub fn new(
        openapi_path: PathBuf,
        openapi_generator_cli_jar_path: PathBuf,
        output_path: PathBuf,
    ) -> Self {
        Opai {
            openapi_path,
            openapi_generator_cli_jar_path,
            output_path,
        }
    }

    pub fn openapi_generate(&self) -> anyhow::Result<()> {
        let ret = self.parse_opai_infos()?;

        for info in ret {
            info!(
                "generating rust code for {}/{}/{}/{} ...",
                info.istio_version, info.api_group, info.api_version, info.resource
            );
            self.openapi_generate_cli_gen(&info)?;
        }

        Ok(())
    }

    fn parse_opai_infos(&self) -> Result<Vec<OpaiInfo>, OpaiError> {
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

    fn openapi_generate_cli_gen(&self, info: &OpaiInfo) -> Result<(), OpaiError> {
        let openapi_json_file_path = self
            .openapi_path
            .join(info.istio_version.clone())
            .join(info.api_group.clone())
            .join(info.api_version.clone())
            .join(info.resource.clone() + ".gen.json");
        let output_rust_code_dir_path = self
            .output_path
            .join(info.istio_version.clone())
            .join(info.api_group.clone())
            .join(info.api_version.clone())
            .join(info.resource.clone());   // need futher pruning

        let jar_arg: String = self.openapi_generator_cli_jar_path.to_string_lossy().into();
        let output_arg: String = output_rust_code_dir_path.to_string_lossy().into();
        let input_arg: String = openapi_json_file_path.to_string_lossy().into();
        let additional_arg: String =
            "--additional-properties=packageName=".to_string() + &info.resource;

        if !output_rust_code_dir_path.exists() {
            if let Err(e) = fs::create_dir_all(output_rust_code_dir_path.as_path()) {
                let err = OpaiError::OutputPathCreateError {
                    path: output_rust_code_dir_path.to_string_lossy().into(),
                    detail: format!("{}", e),
                };
                return Err(err);
            }
        }

        let output = match Command::new(constant::JAVA_COMMAND)
            .args([
                "-jar",
                jar_arg.as_str(),
                "generate",
                "-g",
                "rust",
                "-i",
                input_arg.as_str(),
                "-o",
                output_arg.as_str(),
                additional_arg.as_str(),
            ])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                let err = OpaiError::CommandError {
                    detail: format!("{}", e),
                };
                return Err(err);
            }
        };

        if !output.status.success() {
            let detail = String::from_utf8_lossy(&output.stderr);
            return Err(OpaiError::CodegenError {
                detail: detail.into(),
            });
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum OpaiError {
    #[error("openapi-generator execution failed: {detail:?}")]
    CommandError { detail: String },
    #[error("openapi-generator codegen failed: {detail:?}")]
    CodegenError { detail: String },
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
    #[error("cannot create output path dirs `{path:?}` for generated rust code: `{detail:?}`")]
    OutputPathCreateError { detail: String, path: String },
}

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
