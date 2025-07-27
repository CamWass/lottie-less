use std::env;
use std::fs;

use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file_name = args[1].as_str();
    let output_file_name = args[2].as_str();

    let file = fs::read_to_string(input_file_name).expect("failed to read file");

    let mut json: Value = serde_json::from_str(&file).expect("failed to parse json");

    remove_names(&mut json);

    let result = json.to_string();

    fs::write(output_file_name, &result).expect("failed to write file");
}

fn remove_names(json: &mut Value) {
    match json {
        Value::Array(values) => {
            for value in values {
                remove_names(value);
            }
        }
        Value::Object(map) => {
            map.retain(|k, v| {
                if *k == "nm" {
                    debug_assert!(matches!(v, Value::String(_)));
                    return false;
                }
                remove_names(v);
                return true;
            });
        }

        _ => {}
    }
}
