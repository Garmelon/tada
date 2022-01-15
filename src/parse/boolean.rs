use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

pub fn bool(input: &str) -> IResult<&str, bool> {
    alt((map(tag("true"), |_| true), map(tag("false"), |_| false)))(input)
}
