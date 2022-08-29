use log::info;

mod constant;
mod fetcher;
mod resolver;
mod generator;
mod option;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    env_logger::init();
    
    info!("istio-api-rs-codegen now running!");

    let mut fetcher = fetcher::Fetcher::new();
    fetcher.initialize();

    let mut fetch_promises = Vec::new();
    constant::ISTIO_VERSIONS.iter().for_each(|version| {
        fetch_promises.push(fetcher.fetch(version));
    });
    for promise in fetch_promises {
        let resp = promise.await;
        if let Some(yaml) = resp {
            println!("{}", yaml);
        }
    }


    Ok(())
}
