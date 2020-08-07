use nom::{branch::alt, IResult};
use nom::bytes::complete::tag;
use nom::character::complete::{digit0, one_of};
use nom::combinator::{opt, recognize};
use nom::sequence::pair;

use crate::JSONParseError;
use crate::Node;

// This can be done a few different ways:
// one_of("123456789"),
// anychar("0123456789"),
// we could also extract the character value as u32 and do range checks...

fn digit1to9(input: &str) -> IResult<&str, char, JSONParseError> {
    one_of("123456789")
        (input)
}

// unsigned_integer = zero / ( digit1-9 *DIGIT )
pub fn uint(input: &str) -> IResult<&str, &str, JSONParseError> {
    alt((
        tag("0"),
        recognize(
            pair(
                digit1to9,
                digit0,
            )
        )
    ))
        (input)
}

fn integer_body(input: &str) -> IResult<&str, &str, JSONParseError> {
    recognize(
        pair(
            opt(tag("-")),
            uint,
        )
    )
        (input)
}

pub fn json_integer(input: &str) -> IResult<&str, Node, JSONParseError> {
    let (remain, raw_int) = integer_body(input)?;
    match raw_int.parse::<i64>() {
        Ok(i) => Ok((remain, Node::Integer(i))),
        Err(_) => Err(nom::Err::Failure(JSONParseError::BadInt)),
    }
}


#[test]
fn test_integer() {
    assert_eq!(json_integer("42"), Ok(("", Node::Integer(42))));
    assert_eq!(json_integer("-123"), Ok(("", Node::Integer(-123))));
    assert_eq!(json_integer("0"), Ok(("", Node::Integer(0))));
    assert_eq!(json_integer("01"), Ok(("1", Node::Integer(0))));
    assert_eq!(json_integer("9999999999999999999"), Err(nom::Err::Failure(JSONParseError::BadInt)));
}