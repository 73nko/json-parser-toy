use nom::{branch::alt, IResult};
use nom::combinator::all_consuming;

use array_parser::json_array;
use bool_parser::json_bool;
use float_parser::json_float;
use integer_parser::json_integer;
use null_parser::json_null;
use object_parser::json_object;
use string_parser::json_string;
use utils::{JSONParseError, spacey};

mod utils;
mod integer_parser;
mod float_parser;
mod string_parser;
mod bool_parser;
mod null_parser;
mod array_parser;
mod object_parser;


#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Str(String),
    Array(Vec<Node>),
    Object(Vec<(String, Node)>),
}

pub fn parse_json(input: &str) -> Result<Node, JSONParseError> {
    let (_, result) = all_consuming(json_value)(input).map_err(|nom_err| {
        match nom_err {
            nom::Err::Incomplete(_) => unreachable!(),
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
        }
    })?;
    Ok(result)
}

fn json_value(input: &str) -> IResult<&str, Node, JSONParseError> {
    spacey(alt((
        json_array,
        json_object,
        json_string,
        json_float,
        json_integer,
        json_bool,
        json_null
    )))(input)
}


#[test]
fn test_values() {
    assert_eq!(parse_json(" 56 "), Ok(Node::Integer(56)));
    assert_eq!(parse_json(" 78.0 "), Ok(Node::Float(78.0)));
    assert_eq!(parse_json(r#" "Hello" "#), Ok(Node::Str("Hello".into())));
    // These two tests aren't relevant for JSON. They verify that `json_float`
    // will never mistake integers for floats in other grammars that might
    // allow a `.` or `e` character after a literal integer.
    assert_eq!(json_value("123else"), Ok(("else", Node::Integer(123))));
    assert_eq!(json_value("123.x"), Ok((".x", Node::Integer(123))));

    assert_eq!(parse_json("123else"), Err(JSONParseError::Unparseable));
    assert_eq!(parse_json("123.x"), Err(JSONParseError::Unparseable));
    assert_eq!(parse_json("[ 56, "), Err(JSONParseError::Unparseable));
    assert_eq!(parse_json(r#"{ "a": "b" "#), Err(JSONParseError::Unparseable));
    assert_eq!(parse_json(" 56 a"), Err(JSONParseError::Unparseable));

    assert_eq!(parse_json("9999999999999999999"), Err(JSONParseError::BadInt));
    assert_eq!(parse_json(r#""\ud800""#), Err(JSONParseError::BadEscape));
}