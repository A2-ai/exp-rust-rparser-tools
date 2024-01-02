use std::vec;

use extendr_api::prelude::*;
mod depedencies;
mod constraints;
use crate::depedencies::{packages_list};
/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}
// in the future consider https://josiahparry.com/posts/2023-11-24-dfusionrdr/#handling-arrow-rs-from-r

#[derive(Debug, PartialEq, IntoDataFrameRow)]
struct PackageDependency {
    package: String,
    dependency: String,
    constraint: String,
}

#[extendr]
// @export
fn parse_all_package_dependencies(packages: vec::Vec<String>, deps: vec::Vec<String>) -> Robj {
    if packages.len() != deps.len() {
        return Robj::from("Error: Length of packages and dependencies vectors do not match");
    }
    let mut all_dep_pkgs: Vec<PackageDependency> = vec![];
    for (package, dep) in packages.iter().zip(deps.iter()) {
        let parsed_deps = packages_list(dep);
        match parsed_deps {
            Ok(parsed_deps) => {
                let dep_pkgs: Vec<PackageDependency> = parsed_deps.iter().map(|dep| {
                    PackageDependency {
                        package: package.to_string(),
                        dependency: dep.name.clone(),
                        constraint: dep.constraint.as_ref()
                                      .map_or_else(|| "".to_string(), |c| c.version.clone()),
                    }
                }).collect();
                all_dep_pkgs.extend(dep_pkgs);
            },
            Err(_) => return Robj::from("Error parsing dependencies")
        }
    }
    match all_dep_pkgs.into_dataframe() {
        Ok(dataframe) => dataframe.as_robj().clone(),
        Err(err) => Robj::from(format!("Error converting to DataFrame: {}", err))
    } 
}

#[extendr]
// @export
fn parse_package_dependencies(package: &str, deps: &str) -> Robj {
    let parsed_deps = packages_list(deps);
    match parsed_deps {
        Ok(parsed_deps) => {
            let dep_pkgs: Vec<PackageDependency> = parsed_deps.iter().map(|dep| {
                PackageDependency {
                    package: package.to_string(),
                    dependency: dep.name.clone(),
                    constraint: dep.constraint.as_ref()
                                  .map_or_else(|| "".to_string(), |c| c.version.clone()),
                }
            }).collect();
            match dep_pkgs.into_dataframe() {
                Ok(dataframe) => dataframe.as_robj().clone(),
                Err(err) => Robj::from(format!("Error converting to DataFrame: {}", err))
            }
        },
        Err(_) => Robj::from("Error parsing dependencies")
    }
}
// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rparsertools;
    fn parse_package_dependencies;
    fn parse_all_package_dependencies;
}
