use std::{
    fs,
    path::{Path, PathBuf},
};

use log::{debug, info};
use thiserror::Error;

use crate::util::opai::{Opai, OpaiInfo};

pub struct Adva {
    output_dir: PathBuf,
    openapi_dir: PathBuf,
}

impl Adva {
    pub fn new(openapi_dir: PathBuf, output_dir: PathBuf) -> Self {
        Adva {
            output_dir,
            openapi_dir,
        }
    }

    pub fn adjust(&self) -> anyhow::Result<()> {
        let opai = Opai::new(self.openapi_dir.clone());
        let infos = opai.parse_opai_infos()?;

        for info in infos {
            info!("adjusting {} ...", info);
            self.process_resource_dir(&info)?;
        }
        Ok(())
    }

    fn process_resource_dir(&self, info: &OpaiInfo) -> Result<(), AdvaError> {
        let resource_dir_path = Path::new(self.output_dir.as_path())
            .join(info.istio_version.clone())
            .join(info.api_group.clone())
            .join(info.api_version.clone())
            .join(info.resource.clone());

        self.prune_resource_dir(resource_dir_path.as_path())?;
        self.refactor_resource_dir(resource_dir_path.as_path())?;
        self.rename_resource_dir_files(resource_dir_path.as_path(), &info)?;
        Ok(())
    }

    fn prune_resource_dir(&self, resource_dir_path: &Path) -> Result<(), AdvaError> {
        let rd = fs::read_dir(resource_dir_path).map_err(|e| AdvaError::PruneError {
            path: format!("{}", resource_dir_path.display()),
            detail: format!("{}", e),
        })?;

        for entry in rd {
            let entry = entry.map_err(|e| AdvaError::PruneError {
                path: format!("{}", resource_dir_path.display()),
                detail: format!("{}", e),
            })?;

            let filename: String = entry.file_name().to_string_lossy().into();
            if filename == "src" {
                // remove `src/apis`
                let apis_path = entry.path().join("apis");
                if apis_path.exists() && apis_path.is_dir() {
                    fs::remove_dir_all(apis_path.as_path()).map_err(|e| AdvaError::PruneError {
                        path: format!("{}", apis_path.display()),
                        detail: format!("{}", e),
                    })?;
                }
                // remove `src/lib.rs`
                let lib_rs_path = entry.path().join("lib.rs");
                if lib_rs_path.exists() && lib_rs_path.is_file() {
                    fs::remove_file(lib_rs_path.as_path()).map_err(|e| AdvaError::PruneError {
                        path: format!("{}", lib_rs_path.display()),
                        detail: format!("{}", e),
                    })?;
                }
                continue;
            }

            if entry.path().is_dir() {
                fs::remove_dir_all(entry.path()).map_err(|e| AdvaError::PruneError {
                    path: format!("{}", entry.path().display()),
                    detail: format!("{}", e),
                })?;
            } else {
                fs::remove_file(entry.path()).map_err(|e| AdvaError::PruneError {
                    path: format!("{}", entry.path().display()),
                    detail: format!("{}", e),
                })?;
            }
        }

        Ok(())
    }

    fn refactor_resource_dir(&self, resource_dir_path: &Path) -> Result<(), AdvaError> {
        let src_dir_path = resource_dir_path.join("src");
        if src_dir_path.exists() && src_dir_path.is_dir() {
            let src_models_dir_path = src_dir_path.join("models");
            if src_models_dir_path.exists() && src_models_dir_path.is_dir() {
                let rd = fs::read_dir(src_models_dir_path.as_path()).map_err(|e| {
                    AdvaError::RefactorError {
                        path: format!("{}", src_models_dir_path.display()),
                        detail: format!("{}", e),
                    }
                })?;

                for entry in rd {
                    let entry = entry.map_err(|e| AdvaError::RefactorError {
                        path: format!("{}", src_models_dir_path.display()),
                        detail: format!("{}", e),
                    })?;

                    let move_to = resource_dir_path.join(entry.file_name());
                    fs::rename(entry.path(), move_to).map_err(|e| AdvaError::RefactorError {
                        path: format!("{}", entry.path().display()),
                        detail: format!("{}", e),
                    })?;
                }
            }
            fs::remove_dir_all(src_dir_path.as_path()).map_err(|e| AdvaError::RefactorError {
                path: format!("{}", src_dir_path.display()),
                detail: format!("{}", e),
            })?;
        }

        Ok(())
    }

    fn rename_resource_dir_files(
        &self,
        resource_dir_path: &Path,
        info: &OpaiInfo,
    ) -> Result<(), AdvaError> {
        let filename_prefix = format!("istio_{}_{}_", info.api_group, info.api_version);
        let rd = fs::read_dir(resource_dir_path).map_err(|e| AdvaError::RenameError {
            path: format!("{}", resource_dir_path.display()),
            detail: format!("{}", e),
        })?;
        for entry in rd {
            let entry = entry.map_err(|e| AdvaError::RenameError {
                path: format!("{}", resource_dir_path.display()),
                detail: format!("{}", e),
            })?;

            if entry.path().is_file() {
                let filename: String = entry.file_name().to_string_lossy().into();

                // extra check for `istio_type_v1beta1_workload_selector.rs`
                if filename == "istio_type_v1beta1_workload_selector.rs" {
                    if info.api_group != "type" {
                        fs::remove_file(entry.path()).map_err(|e| AdvaError::RenameError {
                            path: format!("{}", entry.path().display()),
                            detail: format!("{}", e),
                        })?;
                        continue;
                    }
                }

                if filename.starts_with(&filename_prefix) {
                    let new_filename = &filename[filename_prefix.len()..].to_string();
                    let new_path = resource_dir_path.join(new_filename);
                    debug!(
                        "moving {} to {} ...",
                        entry.path().display(),
                        new_path.display()
                    );
                    fs::rename(entry.path(), new_path).map_err(|e| AdvaError::RenameError {
                        path: format!("{}", entry.path().display()),
                        detail: format!("{}", e),
                    })?;
                }
            }
        }

        Ok(())
    }

    fn modify_codes(&self, resource_dir_path: &Path, info: &OpaiInfo) -> Result<(), AdvaError> {
        todo!()
    }

    fn modify_mod_rs(&self, mod_rs_path: &Path, info: &OpaiInfo) -> Result<(), AdvaError> {
        todo!()
    }

    fn modify_compoment_rs(&self, component_rs_path: &Path, info: &OpaiInfo) -> Result<(), AdvaError> {
        todo!()
    }

    fn modify_spec_rs(&self, spec_rs_path: &Path, info: &OpaiInfo) -> Result<(), AdvaError> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum AdvaError {
    #[error("cannot prune output directory `{path:?}` : {detail:?}")]
    PruneError { path: String, detail: String },
    #[error("cannot refactor output resource directory `{path:?}` : {detail:?}")]
    RefactorError { path: String, detail: String },
    #[error("cannot rename output rust codes in directory `{path:?}` : {detail:?}")]
    RenameError { path: String, detail: String },
    #[error("cannot modify rust codefile content at `{path:?}` : {detail:?}")]
    ModifyError { path: String, detail: String },
}
