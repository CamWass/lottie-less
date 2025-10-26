use std::env;
use std::fs;

use lottie_less::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file_name = args[1].as_str();
    let output_file_name = args[2].as_str();

    let file = fs::read_to_string(input_file_name).expect("failed to read file");

    let result = lottie_less::process(&file, Config::default());

    fs::write(output_file_name, &result).expect("failed to write file");
}
