# This python script will extract directory structure and OpenAPI scheme JSON file
# from `istio/api`, call for openapi-generator to generate relate Rust module codes.
# After extraction, the Rust module codes will be rename, prune, modify, put together
# as the final artifect.

import adjust
import generate
import modify
import prune
import constant
import util


def main():
    metas = util.generateMetadatas(constant.openapi_json_scheme_dir)

    for meta in metas:
        # istio_version, section, apiVersion, OpenAPI Scheme
        generate.generateRustCode(meta[0], meta[1], meta[2], meta[3])
        prune.pruneRustCode(meta[0], meta[1], meta[2], meta[3])
        adjust.adjustRustCode(meta[0], meta[1], meta[2], meta[3])
        modify.modifyRustCode(meta[0], meta[1], meta[2], meta[3])

    modify.MakeupModRsTree(metas[0][0])


if __name__ == "__main__":
    main()
