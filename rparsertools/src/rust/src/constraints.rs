

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParseError(Vec<String>);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for err in &self.0 {
            writeln!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VersionConstraint {
    GreaterThanEqual, // >=
    Equal,            // ==
    GreaterThan,      // >
    None,             // ""
}

impl std::str::FromStr for VersionConstraint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(VersionConstraint::None),
            ">=" => Ok(VersionConstraint::GreaterThanEqual),
            "==" => Ok(VersionConstraint::Equal),
            ">" => Ok(VersionConstraint::GreaterThan),
            _ => Err(ParseError(vec![format!(
                "Invalid version constraint: {}",
                s
            )])),
        }
    }
}

impl ToString for VersionConstraint {
    fn to_string(&self) -> String {
        match self {
            VersionConstraint::GreaterThanEqual => ">=".to_owned(),
            VersionConstraint::Equal => "==".to_owned(),
            VersionConstraint::GreaterThan => ">".to_owned(),
            VersionConstraint::None => "".to_owned(),
        }
    }
}