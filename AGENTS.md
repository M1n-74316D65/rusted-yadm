# Agent Guidelines for rusted-yadm

## Build/Lint/Test Commands
- Build: `cargo build`
- Run: `cargo run`
- Test: `cargo test`
- Lint: `cargo clippy`
- Format: `cargo fmt`
- Single test: `cargo test <test_name>`

## Code Style Guidelines
- Use Rust 2021 edition
- Follow standard Rust naming conventions (snake_case for functions/vars, PascalCase for types)
- Use `Result<T, E>` for error handling, avoid `unwrap()` in production code
- Prefer `?` operator for error propagation
- Use `clap` for CLI argument parsing with derive macros
- Import modules with `use crate::module_name`
- Use descriptive variable names and add doc comments for public functions
- Handle SSH authentication with git2 credentials callbacks
- Use `dirs` crate for cross-platform directory operations