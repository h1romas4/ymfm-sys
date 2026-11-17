const fs = require("node:fs");
const assert = require("node:assert/strict");

const moduleBytes = fs.readFileSync(process.argv[2]);
const wasm = new WebAssembly.Module(moduleBytes);
assert.equal(new WebAssembly.Instance(wasm, {}).exports.main(), 0);
for (const name of ["test_abort_traps", "test_assertion_traps"]) {
    const instance = new WebAssembly.Instance(wasm, {});
    assert.throws(() => instance.exports[name](), WebAssembly.RuntimeError);
}
console.log("Wasm runtime smoke test passed");
