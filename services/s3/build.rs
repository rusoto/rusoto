extern crate rustc_version;
extern crate rusoto_codegen;
extern crate rayon;

use std::env;
use std::path::Path;
use std::io::Write;
use std::fs::File;

use rusoto_codegen::{Service, generate};
use rayon::prelude::*;

/// Parses and generates variables used to construct a User-Agent.
///
/// This is used to create a User-Agent header string resembling
/// `rusoto/x.y.z rust/x.y.z <os>`.
fn generate_user_agent_vars(output_path: &Path) {
    let rust_version = rustc_version::version();
    let mut f = File::create(&output_path.join("user_agent_vars.rs"))
        .expect("Could not create user agent file");
    f.write_all(format!("static RUST_VERSION: &'static str = \"{}\";", rust_version).as_bytes())
        .expect("Unable to write user agent");
}

// expand to use cfg!() so codegen only gets run for services
// in the features list
macro_rules! services {
    ( $( [$name:expr, $date:expr] ),* ) => {
        {
            let mut services = Vec::new();
            $(
                if cfg!(feature = $name) {
                    services.push(Service::new($name, $date));
                }
            )*
            services
        }
    }
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir).to_owned();

    let services = services! {
        ["s3", "2006-03-01"]
    };

    let count: usize =
        services.into_par_iter().map(|service| generate(service, &out_path.clone())).count();
    println!("\nGenerated {:?} services.\n", count);

    generate_user_agent_vars(&out_path);

    // let codegen_dir = Path::new("codegen");
    // avoid unnecessary recompiles when used as a crates.io dependency
    // if codegen_dir.exists() {
    //     println!("cargo:rerun-if-changed=codegen");
    // }
}
