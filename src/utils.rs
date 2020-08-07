use nom::{branch::alt, IResult};
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::recognize;
use nom::sequence::{delimited, pair};
use nom::error::{ErrorKind, ParseError};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum JSONParseError {
    #[error("bad integer")]
    BadInt,
    #[error("bad float")]
    BadFloat,
    #[error("bad escape sequence")]
    BadEscape,
    #[error("unknown parser error")]
    Unparseable,
}

impl<I> ParseError<I> for JSONParseError {
    fn from_error_kind(_input: I, _kind: ErrorKind) -> Self {
        // Because JSONParseError is a simplified public error type,
        // we discard the nom error parameters.
        JSONParseError::Unparseable
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

pub fn spacey<F, I, O, E>(f: F) -> impl Fn(I) -> IResult<I, O, E>
    where
        F: Fn(I) -> IResult<I, O, E>,
        I: nom::InputTakeAtPosition,
        <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
        E: nom::error::ParseError<I>,
{
    delimited(multispace0, f, multispace0)
}

// There are only two types of escape allowed by RFC 8259.
// - single-character escapes \" \\ \/ \b \f \n \r \t
// - general-purpose \uXXXX
// Note: we don't enforce that escape codes are valid here.
// There must be a decoder later on.
pub fn escape_code(input: &str) -> IResult<&str, &str, JSONParseError> {
    recognize(
        pair(
            tag("\\"),
            alt((
                tag("\""),
                tag("\\"),
                tag("/"),
                tag("b"),
                tag("f"),
                tag("n"),
                tag("r"),
                tag("t"),
                tag("u"),
            ))
        )
    )
        (input)
}