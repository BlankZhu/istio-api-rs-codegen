import os

openapi_json_scheme_dir = 'openapi-json'
generated_dir = 'generated'
rust_reserved_words = ['type']
section_with_no_crd = ['type']


def findAllFile(base):
    for root, ds, fs in os.walk(base):
        for f in fs:
            fullname = os.path.join(root, f)
            yield fullname
