use serde_json::Value;

pub fn remove_names(json: &mut Value) {
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
