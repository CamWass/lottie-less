import init, { process_json } from "./wasm/wasm.js";

await init();

window.process_json = process_json;
