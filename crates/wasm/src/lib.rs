use lottie_less::Config;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct WasmConfig {
    pretty_print: Option<bool>,
    precision: Option<u8>,
    minify_numbers: Option<bool>,
}

#[wasm_bindgen]
impl WasmConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(
        pretty_print: Option<bool>,
        precision: Option<u8>,
        minify_numbers: Option<bool>,
    ) -> WasmConfig {
        WasmConfig {
            pretty_print,
            precision,
            minify_numbers,
        }
    }
}

impl From<WasmConfig> for Config {
    fn from(value: WasmConfig) -> Self {
        Config {
            pretty_print: value.pretty_print,
            precision: value.precision,
            minify_numbers: value.minify_numbers,
        }
    }
}

#[wasm_bindgen]
pub fn process_json(input: &str, config: WasmConfig) -> String {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let result = lottie_less::process(input, Config::from(config));

    String::from_utf8(result).expect("invalid string")
}
