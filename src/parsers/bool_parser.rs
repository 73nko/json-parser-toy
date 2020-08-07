use nom::{branch::alt, IResult};
use nom::bytes::complete::tag;
use nom::combinator::value;

use crate::Node;
use crate::utils::JSONParseError;

pub fn json_bool(input: &str) -> IResult<&str, Node, JSONParseError> {
    alt((
        value(Node::Bool(false), tag("false")),
        value(Node::Bool(true), tag("true")),
    ))
        (input)
}


#[test]
fn test_bool() {
    assert_eq!(json_bool("false"), Ok(("", Node::Bool(false))));
    assert_eq!(json_bool("true"), Ok(("", Node::Bool(true))));
    assert!(json_bool("foo").is_err());
}

