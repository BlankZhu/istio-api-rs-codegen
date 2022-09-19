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
    pub target_openapi_file: &'static [&'static str],
}

pub const ISTIO_1_10_TARGET_FILES: &'static [&'static str] = &[
    "networking/v1alpha3/destination_rule.gen.json",
    "networking/v1alpha3/envoy_filter.gen.json",
    "networking/v1alpha3/gateway.gen.json",
    "networking/v1alpha3/service_entry.gen.json",
    "networking/v1alpha3/sidecar.gen.json",
    "networking/v1alpha3/virtual_service.gen.json",
    "networking/v1alpha3/workload_entry.gen.json",
    "networking/v1alpha3/workload_group.gen.json",
    "networking/v1beta1/destination_rule.gen.json",
    "networking/v1beta1/gateway.gen.json",
    "networking/v1beta1/service_entry.gen.json",
    "networking/v1beta1/sidecar.gen.json",
    "networking/v1beta1/virtual_service.gen.json",
    "networking/v1beta1/workload_entry.gen.json",
    "security/v1beta1/authorization_policy.gen.json",
    "security/v1beta1/peer_authentication.gen.json",
    "security/v1beta1/request_authentication.gen.json",
    "telemetry/v1alpha1/telemetry.gen.json",
    "type/v1beta1/selector.gen.json",
    "operator/v1alpha1/operator.gen.json",
];
pub const ISTIO_1_11_TARGET_FILES: &'static [&'static str] = &[
    "networking/v1alpha3/destination_rule.gen.json",
    "networking/v1alpha3/envoy_filter.gen.json",
    "networking/v1alpha3/gateway.gen.json",
    "networking/v1alpha3/service_entry.gen.json",
    "networking/v1alpha3/sidecar.gen.json",
    "networking/v1alpha3/virtual_service.gen.json",
    "networking/v1alpha3/workload_entry.gen.json",
    "networking/v1alpha3/workload_group.gen.json",
    "networking/v1beta1/destination_rule.gen.json",
    "networking/v1beta1/gateway.gen.json",
    "networking/v1beta1/service_entry.gen.json",
    "networking/v1beta1/sidecar.gen.json",
    "networking/v1beta1/virtual_service.gen.json",
    "networking/v1beta1/workload_entry.gen.json",
    "security/v1beta1/authorization_policy.gen.json",
    "security/v1beta1/peer_authentication.gen.json",
    "security/v1beta1/request_authentication.gen.json",
    "telemetry/v1alpha1/telemetry.gen.json",
    "type/v1beta1/selector.gen.json",
    "operator/v1alpha1/operator.gen.json",
];
pub const ISTIO_1_12_TARGET_FILES: &'static [&'static str] = &[
    "extensions/v1alpha1/wasm.gen.json",
    "networking/v1alpha3/destination_rule.gen.json",
    "networking/v1alpha3/envoy_filter.gen.json",
    "networking/v1alpha3/gateway.gen.json",
    "networking/v1alpha3/service_entry.gen.json",
    "networking/v1alpha3/sidecar.gen.json",
    "networking/v1alpha3/virtual_service.gen.json",
    "networking/v1alpha3/workload_entry.gen.json",
    "networking/v1alpha3/workload_group.gen.json",
    "networking/v1beta1/destination_rule.gen.json",
    "networking/v1beta1/gateway.gen.json",
    "networking/v1beta1/service_entry.gen.json",
    "networking/v1beta1/sidecar.gen.json",
    "networking/v1beta1/virtual_service.gen.json",
    "networking/v1beta1/workload_entry.gen.json",
    "security/v1beta1/authorization_policy.gen.json",
    "security/v1beta1/peer_authentication.gen.json",
    "security/v1beta1/request_authentication.gen.json",
    "telemetry/v1alpha1/telemetry.gen.json",
    "type/v1beta1/selector.gen.json",
    "operator/v1alpha1/operator.gen.json",
];
pub const ISTIO_1_13_TARGET_FILES: &'static [&'static str] = &[
    "extensions/v1alpha1/wasm.gen.json",
    "networking/v1alpha3/destination_rule.gen.json",
    "networking/v1alpha3/envoy_filter.gen.json",
    "networking/v1alpha3/gateway.gen.json",
    "networking/v1alpha3/service_entry.gen.json",
    "networking/v1alpha3/sidecar.gen.json",
    "networking/v1alpha3/virtual_service.gen.json",
    "networking/v1alpha3/workload_entry.gen.json",
    "networking/v1alpha3/workload_group.gen.json",
    "networking/v1beta1/destination_rule.gen.json",
    "networking/v1beta1/gateway.gen.json",
    "networking/v1beta1/proxy_config.gen.json",
    "networking/v1beta1/service_entry.gen.json",
    "networking/v1beta1/sidecar.gen.json",
    "networking/v1beta1/virtual_service.gen.json",
    "networking/v1beta1/workload_entry.gen.json",
    "networking/v1beta1/workload_group.gen.json",
    "security/v1beta1/authorization_policy.gen.json",
    "security/v1beta1/peer_authentication.gen.json",
    "security/v1beta1/request_authentication.gen.json",
    "telemetry/v1alpha1/telemetry.gen.json",
    "type/v1beta1/selector.gen.json",
    "operator/v1alpha1/operator.gen.json",
];
pub const ISTIO_1_14_TARGET_FILES: &'static [&'static str] = &[
    "extensions/v1alpha1/wasm.gen.json",
    "networking/v1alpha3/destination_rule.gen.json",
    "networking/v1alpha3/envoy_filter.gen.json",
    "networking/v1alpha3/gateway.gen.json",
    "networking/v1alpha3/service_entry.gen.json",
    "networking/v1alpha3/sidecar.gen.json",
    "networking/v1alpha3/virtual_service.gen.json",
    "networking/v1alpha3/workload_entry.gen.json",
    "networking/v1alpha3/workload_group.gen.json",
    "networking/v1beta1/destination_rule.gen.json",
    "networking/v1beta1/gateway.gen.json",
    "networking/v1beta1/proxy_config.gen.json",
    "networking/v1beta1/service_entry.gen.json",
    "networking/v1beta1/sidecar.gen.json",
    "networking/v1beta1/virtual_service.gen.json",
    "networking/v1beta1/workload_entry.gen.json",
    "networking/v1beta1/workload_group.gen.json",
    "security/v1beta1/authorization_policy.gen.json",
    "security/v1beta1/peer_authentication.gen.json",
    "security/v1beta1/request_authentication.gen.json",
    "telemetry/v1alpha1/telemetry.gen.json",
    "type/v1beta1/selector.gen.json",
    "operator/v1alpha1/operator.gen.json",
];
pub const ISTIO_1_15_TARGET_FILES: &'static [&'static str] = &[
    "extensions/v1alpha1/wasm.gen.json",
    "networking/v1alpha3/destination_rule.gen.json",
    "networking/v1alpha3/envoy_filter.gen.json",
    "networking/v1alpha3/gateway.gen.json",
    "networking/v1alpha3/service_entry.gen.json",
    "networking/v1alpha3/sidecar.gen.json",
    "networking/v1alpha3/virtual_service.gen.json",
    "networking/v1alpha3/workload_entry.gen.json",
    "networking/v1alpha3/workload_group.gen.json",
    "networking/v1beta1/destination_rule.gen.json",
    "networking/v1beta1/gateway.gen.json",
    "networking/v1beta1/proxy_config.gen.json",
    "networking/v1beta1/service_entry.gen.json",
    "networking/v1beta1/sidecar.gen.json",
    "networking/v1beta1/virtual_service.gen.json",
    "networking/v1beta1/workload_entry.gen.json",
    "networking/v1beta1/workload_group.gen.json",
    "security/v1beta1/authorization_policy.gen.json",
    "security/v1beta1/peer_authentication.gen.json",
    "security/v1beta1/request_authentication.gen.json",
    "telemetry/v1alpha1/telemetry.gen.json",
    "type/v1beta1/selector.gen.json",
    "operator/v1alpha1/operator.gen.json",
];

