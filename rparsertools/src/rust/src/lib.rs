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
    name: String,
    constraint: String,
}

#[extendr]
// @export
fn parse_package_dependencies(package: &str, deps: &str) -> Robj {
    let parsed_deps = packages_list(deps);
    match parsed_deps {
        Ok(parsed_deps) => {
            let dep_pkgs: Vec<PackageDependency> = parsed_deps.iter().map(|dep| {
                PackageDependency {
                    name: dep.name.clone(),
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
}
