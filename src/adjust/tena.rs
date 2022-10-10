use std::{fs, path::PathBuf};

use log::{debug, info};
use thiserror::Error;

pub struct Tena {
    output_dir: PathBuf,
}

impl Tena {
    pub fn new(output_dir: PathBuf) -> Self {
        return Tena { output_dir };
    }

    pub fn setup_rust_structure_tree(&self) {
        todo!()
    }

    pub fn setup_lib_rs(&self) -> Result<(), TenaError> {
        // list all istio_versions
        let rd =
            fs::read_dir(self.output_dir.as_path()).map_err(|e| TenaError::LibRsSetupError {
                path: format!("{}", self.output_dir.display()),
                detail: format!("{}", e),
            })?;

        for entry in rd {
            let entry = entry.map_err(|e| TenaError::LibRsSetupError {
                path: format!("{}", self.output_dir.display()),
                detail: format!("{}", e),
            })?;

            // setup lib.rs content

            // deep dive into mod.rs tree
        }

        todo!()
    }

    pub fn setup_mod_rs(&self) {
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum TenaError {
    #[error("cannot setup lib.rs at `{path:?}` : `{detail:?}`")]
    LibRsSetupError { path: String, detail: String },
    #[error("cannot setup mod.rs at `{path:?}` : `{detail:?}`")]
    ModRsSetupError { path: String, detail: String },
}
