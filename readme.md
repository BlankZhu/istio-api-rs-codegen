# istio-openapi-to-rs

This repository provides a way to generate rust codes that meets the CRD traits in [kube-rs](https://github.com/kube-rs/kube-rs). 

## prerequisite

To generate the codes, the following is required:

* make, for makefile
* python3, to run the main codes
* openapi-generator-cli.jar (>= 6.0.0), to generate raw rust codes from OpenAPI JSONs
* jre, to run jar

NOTE: `openapi-generator-cli.jar` should be placed at: `./jar` 

## getting started

Just run:

```shell
mkdir generated
make
```

And got the output codes in `./generated`.

To clean the generated codes, use:

```shell
make clean
```

## how 

If you're interested in how this works, check `./openapi-json/readme.md` & `./main.py` for more details.