use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn main() {
    let mut out =
        File::create(PathBuf::from(env::var("OUT_DIR").unwrap()).join("doctests.rs")).unwrap();
    let skeptical = env::var("CARGO_MANIFEST_DIR").unwrap();
    let botocore = Path::new(&skeptical)
        .join("..")
        .join("service_crategen")
        .join("botocore")
        .canonicalize()
        .unwrap();
    for path in glob::glob(&format!("{}/../**/*.md", skeptical)).unwrap() {
        let path = path.unwrap().canonicalize().unwrap();
        if path.starts_with(&botocore) {
            continue;
        }
        let path = path.to_str().unwrap();
        println!("cargo:rerun-if-changed={}", path);
        writeln!(out, "doctest!(\"{}\");", path).unwrap();
    }
}
