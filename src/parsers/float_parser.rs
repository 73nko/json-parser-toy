use nom::{branch::alt, IResult};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{opt, recognize};
use nom::sequence::{pair, tuple};

use crate::JSONParseError;
use crate::Node;
use crate::parsers::integer_parser::uint;

// number = [ minus ] int [ frac ] [ exp ]
//
//       decimal-point = %x2E       ; .
//       digit1-9 = %x31-39         ; 1-9
//       e = %x65 / %x45            ; e E
//       exp = e [ minus / plus ] 1*DIGIT
//       frac = decimal-point 1*DIGIT
//       int = zero / ( digit1-9 *DIGIT )
//       minus = %x2D               ; -
//       plus = %x2B                ; +
//       zero = %x30                ; 0

fn frac(input: &str) -> IResult<&str, &str, JSONParseError> {
    recognize(
        pair(
            tag("."),
            digit1,
        )
    )
        (input)
}

fn exp(input: &str) -> IResult<&str, &str, JSONParseError> {
    recognize(
        tuple((
            tag("e"),
            opt(alt((
                tag("-"),
                tag("+")
            ))),
            digit1
        ))
    )
        (input)
}

fn float_body(input: &str) -> IResult<&str, &str, JSONParseError> {
    recognize(
        tuple((
            opt(tag("-")),
            uint,
            alt((
                recognize(pair(
                    frac,
                    opt(exp),
                )),
                exp
            )),
        ))
    )
        (input)
}

pub fn json_float(input: &str) -> IResult<&str, Node, JSONParseError> {
    let (remain, raw_float) = float_body(input)?;
    match raw_float.parse::<f64>() {
        Ok(f) => Ok((remain, Node::Float(f))),
        Err(_) => Err(nom::Err::Failure(JSONParseError::BadFloat)),
    }
}

#[test]
fn test_float() {
    assert_eq!(json_float("42.0"), Ok(("", Node::Float(42.0))));
    assert_eq!(json_float("-123.99"), Ok(("", Node::Float(-123.99))));
    assert_eq!(json_float("6.02214086e23"), Ok(("", Node::Float(6.02214086e23))));
    assert_eq!(json_float("-1e6"), Ok(("", Node::Float(-1000000.0))));
    // f64::from_str overflows to infinity instead of throwing an error
    assert_eq!(json_float("1e9999"), Ok(("", Node::Float(f64::INFINITY))));

    // Although there are some literal floats that will return errors,
    // they are considered bugs so we shouldn't expect that behavior forever.
    // See https://github.com/rust-lang/rust/issues/31407
    // assert_eq!(
    //     json_float("2.47032822920623272e-324"),
    //     Err(nom::Err::Failure(utils::JSONParseError::BadFloat))
    // );
}
