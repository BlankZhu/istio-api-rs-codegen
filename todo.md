1. get crd-all.gen.yaml of istio 1.10~1.14 to resource/istio/\[version\]/crd-all.gen.yaml
2. split crd-all.gen.yaml into single yaml files
3. for each CRD yaml file, check api version
4. for each api version, use kopium to generate rust codes
5. output to generated/API_GROUP/API_VERSION/CRD.yaml