import init, { process_json } from "./wasm/wasm.js";

await init();

const input = document.getElementById("input");
const output = document.getElementById("output");

input.addEventListener("input", () => {
  try {
    output.value = process_json(input.value);
  } catch {
    output.value = "Invalid input";
  }
});
