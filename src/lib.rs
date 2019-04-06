//! cfg_feature_groups: define feature groups to improve conditional
//! compilation management.
//!
//! This library allows one to define feature group, that may only take one
//! value out of set. Feature groups defined in `Cargo.toml` as metadata:
//! ```Cargo.toml
//! [package.metadata.feature_groups]
//! log = ["dummy", "semihosting", "itm"]
//! ```
//! Where `"dummy", "semihosting", "itm"` are features defined separately.
//! Once feature groups are defined, they can be checked during build time
//! and turned into cfg attributes:
//! ```Cargo.toml
//! [build-dependencies]
//! cfg_feature_groups = "...";
//! ```
//!
//! ```build.rs
//! use cfg_feature_groups::setup_feature_groups;
//! fn main() {
//!     setup_feature_groups();
//! }
//! ```
//!
//! Then in your program you may use something like this:
//! ```main.rs
//! #[cfg(log = "itm")]
//! fn define_itm() {}
//! ```
//!
//! `setup_feature_groups` will ensure that one and only one option is defined
//! for each feature group.
//!
//! See full example in `example` dir.
#![deny(missing_docs)]

use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use toml;

/// Reads feature_groups from Cargo metadata and process supplied features.
/// Checks that for every feature group only one option is provided. Feature
/// groups are turned into `cfg` attributes with supplied value.
pub fn setup_feature_groups() {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("Cargo.toml");
    let cargo_toml = path.as_path();
    let mut cargo_file = File::open(cargo_toml).unwrap();
    let mut content = String::new();
    cargo_file.read_to_string(&mut content).unwrap();
    let value = content.parse::<toml::Value>().unwrap();
    let feature_groups = value
        .get("package")
        .and_then(|d| d.get("metadata"))
        .and_then(|d| d.get("feature_groups"));
    let features: BTreeSet<String> = env::vars()
        .into_iter()
        .filter(|(k, _v)| k.starts_with("CARGO_FEATURE"))
        .map(|(k, _v)| to_feature_name(k))
        .collect();
    if let Some(toml::Value::Table(table)) = feature_groups {
        process_feature_groups(table, &features)
    }

    // rerun if set of features changed
    env::vars()
        .into_iter()
        .filter(|(k, _v)| k.starts_with("CARGO_FEATURE"))
        .for_each(|(k, _)| println!("cargo:rerun-if-env-changed={}", k));
}

fn to_feature_name(s: String) -> String {
    s.replace("CARGO_FEATURE_", "").to_lowercase()
}

fn process_feature_groups(
    groups: &toml::value::Table,
    defined_features: &BTreeSet<String>,
) {
    groups.into_iter().for_each(|(k, v)| {
        if let toml::Value::Array(group_features) = v {
            process_feature_group(k, &group_features, defined_features)
        } else {
            panic!("Feature group should be defined as array of features!, invalid group {}: {:?}", k, v);
        }
    });
}

fn process_feature_group(
    group_name: &str,
    group_features: &Vec<toml::Value>,
    defined_features: &BTreeSet<String>,
) {
    let mut defined = false;
    for group_feature in group_features.iter() {
        if let toml::Value::String(gf) = group_feature {
            let this_defined = defined_features.contains(gf);
            if defined && this_defined {
                panic!(
                    "Multiple options defined for feature group {}!",
                    group_name
                );
            }
            if this_defined {
                defined = this_defined;
                println!("cargo:rustc-cfg={}=\"{}\"", group_name, gf);
            }
        } else {
            panic!("Invalid definition of {} feature group; features should be strings, found: {:?}",
                   group_name, group_feature)
        }
    }
    if !defined {
        panic!("No features defined for feature group {}!", group_name)
    }
}
