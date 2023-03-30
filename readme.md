# istio-api-rs-codegen

istio-api-rs-codegen using Kopium.

pros:
1. Kopium use Istio CRD directly, skipping OpenAPI JSONs.
2. Pure rust codegen.

cons:
1. Currently, kopium will generated duplicate Rust code fields (bug, though using 'rename').
2. Kopium will miss some structs, most of them are duplicate or error generated.

In short, Kopium shows a good vision of kube-rs compatible codegen. However, Kopium is not stable yet, which means it not ready for kube-rs compatible Istio CRD codegen.

The codegen in this branch is workable itself, but the generated codes are not. 

Currently, some of the generated codes are modified by the maintainer, which finally makes the codes runnable.
