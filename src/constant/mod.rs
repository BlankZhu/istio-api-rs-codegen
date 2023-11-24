pub static ISTIO_VERSIONS: &'static [&str] = &[
    "1.10.0", "1.11.0", "1.12.0", "1.13.0", "1.14.0", "1.15.0", "1.16.0", "1.17.0", "1.18.0",
    "1.19.0", "1.20.0",
];
pub static ISTIO_CRD_ALL_URL_PREFIX: &'static str =
    "https://raw.githubusercontent.com/istio/istio/";
pub static ISTIO_CRD_ALL_URL_SUFFIX: &'static str = "/manifests/charts/base/crds/crd-all.gen.yaml";
pub static ISTIO_CRD_TEMP_DIRECTORY: &'static str = "resources/istio";
pub static ISTIO_CRD_FILENAME: &'static str = "crd-all.gen.yaml";
pub static ISTIO_CRD_RUST_CODE_OUTPUT_DIRECTORY: &'static str = "output";
pub static KOPIUM_COMMAND: &'static str = "kopium";
