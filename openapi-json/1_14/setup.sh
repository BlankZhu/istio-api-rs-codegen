if [[ -z "${ISTIO_API_PATH}" ]]; then
    echo "ENV ISTIO_API_PATH not set"
else 
    CURR_DIR=${PWD}
    
    echo "using Istio API path: ${ISTIO_API_PATH}"
    cd ${ISTIO_API_PATH}
    git checkout tags/1.14.0
    cd ${CURR_DIR}

    echo "making up API folders..."
    mkdir -p extensions/v1alpha1
    mkdir -p networking/v1alpha3
    mkdir -p networking/v1beta1
    mkdir -p operator/v1alpha1
    mkdir -p security/v1beta1
    mkdir -p telemetry/v1alpha1
    mkdir -p type/v1beta1

    echo "fetching extensions/v1alpha1 ..."
    cp ${ISTIO_API_PATH}/extensions/v1alpha1/wasm.gen.json extensions/v1alpha1/wasm_plugin.gen.json 
    
    echo "fetching networking/v1alpha3 ..."
    cp ${ISTIO_API_PATH}/networking/v1alpha3/destination_rule.gen.json networking/v1alpha3/destination_rule.gen.json
    cp ${ISTIO_API_PATH}/networking/v1alpha3/envoy_filter.gen.json networking/v1alpha3/envoy_filter.gen.json
    cp ${ISTIO_API_PATH}/networking/v1alpha3/gateway.gen.json networking/v1alpha3/gateway.gen.json
    cp ${ISTIO_API_PATH}/networking/v1alpha3/service_entry.gen.json networking/v1alpha3/service_entry.gen.json
    cp ${ISTIO_API_PATH}/networking/v1alpha3/sidecar.gen.json networking/v1alpha3/sidecar.gen.json
    cp ${ISTIO_API_PATH}/networking/v1alpha3/virtual_service.gen.json networking/v1alpha3/virtual_service.gen.json
    cp ${ISTIO_API_PATH}/networking/v1alpha3/workload_entry.gen.json networking/v1alpha3/workload_entry.gen.json
    cp ${ISTIO_API_PATH}/networking/v1alpha3/workload_group.gen.json networking/v1alpha3/workload_group.gen.json

    echo "fetching networking/v1beta1 ..."
    cp ${ISTIO_API_PATH}/networking/v1beta1/destination_rule.gen.json networking/v1beta1/destination_rule.gen.json
    cp ${ISTIO_API_PATH}/networking/v1beta1/gateway.gen.json networking/v1beta1/gateway.gen.json
    cp ${ISTIO_API_PATH}/networking/v1beta1/proxy_config.gen.json networking/v1beta1/proxy_config.gen.json
    cp ${ISTIO_API_PATH}/networking/v1beta1/service_entry.gen.json networking/v1beta1/service_entry.gen.json
    cp ${ISTIO_API_PATH}/networking/v1beta1/sidecar.gen.json networking/v1beta1/sidecar.gen.json
    cp ${ISTIO_API_PATH}/networking/v1beta1/virtual_service.gen.json networking/v1beta1/virtual_service.gen.json
    cp ${ISTIO_API_PATH}/networking/v1beta1/workload_entry.gen.json networking/v1beta1/workload_entry.gen.json
    cp ${ISTIO_API_PATH}/networking/v1beta1/workload_group.gen.json networking/v1beta1/workload_group.gen.json

    echo "fetching operator/v1alpha1 ..."
    cp ${ISTIO_API_PATH}/operator/v1alpha1/operator.gen.json operator/v1alpha1/operator.gen.json

    echo "fetching security/v1beta1 ..."
    cp ${ISTIO_API_PATH}/security/v1beta1/authorization_policy.gen.json security/v1beta1/authorization_policy.gen.json
    cp ${ISTIO_API_PATH}/security/v1beta1/peer_authentication.gen.json security/v1beta1/peer_authentication.gen.json
    cp ${ISTIO_API_PATH}/security/v1beta1/request_authentication.gen.json security/v1beta1/request_authentication.gen.json

    echo "fetching telemetry/v1alpha1 ..."
    cp ${ISTIO_API_PATH}/telemetry/v1alpha1/telemetry.gen.json telemetry/v1alpha1/telemetry.gen.json

    echo "fetching type/v1beta1 ..."
    cp ${ISTIO_API_PATH}/type/v1beta1/selector.gen.json type/v1beta1/selector.gen.json

    echo "setup completed"
fi
