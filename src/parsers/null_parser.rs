use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;

use crate::Node;
use crate::utils::JSONParseError;

pub fn json_null(input: &str) -> IResult<&str, Node, JSONParseError> {
    value(Node::Null, tag("null"))(input)
}


#[test]
fn test_null() {
    assert_eq!(json_null("null"), Ok(("", Node::Null)));
}