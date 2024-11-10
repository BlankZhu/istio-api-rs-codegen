use clap::Parser;
use log::{error, info};

mod constant;
mod error;
mod fetcher;
mod generator;
mod option;
mod resolver;
mod utility;

#[tokio::main]
async fn main() -> utility::Result<()> {
    let opt = option::Option::parse();

    env_logger::init();
    info!("istio-api-rs-codegen now running!");

    if opt.fetch {
        info!("fetching CRDs...");
        fetch().await;
        info!("fetch finished");
    }

    if opt.resolve {
        info!("resolving CRDs...");
        resolve();
        info!("resolve finished");
    }

    if opt.generate {
        info!("generating rust code...");
        generate();
        info!("generate finished");
    }

    info!("istio-api-rs-codegen finished");
    Ok(())
}

async fn fetch() {
    let fetcher = fetcher::Fetcher::new();

    let mut fetch_promises = Vec::new();
    for version in constant::ISTIO_VERSIONS {
        fetch_promises.push(fetcher.fetch(version));
    }

    for promise in fetch_promises {
        let resp = promise.await;
        if let Err(e) = resp {
            error!("failed to fetch crd.yaml: {}", e);
        }
    }
}

fn resolve() {
    let resolver = resolver::Resolver::new();
    for version in constant::ISTIO_VERSIONS {
        if let Err(e) = resolver.resolve(version) {
            error!("failed to resolve temp files for {}: {}", version, e);
        }
    }
}

fn generate() {
    let generator = generator::Generator::new();
    if let Err(e) = generator.generate(constant::ISTIO_CRD_TEMP_DIRECTORY) {
        error!("failed to generate final rust code: {}", e);
    }
}
