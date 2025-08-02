use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_json(input: &str) -> String {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let result = lottie_less::process(input);

    String::from_utf8(result).expect("invalid string")
}
