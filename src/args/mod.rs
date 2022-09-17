use clap::Parser;
use log::info;
use std::{fs, path::Path};
use thiserror::Error;

/// Command line options for istio-api-rs-codegen
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to Istio/api.
    #[clap(short, long, value_parser)]
    pub istio_api_dir_path: String,

    /// Output directory path for generated rust codes.
    #[clap(short, long, value_parser)]
    pub output_dir_path: String,

    #[clap(short, long, value_parser, default_value_t = false)]
    pub setup_openapi_directory: bool,

    #[clap(short, long, value_parser, default_value_t = false)]
    pub generate_rust_codes: bool,
}

impl Args {
    pub fn check_istio_api_dir_path(&self) -> anyhow::Result<()> {
        let istio_api_dir_path = Path::new(self.istio_api_dir_path.as_str());
        if !istio_api_dir_path.exists() {
            let err = ArgsError::IstioApiPathNotExists {
                istio_api_dir_path: self.istio_api_dir_path.clone(),
            };
            anyhow::bail!("{}", err);
        }
        if !istio_api_dir_path.is_dir() {
            let err = ArgsError::IstioApiPathNotDir {
                istio_api_dir_path: self.istio_api_dir_path.clone(),
            };
            anyhow::bail!("{}", err);
        }
        Ok(())
    }

    pub fn check_output_dir_path(&self) -> anyhow::Result<()> {
        let output_dir_path = Path::new(self.output_dir_path.as_str());
        if !output_dir_path.exists() {
            // create one
            info!("output dir `{}` not exists, making dir...", self.output_dir_path);
            fs::create_dir_all(output_dir_path)?;
            return Ok(());
        }
        if !output_dir_path.is_dir() {
            let err = ArgsError::OutputPathNotDir { output_dir_path: self.output_dir_path.clone() };
            anyhow::bail!("{}", err);
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ArgsError {
    #[error("istio/api dir not exists: {istio_api_dir_path:?}")]
    IstioApiPathNotExists { istio_api_dir_path: String },
    #[error("given istio api path is not a dir: {istio_api_dir_path:?}")]
    IstioApiPathNotDir { istio_api_dir_path: String },
    #[error("given output path is not a dir: {output_dir_path:?}")]
    OutputPathNotDir { output_dir_path: String },
}
