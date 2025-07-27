use std::env;
use std::fs;

use serde::ser::Serialize;
use serde_json::Serializer;
use serde_json::Value;

use crate::formatter::CustomFormatter;

mod formatter;
mod passes;

const PRETTY_PRINT: bool = true;
const PRECISION: u8 = 7;
const MINIFY_NUMBERS: bool = true;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file_name = args[1].as_str();
    let output_file_name = args[2].as_str();

    let file = fs::read_to_string(input_file_name).expect("failed to read file");

    let mut json: Value = serde_json::from_str(&file).expect("failed to parse json");

    passes::remove_names::remove_names(&mut json);

    passes::round_numbers::round_numbers(&mut json, PRECISION);

    let result = print(&json);

    fs::write(output_file_name, &result).expect("failed to write file");
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
