# Changelog

## [Unreleased]

- [ ] Support the `wasm32-wasip2` (Component Model) target.

## [0.3.0]

- Support `wasm32-unknown-unknown` with a bundled libc++ sysroot and runtime.
- Add cross-platform Wasm CI and an `externref` example.

## [0.2.0] - 2026-08-15

- CI: expand cross-platform and `wasm32-wasip1` build verification, including version-tag and manual workflow runs.
- Docs: update README and rustdoc with clearer build, badge, and usage instructions.
- Packaging: stop tracking `Cargo.lock` so dependency resolution follows library users' projects.

## [0.1.0] - 2026-08-12

Initial release.
