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

#[extendr]
// @export
fn parse_package_dependencies(package: &str, deps: &str) -> std::result::Result<List, String> {
   let parsed_deps = packages_list(deps);
   if parsed_deps.is_err() {
      return Err("Error parsing dependencies".to_string());
   }
    let parsed_deps = parsed_deps.unwrap();
   let dep_pkgs: Vec<String> = parsed_deps.iter().map(|dep| dep.name.clone() ).collect();
   Ok(list!(name = package, deps = dep_pkgs ))
}
// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rparsertools;
    fn parse_package_dependencies;
}
