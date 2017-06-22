extern crate rustc_version;

use std::env;
use std::path::Path;
use std::io::Write;
use std::fs::File;

/// Parses and generates variables used to construct a User-Agent.
///
/// This is used to create a User-Agent header string resembling
/// `rusoto/x.y.z rust/x.y.z <os>`.
fn generate_user_agent_vars(output_path: &Path) {
    let rust_version = rustc_version::version()
        .expect("Could not retrieve rustc version");
    let mut f = File::create(&output_path.join("user_agent_vars.rs"))
        .expect("Could not create user agent file");
    f.write_all(format!("static RUST_VERSION: &'static str = \"{}\";", rust_version).as_bytes())
        .expect("Unable to write user agent");
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not specified");
    let out_path = Path::new(&out_dir).to_owned();

    generate_user_agent_vars(&out_path);
}