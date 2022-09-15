use clap::Parser;
use log::{error, info};

use crate::{cue::Cutter, gen::Geno, git::Gitter};

pub mod adjust;
pub mod args;
pub mod constant;
pub mod cue;
pub mod gen;
pub mod git;
pub mod meta;
pub mod tree;
pub mod util;

fn main() {
    env_logger::init();
    let working_directory = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => panic!("failed to get current working directory: {}", e),
    };

    let args = args::Args::parse();
    info!("using clap args: {:?}", args);

    if let Err(e) = args.check_istio_api_dir_path() {
        error!("Error occurs while checking Istio API dir path: {}", e);
        std::process::exit(exitcode::DATAERR);
    }
    if let Err(e) = args.check_output_dir_path() {
        error!("Error occurs while check output dir path: {}", e);
        std::process::exit(exitcode::DATAERR);
    }

    let gitter = Gitter::new(std::path::Path::new(&args.istio_api_dir_path).to_path_buf());
    let cutter = Cutter::new(
        std::path::Path::new(&args.istio_api_dir_path).join(&constant::CUE_CONFIG_FILE_NAME),
    );
    let geno = Geno::new(
        working_directory,
        std::path::Path::new(&args.istio_api_dir_path).to_path_buf(),
    );

    for version_info in constant::ISTIO_API_VERSION_INFOS {
        info!("checking out tag {} ...", version_info.version);
        if let Err(e) = gitter.checkout_tag(version_info.version) {
            error!("failed to checkout tag, detail: {}", e);
            continue;
        }

        info!("modifying cue config ...");
        if let Err(e) = cutter.modify_cue_file() {
            error!("failed to modify cue config file, detail: {}", e);
            continue;
        }

        info!("generating new OpenAPI JSONs ...");
        if let Err(e) = geno.change_to_istio_api_dir() {
            error!("failed to change working directory to istio/api, detail: {}", e);
            if let Err(e) = geno.change_to_codegen_dir() {
                panic!("failed to recover working directory back to codegen's, detail: {}", e);
            }
            continue;
        }
        if let Err(e) = geno.cue_gen() {
            error!("failed to do CUE gen, detail: {}", e);
            if let Err(e) = geno.change_to_codegen_dir() {
                panic!("failed to recover working directory back to codegen's, detail: {}", e);
            }
            continue;
        }
        if let Err(e) = geno.change_to_codegen_dir() {
            panic!("failed to recover working directory back to codegen's, detail: {}", e);
        }

        info!("settting up openapi-json output directory ...");
        if let Err(e) = geno.extract_openapi_to_codegen_dir(version_info) {
            error!("failed to extract OpenAPI JSONs to codegen directory, detail: {}", e);
            continue;
        }

        info!("restoring istio/api on tag {} ...", version_info.version);
        if let Err(e) = gitter.restore() {
            error!("failed to restore istio/api, detail: {}", e);
            continue;
        }

        info!("cleaning(force) istio/api on tag {} ...", version_info.version);
        if let Err(e) = gitter.force_clean() {
            error!("failed to clean istio/api, detail: {}", e);
            continue;
        }
    }

    info!("codegen done");
}
