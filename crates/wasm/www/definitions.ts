export const DEFAULT_INPUT_VALUE = `{
    "foo": 1,
    "nm": "banana",
    "bar": "2",
    "nested": {
      "nm": "apple"
    },
    "array": [{ "nm": "pear" }],
    "roundMe": 0.1234567899999,
    "minifyUs": [
      1000000000000,
      12300000,
      4095000,
      0.000001,
      -0.000001
    ]
  }
  `;

interface Input {
  label: string;
  type: string;
  value: unknown;
}

/** An input that can be hidden/show with a toggle, enabling/disabling it. */
interface ToggleableInput<T> extends Input {
  value: T | undefined;
  enabled: boolean;
}

interface BooleanInput extends Input {
  type: "boolean";
  value: boolean;
}

interface RangeInput extends ToggleableInput<number> {
  type: "range";
  min: number;
  max: number;
}

type ConfigInput = BooleanInput | RangeInput;

// Order of keys should match order of the arguments to the config class
// constructor.
export const configInputs: Record<string, ConfigInput> = {
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
    value: true,
  },
};
