# dprint-plugin-alejandra

[![npm version](https://img.shields.io/npm/v/@dprint/alejandra.svg)](https://www.npmjs.com/package/@dprint/alejandra) [![CI](https://github.com/dprint/dprint-plugin-alejandra/workflows/CI/badge.svg)](https://github.com/dprint/dprint-plugin-alejandra/actions?query=workflow%3ACI)

Adapter for [Alejandra](https://github.com/kamadorueda/alejandra) for use as a formatting plugin in [dprint](https://github.com/dprint/dprint). It formats [Nix](https://nixos.org) files (`.nix`).

## Install

[Install](https://dprint.dev/install/) and [setup](https://dprint.dev/setup/) dprint.

Then in your project's directory with a dprint.json file, run:

```shellsession
dprint add alejandra
```

Note: You do not need Alejandra installed globally as dprint will run Alejandra from the .wasm file in a sandboxed environment.

## Configuration

To add configuration, specify an `"alejandra"` key in your dprint.json:

```jsonc
{
  "alejandra": {
    "indentWidth": 4,
    "spaceAroundBrackets": true,
  },
  "plugins": [
    // ...etc...
  ],
}
```

The options and their defaults match [Alejandra's](https://github.com/kamadorueda/alejandra/blob/main/alejandra.toml), with dprint's names for the common ones:

| dprint                | Alejandra                                    | Default |
| --------------------- | -------------------------------------------- | ------- |
| `useTabs`             | `indentation` (`true` is `"Tabs"`)           | `false` |
| `indentWidth`         | `indentation` (`"TwoSpaces"`/`"FourSpaces"`) | `2`     |
| `newLineKind`         | none                                         | `"lf"`  |
| `spaceAroundBrackets` | `space_around_brackets`                      | `false` |

Alejandra only indents with two spaces, four spaces or tabs, so `indentWidth` can only be `2` or `4`. Since the defaults are Alejandra's, the global `useTabs`, `indentWidth` and `newLineKind` in your dprint.json are used when they're set (a global `indentWidth` other than `2` or `4` is ignored) and Alejandra's defaults otherwise. Alejandra marks `indentation` and `space_around_brackets` as experimental.

The plugin doesn't read `alejandra.toml` files, so move that configuration into dprint.json. Range formatting isn't supported.

For an overview of the config, see https://github.com/dprint/dprint-plugin-alejandra/blob/main/deployment/schema.json

## JS Formatting API

- [JS Formatter](https://github.com/dprint/js-formatter) - Browser/Deno and Node
- [npm package](https://www.npmjs.com/package/@dprint/alejandra)

## Versioning

This repo automatically upgrades to the latest release of Alejandra once a day. You can check which version of Alejandra is being used by looking at the `alejandra` entry in the Cargo.toml file in this repo:

https://github.com/dprint/dprint-plugin-alejandra/blob/main/Cargo.toml

At the moment, the version of this plugin does not reflect the version of Alejandra.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. Alejandra is released into the public domain under [the Unlicense](https://github.com/kamadorueda/alejandra/blob/main/UNLICENSE).
