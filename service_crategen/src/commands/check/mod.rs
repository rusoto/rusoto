use std::collections::BTreeMap;

use crate::{ServiceConfig, ServiceDefinition};

pub fn check(services: BTreeMap<String, ServiceConfig>) {
    let definitions = ServiceDefinition::load_all().unwrap();

    let mut missing = vec![];
    let mut outdated = vec![];

    for (name, definition) in definitions {
        if let Some(config) = services.get(&name) {
            if config.protocol_version != definition.metadata.api_version {
                outdated.push((name, config, definition));
            }
        } else {
            missing.push((name, definition));
        }
    }

    if !missing.is_empty() {
        println!();
        println!("Missing Services");
        println!("================");

        for (name, definition) in missing {
            println!(
                "{} ({}, {})",
                &name, &definition.metadata.protocol, &definition.metadata.api_version
            );
        }
    }

    if !outdated.is_empty() {
        println!();
        println!("Outdated Services");
        println!("=================");

        for (name, config, definition) in outdated {
            println!(
                "{} ({} => {})",
                &name, &config.protocol_version, &definition.metadata.api_version
            );
        }
    }
}