pub const ISTIO_API_VERSION_INFO_1_10: &'static IstioApiVersionInfo = &IstioApiVersionInfo {
    version: "1.10.0",
    target_openapi_file: ISTIO_1_10_TARGET_FILES,
};
pub const ISTIO_API_VERSION_INFO_1_11: &'static IstioApiVersionInfo = &IstioApiVersionInfo {
    version: "1.11.0",
    target_openapi_file: ISTIO_1_11_TARGET_FILES,
};
pub const ISTIO_API_VERSION_INFO_1_12: &'static IstioApiVersionInfo = &IstioApiVersionInfo {
    version: "1.12.0",
    target_openapi_file: ISTIO_1_12_TARGET_FILES,
};
pub const ISTIO_API_VERSION_INFO_1_13: &'static IstioApiVersionInfo = &IstioApiVersionInfo {
    version: "1.13.0",
    target_openapi_file: ISTIO_1_13_TARGET_FILES,
};
pub const ISTIO_API_VERSION_INFO_1_14: &'static IstioApiVersionInfo = &IstioApiVersionInfo {
    version: "1.14.0",
    target_openapi_file: ISTIO_1_14_TARGET_FILES,
};
pub const ISTIO_API_VERSION_INFO_1_15: &'static IstioApiVersionInfo = &IstioApiVersionInfo {
    version: "1.15.0",
    target_openapi_file: ISTIO_1_15_TARGET_FILES,
};

pub const ISTIO_API_VERSION_INFOS: &'static [&'static IstioApiVersionInfo] = &[
    ISTIO_API_VERSION_INFO_1_10,
    ISTIO_API_VERSION_INFO_1_11,
    ISTIO_API_VERSION_INFO_1_12,
    ISTIO_API_VERSION_INFO_1_13,
    ISTIO_API_VERSION_INFO_1_14,
    ISTIO_API_VERSION_INFO_1_15,
];
