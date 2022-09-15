pub const ISTIO_API_DIR: &'static str = "api";
pub const OPENAPI_JSON_DIR: &'static str = "openapi-json";
pub const OUTPUT_DIR: &'static str = "output";

pub const CUE_CONFIG_FILE_NAME: &'static str = "cue.yaml";

pub const RUST_RESERVED_WORDS: &'static [&'static str] = &["type"];
pub const SECTION_WITH_NO_CRD: &'static [&'static str] = &["type"];

pub const GIT_COMMAND: &'static str = "git";
pub const CUE_GEN_COMMAND: &'static str = "cue-gen";

pub struct IstioApiVersionInfo {
    pub version: &'static str,
    pub target_directories: &'static [&'static str],
}

pub const ISTIO_1_10_TARGET_DIRECTORIES: &'static [&'static str] = &[
    "type/v1beta1",
    "networking/v1alpha3",
    "networking/v1beta1",
    "security/v1beta1",
    "telemetry/v1alpha1",
    "operator/v1alpha1",
];
pub const ISTIO_1_11_TARGET_DIRECTORIES: &'static [&'static str] = &[
    "type/v1beta1",
    "networking/v1alpha3",
    "networking/v1beta1",
    "security/v1beta1",
    "telemetry/v1alpha1",
    "operator/v1alpha1",
];
pub const ISTIO_1_12_TARGET_DIRECTORIES: &'static [&'static str] = &[
    "type/v1beta1",
    "extensions/v1alpha1",
    "networking/v1alpha3",
    "networking/v1beta1",
    "security/v1beta1",
    "telemetry/v1alpha1",
    "operator/v1alpha1",
];
pub const ISTIO_1_13_TARGET_DIRECTORIES: &'static [&'static str] = &[
    "type/v1beta1",
    "extensions/v1alpha1",
    "networking/v1alpha3",
    "networking/v1beta1",
    "security/v1beta1",
    "telemetry/v1alpha1",
    "operator/v1alpha1",
];
pub const ISTIO_1_14_TARGET_DIRECTORIES: &'static [&'static str] = &[
    "type/v1beta1",
    "extensions/v1alpha1",
    "networking/v1alpha3",
    "networking/v1beta1",
    "security/v1beta1",
    "telemetry/v1alpha1",
    "operator/v1alpha1",
];
pub const ISTIO_1_15_TARGET_DIRECTORIES: &'static [&'static str] = &[
    "type/v1beta1",
    "extensions/v1alpha1",
    "networking/v1alpha3",
    "networking/v1beta1",
    "security/v1beta1",
    "telemetry/v1alpha1",
    "operator/v1alpha1",
];

pub const ISTIO_API_VERSION_INFO_1_10: &'static IstioApiVersionInfo = &IstioApiVersionInfo {
    version: "1.10.0",
    target_directories: ISTIO_1_10_TARGET_DIRECTORIES,
};
pub const ISTIO_API_VERSION_INFO_1_11: &'static IstioApiVersionInfo = &IstioApiVersionInfo {
    version: "1.11.0",
    target_directories: ISTIO_1_11_TARGET_DIRECTORIES,
};
pub const ISTIO_API_VERSION_INFO_1_12: &'static IstioApiVersionInfo = &IstioApiVersionInfo {
    version: "1.12.0",
    target_directories: ISTIO_1_12_TARGET_DIRECTORIES,
};
pub const ISTIO_API_VERSION_INFO_1_13: &'static IstioApiVersionInfo = &IstioApiVersionInfo {
    version: "1.13.0",
    target_directories: ISTIO_1_13_TARGET_DIRECTORIES,
};
pub const ISTIO_API_VERSION_INFO_1_14: &'static IstioApiVersionInfo = &IstioApiVersionInfo {
    version: "1.14.0",
    target_directories: ISTIO_1_14_TARGET_DIRECTORIES,
};
pub const ISTIO_API_VERSION_INFO_1_15: &'static IstioApiVersionInfo = &IstioApiVersionInfo {
    version: "1.15.0",
    target_directories: ISTIO_1_15_TARGET_DIRECTORIES,
};

pub const ISTIO_API_VERSION_INFOS: &'static [&'static IstioApiVersionInfo] = &[
    ISTIO_API_VERSION_INFO_1_10,
    ISTIO_API_VERSION_INFO_1_11,
    ISTIO_API_VERSION_INFO_1_12,
    ISTIO_API_VERSION_INFO_1_13,
    ISTIO_API_VERSION_INFO_1_14,
    ISTIO_API_VERSION_INFO_1_15,
];
