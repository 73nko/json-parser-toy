use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;
use nom::multi::separated_list;
use nom::sequence::{delimited, separated_pair};

use crate::{json_value, Node};
use crate::string_parser::string_literal;
use crate::utils::{JSONParseError, spacey};

// "key: value", where key and value are any JSON type.
fn object_member(input: &str) -> IResult<&str, (String, Node), JSONParseError> {
    separated_pair(string_literal, spacey(tag(":")), json_value)
        (input)
}

pub fn json_object(input: &str) -> IResult<&str, Node, JSONParseError> {
    let parser = delimited(
        spacey(tag("{")),
        separated_list(
            spacey(tag(",")),
            object_member,
        ),
        spacey(tag("}")),
    );
    map(parser, |v| {
        Node::Object(v)
    })
        (input)
}


#[test]
fn test_object() {
    assert_eq!(json_object("{ }"), Ok(("", Node::Object(vec![]))));
    let expected = Node::Object(vec![("1".into(), Node::Integer(2))]);
    assert_eq!(json_object(r#" { "1" : 2 } "#), Ok(("", expected)));
}
