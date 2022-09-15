use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use log::debug;
use thiserror::Error;

use crate::constant::{self, IstioApiVersionInfo};
use crate::util::{dot_2_underscore, extract_major_and_minor};
pub struct Geno {
    istio_api_path: PathBuf,
    codegen_working_directory: PathBuf,
}

impl Geno {
    pub fn new(codegen_working_directory: PathBuf, istio_api_path: PathBuf) -> Self {
        Geno {
            istio_api_path,
            codegen_working_directory,
        }
    }

    pub fn cue_gen(&self) -> anyhow::Result<()> {
        let cue_config_file_path = Path::new(constant::CUE_CONFIG_FILE_NAME);
        if !cue_config_file_path.exists() || !cue_config_file_path.is_file() {
            let err = GenoError::CueConfigNotExistError {
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
                let err = GenoError::CommandError {
                    detail: format!("{}", e),
                };
                anyhow::bail!("{}", err);
            }
        };

        if !output.status.success() {
            let detail = String::from_utf8_lossy(&output.stderr);
            let err = GenoError::CueGenError {
                detail: detail.into(),
            };
            anyhow::bail!("{}", err);
        }

        Ok(())
    }

    pub fn change_to_istio_api_dir(&self) -> Result<(), GenoError> {
        self.change_working_directory(&self.istio_api_path)
    }

    pub fn change_to_codegen_dir(&self) -> Result<(), GenoError> {
        self.change_working_directory(&self.codegen_working_directory)
    }

    fn change_working_directory(&self, dir: &Path) -> Result<(), GenoError> {
        env::set_current_dir(dir).map_err(|e| GenoError::ChangeDirectoryError {
            dir: dir.display().to_string(),
            detail: format!("{}", e),
        })
    }

    pub fn extract_openapi_to_codegen_dir(
        &self,
        istio_api_version_info: &IstioApiVersionInfo,
    ) -> anyhow::Result<()> {
        for directory in istio_api_version_info.target_directories {
            self.copy_gen_json(istio_api_version_info.version, directory)?;
        }
        Ok(())
    }

    fn copy_gen_json(&self, istio_version: &str, target_directory: &str) -> anyhow::Result<()> {
        // find '*.gen.json' in {istio_api_path}/{target_directory}
        // then copy them to {codegen_working_directory}/{openapi_json_dir}/{istio_version}/{target_directory}
        let target_dir_path = self.istio_api_path.join(target_directory);
        if !target_dir_path.exists() {
            panic!("Path `{}` not exists! You may be using incorrect istio/api version info! Check the codegen's codes!", target_dir_path.display())
        }
        let istio_version_section =
            dot_2_underscore(extract_major_and_minor(istio_version).as_str());
        let openapi_json_dir_path = self
            .codegen_working_directory
            .join(constant::OPENAPI_JSON_DIR)
            .join(istio_version_section)
            .join(target_directory);
        if !openapi_json_dir_path.exists() {
            fs::create_dir_all(&openapi_json_dir_path)?;
        }

        let read_dir = fs::read_dir(&target_dir_path)?;
        for dir_entry in read_dir {
            let dir_entry = dir_entry?;
            let filename: String = dir_entry.file_name().to_string_lossy().into();
            if filename.contains(".gen.json") {
                let copy_to = openapi_json_dir_path.join(&filename);
                debug!("copying from `{}` to `{}`", dir_entry.path().display(), copy_to.display());
                fs::copy(dir_entry.path(), copy_to)?;
            }
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum GenoError {
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
}
