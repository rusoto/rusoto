extern crate stopwatch;
extern crate env_logger;

use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use self::stopwatch::Stopwatch;
use rayon::prelude::*;
use rustfmt;
use toml;

mod codegen;

use cargo;
use log::LogLevel::Debug;
use ::{Service, ServiceConfig, ServiceDefinition};

pub fn generate_services(services: BTreeMap<String, ServiceConfig>, out_dir: &Path) {
    let _ = env_logger::init(); // This initializes the `env_logger`
    if log_enabled!(Debug) {
        debug!("\n\nwoohoo enabled");
    }
    if !out_dir.exists() {
        fs::create_dir(out_dir).expect("Unable to create output directory");
    }

    services.par_iter().for_each(|(name, service_config)| {
        let sw = Stopwatch::start_new();
        let service = {
            let service_definition = ServiceDefinition::load(name, &service_config.protocol_version)
                .expect(&format!("Failed to load service {}. Make sure the botocore submodule has been initialized!", name));
            Service::new(service_config.clone(), service_definition)
        };

        let crate_dir = out_dir.join(&name);
        let crate_name = format!("rusoto_{}", &name.replace('-', "_"));

        println!("Generating crate for {} @ {}...", service.full_name(), service.api_version());

        if !crate_dir.exists() {
            fs::create_dir(&crate_dir).expect(&format!("Unable to create directory at {}", crate_dir.display()));
        }

        let service_dependencies = service.get_dependencies();

        let extern_crates = service_dependencies.iter().map(|(k, _)| {
            if k == "xml-rs" {
                return "extern crate xml;".into();
            }
            let safe_name = k.replace("-", "_");
            let use_macro = k == "serde_derive" || k == "log" || k == "lazy_static";
            if use_macro {
                return format!("#[macro_use]\nextern crate {};", safe_name);
            }
            format!("extern crate {};", safe_name)
        }).collect::<Vec<String>>().join("\n");

        let mut cargo_manifest = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(crate_dir.join("Cargo.toml"))
            .expect("Unable to write Cargo.toml");

        let mut name_for_keyword = name.clone().to_string();
        if name_for_keyword.len() >= 20 {
            name_for_keyword = name_for_keyword[..20].to_string();
        }

        let manifest = cargo::Manifest {
            package: cargo::Metadata {
                authors: Some(vec![
                    "Anthony DiMarco <ocramida@gmail.com>".into(),
                    "Jimmy Cuadra <jimmy@jimmycuadra.com>".into(),
                    "Matthew Mayer <matthewkmayer@gmail.com>".into(),
                    "Nikita Pekin <contact@nikitapek.in>".into()
                ]),
                description: Some(format!("AWS SDK for Rust - {} @ {}", service.full_name(), service.api_version())),
                documentation: Some("https://rusoto.github.io/rusoto/rusoto_core/index.html".into()),
                keywords: Some(vec!["AWS".into(), "Amazon".into(), name_for_keyword]),
                license: Some("MIT".into()),
                name: crate_name.clone(),
                readme: Some("README.md".into()),
                repository: Some("https://github.com/rusoto/rusoto".into()),
                version: service_config.version.clone(),
                homepage: Some("https://www.rusoto.org/".into()),
                ..cargo::Metadata::default()
            },
            dependencies: service_dependencies,
            dev_dependencies: vec![
                ("rusoto_mock".to_owned(), cargo::Dependency::Extended {
                    path: Some("../../../mock".into()),
                    version: Some("0.25.0".into()),
                    optional: None,
                    default_features: None,
                    features: None
                })
            ].into_iter().collect(),
            ..cargo::Manifest::default()
        };

        cargo_manifest.write_all(toml::to_string(&manifest).unwrap().as_bytes()).unwrap();

        let mut readme_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(crate_dir.join("README.md"))
            .expect("Unable to write README.md");

        let readme = format!(r#"
# Rusoto {short_name}
Rust SDK for {aws_name}

You may be looking for:

* [An overview of Rusoto][rusoto-overview]
* [AWS services supported by Rusoto][supported-aws-services]
* [API documentation][api-documentation]
* [Getting help with Rusoto][rusoto-help]

## Requirements

Rust stable or beta are required to use Rusoto. Nightly is tested, but not guaranteed to be supported. Older
versions _may_ be supported. The currently supported Rust versions can be found in the Rusoto project 
[`travis.yml`](https://github.com/rusoto/rusoto/blob/master/.travis.yml).

On Linux, OpenSSL is required.

## Installation

To use `{crate_name}` in your application, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
{crate_name} = "{version}"
```

## Contributing

See [CONTRIBUTING][contributing].

## License

Rusoto is distributed under the terms of the MIT license.

See [LICENSE][license] for details.

[api-documentation]: https://rusoto.github.io/rusoto/rusoto/ "API documentation"
[license]: https://github.com/rusoto/rusoto/blob/master/LICENSE "MIT License"
[contributing]: https://github.com/rusoto/rusoto/blob/master/CONTRIBUTING.md "Contributing Guide"
[rusoto-help]: https://www.rusoto.org/help.html "Getting help with Rusoto"
[rusoto-overview]: https://www.rusoto.org/ "Rusoto overview"
[supported-aws-services]: https://www.rusoto.org/supported-aws-services.html "List of AWS services supported by Rusoto"
        "#,
        short_name = service.service_type_name(),
        aws_name = service.full_name(),
        crate_name = crate_name,
        version = service_config.version
        );

        readme_file.write_all(readme.as_bytes()).unwrap();

        {
            let src_dir = crate_dir.join("src");

            if !src_dir.exists() {
                fs::create_dir(&src_dir).expect(&format!("Unable to create directory at {}", src_dir.display()));
            }

            let lib_file_path = src_dir.join("lib.rs");

            let mut lib_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&lib_file_path)
                .expect("Unable to write lib.rs");

            let lib_file_contents = format!(r#"
// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

//! {service_docs}
//!
//! If you're using the service, you're probably looking for [{client_name}](struct.{client_name}.html) and [{trait_name}](trait.{trait_name}.html).

{extern_crates}

mod generated;
mod custom;

pub use generated::*;
pub use custom::*;
            "#,
            service_docs = service.documentation().unwrap_or(&service.full_name().to_owned()),
            client_name = service.client_type_name(),
            trait_name = service.service_type_name(),
            extern_crates = extern_crates
            );

            lib_file.write_all(lib_file_contents.as_bytes()).unwrap();

            let gen_file_path = src_dir.join("generated.rs");

            let gen_file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&gen_file_path)
                .expect("Unable to write generated.rs");

            let mut gen_writer = BufWriter::new(gen_file);

            codegen::generate_source(&service, &mut gen_writer).unwrap();

            let custom_dir_path = src_dir.join("custom");

            if !custom_dir_path.exists() {
                fs::create_dir(&custom_dir_path).expect(&format!("Unable to create directory at {}", custom_dir_path.display()));
            }

            let custom_mod_file_path = custom_dir_path.join("mod.rs");

            if !custom_mod_file_path.exists() {
                OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&custom_mod_file_path)
                    .expect("Unable to write mod.rs");
            }
            if log_enabled!(Debug) {
                debug!("Service generation of {} took {}ms", service.full_name(), sw.elapsed_ms());
            }
        }

        {
            let sw = Stopwatch::start_new();
            let src_dir = crate_dir.join("src");
            let gen_file_path = src_dir.join("generated.rs");

            let _ = rustfmt::run(rustfmt::Input::File(gen_file_path), &rustfmt::config::Config {
                write_mode: rustfmt::config::WriteMode::Overwrite,
                error_on_line_overflow: false,
                ..rustfmt::config::Config::default()
            });
            if log_enabled!(Debug) {
                println!("Rustfmt of {} took {}ms", service.full_name(), sw.elapsed_ms());
            }
        }

        {
            let sw = Stopwatch::start_new();
            let test_resources_dir = crate_dir.join("test_resources");

            if !test_resources_dir.exists() {
                fs::create_dir(&test_resources_dir).expect(&format!("Unable to create directory at {}", test_resources_dir.display()));
            }

            let generated_test_resources_dir = test_resources_dir.join("generated");

            if !generated_test_resources_dir.exists() {
                fs::create_dir(&generated_test_resources_dir).expect(&format!("Unable to create directory at {}", generated_test_resources_dir.display()));
            }

            let test_valid_resources = codegen::tests::find_valid_responses_for_service(&service);
            if !test_valid_resources.is_empty() {
                let test_valid_resources_dir = generated_test_resources_dir.join("valid");

                if !test_valid_resources_dir.exists() {
                    fs::create_dir(&test_valid_resources_dir).expect(&format!("Unable to create directory at {}", generated_test_resources_dir.display()));
                }

                for resource in test_valid_resources {
                    fs::copy(resource.full_path, test_valid_resources_dir.join(&resource.file_name)).expect("Failed to copy test resource file");
                }
            }

            let test_error_resources = codegen::tests::find_error_responses_for_service(&service);
            if !test_error_resources.is_empty() {
                let test_error_resources_dir = generated_test_resources_dir.join("error");

                if !test_error_resources_dir.exists() {
                    fs::create_dir(&test_error_resources_dir).expect(&format!("Unable to create directory at {}", generated_test_resources_dir.display()));
                }

                for resource in test_error_resources {
                    fs::copy(resource.full_path, test_error_resources_dir.join(&resource.file_name)).expect("Failed to copy test resource file");
                }
            }
            if log_enabled!(Debug) {
                println!("Service test generation from botocore for {} took {}ms", service.full_name(), sw.elapsed_ms());
            }
        }
    });
}