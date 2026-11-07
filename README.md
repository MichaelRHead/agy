# agy — `hello` in Rust

A lightweight command-line greeting utility written in Rust (2024 edition).

## Overview

`hello` prints a friendly greeting to standard output:
- Running `hello` with no arguments outputs `Hello, world!`.
- Passing one or more arguments greets the specified name (e.g., `hello Antigravity` outputs `Hello, Antigravity!`).
- Passing `-h` or `--help` prints usage instructions.

## Usage

### Run with Cargo

```bash
# Default greeting
cargo run
# Output: Hello, world!

# Custom greeting
cargo run -- Rustaceans
# Output: Hello, Rustaceans!

# Help message
cargo run -- --help
```

### Build and Test

```bash
# Compile a release binary
cargo build --release

# Run the unit test suite
cargo test
```

## Project Structure

- `Cargo.toml` — Package manifest configuring the `hello` binary crate (Rust 2024 edition).
- `src/main.rs` — CLI entrypoint, `greet()` formatting function, and unit tests.
- `.gitignore` — Excludes Cargo's `/target` build directory from version control.

## How This Was Created

1. **Environment & Repository Inspection**: Checked the existing `MichaelRHead/agy` Git repository status and verified the installed Rust toolchain (`rustc` and `cargo` 1.99.1).
2. **Project Initialization**: Ran `cargo init --name hello .` inside the repository root to scaffold the `Cargo.toml` manifest, `.gitignore`, and `src/main.rs`.
3. **Implementation & Testing**: Expanded `src/main.rs` with a reusable `greet()` function, command-line argument parsing (supporting custom names and `--help`), and unit tests covering default, custom, and empty/whitespace inputs.
4. **Verification**: Built and ran the program and executed `cargo test` to verify all unit tests pass.
5. **GitHub Backup**: Committed the Cargo manifest, source code, lockfile, `.gitignore`, and this `README.md`, then pushed the changes to `origin/main` on GitHub.

