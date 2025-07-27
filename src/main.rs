use std::env;
use std::fs;

use serde_json::Value;

pub mod passes;

const PRETTY_PRINT: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file_name = args[1].as_str();
    let output_file_name = args[2].as_str();

    let file = fs::read_to_string(input_file_name).expect("failed to read file");

    let mut json: Value = serde_json::from_str(&file).expect("failed to parse json");

    passes::remove_names::remove_names(&mut json);

    let result = if PRETTY_PRINT {
        serde_json::to_string_pretty(&json).expect("failed to write json to string")
    } else {
        json.to_string()
    };

    fs::write(output_file_name, &result).expect("failed to write file");
}
