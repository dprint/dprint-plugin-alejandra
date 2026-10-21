// @ts-check
const assert = require("assert");
const createFromBuffer = require("@dprint/formatter").createFromBuffer;
const getBuffer = require("./index").getBuffer;

const formatter = createFromBuffer(getBuffer());
const result = formatter.formatText({
  filePath: "file.nix",
  fileText: "{a=1;b=[1 2];}",
});

assert.strictEqual(result, "{\n  a = 1;\n  b = [1 2];\n}\n");
