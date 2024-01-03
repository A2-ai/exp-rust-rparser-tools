use std::vec;

use extendr_api::prelude::*;
mod depedencies;
mod constraints;
use crate::depedencies::{packages_list};

// in the future consider https://josiahparry.com/posts/2023-11-24-dfusionrdr/#handling-arrow-rs-from-r

#[derive(Debug, PartialEq, IntoDataFrameRow)]
struct PackageDependency {
    package: String,
    dependency: Option<String>,
    constraint: Option<String>,
}

#[extendr]
// @export
fn parse_all_package_dependencies(packages: vec::Vec<String>, deps: Robj) -> Robj {
    if packages.len() != deps.len() {
        return Robj::from("Error: Length of packages and dependencies vectors do not match");
    }
    let mut all_dep_pkgs: Vec<PackageDependency> = vec![];
    // originally, I started with deps being a vec of strings too, but you can't make it a vec of optional strings
    // to identify if there are nas - so instead using the iterator
    // one example from the iterator docs was showing the unwrap then check if its na
    // let obj = Robj::from(vec![Some("a"), Some("b"), None]);
    //assert_eq!(obj.as_str_iter().unwrap().map(|s| s.is_na()).collect::<Vec<_>>(), vec![false, false, true]);
    // I also got this idea a little from reading:
    // https://stackoverflow.com/questions/75608152/calling-rust-from-r-error-expected-a-vector-type
    // which shows using an input Robj then matching against the type within the function
    let deps = deps.as_str_iter().unwrap();
    for (package, dep) in packages.iter().zip(deps) {
        if dep.is_na() {
           all_dep_pkgs.push(PackageDependency{
                package: package.to_string(),
                dependency: None,
                constraint: None,
              });
              continue;
           } 
        let parsed_deps = packages_list(&dep);
        match parsed_deps {
            Ok(parsed_deps) => {
                let dep_pkgs: Vec<PackageDependency> = parsed_deps.iter().map(|dep| {
                    let constraint = match &dep.constraint {
                        Some(c) => Some(c.version.clone()),
                        None => None,
                    };
                    PackageDependency {
                        package: package.to_string(),
                        dependency: Some(dep.name.clone()),
                        constraint: constraint,
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
                let constraint = match &dep.constraint {
                    Some(c) => Some(c.version.clone()),
                    None => None,
                };
                PackageDependency {
                    package: package.to_string(),
                    dependency: Some(dep.name.clone()),
                    constraint: constraint,
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
