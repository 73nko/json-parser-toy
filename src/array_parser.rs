use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list;
use nom::sequence::delimited;

use crate::{json_value, Node};
use crate::utils::{JSONParseError, spacey};

pub fn json_array(input: &str) -> IResult<&str, crate::Node, JSONParseError> {
    let parser = delimited(
        spacey(tag("[")),
        separated_list(spacey(tag(",")), json_value),
        spacey(tag("]")),
    );

    map(parser, |v| {
        Node::Array(v)
    })(input)
}


#[test]
fn test_array() {
    assert_eq!(json_array("[ ]"), Ok(("", Node::Array(vec![]))));
    assert_eq!(json_array("[ 1 ]"), Ok(("", Node::Array(vec![Node::Integer(1)]))));

    let expected = Node::Array(vec![Node::Integer(1), Node::Str("x".into())]);
    assert_eq!(json_array(r#" [ 1 , "x" ] "#), Ok(("", expected)));
}
