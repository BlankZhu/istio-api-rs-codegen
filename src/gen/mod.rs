use std::path::{PathBuf, Path};
use std::env;

pub struct Geno {
    istio_api_path: PathBuf,
    codegen_working_directory: PathBuf,
}

impl Geno {
    pub fn new(codegen_working_directory: PathBuf, istio_api_path: PathBuf) -> Self {
        return Geno { istio_api_path, codegen_working_directory };
    }

    pub fn generate(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn change_working_directory(dir: &Path) -> anyhow::Result<()> {
        env::set_current_dir(dir)?;
        Ok(())
    }
}
