# agy — `hello` in Rust 🦃🍂

A lightweight command-line greeting utility written in Rust (2024 edition) with festive **November & Thanksgiving** features!

## Overview

`hello` prints a friendly greeting to standard output and automatically detects when it's running in **November**:
- Running `hello` with no arguments greets `Hello, world!`.
- Passing one or more arguments greets the specified name (e.g., `hello Antigravity` outputs `Hello, Antigravity!`).
- **November & Thanksgiving Mode**:
  - Automatically calculates the exact date of US Thanksgiving (the 4th Thursday of November) for the current year using zero external dependencies.
  - Displays an ASCII art turkey (`🦃 Gobble gobble!`) delivering your greeting.
  - Shows a live countdown to Thanksgiving Day (or celebrates on Thanksgiving Day, or recommends leftover sandwiches afterward).
  - Recommends a rotating **Featured Feast Dish of the Day**.
- Supports `-t` / `--turkey`, `-f` / `--feast`, `--date YYYY-MM-DD` (to time-travel and test any date), and `-h` / `--help`.

## Usage

### Run with Cargo

```bash
# Default greeting (automatically festive in November!)
cargo run

# Custom greeting
cargo run -- Rustaceans

# Time-travel to Thanksgiving Day 2026
cargo run -- --date 2026-11-26 Pilgrim

# Force turkey & feast output on any date
cargo run -- --turkey --feast

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
- `src/main.rs` — CLI entrypoint, `greet()`, pure-Rust calendar/Thanksgiving date calculation (`date_from_unix_secs`, `thanksgiving_day_of_november`), ASCII turkey generator, and unit tests.
- `.gitignore` — Excludes Cargo's `/target` build directory from version control.

## How This Was Created

1. **Environment & Repository Inspection**: Checked the existing `MichaelRHead/agy` Git repository status and verified the installed Rust toolchain (`rustc` and `cargo` 1.99.1).
2. **Project Initialization**: Ran `cargo init --name hello .` inside the repository root to scaffold the `Cargo.toml` manifest, `.gitignore`, and `src/main.rs`.
3. **Implementation & Seasonal Features**:
   - Built `greet()` and CLI option parsing (`--help`, `--turkey`, `--feast`, `--date YYYY-MM-DD`).
   - Implemented pure-Rust civil calendar conversion from `SystemTime` and Sakamoto's day-of-week algorithm to pinpoint the 4th Thursday of November for any year.
   - Added automatic November detection with a Thanksgiving countdown, rotating Feast Dish of the Day, and an ASCII art turkey.
4. **Verification**: Built and ran the program and executed `cargo test` to verify all unit tests pass.
5. **GitHub Backup**: Committed the Cargo manifest, source code, lockfile, `.gitignore`, and this `README.md`, then pushed the changes to `origin/main` on GitHub.


