use adjust::Adva;
use clap::Parser;
use log::{error, info};

use crate::{adjust::Tena, cue::Cuge, cue::Cutter, git::Gitter, openapi::Ogen};

pub mod adjust;
pub mod args;
pub mod constant;
pub mod cue;
pub mod git;
pub mod openapi;
pub mod util;

fn main() {
    env_logger::init();

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
    if let Err(e) = args.check_openapi_generator_cli_jar() {
        error!("Error occurs while check openapi-generator jar path: {}", e);
        std::process::exit(exitcode::DATAERR);
    }

    if args.setup_openapi_directory {
        setup_openapi_directory(&args);
    }

    if args.generate_rust_codes {
        generate_rust_codes(&args);
    }

    if args.adjust_generated_rust_codes {
        adjust_generated_rust_codes(&args)
    }

    info!("codegen done");
}

fn setup_openapi_directory(args: &args::Args) {
    info!("setting up openapi directory ...");

    let working_directory = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(e) => panic!("failed to get current working directory: {}", e),
    };

    let gitter = Gitter::new(std::path::Path::new(&args.istio_api_dir_path).to_path_buf());
    let cutter = Cutter::new(
        std::path::Path::new(&args.istio_api_dir_path).join(&constant::CUE_CONFIG_FILE_NAME),
    );
    let cuge = Cuge::new(
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
        if let Err(e) = cuge.change_to_istio_api_dir() {
            error!(
                "failed to change working directory to istio/api, detail: {}",
                e
            );
            if let Err(e) = cuge.change_to_codegen_dir() {
                panic!(
                    "failed to recover working directory back to codegen's, detail: {}",
                    e
                );
            }
            continue;
        }
        if let Err(e) = cuge.cue_gen() {
            error!("failed to do CUE gen, detail: {}", e);
            if let Err(e) = cuge.change_to_codegen_dir() {
                panic!(
                    "failed to recover working directory back to codegen's, detail: {}",
                    e
                );
            }
            continue;
        }
        if let Err(e) = cuge.change_to_codegen_dir() {
            panic!(
                "failed to recover working directory back to codegen's, detail: {}",
                e
            );
        }

        info!("setting up openapi-json middle directory ...");
        if let Err(e) = cuge.extract_openapi_to_codegen_dir(version_info) {
            error!(
                "failed to extract OpenAPI JSONs to codegen directory, detail: {}",
                e
            );
            continue;
        }

        info!("restoring istio/api on tag {} ...", version_info.version);
        if let Err(e) = gitter.restore() {
            error!("failed to restore istio/api, detail: {}", e);
            continue;
        }

        info!(
            "cleaning(force) istio/api on tag {} ...",
            version_info.version
        );
        if let Err(e) = gitter.force_clean() {
            error!("failed to clean istio/api, detail: {}", e);
            continue;
        }
    }

    info!("setup openapi directory completed");
}

fn generate_rust_codes(args: &args::Args) {
    info!("generating rust codes ...");

    let ogen = Ogen::new(
        std::path::Path::new(constant::OPENAPI_JSON_DIR).to_path_buf(),
        std::path::Path::new(&args.openapi_generator_cli_jar_path).to_path_buf(),
        std::path::Path::new(&args.output_dir_path).to_path_buf(),
    );
    info!("generating rust code from OpenAPI JSONs ...");
    if let Err(e) = ogen.openapi_generate() {
        error!("failed to generate rust code, detail: {}", e);
    }

    info!("rust codes generated");
}

fn adjust_generated_rust_codes(args: &args::Args) {
    info!("adjusting generated rust codes ...");
    let adva = Adva::new(
        std::path::Path::new(constant::OPENAPI_JSON_DIR).to_path_buf(),
        std::path::Path::new(&args.output_dir_path).to_path_buf(),
    );
    if let Err(e) = adva.adjust() {
        error!("failed to adjust rust code, detail: {}", e);
    }
    let tena = Tena::new(std::path::Path::new(&args.output_dir_path).to_path_buf());
    if let Err(e) = tena.setup_rust_structure_tree() {
        error!("failed to adjust rust structure tree, detail: {}", e);
    }
    info!("rust codes adjusting fininshed")
}
