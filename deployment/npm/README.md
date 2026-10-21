# @dprint/alejandra

npm distribution of [dprint-plugin-alejandra](https://github.com/dprint/dprint-plugin-alejandra) which is an adapter plugin for [Alejandra](https://github.com/kamadorueda/alejandra).

Use this with [@dprint/formatter](https://github.com/dprint/js-formatter) or just use @dprint/formatter and download the [dprint-plugin-alejandra Wasm file](https://github.com/dprint/dprint-plugin-alejandra/releases).

## Example

```ts
import { getBuffer } from "@dprint/alejandra";
import { createFromBuffer } from "@dprint/formatter";

const formatter = createFromBuffer(getBuffer());

console.log(formatter.formatText({
  filePath: "file.nix",
  fileText: "{a=1;}",
}));
```
