# istio-api-rs-codegen

istio-api-rs-codegen is a code generator using Kopium. It generate Rust code directly from the Istio CRD yaml file.

## Getting started

To start quickly, just run:

```shell
cargo run -- -f -r -g
```

All the codes will be generated at `./output`, you can also check `./resources` to for the mid product.

You can also make a release build the binary instead of using `cargo run` for better performance. For more information, use `-h`, or just dive into the sources!

Feel free to use or start any issue!

## Add new version for istio

To update the generated code for newest istio, do:

1. Check new version of `kopium` simply by running `cargo install kopium`.
2. Check new version of `k8s-openapi` to update the supported kubernetes API version (with the crate feature). Also check the `kube` crate's version if possible.
3. Add new version string in `src/constant/mod.rs`: modify the `ISTIO_VERSIONS` variable by adding a `MAJOR.MINOR.0` version string.
4. Run `istio-api-rs-codegen` by `cargo run -- -f -r -g`.
5. Fetch the newest codes in `output/`