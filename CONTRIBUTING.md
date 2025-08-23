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
- To test JavaScript code in the `binding/` folder, navigate to the folder and run:
  ```sh
  yarn test
  ```

## Pull Requests and Commit Messages

- Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for commit messages. Ensure your commit messages are clear and descriptive.
- When creating a pull request, ensure it is well-documented and linked to any relevant issues.

## Project Notes

- This project includes [**NAPI-RS**](https://napi.rs/), a framework for building native Node.js modules in Rust. Familiarity with NAPI-RS is helpful when working on bindings.
