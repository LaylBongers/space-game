#![allow(dead_code)]

#[macro_use]
extern crate nom;

use std::io::{Read};
use nom::{alphanumeric};
use nom::types::{CompleteStr};

pub struct Template {
}

impl Template {
    pub fn from_reader<R: Read>(mut reader: R) -> Self {
        let mut text = String::new();
        reader.read_to_string(&mut text).unwrap();
        Self::from_str(&text)
    }

    pub fn from_str(text: &str) -> Self {
        Template {
        }
    }
}

named!(value<CompleteStr, (CompleteStr, CompleteStr)>,
    do_parse!(
key:    call!(alphanumeric) >>
        char!('=') >>
val:    call!(alphanumeric) >>
        eof!() >>
        (key, val)
    )
);

#[test]
fn do_thing() {
    let res = value(CompleteStr("Blah=Meh"));
    panic!("{:?}", res);
}
