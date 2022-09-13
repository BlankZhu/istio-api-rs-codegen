import subprocess

import constant


def callOpenAPIClient(input_src, output_dest, crd):
    rust_properties = '--additional-properties=packageName=' + crd
    result = subprocess.run([
        'java', '-jar', './jar/openapi-generator-cli.jar', 'generate', '-a', 'BlankZhu',
        '-g', 'rust', '-i', input_src, '-o', output_dest, rust_properties
    ], stdout=subprocess.PIPE)
    return result


def generateRustCode(istio_version, section, api_version, crd):
    input_src = '/'.join([constant.openapi_json_scheme_dir, istio_version,
                         section, api_version, crd+'.gen.json'])
    output_dest = '/'.join([constant.generated_dir, istio_version,
                           section, api_version, crd])

    print('Generating code for:',
          '/'.join([istio_version, section, api_version, crd]))
    ret = callOpenAPIClient(input_src, output_dest, crd)
    ret.check_returncode()

    if ret.returncode == 0:
        print('Generation completed on',
              '/'.join([istio_version, section, api_version, crd]))
