const assert = require("node:assert/strict");
const path = require("node:path");

const modulePath = path.resolve(process.argv[2]);
const api = require(modulePath);
delete require.cache[modulePath];
const otherApi = require(modulePath);

// wasm-bindgen --debug exposes the processed Wasm exports as __wasm.
const rawReference = api.__wasm.ymfm_create(0, 2_000_000);
assert.equal(typeof rawReference, "object");
assert.notEqual(rawReference, null);
assert.equal(api.__wasm.ymfm_reset(rawReference), 0);
assert.equal(api.ymfm_destroy(rawReference), 0);

const first = api.ymfm_create(0, 2_000_000); // YM2149
const second = api.ymfm_create(0, 2_000_000);
const other = otherApi.ymfm_create(0, 2_000_000);
assert.equal(typeof first, "object");
assert.notEqual(first, null);
assert.notEqual(first, second);
assert.equal(otherApi.ymfm_reset(first), -1);
assert.equal(api.ymfm_reset(other), -1);
assert.equal(otherApi.ymfm_destroy(other), 0);
assert.equal(api.ymfm_create(24, 2_000_000), null);
assert.equal(api.ymfm_create(0, 0), null);
assert.equal(api.ymfm_reset(first), 0);
assert.equal(api.ymfm_reset(second), 0);
assert.equal(api.ymfm_reset({}), -1);
assert.equal(api.ymfm_reset(null), -1);
assert.equal(api.ymfm_reset(undefined), -1);
assert.equal(api.ymfm_reset(1), -1);
assert.equal(api.ymfm_write(first, 0, 7), 0);
assert.equal(api.ymfm_write(first, 1, 0), 0);
assert.equal(api.ymfm_write(first, 2, 0x3f), 0);
assert.equal(api.ymfm_write(second, 0, 7), 0);
assert.equal(api.ymfm_write(second, 1, 0), 0);
assert.equal(api.ymfm_read(first, 3), 0x3f);
assert.equal(api.ymfm_read(second, 3), 0);

const channels = api.ymfm_channels(first);
assert.ok(channels > 0);
assert.ok(api.ymfm_sample_rate(first) > 0n);
const frames = 128;
const address = api.ymfm_generate(first, frames);
assert.notEqual(address, 0);
const samples = new Int32Array(api.ymfm_memory().buffer, address, frames * channels);
assert.equal(samples.length, frames * channels);
assert.equal(api.ymfm_generate(first, 0), 0);
assert.equal(api.ymfm_generate(first, 65_537), 0);

assert.equal(api.ymfm_destroy(first), 0);
assert.equal(api.ymfm_destroy(first), -1);
assert.equal(api.ymfm_reset(first), -1);
assert.equal(api.ymfm_write(first, 0, 0), -1);
assert.equal(api.ymfm_read(first, 0), -1);
assert.equal(api.ymfm_channels(first), -1);
assert.equal(api.ymfm_sample_rate(first), -1n);
assert.equal(api.ymfm_generate(first, 1), 0);
assert.equal(api.ymfm_read(second, 3), 0);
const third = api.ymfm_create(9, 7_670_000); // YM2612
assert.notEqual(third, null);
assert.notEqual(third, first);
assert.equal(api.ymfm_reset(third), 0);
for (const [reg, value] of [
    [0x30, 1], [0x34, 1], [0x38, 1], [0x3c, 1],
    [0x40, 0], [0x44, 0], [0x48, 0], [0x4c, 0],
    [0x50, 0x1f], [0x54, 0x1f], [0x58, 0x1f], [0x5c, 0x1f],
    [0x60, 0], [0x64, 0], [0x68, 0], [0x6c, 0],
    [0x70, 0], [0x74, 0], [0x78, 0], [0x7c, 0],
    [0x80, 0x0f], [0x84, 0x0f], [0x88, 0x0f], [0x8c, 0x0f],
    [0xb0, 7], [0xa4, 0x22], [0xa0, 0x69], [0x28, 0xf0],
]) {
    assert.equal(api.ymfm_write(third, 0, reg), 0);
    assert.equal(api.ymfm_write(third, 1, value), 0);
}
const stereoChannels = api.ymfm_channels(third);
const audioAddress = api.ymfm_generate(third, 200);
assert.notEqual(audioAddress, 0);
const audio = new Int32Array(api.ymfm_memory().buffer, audioAddress, 200 * stereoChannels);
assert.ok(audio.some((sample) => sample !== 0));
assert.equal(api.ymfm_destroy(second), 0);
assert.equal(api.ymfm_destroy(third), 0);
for (let chipType = 0; chipType < 24; chipType++) {
    const handle = api.ymfm_create(chipType, 8_000_000);
    assert.notEqual(handle, null);
    assert.equal(api.ymfm_reset(handle), 0);
    assert.ok(api.ymfm_channels(handle) > 0);
    assert.notEqual(api.ymfm_generate(handle, 8), 0);
    assert.equal(api.ymfm_destroy(handle), 0);
}
console.log("Wasm externref example passed");
