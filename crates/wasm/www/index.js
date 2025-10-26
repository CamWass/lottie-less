import init, { process_json, WasmConfig } from "./wasm/wasm.js";

await init();

const input = document.getElementById("input");
const output = document.getElementById("output");

input.value = `{
  "foo": 1,
  "nm": "banana",
  "bar": "2",
  "nested": {
    "nm": "apple"
  },
  "array": [{ "nm": "pear" }],
  "roundMe": 1.23456789,
  "minifyMe": 1000000000000
}
`;

const configInputs = {
  pretty_print: {
    label: "Pretty print",
    type: "boolean",
    value: true,
  },
  precision: {
    label: "Precision",
    type: "range",
    min: 0,
    max: 7,
    value: 5,
    enabled: false,
  },
  minify_numbers: {
    label: "Minify numbers",
    type: "boolean",
    value: false,
  },
};

const configContainer = document.getElementById("config");

for (const [key, value] of Object.entries(configInputs)) {
  switch (value.type) {
    case "boolean": {
      const container = document.createElement("div");
      container.className = "config-input-container";

      const label = document.createElement("label");
      label.htmlFor = key;
      label.textContent = value.label;
      container.append(label);

      const input = document.createElement("input");
      input.type = "checkbox";
      input.id = key;
      input.checked = value.value;
      input.addEventListener("input", () => {
        value.value = input.checked;
        run();
      });
      container.append(input);

      configContainer.append(container);

      break;
    }
    case "range": {
      const container = document.createElement("div");
      container.className = "config-toggleable-input-container";

      const enableToggleContainer = document.createElement("div");
      enableToggleContainer.className = "config-input-container";

      container.append(enableToggleContainer);

      const enableToggleLabel = document.createElement("label");
      enableToggleLabel.htmlFor = `${key}_enable`;
      enableToggleLabel.textContent = value.label;
      enableToggleContainer.append(enableToggleLabel);

      const input = document.createElement("input");
      input.type = "range";
      input.min = value.min;
      input.max = value.max;
      input.value = `${value.value}`;
      input.style.display = value.enabled ? "flex" : "none";
      input.addEventListener("input", () => {
        value.value = Number(input.value);
        run();
      });

      const enableToggle = document.createElement("input");
      enableToggle.type = "checkbox";
      enableToggle.id = `${key}_enable`;
      enableToggle.checked = value.enabled;
      enableToggle.addEventListener("input", () => {
        value.enabled = enableToggle.checked;
        value.value = enableToggle.checked ? value.value : undefined;
        input.style.display = enableToggle.checked ? "flex" : "none";
        run();
      });
      enableToggleContainer.append(enableToggle);

      container.append(input);

      configContainer.append(container);

      break;
    }
  }
}

function run() {
  try {
    const wasmConfig = new WasmConfig(
      ...Object.values(configInputs).map((v) => {
        const enabled = v.enabled === undefined || v.enabled === true;
        return enabled ? v.value : undefined;
      })
    );

    output.value = process_json(input.value, wasmConfig);
  } catch (e) {
    console.error(e);
    output.value = "Invalid input";
  }
}

input.addEventListener("input", () => {
  run();
});

run();
