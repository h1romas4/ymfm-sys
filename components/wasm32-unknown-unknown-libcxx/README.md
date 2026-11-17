# wasm32-unknown-unknown libc++ sysroot

This directory contains a trimmed sysroot for building `ymfm-sys` for `wasm32-unknown-unknown`.

It uses the following project:

> [maximmax345/wasm32-unknown-unknown-libcxx](https://github.com/maximmax345/wasm32-unknown-unknown-libcxx)
>
> This project provides C++ standard library and LibC support for the wasm32-unknown-unknown target, enabling C++/Rust interop through bindgen and cxx.

## Source

The sysroot was taken from the `v0.1.6` release of the fork:

- Repository: <https://github.com/maximmaxim345/wasm32-unknown-unknown-libcxx>
- Release: <https://github.com/maximmaxim345/wasm32-unknown-unknown-libcxx/releases/tag/v0.1.6>
- Source commit: `a35a38c853a7de7b8838845bdadf69d1483c7e94`
- Release archive: <https://github.com/maximmaxim345/wasm32-unknown-unknown-libcxx/releases/download/v0.1.6/wasm32-unknown-unknown-libcxx.tar.gz>
- SHA-256: `7d8802f2d5485ea6103fb3daeef4114de9764faddf403ef2d7a406d0823106e5`

Only the headers and static libraries needed by this crate were copied from
the release archive. Unrelated OpenBSD headers, module files, and packaging
files were omitted to keep the crate smaller.

## Contents

- `include/c++/v1/`: libc++ headers
- `include/`, `include/machine/`, `include/sys/`: C and target system headers
- `lib/libstdc++.a`: the fork's installed libc++ archive (renamed by its
  install procedure)
- `lib/libc++abi.a`, `lib/libunwind.a`, `lib/libc.a`,
  `lib/libcppexception.a`
- `lib/generic/libclang_rt.builtins-wasm32.a`
- `licenses/`: license texts shipped with the upstream release

These are prebuilt files for `wasm32-unknown-unknown`; they are not host
libraries. Cargo build-script and linker configuration must select this
sysroot when building for that target.
