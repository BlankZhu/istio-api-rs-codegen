use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::openapi::OpaiInfo;

use thiserror::Error;

pub struct Adva {
    output_dir: PathBuf,
}

impl Adva {
    pub fn new(output_dir: PathBuf) -> Self {
        Adva { output_dir }
    }

    pub fn adjust(&self, info: &OpaiInfo) -> anyhow::Result<()> {
        self.prune(info)?;
        Ok(())
    }

    fn prune(&self, info: &OpaiInfo) -> Result<(), AdvaError> {
        let resource_dir_path = Path::new(self.output_dir.as_path())
            .join(info.istio_version.clone())
            .join(info.api_group.clone())
            .join(info.api_version.clone())
            .join(info.resource.clone());

        let rd = fs::read_dir(resource_dir_path.as_path()).map_err(|e| AdvaError::PruneError {
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
}

#[derive(Error, Debug)]
pub enum AdvaError {
    #[error("cannot prune output directory `{path:?}` : {detail:?}")]
    PruneError { path: String, detail: String },
}
