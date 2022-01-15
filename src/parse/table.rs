use std::collections::HashMap;

use nom::branch::alt;
use nom::character::complete::{char, multispace0};
use nom::combinator::map;
use nom::sequence::{delimited, pair, tuple};
use nom::IResult;

use super::boolean::bool;
use super::integer::integer;

enum Value {
    Bool(bool),
    Int(i64),
    Table(Table),
}

type Table = HashMap<Value, Value>;

fn value(input: &str) -> IResult<&str, Value> {
    alt((
        map(bool, Value::Bool),
        map(integer, Value::Int),
        map(table, Value::Table),
    ))(input)
}

fn table(input: &str) -> IResult<&str, Table> {
    map(
        delimited(
            pair(multispace0, char('(')),
            char('*'),
            pair(multispace0, char(')')),
        ),
        |_| HashMap::new(),
    )(input)
}
