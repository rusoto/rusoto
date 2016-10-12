# Rusoto codegen

Code generation crate for [Rusoto](https://github.com/rusoto/rusoto).

## Parts

* [botocore](botocore) - AWS' low level SDK [https://github.com/boto/botocore](https://github.com/boto/botocore)
* [build.rs](build.rs) - Codegen build script
* [src](src) - Bulk of the project: containers code generators to read in botocore service definitions and emit Rust

## Instructions

### Updating git submodule

After a fresh clone of the rusoto repo:

```bash
cd rusoto
git submodule init
git submodule update
```

Or: `git clone --recursive git@github.com:rusoto/rusoto.git`