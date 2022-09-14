use clap::Parser;
use log::{error, info};

use crate::{cue::Cutter, git::Gitter};

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
    for version in constant::ISTIO_VERSIONS {
        info!("checking out tag {} ...", version);
        if let Err(e) = gitter.checkout_tag(version) {
            error!("failed to checkout tag, detail: {}", e);
            continue;
        }
        info!("modifying cue config on tag {} ...", version);
        if let Err(e) = cutter.modify_cue_file() {
            error!("failed to modify cue config file, detail: {}", e);
            continue;
        }

        info!("restoring istio/api on tag {} ...", version);
        if let Err(e) = gitter.restore() {
            error!("failed to restore istio/api, detail: {}", e);
            continue;
        }

        info!("cleaning(foce) istio/api on tag {} ...", version);
        if let Err(e) = gitter.force_clean() {
            error!("failed to clean istio/api, detail: {}", e);
            continue;
        }
    }

    info!("codegen done");
}
