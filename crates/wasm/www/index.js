import init, { process_json, WasmConfig } from "./wasm/wasm.js";

await init();

const input = document.getElementById("input");
const output = document.getElementById("output");

input.addEventListener("input", () => {
  try {
    const config = new WasmConfig(false, 1, true);
    output.value = process_json(input.value, config);
  } catch (e) {
    console.error(e);
    output.value = "Invalid input";
  }
});
