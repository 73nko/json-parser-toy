use crate::utils::JSONParseError;
use crate::Node;

use nom::{branch::alt, IResult};
use nom::combinator::value;
use nom::bytes::complete::tag;

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

