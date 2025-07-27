use serde_json::{Number, Value};

pub fn round_numbers(json: &mut Value, precision: u8) {
    match json {
        Value::Number(number) => {
            // Always `Some` when serde_json's arbitrary_precision feature is disabled.
            let num = number.as_f64().unwrap();

            *number = Number::from_f64(to_fixed(num, precision)).unwrap();
        }
        Value::Array(values) => values.iter_mut().for_each(|v| round_numbers(v, precision)),
        Value::Object(map) => map.values_mut().for_each(|v| round_numbers(v, precision)),

        _ => {}
    }
}

fn to_fixed(num: f64, precision: u8) -> f64 {
    let pow = u32::pow(10, precision as u32) as f64;
    f64::round(num * pow) / pow
}
