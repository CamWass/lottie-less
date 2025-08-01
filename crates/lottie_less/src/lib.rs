use serde::ser::Serialize;
use serde_json::Serializer;
use serde_json::Value;

use crate::formatter::CustomFormatter;

mod formatter;
mod passes;

const PRETTY_PRINT: bool = true;
const PRECISION: u8 = 7;
const MINIFY_NUMBERS: bool = true;

pub fn process(input: &str) -> Vec<u8> {
    let mut json: Value = serde_json::from_str(input).expect("failed to parse json");

    passes::remove_names::remove_names(&mut json);

    passes::round_numbers::round_numbers(&mut json, PRECISION);

    print(&json)
}

fn print(value: &Value) -> Vec<u8> {
    let formatter = CustomFormatter::new(PRETTY_PRINT, MINIFY_NUMBERS);

    let mut result = Vec::new();

    let mut serializer = Serializer::with_formatter(&mut result, formatter);

    value
        .serialize(&mut serializer)
        .expect("failed to write json to string");

    result
}
