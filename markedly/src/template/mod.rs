mod parse;

use std::io::{Read};

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
        let values = parse::template(CompleteStr(text));
        println!("{:?}", values);

        Template {
        }
    }
}
