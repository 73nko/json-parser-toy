use crate::utils::JSONParseError;
use crate::Node;

use nom::IResult;
use nom::combinator::value;
use nom::bytes::complete::tag;


pub fn json_null(input: &str) -> IResult<&str, Node, JSONParseError> {
    value(Node::Null, tag("null"))(input)
}


#[test]
fn test_null() {
    assert_eq!(json_null("null"), Ok(("", Node::Null)));
}