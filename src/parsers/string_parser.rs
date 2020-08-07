use escape8259::unescape;
use nom::{branch::alt, IResult};
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{map, recognize};
use nom::multi::many0;
use nom::sequence::delimited;

use crate::Node;
use crate::utils::{escape_code, JSONParseError};

// A character that is:
// NOT a control character (0x00 - 0x1F)
// NOT a quote character (0x22)
// NOT a backslash character (0x5C)
// Is within the unicode range (< 0x10FFFF) (this is already guaranteed by Rust char)
fn is_nonescaped_string_char(c: char) -> bool {
    let cv = c as u32;
    (cv >= 0x20) && (cv != 0x22) && (cv != 0x5C)
}

// One or more unescaped text characters
fn nonescaped_string(input: &str) -> IResult<&str, &str, JSONParseError> {
    take_while1(is_nonescaped_string_char)(input)
}


// Zero or more text characters
fn string_body(input: &str) -> IResult<&str, &str, JSONParseError> {
    recognize(
        many0(
            alt((
                nonescaped_string,
                escape_code
            ))
        )
    )(input)
}

pub fn string_literal(input: &str) -> IResult<&str, String, JSONParseError> {
    let (remain, raw_string) = delimited(
        tag("\""),
        string_body,
        tag("\""),
    )(input)?;

    match unescape(raw_string) {
        Ok(s) => Ok((remain, s)),
        Err(_) => Err(nom::Err::Failure(JSONParseError::BadEscape)),
    }
}

pub fn json_string(input: &str) -> IResult<&str, Node, JSONParseError> {
    map(string_literal, |s| {
        Node::Str(s)
    })(input)
}


#[test]
fn test_string() {
    // Plain Unicode strings with no escaping
    assert_eq!(json_string(r#""""#), Ok(("", Node::Str("".into()))));
    assert_eq!(json_string(r#""Hello""#), Ok(("", Node::Str("Hello".into()))));
    assert_eq!(json_string(r#""の""#), Ok(("", Node::Str("の".into()))));
    assert_eq!(json_string(r#""𝄞""#), Ok(("", Node::Str("𝄞".into()))));

    // valid 2-character escapes
    assert_eq!(json_string(r#""  \\  ""#), Ok(("", Node::Str("  \\  ".into()))));
    assert_eq!(json_string(r#""  \"  ""#), Ok(("", Node::Str("  \"  ".into()))));

    // valid 6-character escapes
    assert_eq!(json_string(r#""\u0000""#), Ok(("", Node::Str("\x00".into()))));
    assert_eq!(json_string(r#""\u00DF""#), Ok(("", Node::Str("ß".into()))));
    assert_eq!(json_string(r#""\uD834\uDD1E""#), Ok(("", Node::Str("𝄞".into()))));

    // Invalid because surrogate characters must come in pairs
    assert!(json_string(r#""\ud800""#).is_err());
    // Unknown 2-character escape
    assert!(json_string(r#""\x""#).is_err());
    // Not enough hex digits
    assert!(json_string(r#""\u""#).is_err());
    assert!(json_string(r#""\u001""#).is_err());
    // Naked control character
    assert!(json_string(r#""\x0a""#).is_err());
    // Not a JSON string because it's not wrapped in quotes
    assert!(json_string("abc").is_err());
    // An unterminated string (because the trailing quote is escaped)
    assert!(json_string(r#""\""#).is_err());

    // Parses correctly but has escape errors due to incomplete surrogate pair.
    assert_eq!(json_string(r#""\ud800""#), Err(nom::Err::Failure(JSONParseError::BadEscape)));
}