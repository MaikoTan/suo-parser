# Contributing to suo-parser

Thank you for considering contributing to suo-parser! Contributions are welcome and greatly appreciated. This guide will help you get started.

## Tools Required

Before contributing, ensure you have the following tools installed:

- **Rust**: Install Rust using [rustup](https://rustup.rs/).
- **Cargo**: Comes with Rust installation.
- **cargo-insta**: Install it using the command:
  ```sh
  cargo install cargo-insta
  ```

## Code Style

- Always format your code using `rustfmt`. You can run it with:
  ```sh
  cargo fmt
  ```
- Lint your code with `clippy` to ensure it adheres to best practices:
  ```sh
  cargo clippy
  ```

## Testing

- To test Rust code, use:
  ```sh
  cargo test
  ```

## Pull Requests and Commit Messages

- Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for commit messages. Ensure your commit messages are clear and descriptive.
- When creating a pull request, ensure it is well-documented and linked to any relevant issues.

## Project Notes

- The parser is written in Rust and published to crates.io as `suo-parser-core`.
- WASM bindings (for Node/browser) live in the `wasm/` crate and are built with `wasm-pack`:
  ```sh
  wasm-pack build wasm --target nodejs
  ```
- The npm package `suo-parser` bundles the WASM build plus a `bin/suo.js` CLI entry.
  Build it (outputs to `pkg/`) with:
  ```sh
  npm run build
  ```
  After `npm publish`, users can run `npx suo-parser timeline.txt`.
- A native CLI binary is also available from `core/src/bin/suo.rs` for users who prefer a
  standalone executable:
  ```sh
  cargo build --release --bin suo
  ```
