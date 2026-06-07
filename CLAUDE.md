# csv++ Development Guidelines

## Release Flow

- Bump the root crate version in `Cargo.toml` and update `Cargo.lock`.
- Run `make builddev` before publishing. It runs docs, clippy, fmt check, and tests.
- Use `make publish` from the repository root to publish and create the signed annotated git tag.
  - The tag name comes from the root `Cargo.toml` version, e.g. `v0.9.0`.
  - The target runs `cargo publish`, `git tag -as v${version} -m "${version}"`, then `make -C csvp/ publish`.
- Push the release tag with `git push origin --tags`.
- Pushing the tag triggers the GitHub Actions `Release` workflow, which builds installers and release archives with `cargo-dist`.
- The root `make publish` target does not bump versions automatically.

## No Panics Policy

This codebase must never panic in production. All fallible operations must return a structured error via `Result`.

**Never use:**
- `.unwrap()` — use `?` or explicit error mapping
- `.expect(...)` — same as above
- `panic!(...)` — return an `Err` instead
- `unreachable!(...)` — return an `Err` or use an exhaustive match
- `todo!()` / `unimplemented!()` — don't leave unimplemented stubs in committed code
- direct indexing `arr[i]` on slices/vecs — use `.get(i)` and handle `None`

These are enforced programmatically via clippy lints in `src/lib.rs` and `csvp/src/lib.rs`:

```rust
#![warn(
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::unreachable,
    clippy::unwrap_used,
)]
```

> **Goal:** promote all of these from `warn` to `deny` once existing violations are resolved.

Test code may use `.unwrap()` where appropriate (a test panic is a test failure, not a production crash). Add `#[allow(clippy::unwrap_used)]` to test modules when needed.
