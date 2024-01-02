extern crate nom;
mod constraints;
mod depedencies;
use crate::depedencies::{Constraint, Dependency, package_dependency, packages_list};
use crate::constraints::VersionConstraint;
use nom::{
    branch::{alt},
    bytes::complete::{tag, take_while1},
    character::complete::multispace0,
    character::complete::*,
    combinator::{opt},
    multi::many0,
    IResult,
    sequence::{tuple},
};
use debcontrol::parse_str;
use debcontrol::{Field, Paragraph};

const ABACUS_DESC: &str = "Package: ABACUS\nVersion: 1.0.0\nDepends: R (>= 3.1.0)\nImports: ggplot2 (>= 3.1.0), shiny (>= 1.3.1),\nSuggests: rmarkdown (>= 1.13), knitr (>= 1.22)\nLicense: GPL-3\nMD5sum: 50c54c4da09307cb95a70aaaa54b9fbd\nNeedsCompilation: no\n";
 
const DPLYR_DESC: &str = r#"
Type: Package
Package: dplyr
Title: A Grammar of Data Manipulation
Version: 1.1.4
Authors@R: c( person("Hadley", "Wickham", , "hadley@posit.co", role =
        c("aut", "cre"), comment = c(ORCID = "0000-0003-4757-117X")),
        person("Romain", "François", role = "aut", comment = c(ORCID =
        "0000-0002-2444-4226")), person("Lionel", "Henry", role = "aut"),
        person("Kirill", "Müller", role = "aut", comment = c(ORCID =
        "0000-0002-1416-3412")), person("Davis", "Vaughan", ,
        "davis@posit.co", role = "aut", comment = c(ORCID =
        "0000-0003-4777-038X")), person("Posit Software, PBC", role =
        c("cph", "fnd")) )
Description: A fast, consistent tool for working with data frame like
        objects, both in memory and out of memory.
License: MIT + file LICENSE
URL: https://dplyr.tidyverse.org, https://github.com/tidyverse/dplyr
BugReports: https://github.com/tidyverse/dplyr/issues
Depends: R (>= 3.5.0)
Imports: cli (>= 3.4.0), generics, glue (>= 1.3.2), lifecycle (>= 1.0.3),
        magrittr (>= 1.5), methods, pillar (>= 1.9.0), R6, rlang (>=
        1.1.0), tibble (>= 3.2.0), tidyselect (>= 1.2.0), utils, vctrs (>=
        0.6.4)
Suggests: bench, broom, callr, covr, DBI, dbplyr (>= 2.2.1), ggplot2,
        knitr, Lahman, lobstr, microbenchmark, nycflights13, purrr,
        rmarkdown, RMySQL, RPostgreSQL, RSQLite, stringi (>= 1.7.6),
        testthat (>= 3.1.5), tidyr (>= 1.3.0), withr
