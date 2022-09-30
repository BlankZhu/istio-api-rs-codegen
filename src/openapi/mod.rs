use std::{fs, path::PathBuf, process::Command};

use log::info;
use thiserror::Error;

use crate::{
    constant,
    util::opai::{Opai, OpaiInfo},
};

pub struct Ogen {
    openapi_path: PathBuf,
    openapi_generator_cli_jar_path: PathBuf,
    output_path: PathBuf,
}

impl Ogen {
    pub fn new(
        openapi_path: PathBuf,
        openapi_generator_cli_jar_path: PathBuf,
        output_path: PathBuf,
    ) -> Self {
        Ogen {
            openapi_path,
            openapi_generator_cli_jar_path,
            output_path,
        }
    }

    pub fn openapi_generate(&self) -> anyhow::Result<()> {
        let opai = Opai::new(self.openapi_path.clone());
        let ret = opai.parse_opai_infos()?;

        for info in ret {
            info!(
                "generating rust code for {}/{}/{}/{} ...",
                info.istio_version, info.api_group, info.api_version, info.resource
            );
            self.openapi_generate_cli_gen(&info)?;
        }

        Ok(())
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
            .join(info.resource.clone()); // need futher pruning

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
    #[error("cannot create output path dirs `{path:?}` for generated rust code: `{detail:?}`")]
    OutputPathCreateError { detail: String, path: String },
}
