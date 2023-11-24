# istio-api-rs-codegen

istio-api-rs-codegen is a code generator using Kopium. It generate Rust code directly from the Istio CRD yaml file.

pros:
1. Kopium use Istio CRD directly, skipping OpenAPI JSONs.
2. Pure rust codegen.

cons:
1. Currently, kopium will generated duplicate Rust code fields (bug, though using 'rename').
2. Kopium will miss some structs, most of them are duplicate or error generated.

In short, Kopium shows a good vision of kube-rs compatible codegen. However, Kopium is not stable yet, which means it not ready for kube-rs compatible Istio CRD codegen.

The codegen in this branch is workable itself, but the generated codes are not. 

Currently, some of the generated codes should modified by the maintainer, which finally makes the codes runnable.

## Getting started

To start quickly, just run:

```shell
cargo run -- -f -r -g
```

All the codes will be generated at `./output`, you can also check `./resources` to for the mid product.

You can also make a release build the binary instead of using `cargo run` for better performance. For more information, use `-h`, or just dive into the sources!

Feel free to use or start any issue!

## Output Modification

Currently, generated codes require following modifications:


Delete:

```rust

    /// Percentage of the traffic to be mirrored by the `mirror` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_percent: Option<i64>,
```

Delete:

```rust

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_percent: Option<i64>,
```

Delete:

```rust

    /// URL of the provider's public key set to validate signature of the JWT.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwks_uri: Option<String>,
```