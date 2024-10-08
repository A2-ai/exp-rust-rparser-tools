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
use crate::constraints;

#[derive(Debug, PartialEq, Clone)]
pub struct Constraint {
    pub version: String,
    pub operator: constraints::VersionConstraint,
}


#[derive(Debug, PartialEq, Clone)]
pub struct Dependency {
    pub name: String,
    pub constraint: Option<Constraint>,
}

pub fn package_dependency(input: &str) -> IResult<&str, Dependency> {
    let (input, (name, opt_constraint, _)) = tuple((
        take_while1(|c: char| c.is_alphanumeric() || c == '.' || c == '-'),
        opt(constraint),
        multispace0,
    ))(input)?;

    Ok((
        input,
        Dependency {
            name: name.trim().to_string(),
            constraint: opt_constraint,
        },
    ))
}


pub fn constraint(input: &str) -> IResult<&str, Constraint> {
    let (input, (_, _, _, constraint, _, version, _, _)) = tuple((
        multispace0,
        tag("("),
        multispace0,
        alt((tag(">="), tag("=="), tag(">"), tag("<"), tag("<="))),
        //opt(line_ending),
        multispace0,
        take_while1(|c: char| c.is_numeric() || c == '.' || c == '-'),
        multispace0,
        tag(")"),
    ))(input)?;
    Ok((
        input,
        Constraint {
            operator: constraint.parse::<constraints::VersionConstraint>().unwrap_or(constraints::VersionConstraint::NotParseable),
            version: version.to_string(),
        },
    ))
}


pub fn packages_list(input: &str) -> Result<Vec<Dependency>, nom::Err<nom::error::Error<&str>>> {
    let (input, leading_elements) =
        many0(tuple((package_dependency, char(','), multispace0)))(input)?;
    let leading_elements: Vec<_> = leading_elements
        .into_iter()
        .map(|(elem, _, _)| elem)
        .collect();

    if input.is_empty() {
        return Ok(leading_elements);
    } 
    let mut all_elements = leading_elements;
    let (input, last_element) = package_dependency(input)?;
    all_elements.push(last_element);
    Ok(all_elements)
}

