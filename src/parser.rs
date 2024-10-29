use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alphanumeric1, char, multispace0, space1},
    combinator::map,
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug)]
enum AttributeType {
    Integrator,
    Transform,
    Sampler,
    PixelFilter,
    Film,
}

#[derive(Debug)]
pub struct Attribute {
    attr_type: AttributeType,
    parameters: HashMap<String, String>,
}

// Helper to parse an identifier
pub fn identifier(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

// Parser for attribute types
pub fn attribute_type(input: &str) -> IResult<&str, AttributeType> {
    alt((
        map(tag("Integrator"), |_| AttributeType::Integrator),
        map(tag("Transform"), |_| AttributeType::Transform),
        map(tag("Sampler"), |_| AttributeType::Sampler),
        map(tag("PixelFilter"), |_| AttributeType::PixelFilter),
        map(tag("Film"), |_| AttributeType::Film),
    ))(input)
}

// Parser for parameters (e.g., "integer maxdepth" [ 65 ])
pub fn parameter(input: &str) -> IResult<&str, (String, String)> {
    let (input, key) = preceded(multispace0, identifier)(input)?;
    let (input, _) = space1(input)?;
    let (input, value) = delimited(
        char('"'),
        take_while(|c: char| c != '"'),
        char('"')
    )(input)?;

    Ok((input, (key.to_string(), value.to_string())))
}

// Parse an entire attribute block
pub fn attribute(input: &str) -> IResult<&str, Attribute> {
    let (input, attr_type) = attribute_type(input)?;
    let (input, _) = space1(input)?;

    // Collect parameters in a hashmap
    let (input, parameters) = map(
        separated_list0(char('\n'), parameter),
        |param_vec| param_vec.into_iter().collect::<HashMap<String, String>>()
    )(input)?;

    Ok((input, Attribute { attr_type, parameters }))
}

// Parse the entire file
pub fn parse_file(input: &str) -> IResult<&str, Vec<Attribute>> {
    separated_list0(char('\n'), attribute)(input)
}
