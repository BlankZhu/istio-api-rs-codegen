use crate::constant;

#[derive(Debug)]
pub struct Fetcher {

}

impl Fetcher {
    pub fn new() -> Self {
        return Fetcher {};
    }

    pub fn fetch(&self, yaml_body: String) -> Option<String> {
        todo!()
    }

    // fn start_fetching(&self) {
    //     constant::ISTIO_VERSIONS.iter().for_each(|item| {
    //         let url = constant::ISTIO_CRD_ALL_URL_PREFIX.to_string() + &item.to_string() + &constant::ISTIO_CRD_ALL_URL_SUFFIX.to_string();
    //         println!("fetching url: {}", url)
    //     })
    // }
}
