use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use log::debug;
use serde::Deserialize;
use serde_json::{json, Deserializer, Value};
use thiserror::Error;

use crate::constant::{self, IstioApiVersionInfo};
use crate::util::{dot_2_underscore, extract_major_and_minor};
pub struct Cuge {
    istio_api_path: PathBuf,
    codegen_working_directory: PathBuf,
}

impl Cuge {
    pub fn new(codegen_working_directory: PathBuf, istio_api_path: PathBuf) -> Self {
        Cuge {
            istio_api_path,
            codegen_working_directory,
        }
    }

    pub fn cue_gen(&self) -> anyhow::Result<()> {
        let cue_config_file_path = Path::new(constant::CUE_CONFIG_FILE_NAME);
        if !cue_config_file_path.exists() || !cue_config_file_path.is_file() {
            let err = CugeError::CueConfigNotExistError {
                cue_config_filename: constant::CUE_CONFIG_FILE_NAME.to_string(),
                istio_api_dir_path: self.istio_api_path.display().to_string(),
            };
            anyhow::bail!("{}", err);
        }

        let include_path = "-paths=common-protos".to_string();
        let cue_config_file = "-f=".to_string() + constant::CUE_CONFIG_FILE_NAME;

        let output = match Command::new(constant::CUE_GEN_COMMAND)
            .args([include_path.as_str(), cue_config_file.as_str()])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                let err = CugeError::CommandError {
                    detail: format!("{}", e),
                };
                anyhow::bail!("{}", err);
            }
        };

        if !output.status.success() {
            let detail = String::from_utf8_lossy(&output.stderr);
            let err = CugeError::CueGenError {
                detail: detail.into(),
            };
            anyhow::bail!("{}", err);
        }

        Ok(())
    }

    pub fn change_to_istio_api_dir(&self) -> Result<(), CugeError> {
        self.change_working_directory(&self.istio_api_path)
    }

    pub fn change_to_codegen_dir(&self) -> Result<(), CugeError> {
        self.change_working_directory(&self.codegen_working_directory)
    }

    fn change_working_directory(&self, dir: &Path) -> Result<(), CugeError> {
        env::set_current_dir(dir).map_err(|e| CugeError::ChangeDirectoryError {
            dir: dir.display().to_string(),
            detail: format!("{}", e),
        })
    }

    pub fn extract_openapi_to_codegen_dir(
        &self,
        istio_api_version_info: &IstioApiVersionInfo,
    ) -> anyhow::Result<()> {
        for file in istio_api_version_info.target_openapi_file {
            self.copy_gen_json(istio_api_version_info.version, &file)?;
        }
        Ok(())
    }

    fn copy_gen_json(&self, istio_version: &str, target_file: &str) -> anyhow::Result<()> {
        // then copy them to {codegen_working_directory}/{openapi_json_dir}/{istio_version}/{target_directory}/*.gen.json
        let target_file_path = self.istio_api_path.join(target_file);
        if !target_file_path.exists() {
            panic!("Path `{}` not exists! You may be using incorrect istio/api version info! Check the codegen's codes!", target_file_path.display())
        }
        let istio_version_section =
            dot_2_underscore(extract_major_and_minor(istio_version).as_str());

        
        let mut openapi_json_file_path = self
            .codegen_working_directory
            .join(constant::OPENAPI_JSON_DIR)
            .join(istio_version_section);
        
        if let Some(new_target_file) = constant::ISTIO_SPECIAL_OPENAPI_TARGET_FILE.get(target_file) {
            openapi_json_file_path = openapi_json_file_path.join(new_target_file);
        } else {
            openapi_json_file_path = openapi_json_file_path.join(target_file)
        }

        let mut openapi_json_dir = openapi_json_file_path.clone();
        openapi_json_dir.pop();
        if !openapi_json_dir.exists() {
            fs::create_dir_all(openapi_json_dir.as_path())?;
        }

        debug!(
            "copying from `{}` to `{}`",
            target_file_path.display(),
            openapi_json_file_path.display()
        );
        fs::copy(target_file_path.as_path(), openapi_json_file_path.as_path())?;

        debug!(
            "adding `Path` field to `{}`",
            openapi_json_file_path.display()
        );
        self.add_path_field_to_openapi_json(openapi_json_file_path.as_path())?;

        Ok(())
    }

    fn add_path_field_to_openapi_json(
        &self,
        openapi_json_file_path: &Path,
    ) -> Result<(), CugeError> {
        let content = match fs::read_to_string(openapi_json_file_path) {
            Ok(c) => c,
            Err(e) => {
                let err = CugeError::AddPathFieldError {
                    path: openapi_json_file_path.to_string_lossy().into(),
                    detail: format!("{}", e),
                };
                return Err(err);
            }
        };
        let mut de = Deserializer::from_str(content.as_str());
        let mut json_value =
            Value::deserialize(&mut de).map_err(|e| CugeError::ModifyOpenApiJsonError {
                path: openapi_json_file_path.to_string_lossy().into(),
                detail: format!("{}", e),
            })?;

        let key = String::from("paths");
        let value = json!({});

        if let Some(obj) = json_value.as_object_mut() {
            obj.insert(key, value);
            let new_content = serde_json::to_string_pretty(obj).map_err(|e| {
                CugeError::ModifyOpenApiJsonError {
                    path: openapi_json_file_path.to_string_lossy().into(),
                    detail: format!("{}", e),
                }
            })?;
            fs::write(openapi_json_file_path, new_content).map_err(|e| {
                CugeError::ModifyOpenApiJsonError {
                    path: openapi_json_file_path.to_string_lossy().into(),
                    detail: format!("{}", e),
                }
            })?;
        } else {
            return Err(CugeError::ModifyOpenApiJsonError {
                path: openapi_json_file_path.to_string_lossy().into(),
                detail: format!("cannot read JSON object, check content"),
            });
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum CugeError {
    #[error("cannot change working directory to `{dir:?}` : {detail:?}")]
    ChangeDirectoryError { dir: String, detail: String },
    #[error("cue-gen command execution failed: {detail:?}")]
    CommandError { detail: String },
    #[error("cue-gen failed: {detail:?}")]
    CueGenError { detail: String },
    #[error("cannot find file `{cue_config_filename:?}` at `{istio_api_dir_path:?}`")]
    CueConfigNotExistError {
        cue_config_filename: String,
        istio_api_dir_path: String,
    },
    #[error("cannot add field to `{path:?}` : {detail:?}")]
    AddPathFieldError { path: String, detail: String },
    #[error("cannot modify OpenAPI JSON on `{path:?}` : {detail:?}")]
    ModifyOpenApiJsonError { path: String, detail: String },
}
