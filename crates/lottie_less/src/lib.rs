use serde::ser::Serialize;
use serde_json::Serializer;
use serde_json::Value;

use crate::formatter::CustomFormatter;

mod formatter;
mod passes;

const DEFAULT_PRETTY_PRINT: bool = true;
const DEFAULT_PRECISION: u8 = 7;
const DEFAULT_MINIFY_NUMBERS: bool = true;

#[derive(Copy, Clone, Default)]
pub struct Config {
    pub pretty_print: Option<bool>,
    pub precision: Option<u8>,
    pub minify_numbers: Option<bool>,
}

pub fn process(input: &str, config: Config) -> Vec<u8> {
    let pretty_print = config.pretty_print.unwrap_or(DEFAULT_PRETTY_PRINT);
    let precision = config.precision.unwrap_or(DEFAULT_PRECISION);
    let minify_numbers = config.minify_numbers.unwrap_or(DEFAULT_MINIFY_NUMBERS);

    let mut json: Value = serde_json::from_str(input).expect("failed to parse json");

    passes::remove_names::remove_names(&mut json);

    passes::round_numbers::round_numbers(&mut json, precision);

    print(&json, pretty_print, minify_numbers)
}

fn print(value: &Value, pretty_print: bool, minify_numbers: bool) -> Vec<u8> {
    let formatter = CustomFormatter::new(pretty_print, minify_numbers);

    let mut result = Vec::new();

    let mut serializer = Serializer::with_formatter(&mut result, formatter);

    value
        .serialize(&mut serializer)
        .expect("failed to write json to string");

    result
}
