use nom::{IResult, alphanumeric, line_ending};
use nom::types::{CompleteStr};

#[derive(Debug)]
pub enum Value {
    Value(String),
    List(String, Vec<Value>),
}

#[inline(always)]
pub fn template(input: CompleteStr) -> IResult<CompleteStr, Vec<Value>> {
    terminated!(input,
        map!(
            opt!(apply!(lists_values, 0)),
            |v| v.unwrap_or(Vec::new())
        ),
        eof!()
    )
}

#[inline(always)]
fn lists_values(input: CompleteStr, indent_level: usize) -> IResult<CompleteStr, Vec<Value>> {
    many1!(input,
        terminated!(
            alt!(
                complete!(apply!(value_list, indent_level)) |
                apply!(value_value, indent_level)
            ),
            many0!(line_ending)
        )
    )
}

#[inline(always)]
fn value_list(input: CompleteStr, indent_level: usize) -> IResult<CompleteStr, Value> {
    do_parse!(input,
name:   apply!(value, indent_level) >>
list:   apply!(lists_values, indent_level+1) >>
        ( Value::List(name, list) )
    )
}

#[inline(always)]
fn value_value(input: CompleteStr, indent_level: usize) -> IResult<CompleteStr, Value> {
    map!(input,
        apply!(value, indent_level),
        |s| Value::Value(s)
    )
}

#[inline(always)]
fn value(input: CompleteStr, indent_level: usize) -> IResult<CompleteStr, String> {
    delimited!(input,
        count!(indent_any, indent_level),
        map!(alphanumeric, |s| { s.0.to_string() }),
        line_ending
    )
}

named!(indent_any<CompleteStr, CompleteStr>,
    alt_complete!(
        tag!("    ")
      | tag!("\t")
    )
);
