use std::{
    fs,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::{constant::RUST_PRESERVED_KEYWORDS, util::snake_2_camel};

pub struct Tena {
    output_dir: PathBuf,
}

impl Tena {
    pub fn new(output_dir: PathBuf) -> Self {
        return Tena { output_dir };
    }

    pub fn setup_rust_structure_tree(&self) -> anyhow::Result<()> {
        self.setup_lib_rs()?;
        Ok(())
    }

    fn setup_lib_rs(&self) -> Result<(), TenaError> {
        // list all istio_versions
        let rd =
            fs::read_dir(self.output_dir.as_path()).map_err(|e| TenaError::LibRsSetupError {
                path: format!("{}", self.output_dir.display()),
                detail: format!("{}", e),
            })?;

        let mut entries = Vec::new();
        for entry in rd {
            let entry = entry.map_err(|e| TenaError::LibRsSetupError {
                path: format!("{}", self.output_dir.display()),
                detail: format!("{}", e),
            })?;
            entries.push(entry);
        }
        entries.sort_by(|lhs, rhs| lhs.file_name().cmp(&rhs.file_name()));

        let mut lib_rs_content = String::new();
        for entry in entries {
            // setup lib.rs content
            let istio_api_version: String = entry.file_name().to_string_lossy().into();
            let versioned_content = format!(
                "#[cfg(feature = \"{}\")] mod {};\n#[cfg(feature = \"{}\")] pub use self::{}::*;\n\n",
                istio_api_version, istio_api_version, istio_api_version, istio_api_version
            );
            lib_rs_content += &versioned_content;

            if entry.path().is_dir() {
                // deep dive into mod.rs tree
                self.setup_mod_rs(entry.path().as_path())?;
            }
        }
        let lib_rs_content = lib_rs_content.trim();

        // write lib_rs_content to ./lib.rs
        let lib_rs_path = self.output_dir.join("lib.rs");
        fs::write(lib_rs_path.as_path(), lib_rs_content).map_err(|e| {
            TenaError::LibRsSetupError {
                path: format!("{}", lib_rs_path.display()),
                detail: format!("{}", e),
            }
        })?;

        Ok(())
    }

    fn setup_mod_rs(&self, curr_dir_path: &Path) -> Result<(), TenaError> {
        let rd = fs::read_dir(curr_dir_path).map_err(|e| TenaError::LibRsSetupError {
            path: format!("{}", curr_dir_path.display()),
            detail: format!("{}", e),
        })?;

        let mut mod_rs_content = String::new();
        let mut entries = Vec::new();
        for entry in rd {
            let entry = entry.map_err(|e| TenaError::LibRsSetupError {
                path: format!("{}", curr_dir_path.display()),
                detail: format!("{}", e),
            })?;
            entries.push(entry);
        }
        entries.sort_by(|lhs, rhs| lhs.file_name().cmp(&rhs.file_name()));

        let is_final = entries
            .iter()
            .map(|entry| entry.path().is_file())
            .fold(true, |acc, curr| acc && curr);

        for entry in entries.iter() {
            // setup mod.rs content
            let mut mod_name: String = entry.file_name().to_string_lossy().into();
            if entry.path().is_file() {
                mod_name = entry.path().file_stem().unwrap().to_string_lossy().into();
                if mod_name == "mod" {
                    continue;
                }
            }
            if entry.path().is_dir() {
                self.setup_mod_rs(entry.path().as_path())?;
            }

            let mod_content = self.gen_mod_rs_content_line(is_final, mod_name.as_str());
            mod_rs_content += &mod_content;
        }
        let mod_rs_content = mod_rs_content.trim();

        let mod_rs_path = curr_dir_path.join("mod.rs");
        fs::write(mod_rs_path.as_path(), mod_rs_content).map_err(|e| {
            TenaError::ModRsSetupError {
                path: format!("{}", mod_rs_path.display()),
                detail: format!("{}", e),
            }
        })?;

        Ok(())
    }

    fn gen_mod_rs_content_line(&self, is_final: bool, mod_name: &str) -> String {
        if is_final {
            if RUST_PRESERVED_KEYWORDS.contains(mod_name) {
                return format!(
                    "pub mod r#{};\npub use self::r#{}::{};\n",
                    mod_name,
                    mod_name,
                    snake_2_camel(mod_name)
                );
            } else {
                return format!(
                    "pub mod {};\npub use self::{}::{};\n",
                    mod_name,
                    mod_name,
                    snake_2_camel(mod_name)
                );
            }
        } else {
            if RUST_PRESERVED_KEYWORDS.contains(mod_name) {
                return format!("pub mod r#{};\n", mod_name);
            } else {
                return format!("pub mod {};\n", mod_name);
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum TenaError {
    #[error("cannot setup lib.rs at `{path:?}` : `{detail:?}`")]
    LibRsSetupError { path: String, detail: String },
    #[error("cannot setup mod.rs at `{path:?}` : `{detail:?}`")]
    ModRsSetupError { path: String, detail: String },
}