VignetteBuilder: knitr
Config/Needs/website: tidyverse, shiny, pkgdown, tidyverse/tidytemplate
Config/testthat/edition: 3
Encoding: UTF-8
LazyData: true
RoxygenNote: 7.2.3
NeedsCompilation: yes
Packaged: 2023-11-16 21:48:56 UTC; hadleywickham
Author: Hadley Wickham [aut, cre]
        (<https://orcid.org/0000-0003-4757-117X>), Romain François [aut]
        (<https://orcid.org/0000-0002-2444-4226>), Lionel Henry [aut],
        Kirill Müller [aut] (<https://orcid.org/0000-0002-1416-3412>),
        Davis Vaughan [aut] (<https://orcid.org/0000-0003-4777-038X>),
        Posit Software, PBC [cph, fnd]
Maintainer: Hadley Wickham <hadley@posit.co>
Repository: RSPM
Date/Publication: 2023-11-17 16:50:02 UTC
Built: R 4.3.0; x86_64-pc-linux-gnu; 2023-11-20 12:40:25 UTC; unix
"#;

const MASS_DESC: &str = r#"
Package: MASS
Priority: recommended
Version: 7.3-60
Date: 2023-05-02
Revision: $Rev: 3621 $
Depends: R (>= 4.0), grDevices, graphics, stats, utils
Imports: methods
Suggests: lattice, nlme, nnet, survival
Authors@R: c(person("Brian", "Ripley", role = c("aut", "cre", "cph"),
        email = "ripley@stats.ox.ac.uk"), person("Bill", "Venables", role
        = "ctb"), person(c("Douglas", "M."), "Bates", role = "ctb"),
        person("Kurt", "Hornik", role = "trl", comment = "partial port ca
        1998"), person("Albrecht", "Gebhardt", role = "trl", comment =
        "partial port ca 1998"), person("David", "Firth", role = "ctb"))
Description: Functions and datasets to support Venables and Ripley,
        "Modern Applied Statistics with S" (4th edition, 2002).
Title: Support Functions and Datasets for Venables and Ripley's MASS
LazyData: yes
ByteCompile: yes
License: GPL-2 | GPL-3
URL: http://www.stats.ox.ac.uk/pub/MASS4/
Contact: <MASS@stats.ox.ac.uk>
NeedsCompilation: yes
Packaged: 2023-05-02 16:42:41 UTC; ripley
Author: Brian Ripley [aut, cre, cph], Bill Venables [ctb], Douglas M.
        Bates [ctb], Kurt Hornik [trl] (partial port ca 1998), Albrecht
        Gebhardt [trl] (partial port ca 1998), David Firth [ctb]
Maintainer: Brian Ripley <ripley@stats.ox.ac.uk>
Repository: CRAN
Date/Publication: 2023-05-04 07:32:21 UTC
Built: R 4.3.2; aarch64-apple-darwin20; 2023-11-01 22:07:25 UTC; unix
"#;

struct RDescription {
    ptype: String,
    package: String,
    priority: String,
    version: String,
    date: String,
    revision: String,
    depends: String,
    imports: String,
    suggests: String,
    authors: String,
    description: String,
    title: String,
    lazy_data: String,
    byte_compile: String,
    license: String,
    url: String,
    contact: String,
    needs_compilation: String,
    packaged: String,
    roxygen_note: String,
    config_needs_website: String,
    config_testthat_edition: String,
    vignette_builder: String,
    repository: String,
    date_publication: String,
    maintainer: String,
}

fn get_field<'a>(paragraph: &'a Paragraph, name: &str) -> Field<'a> {
    let res = paragraph.fields.iter().find(|field| field.name == name);
    if let Some(field) = res {
        return field.clone();
    }
    return Field {
        name: "",
        value: "".to_string(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_no_constraint() {
        let pkg = "shiny";
        let (_, parsed) = package_dependency(pkg).unwrap();

        assert_eq!(parsed.name, "shiny");
        assert_eq!(
            parsed.constraint,
            None 
        );
    }

    #[test]
    fn test_constraint_gte() {
        let example1 = "R (>= 3.2.0)";
        let (_, parsed) = package_dependency(example1).unwrap();

        assert_eq!(parsed.name, "R");
        assert_eq!(
            parsed.constraint,
            Some(Constraint {
                operator: Ok(VersionConstraint::GreaterThanEqual),
                version: "3.2.0".to_string(),
            })
        );
    }
    #[test]
    fn test_constraint_gt() {
        let example1 = "R (> 3.2.0)";
        let (_, parsed) = package_dependency(example1).unwrap();

        assert_eq!(parsed.name, "R");
        assert_eq!(
            parsed.constraint,
            Some(Constraint {
                operator: Ok(VersionConstraint::GreaterThan),
                version: "3.2.0".to_string(),
            })
        );
    }

    #[test]
    fn test_example_with_linebreak_after_constraint() {
        // AFM version 2.0 Imports
        //let example4 = "data.table(>= 1.9.6),stringr(>= 1.0.0),gstat(>=\n        1.0-26),fractaldim(>= 0.8-4),rgl(>= 0.96),pracma(>=\n        1.8.6)";
        let testdata = "data.table(>= 1.9.6),stringr(>= 1.0.0),gstat(>=\n        1.0-26),fractaldim(>= 0.8-4),rgl(>= 0.96),pracma(>=\n        1.8.6)";
        let parsed = packages_list(testdata).unwrap();
        assert_eq!(parsed.len(), 6);
        assert_eq!(parsed[2].name, "gstat");
        assert_eq!(
            parsed[2].constraint,
            Some(Constraint {
                operator: Ok(VersionConstraint::GreaterThanEqual),
                version: "1.0-26".to_string(),
            })
        );
    }

    #[test]
    fn test_trailing_comma() {
        // from Package: ABACUS version 1.0.0 Imports
        let trailing_comma_data = "ggplot2 (>= 3.1.0), shiny (>= 1.3.1),";
        let parsed = packages_list(trailing_comma_data).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].name, "ggplot2");
        assert_eq!(
            parsed[0].constraint,
            Some(Constraint {
                operator: Ok(VersionConstraint::GreaterThanEqual),
                version: "3.1.0".to_string(),
            })
        );
        assert_eq!(parsed[1].name, "shiny");
        assert_eq!(
            parsed[1].constraint,
            Some(Constraint {
                operator: Ok(VersionConstraint::GreaterThanEqual),
                version: "1.3.1".to_string(),
            })
        );
    }
    
}




fn main() {
    let paragraph = parse_str(ABACUS_DESC).unwrap().into_iter().nth(0).unwrap();
    println!("all fields: ");
    paragraph.fields.iter().for_each(|field| {
        println!("{}: {}", field.name, field.value);
    });
    let desc = RDescription {
        ptype: get_field(&paragraph, "Type").value,
        package: get_field(&paragraph, "Package").value,
        priority: get_field(&paragraph, "Priority").value,
        version: get_field(&paragraph, "Version").value,
        date: get_field(&paragraph, "Date").value,
        revision: get_field(&paragraph, "Revision").value,
        depends: get_field(&paragraph, "Depends").value,
        imports: get_field(&paragraph, "Imports").value,
        suggests: get_field(&paragraph, "Suggests").value,
        authors: get_field(&paragraph, "Authors@R").value,
        description: get_field(&paragraph, "Description").value,
        title: get_field(&paragraph, "Title").value,
        lazy_data: get_field(&paragraph, "LazyData").value,
        byte_compile: get_field(&paragraph, "ByteCompile").value,
        license: get_field(&paragraph, "License").value,
        url: get_field(&paragraph, "URL").value,
        contact: get_field(&paragraph, "Contact").value,
        needs_compilation: get_field(&paragraph, "NeedsCompilation").value,
        packaged: get_field(&paragraph, "Packaged").value,
        roxygen_note: get_field(&paragraph, "RoxygenNote").value,
        config_needs_website: get_field(&paragraph, "Config/Needs/website").value,
        config_testthat_edition: get_field(&paragraph, "Config/testthat/edition").value,
        vignette_builder: get_field(&paragraph, "VignetteBuilder").value,
        repository: get_field(&paragraph, "Repository").value,
        date_publication: get_field(&paragraph, "Date/Publication").value,
        maintainer: get_field(&paragraph, "Maintainer").value,

    };

    println!("package: {}", desc.package);
    println!("imports: {}", desc.imports);
}
